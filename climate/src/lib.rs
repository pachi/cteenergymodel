pub mod met;
pub mod solar;

pub use met::*;
pub use solar::{
    nday_from_md, nday_from_str, nday_from_ymd, radiation_for_surface, sun_position,
    sunsurface_angles, Location, SolarRadiation, SunPosition, SunSurfaceAngles,
};

pub const MONTH_N: [u32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
pub const MONTH_DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

// Orientaciones tipo
pub const ORIENTATIONS: [(f32, f32, &str); 9] = [
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

pub const CTE_CLIMATEZONES: [&str; 32] = [
    "A1c", "A2c", "A3c", "A4c", "Alfa1c", "Alfa2c", "Alfa3c", "Alfa4c", "B1c", "B2c", "B3c", "B4c",
    "C1c", "C2c", "C3c", "C4c", "D1c", "D2c", "D3c", "E1c", "A3", "A4", "B3", "B4", "C1", "C2",
    "C3", "C4", "D1", "D2", "D3", "E1",
];

pub const CTE_LATPENINSULA: f32 = 40.7;
pub const CTE_LATCANARIAS: f32 = 28.3;

// Latitude for location ('peninsula' or 'canarias')
pub fn cte_latitude_from_str(location: &str) -> f32 {
    match location {
        "peninsula" => CTE_LATPENINSULA,
        "canarias" => CTE_LATCANARIAS,
        // TODO: esto era un error
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use crate::solar::G_sol_b;

    use super::*;

    const METDATA: &str = include_str!("./zonaD3.met");

    macro_rules! assert_almost_eq {
        ($a:expr, $b:expr) => {
            assert_almost_eq!($a, $b, 0.001)
        };
        ($a:expr, $b:expr, $c:expr) => {
            if ($a - $b).abs() > $c {
                panic!(
                    "assertion failed: `abs(left - right) < {}`, (left: `{}`, right: `{}`)",
                    $c, $a, $b
                );
            }
        };
    }

    #[test]
    fn sunpos_test() {
        // Comparado con https://gml.noaa.gov/grad/solcalc/
        // Hay alguna pequeña diferencia en la declinación 23.053 -> 23.11
        let nday1 = nday_from_str("2001-6-11");
        let nday2 = nday_from_ymd(2001, 6, 11);
        let nday3 = nday_from_md(6, 11);
        let loc = Location {
            latitude: 40.0,
            longitude: 0.0,
            tz: 2,
        };

        assert_eq!(nday1, 162);
        assert_eq!(nday2, 162);
        assert_eq!(nday3, 162);
        let declination = solar::declination_from_nday(162);
        assert_almost_eq!(declination, 23.053);
        assert_almost_eq!(solar::t_eq(nday1), -0.354);
        assert_almost_eq!(solar::t_shift(loc), 2.0);
        assert_almost_eq!(solar::t_sol(14.0, 162, loc), 12.006);
        // Mediodía solar 12.5h
        assert_almost_eq!(solar::nhour_from_t_sol(12.5, 162, loc), 14.494); // mediodía solar
        assert_almost_eq!(solar::hourangle_from_data(14.4941, 162, loc), 0.0); // mediodía solar
        assert_almost_eq!(solar::hourangle_from_tsol(12.5), 0.0); // mediodía solar
        assert_almost_eq!(solar::hourangle_from_tsol(13.0), -7.5);
        assert_almost_eq!(
            solar::altitude_sol_from_data(23.053, 0.0, loc.latitude),
            73.053
        );
        assert_almost_eq!(
            solar::azimuth_sol_from_data(23.053, 0.0, 73.053, loc.latitude),
            0.0
        );
        let sunpos = sun_position(23.053, 0.0, loc);
        assert_almost_eq!(sunpos.altitude, 73.053);
        assert_almost_eq!(sunpos.azimuth, 0.0);
    }

    #[test]
    fn sun_surface_angles_test() {
        let nday = 162;
        let nhour = 12.5; // mediodía solar (12.5h), hora local = 14.4941h
        let loc = Location {
            latitude: 40.0,
            longitude: 0.0,
            tz: 2,
        };

        let nhour = solar::nhour_from_t_sol(nhour, nday, loc);
        let declination = solar::declination_from_nday(nday); // 23.053
        let hourangle = solar::hourangle_from_data(nhour, nday, loc); // 0.0
                                                                      // Son ángulos de la normal con el rayo solar
                                                                      // Horizontal surface
        let ssang = sunsurface_angles(declination, hourangle, loc, 0.0, 0.0);
        assert_almost_eq!(ssang.angle, 16.947); // 90.0 - 73.053
        assert_almost_eq!(ssang.azimuth, 0.0);
        assert_almost_eq!(ssang.tilt, -16.947); // 73.053 - 90.0
                                                // Vertical south oriented surface
        let ssang = sunsurface_angles(declination, hourangle, loc, 90.0, 0.0);
        assert_almost_eq!(ssang.angle, 73.053);
        assert_almost_eq!(ssang.azimuth, 0.0);
        assert_almost_eq!(ssang.tilt, 73.053);
        // Vertical south-east oriented surface
        let ssang = sunsurface_angles(declination, hourangle, loc, 90.0, 45.0);
        assert_almost_eq!(ssang.angle, 78.106);
        assert_almost_eq!(ssang.azimuth, -45.0);
        assert_almost_eq!(ssang.tilt, 73.053);
        // Vertical north oriented surface
        let ssang = sunsurface_angles(declination, hourangle, loc, 90.0, 180.0);
        assert_almost_eq!(ssang.angle, 106.947); // (90-73.053) + 90.0
        assert_almost_eq!(ssang.azimuth, -180.0);
        assert_almost_eq!(ssang.tilt, 73.053);
    }

    #[test]
    fn met_check() {
        // Comparamos resultados de radiación total calculada con modelo y suma de .met
        let metdata = met::parsemet(METDATA).unwrap();
        let latitude = metdata.meta.latitude;

        let july_data: Vec<_> = metdata.data.iter().filter(|d| d.month == 7).collect();
        let surf_tilt = 0.0;
        let surf_azimuth = 0.0;
        let albedo = 0.2;
        let d = july_data[6];
        let altsol = 90.0 - d.zenith;
        let gsolbeam = G_sol_b(d.rdirhor, altsol);

        let idirtot = solar::I_dir_tot(
            d.month,
            d.day,
            d.hour,
            gsolbeam,
            d.rdifhor,
            altsol,
            latitude,
            surf_tilt,
            surf_azimuth,
        );
        let idiftot = solar::I_dif_tot(
            d.month,
            d.day,
            d.hour,
            gsolbeam,
            d.rdifhor,
            altsol,
            latitude,
            surf_tilt,
            surf_azimuth,
            albedo,
        );

        let tot1 = idirtot + idiftot;
        let tot2 = d.rdirhor + d.rdifhor;

        assert_almost_eq!(tot1, tot2, 0.2);
    }

    #[test]
    fn met_utils() {
        // Carga de archivos .met
        let metdata = met::parsemet(METDATA).unwrap();
        assert_eq!(metdata.meta.zc, "D3");
        assert_almost_eq!(metdata.data[8759].wind_dir, 178.0);

        // Calculamos datos mensuales para una superficie dada
        let surf_tilt = 0.0;
        let surf_azimuth = 0.0;
        let albedo = 0.2;

        let mdata = met::monthly_radiation_for_surface(&metdata, surf_tilt, surf_azimuth, albedo);
        assert_almost_eq!(mdata.dir[0], 32.997);
        assert_almost_eq!(mdata.dif[0], 21.072);
    }
}
