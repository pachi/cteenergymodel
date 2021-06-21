// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Conversión desde CtehexmlData a cte::Model

use std::{collections::BTreeMap, convert::TryFrom, convert::TryInto};

use anyhow::{anyhow, bail, format_err, Error};
use log::warn;
use na::{Point2, Point3, Rotation3, Translation3, Vector3};

use crate::utils::{fround2, fround3, normalize, orientation_bdl_to_52016, uuid_from_obj};
use hulc::{
    bdl::{self, Data},
    ctehexml,
};

pub use super::{
    BoundaryType, Geometry, Meta, Model, Orientation, Shade, Space, SpaceType, ThermalBridge,
    ThermalBridgeKind, Tilt, Wall, WallCons, Window, WindowCons, WindowGeometry,
};

// Conversiones de BDL a tipos CTE -------------------

impl From<bdl::BoundaryType> for BoundaryType {
    fn from(boundary: bdl::BoundaryType) -> Self {
        match boundary {
            bdl::BoundaryType::EXTERIOR => Self::EXTERIOR,
            bdl::BoundaryType::INTERIOR => Self::INTERIOR,
            bdl::BoundaryType::GROUND => Self::GROUND,
            bdl::BoundaryType::ADIABATIC => Self::ADIABATIC,
        }
    }
}

impl TryFrom<&ctehexml::CtehexmlData> for Model {
    type Error = Error;
    fn try_from(d: &ctehexml::CtehexmlData) -> Result<Self, Self::Error> {
        let bdl = &d.bdldata;
        let mut windows = vec![];
        let mut shades = vec![];

        let mut walls = walls_from_bdl(&bdl)?;
        let (wins, winshades) = windows_from_bdl(&walls, &bdl);
        windows.extend_from_slice(&wins);
        shades.extend_from_slice(&winshades);
        let othershades = shades_from_bdl(&bdl);
        shades.extend_from_slice(&othershades);
        let thermal_bridges = thermal_bridges_from_bdl(&bdl);
        let wallcons = wallcons_from_bdl(&walls, &bdl)?;
        let wincons = windowcons_from_bdl(&bdl)?;
        let spaces = spaces_from_bdl(&bdl)?;

        // Cambia referencias a nombres por id's
        let spaceids = spaces
            .iter()
            .map(|s| (s.name.clone(), s.id.clone()))
            .collect::<BTreeMap<String, String>>();
        let wallids = walls
            .iter()
            .map(|s| (s.name.clone(), s.id.clone()))
            .collect::<BTreeMap<String, String>>();
        let wallconsids = wallcons
            .iter()
            .map(|s| (s.name.clone(), s.id.clone()))
            .collect::<BTreeMap<String, String>>();
        let winconsids = wincons
            .iter()
            .map(|s| (s.name.clone(), s.id.clone()))
            .collect::<BTreeMap<String, String>>();

        for mut w in &mut walls {
            w.cons = wallconsids.get(&w.cons).unwrap().to_owned();
            w.space = spaceids.get(&w.space).unwrap().to_owned();
            if let Some(ref nxt) = w.nextto {
                w.nextto = match spaceids.get(nxt) {
                    None => {
                        bail!(
                            "ERROR: No se localiza el espacio adyacente {} en el muro {}.",
                            nxt,
                            w.name
                        )
                    }
                    Some(id) => Some(id.clone()),
                };
            };
        }
        windows.iter_mut().for_each(|w| {
            w.cons = winconsids.get(&w.cons).unwrap().to_owned();
            w.wall = wallids.get(&w.wall).unwrap().to_owned();
        });

        // Completa metadatos desde ctehexml y el bdl
        // Desviación general respecto al Norte (criterio BDL)
        let mut d_perim_insulation = 0.0;
        let mut rn_perim_insulation = 0.0;
        if let Some(buildparams) = bdl.meta.get("BUILD-PARAMETERS") {
            d_perim_insulation = buildparams
                .attrs
                .get_f32("D-AISLAMIENTO-PERIMETRAL")
                .unwrap_or(0.0);
            rn_perim_insulation = buildparams
                .attrs
                .get_f32("RA-AISLAMIENTO-PERIMETRAL")
                .unwrap_or(0.0);
        };

        let dg = &d.datos_generales;
        let is_dwelling =
            ["Unifamiliar", "Bloque", "UnaBloque"].contains(&dg.tipo_vivienda.as_str());

        let meta = Meta {
            name: dg.nombre_proyecto.clone(),
            is_new_building: dg.tipo_definicion.as_str() == "Nuevo",
            is_dwelling,
            num_dwellings: dg.num_viviendas_bloque,
            climate: dg
                .archivo_climatico
                .as_str()
                .try_into()
                .map_err(|e| anyhow!("ERROR: {}", e))?,
            global_ventilation_l_s: if is_dwelling {
                Some(dg.valor_impulsion_aire)
            } else {
                None
            },
            n50_test_ach: dg.valor_n50_medido,
            d_perim_insulation,
            rn_perim_insulation,
        };

        Ok(Model {
            meta,
            walls,
            windows,
            thermal_bridges,
            shades,
            spaces,
            wincons,
            wallcons,
            extra: None,
        })
    }
}

