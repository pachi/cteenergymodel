// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Conversión desde CtehexmlData a cte::Model

use std::{
    collections::{BTreeMap, HashSet},
    convert::TryFrom,
    convert::TryInto,
};

use anyhow::{anyhow, format_err, Error};
use nalgebra::{point, Point3, Rotation2, Rotation3, Translation3, Vector3};

use crate::utils::{fround2, normalize, uuid_from_obj};
use hulc::{
    bdl::{self, Data},
    ctehexml,
};

pub use crate::{
    BoundaryType, ConsDb, Frame, Glass, Layer, MatProps, Material, MatsDb, Meta, Model,
    Orientation, Shade, Space, SpaceType, ThermalBridge, ThermalBridgeKind, Tilt, Uuid, Wall,
    WallCons, WallGeometry, Window, WinCons, WindowGeometry,
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
        let id_maps = IdMaps::new(bdl);

        let mut mats = mats_from_bdl(bdl, &id_maps);
        let spaces = spaces_from_bdl(bdl, &id_maps)?;
        let walls = walls_from_bdl(bdl, &id_maps)?;
        let (windows, shades) = windows_and_shades_from_bdl(bdl, &walls, &id_maps);
        let thermal_bridges = thermal_bridges_from_bdl(bdl);
        let wallcons = wallcons_from_bdl(bdl, &mut mats, &id_maps)?;
        let wincons = windowcons_from_bdl(bdl, &mut mats)?;

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
            cons: ConsDb { wincons, wallcons },
            mats,
            extra: None,
        };
        model.update_fshobst();
        Ok(model)
    }
}

