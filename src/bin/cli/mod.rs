#![cfg(not(windows))]
// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use std::process::exit;

use anyhow::Result;

use hulc2envolventecte::{
    collect_hulc_data, get_copytxt,
    parsers::{ctehexml, kyg, tbl},
    PROGNAME,
};

fn get_help() -> String {
    format!(
        "Uso: {} [--skip-kyg] DIRECTORIO

Opciones:
--skip-kyg      Ignorar datos obtenidos del archivo KyGananciasSolares.txt

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

#[derive(Debug, Copy, Clone)]
struct Options {
    use_extra_files: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            use_extra_files: true,
        }
    }
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
                match opt.as_ref() {
                    "--skip-extra" => {
                        eprintln!(
                            "Se ignorará la información en los archivos KyGananciasSolares.txt y NewBDL_O.tbl"
                        );
                        opts.use_extra_files = false;
                    }
                    _ => (),
                }
            }
            (opts, &args[args.len() - 1])
        }
    };

    // Localiza archivos
    eprintln!("Localizando archivos de datos en '{}'", dir);
    let ctehexmlpath = ctehexml::find_ctehexml(&dir)?;
    eprintln!(
        "- {}",
        ctehexmlpath
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or("".to_string())
    );

    let kygpath = if opts.use_extra_files == false {
        None
    } else {
        let kygpath = kyg::find_kyg(&dir)?;
        eprintln!(
            "- {}",
            kygpath
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or("".to_string())
        );
        kygpath
    };

    let tblpath = if opts.use_extra_files == false {
        None
    } else {
        let tblpath = tbl::find_tbl(&dir)?;
        eprintln!(
            "- {}",
            tblpath
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or("".to_string())
        );
        tblpath
    };

    // Lee datos
    let data = collect_hulc_data(ctehexmlpath, kygpath, tblpath)?;

    // Información general
    let climatezone = data.meta.climate;
    let totradjul =
        hulc2envolventecte::cte::climatedata::total_radiation_in_july_by_orientation(&climatezone);
    eprintln!(
        "ZC: {}, A_ref={:.2} m², V/A={:.2} m³/m², K={:.2} W/m²a, q_sol;jul={:.2} kWh/m².mes, n50(he2019)={:.2} 1/h, C_o(he2019)={:.2} m³/h·m², n50={:.2} 1/h, C_o={:.2} m³/h·m²",
        climatezone,
        data.a_ref(),
        data.compacity(),
        data.K_he2019().K,
        data.q_soljul(&totradjul),
        data.n50_he2019(),
        data.C_o_he2019(),
        data.n50(),
        data.C_o()
    );

    // Convierte a JSON
    match data.as_json() {
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