/// Construye diccionario de espacios a partir de datos BDL (Data)
fn spaces_from_bdl(bdl: &Data) -> Result<Vec<Space>, Error> {
    bdl.spaces
        .iter()
        .map(|s| {
            let id = uuid_from_obj(&s);
            let area = fround2(s.area());
            let height = fround2(s.height);
            let exposed_perimeter = Some(fround2(s.exposed_perimeter(&bdl)));
            Ok(Space {
                id,
                name: s.name.clone(),
                area,
                z: s.z,
                exposed_perimeter,
                height,
                inside_tenv: s.insidete,
                multiplier: s.multiplier * s.floor_multiplier,
                space_type: match s.stype.as_ref() {
                    "CONDITIONED" => SpaceType::CONDITIONED,
                    "UNHABITED" => SpaceType::UNINHABITED,
                    _ => SpaceType::UNCONDITIONED,
                },
                n_v: s.airchanges_h,
            })
        })
        .collect::<Result<Vec<Space>, Error>>()
}

/// Construye geometría de muro
///
/// La posición se calcula en coordenadas globales, teniendo en cuenta las coordenadas de espacio y las desviaciones global y de espacio.
///
/// El polígono 3D del muro se obtiene a partir de los datos de muro y del espacio
/// Para cada nivel, primero se gira el azimuth y luego se desplaza x, y, z
fn wall_geometry(wall: &hulc::bdl::Wall, bdl: &Data) -> Geometry {
    let space = bdl.spaces.iter().find(|s| s.name == wall.space).unwrap();
    let space_polygon = &space.polygon;
    let global_deviation = global_deviation_from_north(bdl);

    // Calculamos la posición en coordenadas globales, teniendo en cuenta las posiciones y desviaciones
    // La posición del muro es en coordenadas de espacio == coordenadas de planta == (coordenadas de edificio salvo por Z)
    // Los ángulos los cambiamos a radianes y de sentido horario (criterio BDL) a antihorario (-).
    let angle = -(space.angle_with_building_north + global_deviation).to_radians();
    let rot = na::Rotation3::from_euler_angles(0.0, 0.0, angle);
    let position = rot
        * match wall.location.as_deref() {
            Some(loc) if loc != "TOP" && loc != "BOTTOM" => {
                let [p1, _] = space.polygon.edge_vertices(loc).unwrap();
                Point3::new(
                    p1.x + wall.x + space.x,
                    p1.y + wall.y + space.y,
                    wall.z + space.z,
                )
            }
            _ => {
                let height = match wall.location.as_deref() {
                    // Los elementos definidos por polígono (sin ser el de espacio) ya tiene en la Z la cota final
                    Some("TOP") if wall.polygon.is_none() => space.height,
                    _ => 0.0,
                };
                Point3::new(
                    wall.x + space.x,
                    wall.y + space.y,
                    wall.z + space.z + height,
                )
            }
        };

    // Solamente en el caso de elemntos TOP y BOTTOM de espacio estamos haciendo el giro... deberíamos ver si al resto le hace falta o no
    let polygon = match (wall.location.as_deref(), &wall.polygon) {
        (None, Some(ref polygon)) => polygon.as_vec(),
        (Some("TOP"), Some(ref polygon)) => polygon.as_vec(),
        // En los elementos TOP no necesitamos hacer el tilt, ya que es cero
        (Some("TOP"), None) => {
            let p = bdl::Polygon(space_polygon.as_vec());
            let azimuth = orientation_bdl_to_52016(
                space.angle_with_building_north + wall.angle_with_space_north,
            );
            p.rotate(azimuth.to_radians()).as_vec()
        }
        // Con elementos de suelo hacemos el mirror (y -> -y para cada punto) del polígono sobre el eje X para que al girarlo con el tilt 180 quede igual
        // El azimuth global ya está incluido
        (Some("BOTTOM"), _) => {
            let azimuth = orientation_bdl_to_52016(
                space.angle_with_building_north + wall.angle_with_space_north,
            );
            space_polygon
                .rotate(azimuth.to_radians())
                .mirror_y()
                .as_vec()
        }
        (Some(vertex), _) => {
            // Definimos el polígono con inicio en 0,0 y ancho y alto según vértices y espacio
            // La "position (x, y, z)" que define el origen de coordenadas del muro será la del primer vértice
            // Pero se calcula fuera de esta función
            let [p1, p2] = space_polygon.edge_vertices(vertex).unwrap();
            let width = (p2 - p1).magnitude();
            let height = space.height;
            vec![
                Point2::new(0.0, 0.0),
                Point2::new(width, 0.0),
                Point2::new(width, height),
                Point2::new(0.0, height),
            ]
        }
        _ => {
            panic!("Definición de polígono de muro {} desconocida", wall.name)
        }
    };

    let tilt = fround2(wall.tilt);
    let azimuth = fround2(orientation_bdl_to_52016(
        global_deviation + space.angle_with_building_north + wall.angle_with_space_north,
    ));
    Geometry {
        azimuth,
        tilt,
        position: Some(position),
        polygon,
    }
}

