// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Conversión desde CtehexmlData a cte::Model

use std::{
    collections::{BTreeMap, HashSet},
    convert::TryFrom,
    convert::TryInto,
};

use anyhow::{anyhow, bail, format_err, Error};
use nalgebra::{point, Point3, Rotation2, Rotation3, Translation3, Vector3};

use crate::utils::{fround2, normalize, uuid_from_obj};
use hulc::{
    bdl::{self, Data},
    ctehexml,
};

pub use crate::{
    BoundaryType, Db, Layer, MatProps, Material, Meta, Model, Orientation, Shade, Space, SpaceType,
    ThermalBridge, ThermalBridgeKind, Tilt, Uuid, Wall, WallCons, WallGeometry, Window, WindowCons,
    WindowGeometry,
};

// Utilidades varias de conversión

/// Normaliza aziimuth [-180, 180]
#[inline]
pub fn normalize_azimuth(azimuth: f32) -> f32 {
    normalize(azimuth, -180.0, 180.0)
}

/// Convierte el azimuth desde el criterio del BDL al criterio de la 52016-1 y normaliza
/// BDL: Ángulo entre el eje Y del espacio y la proyección horizontal de la normal exterior del muro (N=0, E=+90, W=-90)
/// UNE-EN ISO 52016-1: S=0, E=+90, W=-90
pub fn orientation_bdl_to_52016(azimuth: f32) -> f32 {
    normalize_azimuth(180.0 - azimuth)
}

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

        let mut walls = walls_from_bdl(bdl)?;
        let (wins, winshades) = windows_from_bdl(&walls, bdl);
        windows.extend_from_slice(&wins);
        shades.extend_from_slice(&winshades);
        let othershades = shades_from_bdl(bdl);
        shades.extend_from_slice(&othershades);
        let thermal_bridges = thermal_bridges_from_bdl(bdl);
        let mut materials = materials_from_bdl(bdl);
        let (wallcons, used_material_ids) = wallcons_from_bdl(&walls, &materials, bdl)?;
        let wincons = windowcons_from_bdl(bdl)?;
        let spaces = spaces_from_bdl(bdl)?;

        // Purgamos materiales no usados
        materials.retain(|v| used_material_ids.contains(&v.id));

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

        let mut model = Model {
            meta,
            walls,
            windows,
            thermal_bridges,
            shades,
            spaces,
            wincons,
            wallcons,
            db: Db { materials },
            extra: None,
        };
        model.update_fshobst();
        Ok(model)
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
            let exposed_perimeter = Some(fround2(s.exposed_perimeter(bdl)));
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
fn wall_geometry(wall: &hulc::bdl::Wall, bdl: &Data) -> WallGeometry {
    let space = bdl.spaces.iter().find(|s| s.name == wall.space).unwrap();
    let space_polygon = &space.polygon;
    let global_deviation = global_deviation_from_north(bdl);

    // Calculamos la posición en coordenadas globales, teniendo en cuenta las posiciones y desviaciones
    // La posición del muro es en coordenadas globales, incluyendo un giro en Z según desviación global del norte y la desviación del espacio
    // Los ángulos los cambiamos a radianes y de sentido horario (criterio BDL) a antihorario (-).
    let angle = -(space.angle_with_building_north + global_deviation).to_radians();
    let rot = Rotation3::from_euler_angles(0.0, 0.0, angle);
    let position = rot
        * match wall.location.as_deref() {
            // 1. Casos definidos por vértice
            Some(loc) if loc != "TOP" && loc != "BOTTOM" => {
                let [p1, _] = space.polygon.edge_vertices(loc).unwrap();
                point![
                    p1.x + wall.x + space.x,
                    p1.y + wall.y + space.y,
                    wall.z + space.z
                ]
            }
            // 2. Casos definidos mediante polígono o por el espacio
            _ => {
                let height = match wall.location.as_deref() {
                    // Los elementos top definidos por el polígono del espacio necesitan añadir la altura en su z
                    Some("TOP") if wall.polygon.is_none() => space.height,
                    // El resto de los definidos por polígono (sin ser el de espacio) ya tienen en la Z la cota final
                    _ => 0.0,
                };
                point![
                    wall.x + space.x,
                    wall.y + space.y,
                    wall.z + space.z + height
                ]
            }
        };

    let polygon = match (wall.location.as_deref(), &wall.polygon) {
        // 1. Elementos definidos por polígono
        (None, Some(ref polygon)) => polygon.as_vec(),
        // 2. Elementos TOP definidos por polígono
        (Some("TOP"), Some(ref polygon)) => polygon.as_vec(),
        // 3. Elementos TOP definidos por la geometría de su espacio
        (Some("TOP"), None) => {
            // Giramos el polígono según la desviación respecto al norte del muro y el espacio
            // El giro global del edificio respecto al norte ya está incluido
            let azimuth = orientation_bdl_to_52016(
                space.angle_with_building_north + wall.angle_with_space_north,
            );
            space_polygon.rotate(azimuth.to_radians()).as_vec()
        }
        // 4. Elementos BOTTOM definidos por la geometría de su espacio
        (Some("BOTTOM"), None) => {
            // Giramos el polígono según la desviación respecto al norte del muro y el espacio
            // El giro global del edificio respecto al norte ya está incluido
            let azimuth = orientation_bdl_to_52016(
                space.angle_with_building_north + wall.angle_with_space_north,
            );
            // Hacemos un mirror (y -> -y para cada punto) sobre el eje X para que el giro del tilt 180 lo deje igual
            space_polygon
                .rotate(azimuth.to_radians())
                .mirror_y()
                .as_vec()
        }
        // 5. Elementos definidos por un vértice del espacio
        (Some(vertex), _) => {
            // Definimos el polígono con inicio en 0,0 y ancho y alto según vértices y espacio
            // La "position (x, y, z)" que define el origen de coordenadas del muro será la del primer vértice
            // Pero se calcula fuera de esta función
            let [p1, p2] = space_polygon.edge_vertices(vertex).unwrap();
            let width = (p2 - p1).magnitude();
            let height = space.height;
            vec![
                point![0.0, 0.0],
                point![width, 0.0],
                point![width, height],
                point![0.0, height],
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
    WallGeometry {
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
            let geometry = wall_geometry(wall, bdl);
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
    //TODO: falta por trasladar la definición de lamas (louvres)
    let mut windows = vec![];
    let mut shades = vec![];

    for win in &bdl.windows {
        let id = uuid_from_obj(win);
        let wall = walls.iter().find(|w| w.name == win.wall).unwrap();

        // Definición del hueco
        let window = Window {
            id,
            name: win.name.clone(),
            cons: win.cons.to_string(),
            wall: win.wall.clone(),
            area: fround2(win.width * win.height),
            // XXX: Usamos un valor por defecto ya que al final se actualiza con model.update_fshobst()
            fshobst: 1.0,
            geometry: WindowGeometry {
                position: Some(point![win.x, win.y]),
                width: win.width,
                height: win.height,
                setback: win.setback,
            },
        };

        // Sombras de contorno de huecos
        // shades.extend(crate::geometry::setback_shades_for_window(wall, &window));

        windows.push(window);

        // Definición de aleros
        if win.overhang.is_some() || win.left_fin.is_some() || win.right_fin.is_some() {
            let wall2world = wall
                .geometry
                .to_global_coords_matrix()
                .expect("El muro debe tener definición geométrica completa");

            // Alero sobre el hueco
            if let Some(overhang) = &win.overhang {
                let geometry = WallGeometry {
                    // inclinación: overhang.angle (0 es paralelo al hueco y 90 es perpendicular al hueco)
                    tilt: wall.geometry.tilt - overhang.angle,
                    azimuth: wall.geometry.azimuth,
                    position: Some(
                        wall2world
                            * point![win.x - overhang.a, win.y + win.height + overhang.b, 0.0],
                    ),
                    polygon: vec![
                        point![0.0, 0.0],
                        point![0.0, -overhang.depth],
                        point![overhang.width, -overhang.depth],
                        point![overhang.width, 0.0],
                    ],
                };
                shades.push(Shade {
                    id: uuid_from_obj(overhang),
                    name: format!("{}_overhang", win.name),
                    geometry,
                })
            };

            // Aleta izquierda
            if let Some(lfin) = &win.left_fin {
                let geometry = WallGeometry {
                    tilt: wall.geometry.tilt,
                    azimuth: wall.geometry.azimuth - 90.0,
                    position: Some(
                        wall2world * point![win.x - lfin.a, win.y + win.height - lfin.b, 0.0],
                    ),
                    polygon: vec![
                        point![0.0, 0.0],
                        point![0.0, -lfin.height],
                        point![lfin.depth, -lfin.height],
                        point![lfin.depth, 0.0],
                    ],
                };
                shades.push(Shade {
                    id: uuid_from_obj(lfin),
                    name: format!("{}_left_fin", win.name),
                    geometry,
                })
            }

            // Aleta derecha
            if let Some(rfin) = &win.right_fin {
                let geometry = WallGeometry {
                    tilt: wall.geometry.tilt,
                    azimuth: wall.geometry.azimuth - 90.0,
                    position: Some(
                        wall2world
                            * point![win.x + win.width + rfin.a, win.y + win.height - rfin.b, 0.0],
                    ),
                    polygon: vec![
                        point![0.0, 0.0],
                        point![0.0, -rfin.height],
                        point![rfin.depth, -rfin.height],
                        point![rfin.depth, 0.0],
                    ],
                };
                shades.push(Shade {
                    id: uuid_from_obj(rfin),
                    name: format!("{}_right_fin", win.name),
                    geometry,
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
                        * point![geom.x, geom.y, geom.z],
                );
                // El azimuth acumula la orientación de la sombra y la desviación del norte (tienen el mismo criterio de giro)
                let azimuth = fround2(orientation_bdl_to_52016(geom.azimuth + global_deviation));
                let polygon = vec![
                    point![0.0, 0.0],
                    point![geom.width, 0.0],
                    point![geom.width, geom.height],
                    point![0.0, geom.height],
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
                    Rotation2::rotation_between(&-Vector3::<f32>::y_axis().xy(), &normal.xy())
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
                geometry: WallGeometry {
                    tilt,
                    azimuth,
                    position,
                    polygon,
                },
            })
        })
        .collect()
}

/// Materiales partir de datos BDL
fn materials_from_bdl(bdl: &Data) -> Vec<Material> {
    let mut materials = Vec::new();
    for (name, material) in &bdl.db.materials {
        let id = uuid_from_obj(material);
        materials.push(Material {
            id,
            name: name.clone(),
            group: material.group.clone(),
            properties: if let Some(p) = material.properties {
                MatProps::Detailed {
                    conductivity: p.conductivity,
                    density: p.density,
                    specificheat: p.specificheat,
                    vapourdiffusivity: p.vapourdiffusivity,
                }
            } else {
                MatProps::Resistance {
                    resistance: material.resistance.unwrap_or_default(),
                }
            },
        })
    }
    materials
}

/// Construcciones de muros a partir de datos BDL
fn wallcons_from_bdl(
    walls: &[Wall],
    materials: &[Material],
    bdl: &Data,
) -> Result<(Vec<WallCons>, HashSet<Uuid>), Error> {
    let mut wcnames = walls
        .iter()
        .map(|w| w.cons.clone())
        .collect::<Vec<String>>();
    wcnames.sort();
    wcnames.dedup();

    let name_to_id = materials
        .iter()
        .map(|m| (&m.name, &m.id))
        .collect::<BTreeMap<&String, &Uuid>>();

    let mut used_material_ids = HashSet::new();
    let mut wclist = Vec::with_capacity(wcnames.len());
    for wcons in &wcnames {
        match bdl.db.wallcons.get(wcons) {
            Some(cons) => {
                let id = uuid_from_obj(wcons);
                let mut ids = Vec::with_capacity(cons.material.len());
                for mat_name in &cons.material {
                    if let Some(id) = name_to_id.get(mat_name).cloned() {
                        ids.push(id.clone());
                        used_material_ids.insert(id.clone());
                    } else {
                        return Err(format_err!(
                            "ERROR: No se ha encontrado el id del material: {}",
                            mat_name,
                        ));
                    };
                }
                let layers = ids
                    .iter()
                    .cloned()
                    .zip(cons.thickness.iter().cloned())
                    .map(|(id, e)| Layer { id, e })
                    .collect();
                let wallcons = WallCons {
                    id,
                    name: cons.name.clone(),
                    group: cons.group.clone(),
                    layers,
                    absorptance: cons.absorptance,
                };
                wclist.push(wallcons)
            }
            _ => {
                return Err(format_err!(
                    "Construcción de muro no encontrada o incorrecta: '{}'\n",
                    wcons,
                ))
            }
        };
    }
    Ok((wclist, used_material_ids))
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
