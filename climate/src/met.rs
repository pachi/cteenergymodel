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
use std::path::Path;

use anyhow::{Context, Error, bail};
use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};

use super::solar::{nday_from_ymd, radiation_for_surface, SolarRadiation};

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


/// Datos de radiación y factores de reducción mensuales para una superficie
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MonthlySurfaceRadData {
    /// Zona climática CTE
    /// [A1c, A2c, A3c, A4c, Alfa1c, Alfa2c, Alfa3c, Alfa4c,
    ///  B1c, B2c, B3c, B4c, C1c, C2c, C3c, C4c, D1c, D2c, D3c, E1c,
    ///  A3, A4, B3, B4, C1, C2, C3, C4, D1, D2, D3, E1]
    pub zc: String,
    /// Nombre de la superficie
    pub name: String,
    /// Inclinación de la superficie, grados sexagesimales [0, 180]
    /// Horizontal hacia arriba: 0, vertical: 90, horiz. hacia abajo: 180
    pub tilt: f32,
    /// Orientación de la superficie, grados sexagesimales [-180, 180]
    /// W: -90, S: 0, E: 90
    pub azimuth: f32,
    /// Valores mensuales de radiación directa, kWh/m²
    #[serde(serialize_with = "round_serialize_2d")]
    pub dir: Vec<f32>,
    /// Valores mensuales de radiación difusa, kWh/m²
    #[serde(serialize_with = "round_serialize_2d")]
    pub dif: Vec<f32>,
    /// Factor mensual de reducción del sombreamiento móvil f_sh_with por encima de 200 W/m² [fracción]
    /// Está calculado como factor de la radiación total, no solo la directa
    #[serde(serialize_with = "round_serialize_2d")]
    pub fshwi200: Vec<f32>,
    /// Factor mensual de reducción del sombreamiento móvil f_sh_with por encima de 300 W/m² [fracción]
    /// Está calculado como factor de la radiación total, no solo la directa
    #[serde(serialize_with = "round_serialize_2d")]
    pub fshwi300: Vec<f32>,
    /// Factor mensual de reducción del sombreamiento móvil f_sh_with por encima de 500 W/m² [fracción]
    /// Está calculado como factor de la radiación total, no solo la directa
    #[serde(serialize_with = "round_serialize_2d")]
    pub fshwi500: Vec<f32>,
}


/// Redondea valor a 2 decimales
#[inline]
pub fn fround2(val: f32) -> f32 {
    (val * 100.0).round() / 100.0
}