/// Construye muros de la envolvente a partir de datos BDL
// Convertimos la posición del muro a coordenadas globales y el polígono está en coordenadas de muro
fn walls_from_bdl(bdl: &Data) -> Result<Vec<Wall>, Error> {
    bdl.walls
        .iter()
        .map(|wall| -> Result<Wall, Error> {
            let id = uuid_from_obj(wall);
            let bounds = wall.bounds.into();
            let geometry = wall_geometry(&wall, bdl);
            Ok(Wall {
                id,
                name: wall.name.clone(),
                cons: wall.cons.to_string(),
                area: fround2(wall.net_area(bdl)?),
                space: wall.space.clone(),
                nextto: wall.nextto.clone(),
                bounds,
                geometry,
            })
        })
        .collect::<Result<Vec<Wall>, _>>()
}

/// Desviacion global del edificio respecto al norte
/// Sigue la misma referencia al Norte que el azimuth, pero un criterio de signos distinto: N=0, E = -90, O=90.
fn global_deviation_from_north(bdl: &Data) -> f32 {
    bdl.meta
        .get("BUILD-PARAMETERS")
        .map(|params| params.attrs.get_f32("AZIMUTH").unwrap_or_default())
        .unwrap_or_default()
}

/// Construye huecos de la envolvente a partir de datos BDL
fn windows_from_bdl(walls: &[Wall], bdl: &Data) -> (Vec<Window>, Vec<Shade>) {
    //TODO: faltan por crear las sombras de retranqueo (superior y laterales)
    //TODO: falta por trasladar la definición de lamas (louvres)
    let mut windows = vec![];
    let mut shades = vec![];

    for win in &bdl.windows {
        let id = uuid_from_obj(win);
        let wall = walls.iter().find(|w| w.name == win.wall).unwrap();
        // TODO: Calcular fshobst teniendo en cuenta algo más que el setback
        let fshobst = fshobst_for_setback(
            wall.geometry.tilt,
            wall.geometry.azimuth,
            win.width,
            win.height,
            win.setback,
        );
        let geometry = WindowGeometry {
            position: Some(Point2::new(win.x, win.y)),
            width: win.width,
            height: win.height,
            setback: win.setback,
        };

        windows.push(Window {
            id,
            name: win.name.clone(),
            cons: win.cons.to_string(),
            wall: win.wall.clone(),
            area: fround2(win.width * win.height),
            fshobst: fround2(fshobst),
            geometry,
        });

        if win.overhang.is_some() || win.left_fin.is_some() || win.right_fin.is_some() {
            let wall2world =
                Rotation3::from_axis_angle(&Vector3::z_axis(), wall.geometry.azimuth.to_radians())
                    * Rotation3::from_axis_angle(
                        &Vector3::x_axis(),
                        wall.geometry.tilt.to_radians(),
                    );
            let wallpos = wall
                .geometry
                .position
                .unwrap_or_else(|| Point3::new(0.0, 0.0, 0.0));

            // Alero sobre el hueco
            if let Some(overhang) = &win.overhang {
                let pos = wall2world
                    * Point3::new(win.x - overhang.a, win.y + win.height + overhang.b, 0.0);
                let position = Some(Point3::new(
                    pos.x + wallpos.x,
                    pos.y + wallpos.y,
                    pos.z + wallpos.z,
                ));
                let polygon = vec![
                    Point2::new(0.0, 0.0),
                    Point2::new(0.0, -overhang.depth),
                    Point2::new(overhang.width, -overhang.depth),
                    Point2::new(overhang.width, 0.0),
                ];

                shades.push(Shade {
                    id: uuid_from_obj(overhang),
                    name: format!("{}_overhang", win.name),
                    geometry: Geometry {
                        // inclinación: overhang.angle (0 es paralelo al hueco y 90 es perpendicular al hueco)
                        tilt: wall.geometry.tilt - overhang.angle,
                        azimuth: wall.geometry.azimuth,
                        position,
                        polygon,
                    },
                })
            };

            // Aleta izquierda
            if let Some(lfin) = &win.left_fin {
                let pos =
                    wall2world * Point3::new(win.x - lfin.a, win.y + win.height - lfin.b, 0.0);
                let position = Some(Point3::new(
                    pos.x + wallpos.x,
                    pos.y + wallpos.y,
                    pos.z + wallpos.z,
                ));
                let polygon = vec![
                    Point2::new(0.0, 0.0),
                    Point2::new(0.0, -lfin.height),
                    Point2::new(lfin.depth, -lfin.height),
                    Point2::new(lfin.depth, 0.0),
                ];

                shades.push(Shade {
                    id: uuid_from_obj(lfin),
                    name: format!("{}_left_fin", win.name),
                    geometry: Geometry {
                        tilt: wall.geometry.tilt,
                        azimuth: wall.geometry.azimuth - 90.0,
                        position,
                        polygon,
                    },
                })
            }

            // Aleta derecha
            if let Some(rfin) = &win.right_fin {
                let pos = wall2world
                    * Point3::new(win.x + win.width + rfin.a, win.y + win.height - rfin.b, 0.0);
                let position = Some(Point3::new(
                    pos.x + wallpos.x,
                    pos.y + wallpos.y,
                    pos.z + wallpos.z,
                ));
                let polygon = vec![
                    Point2::new(0.0, 0.0),
                    Point2::new(0.0, -rfin.height),
                    Point2::new(rfin.depth, -rfin.height),
                    Point2::new(rfin.depth, 0.0),
                ];

                shades.push(Shade {
                    id: uuid_from_obj(rfin),
                    name: format!("{}_right_fin", win.name),
                    geometry: Geometry {
                        tilt: wall.geometry.tilt,
                        azimuth: wall.geometry.azimuth - 90.0,
                        position,
                        polygon,
                    },
                })
            }
        }
    }
    (windows, shades)
}

