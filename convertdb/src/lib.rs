use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use flate2::read::GzDecoder;

use bemodel::{
    utils::uuid_from_obj, ConsDb, Frame, Glass, Groups, Layer, Library, MatProps, Material, Uuid,
    WallCons, WinCons,
};
use hulc::bdl::Data;

pub fn get_library<T: AsRef<Path>>(path: T) -> Library {
    let data_in = BufReader::new(File::open(path).unwrap());
    let mut gz = GzDecoder::new(data_in);
    let mut dbstring = String::new();
    gz.read_to_string(&mut dbstring).unwrap();
    let data = Data::new(&dbstring).unwrap();

    let mut groups = Groups::default();
    let cons = cons_from_bdl(&data, &mut groups);
    Library { cons, groups }
}

/// Construcciones de muros y huecos a partir de datos BDL
fn cons_from_bdl(bdl: &Data, groups: &mut Groups) -> ConsDb {
    let mut materials = Vec::new();

    for (name, material) in &bdl.db.materials {
        let id = uuid_from_obj(material);
        let group = material.group.clone();

        groups.materials.entry(group.clone()).or_default().push(id);

        materials.push(Material {
            id,
            name: name.clone(),
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
        let group = glass.group.clone();

        groups.glasses.entry(group.clone()).or_default().push(id);

        glasses.push(Glass {
            id,
            name: name.clone(),
            u_value: glass.conductivity,
            g_gln: glass.g_gln,
        })
    }
    let mut frames = Vec::new();
    for (name, frame) in &bdl.db.frames {
        let id = uuid_from_obj(frame);
        let group = frame.group.clone();

        groups.frames.entry(group.clone()).or_default().push(id);

        frames.push(Frame {
            id,
            name: name.clone(),
            u_value: frame.conductivity,
            absorptivity: frame.absorptivity,
        })
    }

    // Mapas de nombre a id
    let mat_name_to_id = materials
        .iter()
        .map(|m| (&m.name, m.id))
        .collect::<BTreeMap<&String, Uuid>>();

    let glass_name_to_id = glasses
        .iter()
        .map(|m| (&m.name, m.id))
        .collect::<BTreeMap<&String, Uuid>>();

    let frame_name_to_id = frames
        .iter()
        .map(|m| (&m.name, m.id))
        .collect::<BTreeMap<&String, Uuid>>();

    // Construcciones de opacos
    let mut wallcons = Vec::new();
    for cons in bdl.db.wallcons.values() {
        let mut ids = Vec::with_capacity(cons.material.len());
        for mat_name in &cons.material {
            let id = mat_name_to_id.get(&mat_name).copied().unwrap_or_else(|| {
                eprintln!(
                    "AVISO: Material `{}` de construcción `{}` no encontrado:\n{:#?}\nUsando id por defecto",
                    mat_name, cons.name, cons
                );
                Uuid::default()
            });
            ids.push(id);
        }

        let layers = ids
            .iter()
            .cloned()
            .zip(cons.thickness.iter().cloned())
            .map(|(id, e)| Layer { id, e })
            .collect();

        let id = uuid_from_obj(cons);
        let group = cons.group.clone();

        groups.wallcons.entry(group.clone()).or_default().push(id);

        wallcons.push(WallCons {
            id,
            name: cons.name.clone(),
            layers,
            absorptance: cons.absorptance,
        });
    }

    // Construcciones de huecos
    let mut wincons = Vec::new();
    for cons in bdl.db.wincons.values() {
        let id = uuid_from_obj(cons);
        let group = cons.group.clone();

        // Vidrio del hueco (Glass)
        let glass = glass_name_to_id
            .get(&cons.glass)
            .copied()
            .unwrap_or_else(|| {
                eprintln!(
                    "AVISO: Vidrio `{}` de construcción `{}` no encontrado:\n{:#?}\nUsando id por defecto",
                    &cons.glass, cons.name, cons
                );
                Uuid::default()
            });

        // Marco del hueco (Frame)
        let frame = frame_name_to_id
            .get(&cons.frame)
            .copied()
            .unwrap_or_else(|| {
                eprintln!(
                    "AVISO: Marco `{}` de construcción `{}` no encontrado:\n{:#?}\nUsando id por defecto",
                    &cons.frame, cons.name, cons
                );
                Uuid::default()
            });

        groups.wincons.entry(group.clone()).or_default().push(id);

        wincons.push(WinCons {
            id,
            name: cons.name.clone(),
            glass,
            frame,
            f_f: cons.framefrac,
            delta_u: cons.deltau,
            g_glshwi: cons.gglshwi,
            c_100: cons.infcoeff,
        });
    }

    ConsDb {
        wallcons,
        wincons,
        materials,
        glasses,
        frames,
    }
}
