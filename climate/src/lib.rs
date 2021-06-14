pub mod met;
pub mod solar;

pub use met::*;
pub use solar::{
    nday_from_str, nday_from_ymd, radiation_for_surface, sun_position, sunsurface_angles,
    CTE_latitude, Location, SolarRadiation, SunPosition, SunSurfaceAngles, CTE_LATCANARIAS,
    CTE_LATPENINSULA,
};

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
    fn met_parsing() {
        let metdata = met::parsemet(METDATA).unwrap();
        assert_eq!(metdata.meta.zc, "D3");
        assert_almost_eq!(metdata.data[8759].dirviento, 178.0);
    }

    #[test]
    fn sunpos_test() {
        // Comparado con https://gml.noaa.gov/grad/solcalc/
        // Hay alguna pequeña diferencia en la declinación 23.053 -> 23.11
        let nday1 = nday_from_str("2001-6-11");
        let nday2 = nday_from_ymd(2001, 6, 11);
        let loc = Location {
            latitude: 40.0,
            longitude: 0.0,
            tz: 2,
        };

        assert_eq!(nday1, 162);
        assert_eq!(nday2, 162);
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
        let latitud = metdata.meta.latitud;

        let july_data: Vec<_> = metdata.data.iter().filter(|d| d.mes == 7).collect();
        let surf_tilt = 0.0;
        let surf_azimuth = 0.0;
        let albedo = 0.2;
        let d = july_data[6];
        let altsol = 90.0 - d.cenit;
        let gsolbeam = G_sol_b(d.rdirhor, altsol);

        let idirtot = solar::I_dir_tot(
            d.mes,
            d.dia,
            d.hora,
            gsolbeam,
            d.rdifhor,
            altsol,
            latitud,
            surf_tilt,
            surf_azimuth,
        );
        let idiftot = solar::I_dif_tot(
            d.mes,
            d.dia,
            d.hora,
            gsolbeam,
            d.rdifhor,
            altsol,
            latitud,
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
        // Comparamos resultados mensuales
        let metdata = met::parsemet(METDATA).unwrap();
        let surf_tilt = 0.0;
        let surf_azimuth = 0.0;
        let albedo = 0.2;

        let mdata =
            met::monthly_radiation_for_surface(&metdata, surf_tilt, surf_azimuth, albedo);
        assert_almost_eq!(mdata.dir[0], 32.997);
        assert_almost_eq!(mdata.dif[0], 21.072);
    }
}