/// Construye puentes térmicos de la envolvente a partir de datos BDL
fn thermal_bridges_from_bdl(bdl: &Data) -> Vec<ThermalBridge> {
    bdl.tbridges
        .iter()
        .filter(|tb| tb.name != "LONGITUDES_CALCULADAS")
        .map(|tb| {
            use ThermalBridgeKind::*;
            let id = uuid_from_obj(tb);
            let kind = match tb.name.as_str() {
                "UNION_CUBIERTA" => ROOF,
                "ESQUINA_CONVEXA_FORJADO" => ROOF,
                "ESQUINA_CONCAVA" => CORNER,
                "ESQUINA_CONVEXA" => CORNER,
                "ESQUINA_CONCAVA_CERRAMIENTO" => CORNER,
                "ESQUINA_CONVEXA_CERRAMIENTO" => CORNER,
                "FRENTE_FORJADO" => INTERMEDIATEFLOOR,
                "PILAR" => PILLAR,
                "UNION_SOLERA_PAREDEXT" => GROUNDFLOOR,
                "HUECO_VENTANA" => WINDOW,
                "HUECO_ALFEIZAR" => WINDOW,
                "HUECO_CAPIALZADO" => WINDOW,
                "HUECO_JAMBA" => WINDOW,
                _ => GENERIC,
            };
            ThermalBridge {
                id,
                name: tb.name.clone(),
                kind,
                l: fround2(tb.length.unwrap_or(0.0)),
                psi: tb.psi,
            }
        })
        .collect()
}