/// Construye diccionario de espacios a partir de datos BDL (Data)
fn spaces_from_bdl(bdl: &Data, id_maps: &IdMaps) -> Result<Vec<Space>, Error> {
    bdl.spaces
        .iter()
        .map(|s| {
            Ok(Space {
                id: *id_maps.space_id(&s.name)?,
                name: s.name.clone(),
                area: fround2(s.polygon.area()),
                z: s.z,
                height: fround2(s.height),
                inside_tenv: s.insidete,
                multiplier: s.multiplier * s.floor_multiplier,
                kind: match s.stype.as_ref() {
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

    WallGeometry {
        azimuth: fround2(orientation_bdl_to_52016(
            global_deviation + space.angle_with_building_north + wall.angle_with_space_north,
        )),
        tilt: fround2(wall.tilt),
        position: Some(position),
        polygon,
    }
}

/// Construye muros de la envolvente a partir de datos BDL
// Convertimos la posición del muro a coordenadas globales y el polígono está en coordenadas de muro
fn walls_from_bdl(bdl: &Data, id_maps: &IdMaps) -> Result<Vec<Wall>, Error> {
    bdl.walls
        .iter()
        .map(|wall| -> Result<Wall, Error> {
            Ok(Wall {
                id: *id_maps.wall_id(&wall.name)?,
                name: wall.name.clone(),
                cons: *id_maps.wallcons_id(&wall.cons)?,
                space: *id_maps.space_id(&wall.space)?,
                next_to: match wall.nextto.as_ref() {
                    Some(nextto) => Some(*id_maps.space_id(nextto)?),
                    _ => None,
                },
                bounds: wall.bounds.into(),
                geometry: wall_geometry(wall, bdl),
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
fn windows_and_shades_from_bdl(
    bdl: &Data,
    walls: &[Wall],
    id_maps: &IdMaps,
) -> (Vec<Window>, Vec<Shade>) {
    //TODO: falta por trasladar la definición de lamas (louvres)
    let mut windows = vec![];
    let mut shades = vec![];

    for win in &bdl.windows {
        let id = uuid_from_obj(win);
        let wall = walls.iter().find(|w| w.name == win.wall).unwrap();

        // Definición del hueco
        // TODO: eliminar los unwrap()
        let window = Window {
            id,
            name: win.name.clone(),
            cons: *id_maps.wincons_id(&win.cons).unwrap(),
            wall: *id_maps.wall_id(&win.wall).unwrap(),
            // XXX: Usamos un valor por defecto ya que al final se actualiza con model.update_fshobst()
            f_shobst: 1.0,
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

    // Añade sombras independientes
    let othershades = shades_from_bdl(bdl);
    shades.extend_from_slice(&othershades);

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
fn mats_from_bdl(bdl: &Data, id_maps: &IdMaps) -> MatsDb {
    let mut materials = Vec::new();
    for (name, material) in &bdl.db.materials {
        materials.push(Material {
            id: *id_maps.material_id(&material.name).unwrap(),
            name: name.clone(),
            group: material.group.clone(),
            properties: if let Some(p) = material.properties {
                MatProps::Detailed {
                    conductivity: p.conductivity,
                    density: p.density,
                    specific_heat: p.specificheat,
                    vapour_diff: p.vapourdiffusivity,
                }
            } else {
                MatProps::Resistance {
                    resistance: material.resistance.unwrap_or_default(),
                }
            },
        })
    }
    let mut glasses = Vec::new();
    for (name, glass) in &bdl.db.glasses {
        let id = uuid_from_obj(glass);
        glasses.push(Glass {
            id,
            name: name.clone(),
            group: glass.group.clone(),
            u_value: glass.conductivity,
            g_gln: glass.g_gln,
        })
    }
    let mut frames = Vec::new();
    for (name, frame) in &bdl.db.frames {
        let id = uuid_from_obj(frame);
        frames.push(Frame {
            id,
            name: name.clone(),
            group: frame.group.clone(),
            u_value: frame.conductivity,
            absorptivity: frame.absorptivity,
        })
    }
    MatsDb {
        materials,
        glasses,
        frames,
    }
}

/// Construcciones de muros a partir de datos BDL
fn wallcons_from_bdl(
    bdl: &Data,
    mats: &mut MatsDb,
    id_maps: &IdMaps,
) -> Result<Vec<WallCons>, Error> {
    let mut used_wallcons = bdl
        .walls
        .iter()
        .map(|w| w.cons.clone())
        .collect::<Vec<String>>();
    used_wallcons.sort();
    used_wallcons.dedup();

    let mut used_material_ids = HashSet::new();
    let mut wclist = Vec::with_capacity(used_wallcons.len());
    for wcons in &used_wallcons {
        match bdl.db.wallcons.get(wcons) {
            Some(cons) => {
                let mut ids = Vec::with_capacity(cons.material.len());
                for mat_name in &cons.material {
                    let id = id_maps.material_id(mat_name)?;
                    ids.push(id);
                    used_material_ids.insert(id);
                }
                let layers = ids
                    .iter()
                    .cloned()
                    .zip(cons.thickness.iter().cloned())
                    .map(|(id, e)| Layer { id: *id, e })
                    .collect();
                let wallcons = WallCons {
                    id: *id_maps.wallcons_id(wcons)?,
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
    // Purgamos materiales no usados
    mats.materials.retain(|v| used_material_ids.contains(&v.id));

    // Devolvemos lista
    Ok(wclist)
}

/// Construcciones de huecos a partir de datos BDL
fn windowcons_from_bdl(bdl: &Data, mats: &mut MatsDb) -> Result<Vec<WinCons>, Error> {
    let mut wcnames: Vec<String> = bdl
        .windows
        .iter()
        .map(|w| w.cons.clone())
        .collect::<Vec<String>>();
    wcnames.sort();
    wcnames.dedup();

    let glass_name_to_id = mats
        .glasses
        .iter()
        .map(|m| (&m.name, &m.id))
        .collect::<BTreeMap<&String, &Uuid>>();

    let frame_name_to_id = mats
        .frames
        .iter()
        .map(|m| (&m.name, &m.id))
        .collect::<BTreeMap<&String, &Uuid>>();

    let mut used_glasses_ids = HashSet::new();
    let mut used_frames_ids = HashSet::new();

    let mut wcons = Vec::new();
    for wincons in &wcnames {
        let cons = match bdl.db.wincons.get(wincons) {
            Some(cons) => {
                let id = uuid_from_obj(cons);
                // Vidrio del hueco (Glass)
                let glass = *match glass_name_to_id.get(&cons.glass) {
                    Some(id) => *id,
                    _ => return Err(format_err!("Vidrio no encontrado: {}", cons.glass,)),
                };
                used_glasses_ids.insert(glass);

                // Marco del hueco (Frame)
                let frame = *match frame_name_to_id.get(&cons.frame) {
                    Some(id) => *id,
                    _ => return Err(format_err!("Marco no encontrado: {}", cons.frame,)),
                };
                used_frames_ids.insert(frame);
                WinCons {
                    id,
                    name: cons.name.clone(),
                    group: cons.group.clone(),
                    glass,
                    frame,
                    f_f: cons.framefrac,
                    delta_u: cons.deltau,
                    g_glshwi: cons.gglshwi,
                    c_100: cons.infcoeff,
                }
            }
            _ => {
                return Err(format_err!(
                    "Construcción de hueco no encontrada o mal formada: {}",
                    &wincons,
                ))
            }
        };
        wcons.push(cons);
    }

    // Purgamos materiales no usados
    mats.glasses.retain(|v| used_glasses_ids.contains(&v.id));
    mats.frames.retain(|v| used_frames_ids.contains(&v.id));

    Ok(wcons)
}

/// Mapping de nombres a ids
struct IdMaps<'a> {
    spaces: BTreeMap<&'a str, Uuid>,
    walls: BTreeMap<&'a str, Uuid>,
    wallcons: BTreeMap<&'a str, Uuid>,
    wincons: BTreeMap<&'a str, Uuid>,
    materials: BTreeMap<&'a str, Uuid>,
}

impl<'a> IdMaps<'a> {
    /// Localiza id de muro desde nombre
    fn wall_id<T: AsRef<str>>(&self, name: T) -> Result<&Uuid, anyhow::Error> {
        self.walls
            .get(name.as_ref())
            .ok_or_else(|| format_err!("Muro {} no identificado", name.as_ref()))
    }

    /// Localiza id de espacio desde nombre
    fn space_id<T: AsRef<str>>(&self, name: T) -> Result<&Uuid, anyhow::Error> {
        self.spaces
            .get(name.as_ref())
            .ok_or_else(|| format_err!("Espacio {} no identificado", name.as_ref()))
    }

    /// Localiza id de construcción de muro desde nombre
    fn wallcons_id<T: AsRef<str>>(&self, name: T) -> Result<&Uuid, anyhow::Error> {
        self.wallcons
            .get(name.as_ref())
            .ok_or_else(|| format_err!("Construcción de opaco {} no identificada", name.as_ref()))
    }

    /// Localiza id de construcción de hueco desde nombre
    fn wincons_id<T: AsRef<str>>(&self, name: T) -> Result<&Uuid, anyhow::Error> {
        self.wincons
            .get(name.as_ref())
            .ok_or_else(|| format_err!("Construcción de hueco {} no identificada", name.as_ref()))
    }

    /// Localiza id de material de opaco desde nombre
    fn material_id<T: AsRef<str>>(&self, name: T) -> Result<&Uuid, anyhow::Error> {
        self.materials
            .get(name.as_ref())
            .ok_or_else(|| format_err!("Material de opaco {} no identificado", name.as_ref()))
    }

    fn new(bdl: &'a Data) -> Self {
        IdMaps {
            spaces: bdl
                .spaces
                .iter()
                .map(|s| (s.name.as_str(), uuid_from_obj(&s)))
                .collect::<BTreeMap<&str, Uuid>>(),
            walls: bdl
                .walls
                .iter()
                .map(|s| (s.name.as_str(), uuid_from_obj(&s)))
                .collect::<BTreeMap<&str, Uuid>>(),
            wallcons: bdl
                .db
                .wallcons
                .iter()
                .map(|(name, s)| (name.as_str(), uuid_from_obj(&s)))
                .collect::<BTreeMap<&str, Uuid>>(),
            wincons: bdl
                .db
                .wincons
                .iter()
                .map(|(name, s)| (name.as_str(), uuid_from_obj(&s)))
                .collect::<BTreeMap<&str, Uuid>>(),
            materials: bdl
                .db
                .materials
                .iter()
                .map(|(name, s)| (name.as_str(), uuid_from_obj(&s)))
                .collect::<BTreeMap<&str, Uuid>>(),
        }
    }
}
