use std::path::Path;
use std::vec;

use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};

use crate::met::{self, HourlyData, MetData, CLIMATEZONES, MESES, ORIENTACIONES};
use crate::solar::{nday_from_ymd, radiation_for_surface, SolarRadiation};

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

/// Datos de radiación mensual para una superficie
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MonthlySurfaceRadData {
    /// Zona climática
    pub zc: String,
    /// Nombre de la superficie
    pub surfname: String,
    /// Inclinación de la superficie
    pub surfbeta: f32,
    /// Orientación de la superficie
    pub surfgamma: f32,
    /// Valores mensuales de radiación directa, kWh/m²
    #[serde(serialize_with = "round_serialize_2d")]
    pub dir: Vec<f32>,
    /// Valores mensuales de radiación difusa, kWh/m²
    #[serde(serialize_with = "round_serialize_2d")]
    pub dif: Vec<f32>,
    /// Factor mensual de reducción del sombreamiento móvil f_sh_with por encima de 200 W/m² [fracción]
    #[serde(serialize_with = "round_serialize_2d")]
    pub f_shwith200: Vec<f32>,
    /// Factor mensual de reducción del sombreamiento móvil f_sh_with por encima de 300 W/m² [fracción]
    #[serde(serialize_with = "round_serialize_2d")]
    pub f_shwith300: Vec<f32>,
    /// Factor mensual de reducción del sombreamiento móvil f_sh_with por encima de 500 W/m² [fracción]
    #[serde(serialize_with = "round_serialize_2d")]
    pub f_shwith500: Vec<f32>,
}

/// Datos de radiación para un momento concreto, W/m²
pub struct RadData {
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
pub fn period_radiation_for_surface(
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
            let radiation = radiation_for_surface(nday, d.hora, gsol, latitude, surface_tilt, surface_azimuth, albedo);
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
pub struct MonthlyRadData {
    /// Radiación directa mensual, kWh/m²
    pub dir: Vec<f32>,
    /// Radiación difusa mensual, kWh/m²
    pub dif: Vec<f32>,
    /// Factor mensual de reducción del sombreamiento móvil f_sh_with por encima de 200 W/m² [fracción]
    pub f_shwith200: Vec<f32>,
    /// Factor mensual de reducción del sombreamiento móvil f_sh_with por encima de 300 W/m² [fracción]
    pub f_shwith300: Vec<f32>,
    /// Factor mensual de reducción del sombreamiento móvil f_sh_with por encima de 500 W/m² [fracción]
    pub f_shwith500: Vec<f32>,
}

/// Radiación acumulada mensual (directa, difusa) para un clima y superficie, kWh/m²
pub fn monthly_radiation_for_surface(
    metdata: &MetData,
    surf_tilt: f32,
    surf_azimuth: f32,
    albedo: f32,
) -> MonthlyRadData {
    let latitude = metdata.meta.latitud;
    let surf_radiation = period_radiation_for_surface(&metdata.data, latitude, surf_tilt, surf_azimuth, albedo);

    // Valores acumulados por meses
    let mut dir = vec![];
    let mut dif = vec![];
    let mut f_shwith200 = vec![];
    let mut f_shwith300 = vec![];
    let mut f_shwith500 = vec![];
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
        f_shwith200.push(tot_over_200 / t_tot);
        f_shwith300.push(tot_over_300 / t_tot);
        f_shwith500.push(tot_over_500 / t_tot);
    }

    MonthlyRadData {
        dir,
        dif,
        f_shwith200,
        f_shwith300,
        f_shwith500,
    }
}

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
        let metdata = met::parse_from_path(&metpath).unwrap();
        for (tilt, azimuth, name) in &ORIENTACIONES {
            let MonthlyRadData {
                dir,
                dif,
                f_shwith200,
                f_shwith300,
                f_shwith500,
            } = monthly_radiation_for_surface(&metdata, *tilt, *azimuth, albedo);
            data.push(MonthlySurfaceRadData {
                zc: zona.to_string(),
                surfname: name.to_string(),
                surfbeta: *tilt,
                surfgamma: *azimuth,
                dir,
                dif,
                f_shwith200,
                f_shwith300,
                f_shwith500,
            })
        }
    }
    data
}