/// Construye sombras del edificio partir de datos BDL
/// Hay dos tipos de sombra:
/// - BUILDING-SHADE, que son relativas al edificio (giran y se desplazan con el edificio)
/// - FIXED-SHADE, que no giran ni se desplazan (coordenadas globales)
/// Las BUILDING-SHADE, además, se pueden definir:
/// - por geometría, con X, Y, Z, WIDTH, HEIGHT
/// - por vértices
/// Ver BDL Topics p.158
/// Convertimos todos los casos a geometría como la de los muros: position + tilt + azimuth + Pol2D
fn shades_from_bdl(bdl: &Data) -> Vec<Shade> {
    bdl.shadings
        .iter()
        .filter_map(|sh| {
            let id = uuid_from_obj(sh);
            let name = sh.name.clone();
            let global_deviation = global_deviation_from_north(bdl);
            let (position, tilt, azimuth, polygon) = if let Some(geom) = sh.geometry.as_ref() {
                // 1. Sombras definidas por posición, ancho y alto
                // Sombras de área nula
                if geom.height.abs() < 1e-3 && geom.height.abs() < 1e-3 {
                    return None;
                };
                // El origen simplemente se traslada la desviación global (en sentido inverso a los ángulos en coordenadas (X,-Y))
                let position = Some(
                    Rotation3::from_axis_angle(&Vector3::z_axis(), -global_deviation.to_radians())
                        * Point3::new(geom.x, geom.y, geom.z),
                );
                // El azimuth acumula la orientación de la sombra y la desviación del norte (tienen el mismo criterio de giro)
                let azimuth = fround2(orientation_bdl_to_52016(geom.azimuth + global_deviation));
                let polygon = vec![
                    Point2::new(0.0, 0.0),
                    Point2::new(geom.width, 0.0),
                    Point2::new(geom.width, geom.height),
                    Point2::new(0.0, geom.height),
                ];

                (position, geom.tilt, azimuth, polygon)
            } else if let Some(vertices) = sh.vertices.as_ref() {
                // 2. Sombras definidas por vértices
                // Aquí tenemos que tener cuidado con las operaciones de giros ya que tienen criterios de medición distintos
                let normal = (vertices[1] - vertices[0]).cross(&(vertices[2] - vertices[1]));
                if normal.magnitude() < 10.0 * f32::EPSILON {
                    // XXX: Esto se podría evitar iterando hasta encontrar dos segmentos que no sean colineales
                    // https://community.khronos.org/t/how-to-calculate-polygon-normal/49265/3
                    panic!("Polígono con puntos colineales");
                };
                let tilt = Vector3::z_axis().angle(&normal);
                // Azimuth del elemento de sombra (¡Atención! Criterio EN S=0, E=+90, W=-90)
                let shade_azimuth = if (tilt % std::f32::consts::PI).abs() > (10.0 * f32::EPSILON) {
                    // No es una superficie horizontal y calculamos el azimuth (con el Sur) como el ángulo de -Y y la proyección horizontal de la normal
                    na::Rotation2::rotation_between(&-Vector3::<f32>::y_axis().xy(), &normal.xy())
                        .angle()
                } else {
                    // Es una superficie horizontal y el azimuth (con el Sur) se calcula como si estuviese vertical la superficie -> -Y -> +Z
                    // XXX: Esto no lo tengo claro...
                    Vector3::z_axis().angle(&normal)
                };

                // La desviación global gira en sentido negativo el origen (sentido horario)
                let v0 = vertices[0];
                let position = Some(
                    Rotation3::from_axis_angle(&Vector3::z_axis(), -global_deviation.to_radians())
                        * v0,
                );

                // El giro global produce un giro en sentido negativo (sentido horario) frente al azimuth de la sombra (antihorario)
                let azimuth = fround2(normalize(
                    shade_azimuth.to_degrees() - global_deviation,
                    -180.0,
                    180.0,
                ));

                // Trasladamos al primer vértice y luego deshacemos la inclinación / tilt (giro en x) y luego el azimut de la sombra (giro eje z)
                // El azimuth derivado de la desviación global la transmitimos en el valor final de azimuth y la hemos incorporado en la posición
                // así que no debemos descontarla aquí de la geometría de la sombra
                let transform = Rotation3::from_axis_angle(&Vector3::x_axis(), -tilt)
                    * Rotation3::from_axis_angle(&Vector3::z_axis(), -shade_azimuth)
                    * Translation3::from(Point3::origin() - v0);
                let polygon = vertices.iter().map(|p| (transform * p).xy()).collect();
                (
                    position,
                    normalize(tilt.to_degrees(), 0.0, 360.0),
                    azimuth,
                    polygon,
                )
            } else {
                panic!("Definición inesperada de elemento de sombra");
            };

            Some(Shade {
                id,
                name,
                geometry: Geometry {
                    tilt,
                    azimuth,
                    position,
                    polygon,
                },
            })
        })
        .collect()
}

