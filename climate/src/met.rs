// Copyright (c) 2016-2021 Rafael Villar Burke <pachi@rvburke.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Author(s): Rafael Villar Burke <pachi@rvburke.com>

/// # MET format reader
///
/// Linea 1: nombre de archivo de clima. e.g. zonaD3.met
/// Línea 2: campos con datos de localización:
/// - latitud, grados
/// - longitud, grados
/// - altitud, metros
/// - longitud de referencia, grados
///
/// 8760 líneas siguientes: campos con datos meteorológicos
/// - Día (1 a 31);
/// - Mes (1 a 12);
/// - Hora (1 a 24);
/// - Temperatura seca ( ◦ C);
/// - Temperatura efectiva del cielo ( ◦ C);
/// - Irradiancia solar directa sobre una superficie horizontal (W/m 2 );
/// - Irradiancia solar difusa sobre una superficie horizontal (W/m 2 );
/// - Humedad específica (kgH2O/kgaire seco);
/// - Humedad relativa ( %);
/// - Velocidad del viento (m/s);
/// - Dirección del viento (grados respecto al norte, E+, O-);
/// - Azimut solar (grados);
/// - Cénit solar (grados).
///

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path};
use anyhow::{Context, Error, bail};

pub const MESES: [u32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

pub const CLIMATEZONES: [&str; 32] = [
    "A1c", "A2c", "A3c", "A4c", "Alfa1c", "Alfa2c", "Alfa3c", "Alfa4c", "B1c", "B2c", "B3c", "B4c",
    "C1c", "C2c", "C3c", "C4c", "D1c", "D2c", "D3c", "E1c", "A3", "A4", "B3", "B4", "C1", "C2",
    "C3", "C4", "D1", "D2", "D3", "E1",
];

// Orientaciones tipo
pub const ORIENTACIONES: [(f32, f32, &str); 9] = [
    // (tilt (beta), azimuth (gamma), name)
    (0.0, 0.0, "Horiz."),
    (90.0, -135.0, "NE"),
    (90.0, -90.0, "E"),
    (90.0, -45.0, "SE"),
    (90.0, 0.0, "S"),
    (90.0, 45.0, "SW"),
    (90.0, 90.0, "W"),
    (90.0, 135.0, "NW"),
    (90.0, 180.0, "N"),
];

/// Datos climáticos de archivo .met
#[derive(Debug, Clone, Default)]
pub struct MetData {
    pub meta: Meta,
    pub data: Vec<HourlyData>,
}

/// Metadatos de archivo .met
#[derive(Debug, Clone, Default)]
pub struct Meta {
    /// nombre de archivo de clima. e.g. zonaD3.met
    pub metname: String,
    /// Zona climática. e.g. D3
    pub zc: String,
    /// latitud, grados
    pub latitud: f32,
    /// longitud, grados
    pub longitud: f32,
    /// altitud, metros
    pub altitud: f32,
    /// longitud de referencia, grados
    pub longref: f32,
}

/// Valores horarios de archivo .met
#[derive(Debug, Clone, Default)]
pub struct HourlyData {
    /// Mes (1 a 12)
    pub mes: u32,
    /// Día (1 a 31)
    pub dia: u32,
    /// Hora (1 a 24)
    pub hora: f32,
    /// - Temperatura seca (◦C);
    pub tempseca: f32,
    /// - Temperatura efectiva del cielo (◦C);
    pub tempcielo: f32,
    /// - Irradiancia solar directa sobre una superficie horizontal (W/m² );
    pub rdirhor: f32,
    /// - Irradiancia solar difusa sobre una superficie horizontal (W/m² );
    pub rdifhor: f32,
    /// - Humedad específica (kgH2O/kgaire seco);
    pub humedadabs: f32,
    /// - Humedad relativa (%);
    pub humedadrel: f32,
    /// - Velocidad del viento (m/s);
    pub velviento: f32,
    /// - Dirección del viento (grados respecto al norte, E+, O-);
    pub dirviento: f32,
    /// - Azimut solar (grados);
    pub azimut: f32,
    /// - Cénit solar (grados).
    pub cenit: f32,
}

// Parse hourly data from .met data as string
pub fn parsemet<S: AsRef<str>>(metstring: S) -> Result<MetData, Error> {
    let datalines: Vec<_> = metstring.as_ref()
        .split('\n')
        .map(str::trim)
        .filter(|&line| !line.is_empty())
        .collect();
    // metadata
    let metname = &datalines[0];
    let loc = datalines[1]
        .split(' ')
        .map(str::parse::<f32>)
        .collect::<Result<Vec<f32>, _>>()
        .unwrap();
    if loc.len() != 4 {bail!("Datos de localización incorrectos: {}", &datalines[1])};

    let zc = metname.replace("zona", "").replace(".met", "");
    let meta = Meta {
        metname: metname.to_string(),
        zc,
        latitud: loc[0],
        longitud: loc[1],
        altitud: loc[2],
        longref: loc[3],
    };
    // datalines
    let data: Vec<_> = datalines[2..].iter().map(|x| x.trim()).filter(|x| !x.is_empty()).map(|x| {
      let vals: Vec<_> = x.split_ascii_whitespace().map(str::trim).collect();
        let date: Vec<_> = vals[0..2].iter().map(|x| x.parse::<u32>().unwrap()).collect();
        let values: Vec<_> = vals[2..]
            .iter()
            .map(|x| x.parse::<f32>().unwrap())
            .collect();
        (date, values)
    }).filter_map(|(date, values)| 
        {
          if let [ mes, dia] = date[..] {
            if let [hora, tempseca, tempcielo, rdirhor, rdifhor, humedadabs, humedadrel, velviento, dirviento, azimut, cenit ] = values[..] {
              return Some(HourlyData { mes, dia, hora,
                tempseca, tempcielo,
                rdirhor, rdifhor,
                humedadabs, humedadrel,
                velviento, dirviento,
                azimut, cenit })
            }
          };
          None
          }
      ).collect();
    if data.len() != 8760 {bail!("Datos horarios con un número de entradas distinto a 8760: {}", &data.len())};

    Ok(MetData { meta, data })
}

/// Lee estructura de datos desde patch de archivo .ctehexml
pub fn parse_from_path<T: AsRef<Path>>(path: T) -> Result<MetData, Error> {
  let mut utf8data = String::new();
    BufReader::new(File::open(path.as_ref())?)
        .read_to_string(&mut utf8data)
        .with_context(|| {
            format!(
                "No se ha podido leer el archivo {}",
                path.as_ref().display()
            )
        })?;
  parsemet(&utf8data)
}
