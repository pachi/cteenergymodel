#![cfg(not(windows))]
// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use std::process::exit;

use anyhow::Result;

use hulc2model::{collect_hulc_data, get_copytxt, PROGNAME};

fn get_help() -> String {
    format!(
        "Uso: {} [--use-kyg] DIRECTORIO

Opciones:
--use-extra      Utiliza datos de transmitancia y radiación de KyGananciasSolares.txt y NewBDL_O.tbl

Argumentos:
DIRECTORIO     Directorio del proyecto de HULC

Descripción:
Exporta al formato JSON de EnvolventeCTE los datos de un proyecto HULC.

Emite en formato JSON de EnvolventeCTE los datos de un proyecto HULC.
Puede redirigir la salida de resultados a un archivo para su uso posterior:
    {} DIRECTORIO > archivo_salida.json
",
        PROGNAME, PROGNAME
    )
}

#[derive(Debug, Copy, Clone, Default)]
struct Options {
    use_extra_files: bool,
}

pub fn cli_main() -> Result<()> {
    env_logger::init();

    eprintln!("{}\n", get_copytxt());

    let args = std::env::args().collect::<Vec<_>>();

    let (opts, dir) = match args.len() {
        // Sin argumentos
        1 => {
            eprintln!("{}", get_help());
            exit(1)
        }
        // Directorio de proyecto
        2 => (Options::default(), &args[1]),
        // Opciones + directorio de proyecto
        _ => {
            let mut opts = Options::default();
            for opt in &args[1..args.len() - 1] {
                if opt.as_str() == "--use-extra" {
                    eprintln!(
                            "Se usará la información en los archivos KyGananciasSolares.txt y NewBDL_O.tbl"
                        );
                    opts.use_extra_files = true;
                }
            }
            (opts, &args[args.len() - 1])
        }
    };

    // Localiza archivos
    eprintln!("Localizando archivos de datos en '{}'", dir);
    if opts.use_extra_files {
        eprintln!("- Se usarán los datos de los archivos KyGananciasSolares.txt y NewBDL_O.tbl")
    };
    // Lee datos
    let model = collect_hulc_data(dir, opts.use_extra_files, opts.use_extra_files)?;
    let ind = model.energy_indicators();
    // Información general
    let climatezone = model.meta.climate;
    let n50data = ind.n50_data;
    eprintln!(
        "ZC: {}, A_ref={:.2} m², V/A={:.2} m³/m², K={:.2} W/m²a, q_sol;jul={:.2} kWh/m².mes, n50_ref={:.2} 1/h, C_o_ref={:.2} m³/h·m², n50={:.2} 1/h, C_o={:.2} m³/h·m²",
        climatezone,
        ind.area_ref,
        ind.compactness,
        ind.K_data.K,
        ind.q_soljul_data.q_soljul,
        n50data.n50_ref,
        n50data.walls_c_ref,
        n50data.n50,
        n50data.walls_c
    );

    // Convierte a JSON
    match model.as_json() {
        Ok(json) => {
            eprintln!("Salida de resultados en formato JSON de EnvolventeCTE");
            println!("{}", json);
            Ok(())
        }
        _ => {
            eprintln!("Error al guardar la información en formato JSON de EnvolventeCTE");
            exit(1)
        }
    }
}