/// Construcciones de muros a partir de datos BDL
fn wallcons_from_bdl(walls: &[Wall], bdl: &Data) -> Result<Vec<WallCons>, Error> {
    let mut wcnames = walls
        .iter()
        .map(|w| w.cons.clone())
        .collect::<Vec<String>>();
    wcnames.sort();
    wcnames.dedup();

    wcnames
        .iter()
        .map(|wcons| {
            let wallcons = bdl
                .db
                .wallcons
                .get(wcons)
                .and_then(|cons|{
                    let id = uuid_from_obj(wcons);
                    match cons.r_intrinsic(&bdl.db.materials) {
                        Ok(r) => Some(WallCons {
                            id,
                            name: cons.name.clone(),
                            group: cons.group.clone(),
                            thickness: fround2(cons.total_thickness()),
                            r_intrinsic: fround3(r),
                            absorptance: cons.absorptance,
                        }),
                        _ => {
                            warn!(
                                "ERROR: No es posible calcular la R intrínseca de la construcción: {:?}\n",
                                cons,
                            );
                            None
                        }
                }})
                .ok_or_else(|| {
                    format_err!(
                        "Construcción de muro no encontrada o incorrecta: '{}'\n",
                        wcons,
                    )
                })?;
            Ok(wallcons)
        })
        .collect::<Result<Vec<_>, _>>()
}