/// Redondeo a dos cifras de los valores del vector
fn round_serialize_2d<S>(x: &[f32], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = s.serialize_seq(Some(x.len()))?;
    for e in x {
        seq.serialize_element(&fround2(*e))?;
    }
    seq.end()
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

/// Datos mensuales acucmulados de radiación
pub fn met_monthly_data(metdir: &str) -> Vec<MonthlySurfaceRadData> {
    let albedo = 0.2;
    let mut data = vec![];
    for zona in &CLIMATEZONES {
        let mstr = format!("{}/zona{}.met", metdir, zona);
        let metpath = Path::new(&mstr);
        if !metpath.exists() {
            println!("Archivo no encontrado: {}", &metpath.display());
            continue;
        };
        println!("Leyendo archivo {}", metpath.display());
        let metdata = parse_from_path(&metpath).unwrap();
        for &(tilt, azimuth, name) in &ORIENTACIONES {
            let MonthlyRadData {
                dir,
                dif,
                fshwi200,
                fshwi300,
                fshwi500,
            } = monthly_radiation_for_surface(&metdata, tilt, azimuth, albedo);
            data.push(MonthlySurfaceRadData {
                zc: zona.to_string(),
                name: name.to_string(),
                tilt,
                azimuth,
                dir,
                dif,
                fshwi200,
                fshwi300,
                fshwi500,
            })
        }
    }
    data
}

/// Datos de radiación para un momento concreto, W/m²
pub (crate) struct RadData {
    /// Mes del año [1, 12]
    pub mes: u32,
    /// Día del mes [1, 31]
    pub dia: u32,
    /// Hola de reloj para la localización, h [1.0, 24.0]
    pub hora: f32,
    /// Radiación directa, W/m²
    pub dir: f32,
    /// Radiación difusa, W/m²
    pub dif: f32,
}

/// Calcula radiación directa y difusa en una superficie orientada y con albedo, W/m²
///
/// hourlydata: datos climáticos horarios (.data de climadata)
/// latitude: latitud de la localización
/// surface: superficie inclinada y orientada (inclinación [0, 180], azimuth [-180, 180])
/// albedo: reflectancia del entorno [0.0, 1.0]
fn period_radiation_for_surface(
    hourlydata: &[HourlyData],
    latitude: f32,
    surface_tilt: f32,
    surface_azimuth: f32,
    albedo: f32,
) -> Vec<RadData> {
    hourlydata
        .iter()
        .map(|d| {
            let nday = nday_from_ymd(2001, d.mes, d.dia);
            let gsol = SolarRadiation {
                dir: d.rdirhor,
                dif: d.rdifhor,
            };
            let radiation = radiation_for_surface(
                nday,
                d.hora,
                gsol,
                latitude,
                surface_tilt,
                surface_azimuth,
                albedo,
            );
            RadData {
                mes: d.mes,
                dia: d.dia,
                hora: d.hora,
                dir: radiation.dir,
                dif: radiation.dif,
            }
        })
        .collect()
}

/// Valores mensuales de radiación, kWh/m²
#[derive(Debug, Default, Clone)]
pub(crate) struct MonthlyRadData {
    /// Radiación directa mensual, kWh/m²
    pub dir: Vec<f32>,
    /// Radiación difusa mensual, kWh/m²
    pub dif: Vec<f32>,
    /// Factor mensual de reducción del sombreamiento móvil f_sh_with por encima de 200 W/m² [fracción]
    pub fshwi200: Vec<f32>,
    /// Factor mensual de reducción del sombreamiento móvil f_sh_with por encima de 300 W/m² [fracción]
    pub fshwi300: Vec<f32>,
    /// Factor mensual de reducción del sombreamiento móvil f_sh_with por encima de 500 W/m² [fracción]
    pub fshwi500: Vec<f32>,
}

/// Radiación acumulada mensual (directa, difusa) para un clima y superficie, kWh/m²
pub (crate) fn monthly_radiation_for_surface(
    metdata: &MetData,
    surf_tilt: f32,
    surf_azimuth: f32,
    albedo: f32,
) -> MonthlyRadData {
    let latitude = metdata.meta.latitud;
    let surf_radiation =
        period_radiation_for_surface(&metdata.data, latitude, surf_tilt, surf_azimuth, albedo);

    // Valores acumulados por meses
    let mut dir = vec![];
    let mut dif = vec![];
    let mut fshwi200 = vec![];
    let mut fshwi300 = vec![];
    let mut fshwi500 = vec![];
    for &imes in &MESES {
        let surfrad = surf_radiation.iter().filter(|&d| d.mes == imes);
        let mut t_dir = 0.0;
        let mut t_dif = 0.0;
        let mut t_tot = 0.0;
        let mut tot_over_200 = 0.0;
        let mut tot_over_300 = 0.0;
        let mut tot_over_500 = 0.0;
        for RadData { dir, dif, .. } in surfrad {
            let tot = dir + dif;
            t_dir += dir;
            t_dif += dif;
            t_tot += tot;
            if tot > 200.0 {
                tot_over_200 += tot
            };
            if tot > 300.0 {
                tot_over_300 += tot
            };
            if tot > 500.0 {
                tot_over_500 += tot
            };
        }
        // Convertimos el total de W/m2 a kWh/m2
        dir.push(t_dir / 1000.0);
        dif.push(t_dif / 1000.0);
        // f_sh,wi
        fshwi200.push(tot_over_200 / t_tot);
        fshwi300.push(tot_over_300 / t_tot);
        fshwi500.push(tot_over_500 / t_tot);
    }

    MonthlyRadData {
        dir,
        dif,
        fshwi200,
        fshwi300,
        fshwi500,
    }
}