/// Construcciones de huecos a partir de datos BDL
fn windowcons_from_bdl(bdl: &Data) -> Result<Vec<WindowCons>, Error> {
    let mut wcnames: Vec<String> = bdl
        .windows
        .iter()
        .map(|w| w.cons.clone())
        .collect::<Vec<String>>();
    wcnames.sort();
    wcnames.dedup();

    wcnames
        .iter()
        .map(|wcons| {
            bdl.db
                .windowcons
                .get(wcons)
                .and_then(|cons| {
                    let id = uuid_from_obj(cons);
                    // Vidrio del hueco (Glass)
                    let glass = match bdl
                        .db
                        .glasses
                        .get(&cons.glass)
                        .ok_or_else(|| format_err!("Vidrio no encontrado: {}", cons.glass,))
                    {
                        Ok(glass) => glass,
                        _ => return None,
                    };
                    let ff = cons.framefrac;
                    let gglwi = fround2(glass.g_gln * 0.90);
                    let gglshwi = cons.gglshwi.unwrap_or(gglwi);
                    let infcoeff_100 = cons.infcoeff;
                    let u = fround2(cons.u(&bdl.db.frames, &bdl.db.glasses).unwrap_or_default());
                    Some(WindowCons {
                        id,
                        name: cons.name.clone(),
                        group: cons.group.clone(),
                        u,
                        ff,
                        gglwi,
                        gglshwi,
                        infcoeff_100,
                    })
                })
                .ok_or_else(|| {
                    format_err!(
                        "Construcción de hueco no encontrada o mal formada: {}",
                        &wcons,
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()
}

/// Factor de obstáculos remotos (Fshobst) en función del retranqueo, orientación y geometría del hueco
/// Se calcula, para huecos verticales, de acuerdo a la tabla 17 del DA DB-HE/1 (p. 19).
/// Es un cálculo best-effort. Podríamos mejorarlo implementando la 52016-1 pero lo puede personalizar el usuario luego
pub fn fshobst_for_setback(tilt: f32, azimuth: f32, width: f32, height: f32, setback: f32) -> f32 {
    use Orientation::*;
    use Tilt::*;

    // Calcular según orientación e inclinación
    let rh = setback / height;
    let rw = setback / width;
    match tilt.into() {
        // Elementos verticales - Tabla 17 del DA DB-HE/1 (p.19)
        SIDE => {
            let range_rh = if rh < 0.05 {
                0
            } else if rh <= 0.1 {
                1
            } else if rh <= 0.2 {
                2
            } else if rh <= 0.5 {
                3
            } else {
                4
            };
            let range_rw = if rw < 0.05 {
                0
            } else if rw <= 0.1 {
                1
            } else if rw <= 0.2 {
                2
            } else if rw <= 0.5 {
                3
            } else {
                4
            };
            match azimuth.into() {
                S => match (range_rh, range_rw) {
                    (1, 1) => 0.82,
                    (1, 2) => 0.74,
                    (1, 3) => 0.62,
                    (1, 4) => 0.39,
                    (2, 1) => 0.76,
                    (2, 2) => 0.67,
                    (2, 3) => 0.56,
                    (2, 4) => 0.35,
                    (3, 1) => 0.56,
                    (3, 2) => 0.51,
                    (3, 3) => 0.39,
                    (3, 4) => 0.27,
                    (4, 1) => 0.35,
                    (4, 2) => 0.32,
                    (4, 3) => 0.27,
                    (4, 4) => 0.17,
                    _ => 1.0,
                },
                SE | SW => match (range_rh, range_rw) {
                    (1, 1) => 0.86,
                    (1, 2) => 0.81,
                    (1, 3) => 0.72,
                    (1, 4) => 0.51,
                    (2, 1) => 0.79,
                    (2, 2) => 0.74,
                    (2, 3) => 0.66,
                    (2, 4) => 0.47,
                    (3, 1) => 0.59,
                    (3, 2) => 0.56,
                    (3, 3) => 0.47,
                    (3, 4) => 0.36,
                    (4, 1) => 0.38,
                    (4, 2) => 0.36,
                    (4, 3) => 0.32,
                    (4, 4) => 0.23,
                    _ => 1.0,
                },
                E | W => match (range_rh, range_rw) {
                    (1, 1) => 0.91,
                    (1, 2) => 0.87,
                    (1, 3) => 0.81,
                    (1, 4) => 0.65,
                    (2, 1) => 0.86,
                    (2, 2) => 0.82,
                    (2, 3) => 0.76,
                    (2, 4) => 0.61,
                    (3, 1) => 0.71,
                    (3, 2) => 0.68,
                    (3, 3) => 0.61,
                    (3, 4) => 0.51,
                    (4, 1) => 0.53,
                    (4, 2) => 0.51,
                    (4, 3) => 0.48,
                    (4, 4) => 0.39,
                    _ => 1.0,
                },
                _ => 1.0,
            }
        }
        TOP => {
            // Elementos horizontales: tabla 19 DA DB-HE/1 p.19
            let range_rh = if rh <= 0.1 {
                0
            } else if rh <= 0.5 {
                1
            } else if rh <= 1.0 {
                2
            } else if rh <= 2.0 {
                3
            } else if rh <= 5.0 {
                4
            } else {
                5
            };
            let range_rw = if rw <= 0.1 {
                0
            } else if rw <= 0.5 {
                1
            } else if rw <= 1.0 {
                2
            } else if rw <= 2.0 {
                3
            } else if rw <= 5.0 {
                4
            } else {
                5
            };
            let rmin = i32::min(range_rh, range_rw);
            let rmax = i32::max(range_rh, range_rw);
            match (rmax, rmin) {
                (0, 0) => 0.42,
                (1, 0) => 0.43,
                (1, 1) => 0.46,
                (2, 0) => 0.43,
                (2, 1) => 0.48,
                (2, 2) => 0.52,
                (3, 0) => 0.43,
                (3, 1) => 0.50,
                (3, 2) => 0.55,
                (3, 3) => 0.60,
                (4, 0) => 0.44,
                (4, 1) => 0.51,
                (4, 2) => 0.58,
                (4, 3) => 0.66,
                (4, 4) => 0.75,
                (5, 0) => 0.44,
                (5, 1) => 0.52,
                (5, 2) => 0.59,
                (5, 3) => 0.68,
                (5, 4) => 0.79,
                _ => 0.85,
            }
        }
        BOTTOM => 1.0,
    }
}
