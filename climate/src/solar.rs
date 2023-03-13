#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::too_many_arguments)]

// Copyright (c) 2016 Rafael Villar Burke <pachi@rvburke.com>
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
// Authors: Rafael Villar Burke <pachi@rvburke.com>

use super::MONTH_DAYS;
/// # Solar functions for building science
///
/// Implementation based on formulas from ISO/FDIS 52010-1:2015
use chrono::prelude::*;

/// Site location, considering time zone, longitude and latitude
#[derive(Debug, Default, Copy, Clone)]
pub struct Location {
    /// Latitude of the weather station (φ_w), degrees [-90, +90]
    pub latitude: f32,
    /// Longitude of the weather station (λ_w), degrees [-180, +180]
    /// east+, west-
    pub longitude: f32,
    /// Time zone, actual (clock) time for the location compared to UTC (TZ), h [-12, +12]
    pub tz: i32,
}

/// Solar radiation available, split into direct and diffuse components W/m²
///
/// It can be used to represent irradiance on an horizontal surface or an inclined one
#[derive(Debug, Default, Clone, Copy)]
pub struct SolarRadiation {
    /// Direct irradiation, W/m²
    pub dir: f32,
    /// Diffuse irradiation, W/m²
    pub dif: f32,
}

/// Sun position
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct SunPosition {
    /// Solar azimuth (φ_sol), degrees [-180, +180]
    /// angle from south (0º) east+ (+90), west- (-90)
    pub azimuth: f32,
    /// Solar altitude (α_sol), degrees [0, +90]
    /// angle between the solar beam and the horizontal plane (horiz 0º, vertical 90º)
    pub altitude: f32,
}

/// Sun position with respect to an inclined surface
#[derive(Debug, Default, Clone, Copy)]
pub struct SunSurfaceAngles {
    /// Solar angle of incidence of the solar beam and the inclined surface (θ_sol;ic), degrees [0, +180]
    pub angle: f32,
    /// Azimuth between sun and the inclined surface (γ_sol;ic), degrees [-180, +180]
    pub azimuth: f32,
    /// Tilt angle between sun and the inclined surface (β_sol;ic), degrees [-180,+180]
    pub tilt: f32,
}

// --------------------- Constants ------------------------

pub const G_SC: f32 = 1370.0; // Solar constant, W/m2
pub const Wh2MJ: f32 = 3600.0 * 1e-6; // Wh to MJ conversion factor

// --------------- General utility functions ---------------

#[inline]
fn sind(angle: f32) -> f32 {
    angle.to_radians().sin()
}
#[inline]
fn cosd(angle: f32) -> f32 {
    angle.to_radians().cos()
}
// function tand(angle) { return Math.tan(TO_RAD * angle); }
#[inline]
fn asind(rsin: f32) -> f32 {
    rsin.asin().to_degrees()
}
#[inline]
fn acosd(rcos: f32) -> f32 {
    rcos.acos().to_degrees()
}
// function atand(rtan) { f32::atan(rtan).to_degrees() }

// --------------- Number of day of the year -------------------

/// Number of day in the year for a given day and month [1, 365]
/// month [1-12]
/// day [1-31]
pub fn nday_from_md(month: u32, day: u32) -> u32 {
    assert!(month < 13 && day < 31);
    let past_months_days: u32 = MONTH_DAYS[..(month - 1) as usize].iter().sum();
    past_months_days + day
}

/// Number of day of the year for a given date [1, 366]
/// isodatestring: date string in iso format, e.g. "2016-12-23"
/// year [0-xxx], month [1, 12], day [1, 31]
pub fn nday_from_ymd(year: i32, month: u32, day: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month, day)
        .expect("Se espera una fecha válida")
        .ordinal()
}

pub fn nday_from_str(s: &str) -> u32 {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap().ordinal()
}

// ------------------ Solar declination

/// Solar declination (delta) -> degrees -23.45 <= delta <= 23.45
///
/// angular position of the sun at solar noon with respect to the plane of the equator, north positive, eqs. (1), (2)
/// nday: day of the year (1<= n <= 365)
pub fn declination_from_nday(nday: u32) -> f32 {
    let Rdc = nday as f32 * 360.0 / 365.0; // earth orbit deviation (2)
    0.33281 - 22.984 * cosd(Rdc) - 0.3499 * cosd(2.0 * Rdc) - 0.1398 * cosd(3.0 * Rdc)
        + 3.7872 * sind(Rdc)
        + 0.03205 * sind(2.0 * Rdc)
        + 0.07187 * sind(3.0 * Rdc)
}

// ------------------ Solar hour angle and equation of time

/// Solar hour angle (ω) -> degrees [-180, 180]
///
/// nhour: clock time for the location, h [1.0, 24.0]
/// nday: day of the year (1<= n <= 366)
pub fn hourangle_from_data(nhour: f32, nday: u32, location: Location) -> f32 {
    // tz (TZ): time zone (clock time for the location compared to UTC) [-12, +12]
    // longitude (λ_w): longitude of the weather station (degrees), east+, west- [-180, +180]
    let tsol = t_sol(nhour, nday, location);
    hourangle_from_tsol(tsol)
}

/// Solar hour angle (ω) -> degrees [-180, 180]
///
/// eqs. (10)
/// tsol (t_sol): solar time (hours) [1, 24]
pub fn hourangle_from_tsol(tsol: f32) -> f32 {
    let w = (12.5 - tsol) * 180.0 / 12.0;
    match w {
        v if v > 180.0 => v - 360.0,
        v if v < -180.0 => v + 360.0,
        v => v,
    }
}

/// Equation of time -> minutes
///
/// eqs. (3), (4), (5), (6), (7)
/// nday: day of the year (1<= n <= 366)
pub fn t_eq(nday: u32) -> f32 {
    if nday < 21 {
        2.6 + 0.44 * nday as f32
    } else if nday < 136 {
        5.2 + 9.0 * f32::cos((nday - 43) as f32 * 0.0357)
    } else if nday < 241 {
        1.4 - 5.0 * f32::cos((nday - 135) as f32 * 0.0449)
    } else if nday < 336 {
        -6.3 - 10.0 * f32::cos((nday - 306) as f32 * 0.036)
    } else {
        0.45 * (nday - 359) as f32
    }
}
/// Time shift -> hours
///
/// This results from the fact that the longitude and the path of the sun are not equal
///
/// eqs. (8)
pub fn t_shift(location: Location) -> f32 {
    // tz (TZ): time zone (clock time for the location compared to UTC) [-12, +12]
    // longitude (λ_w): longitude of the weather station (degrees), east+, west- [-180, +180]
    location.tz as f32 - location.longitude / 15.0
}

/// Solar time (t_sol) for clock time -> hours
///
/// eqs. (9)
/// nhour: clock time for the location, h [1.0, 24.0]
/// nday: day of the year (1<= n <= 366)
pub fn t_sol(nhour: f32, nday: u32, location: Location) -> f32 {
    nhour - t_eq(nday) / 60.0 - t_shift(location)
}

/// Clock time (nhour) for solar time -> hours
///
/// tsol: solar time, h
/// nday: day of the year (1<= n <= 366)
pub fn nhour_from_t_sol(tsol: f32, nday: u32, location: Location) -> f32 {
    tsol + t_eq(nday) / 60.0 + t_shift(location)
}

// ------------------ Sun position ---------------------

/// Sun position for a given date, time and geographical situation
pub fn sun_position(declination: f32, hourangle: f32, location: Location) -> SunPosition {
    let altitude = altitude_sol_from_data(declination, hourangle, location.latitude);
    let azimuth = azimuth_sol_from_data(declination, hourangle, altitude, location.latitude);
    SunPosition { azimuth, altitude }
}

/// Solar altitude (α_sol) -> degrees [0, 90]
///
/// angle between the solar beam and the horizontal plane (degrees) eqs. (11)
/// declination (δ): solar declination, degrees
/// hourangle (ω): solar hour angle, degrees [-180, 180]
/// latitude (φ_w): latitude of the weather station (degrees)
pub fn altitude_sol_from_data(declination: f32, hourangle: f32, latitude: f32) -> f32 {
    match asind(
        sind(declination) * sind(latitude) + cosd(declination) * cosd(latitude) * cosd(hourangle),
    ) {
        a_sol if a_sol >= 0.0001 => a_sol,
        _ => 0.0,
    }
}

/// Solar altitude (α_sol) from zenith -> degrees [0, 90]
///
/// zenithsol (θ_z): angle in degrees between the vertical and the line to the sun (angle of incidence of beam radiation on a horizontal surface)
pub fn altitude_sol_from_zenith_sol(zenithsol: f32) -> f32 {
    90.0 - zenithsol
}

/// Solar zenith (θ_z) from solar altitude -> degrees [0, 90]
///
/// angle between the solar beam and the zenith, degrees. eqs. (12)
/// altsol (α_sol): solar altitude, degrees
pub fn zenith_sol_from_altitude_sol(altsol: f32) -> f32 {
    90.0 - altsol
}

/// Solar azimuth (φ_sol) -> degrees [-180, + 180]
///
/// angle from south east+, west-, degrees [-180, +180], eqs. (13)-(16)
/// declination (δ): solar declination for day (degrees)
/// hourangle (ω): hour angle for hour (degrees)
/// altsol (α_sol): solar altitude (degrees)
/// latitude (φ_w): latitude of the weather station (degrees)
pub fn azimuth_sol_from_data(declination: f32, hourangle: f32, altsol: f32, latitude: f32) -> f32 {
    let cos1 = cosd(asind(sind(altsol)));
    let sin_azimaux1 = cosd(declination) * sind(180.0 - hourangle) / cos1;
    let cos_azimaux1 = (cosd(latitude) * sind(declination)
        + sind(latitude) * cosd(declination) * cosd(180.0 - hourangle))
        / cos1;

    let azimaux = asind(cosd(declination) * sind(180.0 - hourangle)) / cos1;

    if sin_azimaux1 >= 0.0 && cos_azimaux1 > 0.0 {
        180.0 - azimaux
    } else if cos_azimaux1 < 0.0 {
        azimaux
    } else {
        -(180.0 + azimaux)
    }
}

// ----------- Sun - surface angles of incidence (azimuth and tilt from sun to surface) ----------------

/// Solar angles of incidence of the solar beam with a surface (angle, azimuth, tilt)
///
/// declination (δ): solar declination for day (degrees)
/// hourangle (ω): hour angle for hour (degrees)
/// latitude (φ_w): latitude of the weather station (degrees)
/// surf_tilt (β_ic): surface tilt angle, degrees [0, 180]
/// surf_azimuth (γ_ic): surface azimuth (deviation from south, E+, W-), degrees [-180, 180]
/// zenithsol (θ_z): solar zenith (degrees)
pub fn sunsurface_angles(
    declination: f32,
    hourangle: f32,
    location: Location,
    surf_tilt: f32,
    surf_azimuth: f32,
) -> SunSurfaceAngles {
    let altitudesol = altitude_sol_from_data(declination, hourangle, location.latitude);
    let zenithsol = zenith_sol_from_altitude_sol(altitudesol);
    let angle = angle_sol_surf(
        declination,
        hourangle,
        location.latitude,
        surf_tilt,
        surf_azimuth,
    );
    let azimuth = azimuth_sol_surf(hourangle, surf_azimuth);
    let tilt = tilt_sol_surf(zenithsol, surf_tilt);
    SunSurfaceAngles {
        angle,
        azimuth,
        tilt,
    }
}

/// Solar angle of incidence of inclined surface, degrees (θ_sol;ic), degrees [0, 180]
///
/// angle of the solar beam on an inclined surface, degrees, eqs. (17)
/// (1.6.2)
/// declination (δ): solar declination for day (degrees)
/// hourangle (ω): hour angle for hour (degrees)
/// latitude (φ_w): latitude of the weather station (degrees)
/// surf_tilt (β_ic): surface tilt angle, degrees [0, 180]
/// surf_azimuth (γ_ic): surface azimuth (deviation from south, E+, W-), degrees [-180, 180]
pub fn angle_sol_surf(
    declination: f32,
    hourangle: f32,
    latitude: f32,
    surf_tilt: f32,
    surf_azimuth: f32,
) -> f32 {
    let sd = sind(declination);
    let cd = cosd(declination);
    let sh = sind(hourangle);
    let ch = cosd(hourangle);
    let sw = sind(latitude);
    let cw = cosd(latitude);
    let sb = sind(surf_tilt);
    let cb = cosd(surf_tilt);
    let sg = sind(surf_azimuth);
    let cg = cosd(surf_azimuth);
    acosd(
        sd * sw * cb - sd * cw * sb * cg
            + cd * cw * cb * ch
            + cd * sw * sb * cg * ch
            + cd * sb * sg * sh,
    )
}

/// Azimuth between sun and the inclined surface (γ_sol;ic) -> degrees [-180, +180]
///
/// eqs. (18)
/// hourangle (ω): solar angle for hour (degrees)
/// surf_azimuth (γ_ic): surface orientation (deviation from south, E+, W-), degrees [-180, 180]
pub fn azimuth_sol_surf(hourangle: f32, surf_azimuth: f32) -> f32 {
    match hourangle - surf_azimuth {
        azim if azim > 180.0 => azim - 360.0,
        azim if azim < -180.0 => azim + 360.0,
        azim => azim,
    }
}

/// Tilt angle between sun and the inclined surface (β_sol;ic) -> degrees [-180,+180]
///
/// eqs. (19)
/// zenithsol (θ_z): solar zenith (degrees)
/// surf_tilt (β_ic): surface tilt angle, degrees [0, 180]
pub fn tilt_sol_surf(zenithsol: f32, surf_tilt: f32) -> f32 {
    match surf_tilt - zenithsol {
        tilt if tilt > 180.0 => tilt - 360.0,
        tilt if tilt < -180.0 => tilt + 360.0,
        tilt => tilt,
    }
}

// ------------- Irradiation --------------------

/// Compute solar radiation available on surface, W/m²
///
/// nday: day of the year (1<= n <= 366)
/// nhour: clock time for the location, h [1.0, 24.0]
/// gsol: solar radiation available on an horizontal surface
/// latitude (φ_w): latitude of the weather station (degrees)
/// surf_tilt: surface tilt angle (β_ic), degrees [0, 180]
/// surf_azimuth: surface orientation (deviation from south, E+, W-) (γ_ic), degrees [-180, 180]
/// albedo (ρ_sol;grnd): solar reflectivity of the ground [0.0, 1.0]
pub fn radiation_for_surface(
    nday: u32,
    hour: f32,
    gsol: SolarRadiation,
    latitude: f32,
    surf_tilt: f32,
    surf_azimuth: f32,
    albedo: f32,
) -> SolarRadiation {
    let declination = declination_from_nday(nday);
    let hourangle = hourangle_from_tsol(hour);
    let anglesolsurf = angle_sol_surf(declination, hourangle, latitude, surf_tilt, surf_azimuth);
    let altsol = altitude_sol_from_data(declination, hourangle, latitude);
    let gsolbeam = G_sol_b(gsol.dir, altsol);
    let DiffuseParams { a, b, F1, F2 } =
        get_diffuse_params(nday, gsolbeam, gsol.dif, altsol, anglesolsurf);

    // idir: direct irradiance on the inclined surface, W/m2
    let idir = I_dir(gsolbeam, anglesolsurf);
    // icircum: circumsolar irradiance, W/m2
    let icircum = I_circum_eq(gsol.dif, F1, a, b);
    // idif: diffuse irradiance on the inclined surface, W/m2
    let idif = I_dif_eq(gsol.dif, F1, F2, a, b, surf_tilt);
    // idifgrnd: irradiance on the inclined surface by ground reflection, W/m2
    let idifgrnd = I_dif_grnd(gsolbeam, gsol.dif, altsol, surf_tilt, albedo);

    let idirtot = I_dir_tot_eq(idir, icircum);
    let idiftot = I_dif_tot_eq(idif, icircum, idifgrnd);

    SolarRadiation {
        dir: idirtot,
        dif: idiftot,
    }
}

/// Air mass (m) -> dimensionless
///
/// eqs. (20), (21)
/// altsol (α_sol): solar altitude angle, degrees
pub fn airmass(altsol: f32) -> f32 {
    let sa = sind(altsol);
    if altsol >= 10.0 {
        1.0 / sa
    } else {
        1.0 / (sa + 0.15 * f32::powf(altsol + 3.885, -1.253))
    }
}

// Split between direct and diffuse solar irradiance
// We skip correlations (22)-(25) to estimate the direct and diffuse split as we have climatic data

/// Solar direct (beam) radiation (G_sol;b) (at normal incidence) from solar direct radiation on an horizontal surface (G_sol;hor) -> W/m2
/// gsoldir: solar direct irradiance on an horizontal surface, W/m2
/// altsol (α_sol): solar altitude, degrees
pub fn G_sol_b(gsoldir: f32, altsol: f32) -> f32 {
    let a = if altsol < 0.01 { 0.01 } else { altsol };
    gsoldir / sind(a)
}

/// Direct irradiance on inclined surface (I_dir) -> W/m2
///
/// eqs. (26)
/// gsolbeam (G_sol;b): solar direct (beam) radiation (G_sol;b), W/m2
/// anglesolsurf (θ_sol;ic): solar angle of incidence on the inclined surface, degrees
pub fn I_dir(gsolbeam: f32, anglesolsurf: f32) -> f32 {
    f32::max(0.0, gsolbeam * cosd(anglesolsurf))
}

/// Extra-terrestrial radiation (I_ext) -> W/m2
///
/// Normal irradiation out of the atmosphere, by day. eqs. (27)
/// n_day: day of the year (1<= n <= 366)
pub fn I_ext(nday: u32) -> f32 {
    G_SC * (1.0 + 0.033 * cosd(nday as f32 * 360.0 / 365.0))
}

/// Clearness index of the atmophere [-]
///
/// Ratio of global irradiance on the ground to the etraterrestrial global irradiance
/// k_T = G_sol;g / I_ext eq (24)
pub fn clearness_index(nday: u32, gsolhor: f32) -> f32 {
    gsolhor / I_ext(nday)
}

// Diffuse irradiance helper functions

/// clearness parameter (ε), adimensional eq.(30)
///
/// gsolbeam (G_sol;b): solar direct (beam) radiation, W/m2
/// gsoldiff: solar diffuse radiation on an horizontal plane, W/m2
/// altsol (α_sol): solar altitude, degrees
fn clearness(gsolbeam: f32, gsoldiff: f32, altsol: f32) -> f32 {
    if gsoldiff < 0.01 {
        return 999.0;
    };
    const K: f32 = 1.014; // rad^-3
    let kk = K * f32::powf(altsol.to_radians(), 3.0);
    (((gsoldiff + gsolbeam) / gsoldiff) + kk) / (1.0 + kk)
}

#[derive(Debug)]
struct BrightnessCoefs {
    f11: f32,
    f12: f32,
    f13: f32,
    f21: f32,
    f22: f32,
    f23: f32,
}

/// Brightness coefficients (table 9) f_ij
///
/// clearness parameter (ε) [-]
fn brightness_coefficients(clearness: f32) -> BrightnessCoefs {
    if clearness < 1.065 {
        BrightnessCoefs {
            f11: -0.008,
            f12: 0.588,
            f13: -0.062,
            f21: -0.060,
            f22: 0.072,
            f23: -0.022,
        }
    } else if clearness < 1.230 {
        BrightnessCoefs {
            f11: 0.130,
            f12: 0.683,
            f13: -0.151,
            f21: -0.019,
            f22: 0.066,
            f23: -0.029,
        }
    } else if clearness < 1.500 {
        BrightnessCoefs {
            f11: 0.330,
            f12: 0.487,
            f13: -0.221,
            f21: 0.055,
            f22: -0.064,
            f23: -0.026,
        }
    } else if clearness < 1.950 {
        BrightnessCoefs {
            f11: 0.568,
            f12: 0.187,
            f13: -0.295,
            f21: 0.109,
            f22: -0.152,
            f23: -0.014,
        }
    } else if clearness < 2.280 {
        BrightnessCoefs {
            f11: 0.873,
            f12: -0.392,
            f13: -0.362,
            f21: 0.226,
            f22: -0.462,
            f23: 0.001,
        }
    } else if clearness < 4.500 {
        BrightnessCoefs {
            f11: 1.132,
            f12: -1.237,
            f13: -0.412,
            f21: 0.288,
            f22: -0.823,
            f23: 0.056,
        }
    } else if clearness < 6.200 {
        BrightnessCoefs {
            f11: 1.060,
            f12: -1.600,
            f13: -0.359,
            f21: 0.264,
            f22: -1.127,
            f23: 0.131,
        }
    } else {
        BrightnessCoefs {
            f11: 0.678,
            f12: -0.327,
            f13: -0.250,
            f21: 0.156,
            f22: -1.377,
            f23: 0.251,
        }
    }
}

/// Diffuse coefs from eqs (28)-(33)
///
/// These are needed to compute diffuse and circumsolar irradiance on a surface
struct DiffuseParams {
    a: f32,
    b: f32,
    F1: f32,
    F2: f32,
}

/// Diffuse irradiance (without ground reflection) I_dif -> W/m2
///
/// I_dif, eqs. (28)-(34)
/// nday: day of the year (1<= n <= 366)
/// gsolbeam (G_sol;b): solar direct (beam) radiation, W/m2
/// gsoldiff: solar diffuse radiation on an horizontal plane, W/m2
/// altsol (α_sol): solar altitude, degrees
/// anglesolsurf (θ_sol;ic): solar angle of incidence on the inclined surface, degrees
/// betasurf (β_ic): surface tilt angle, degrees [0, 180]
pub fn I_dif(
    nday: u32,
    gsolbeam: f32,
    gsoldiff: f32,
    altsol: f32,
    anglesolsurf: f32,
    betasurf: f32,
) -> f32 {
    let DiffuseParams { a, b, F1, F2 } =
        get_diffuse_params(nday, gsolbeam, gsoldiff, altsol, anglesolsurf);
    I_dif_eq(gsoldiff, F1, F2, a, b, betasurf)
}

/// Diffuse radiation parameters
///
/// These are needed to compute diffuse and circumsolar radiation on a surface
///
/// nday: day of the year (1<= n <= 366)
/// gsolbeam (G_sol;b): solar direct (beam) radiation, W/m2
/// gsoldiff: solar diffuse radiation on an horizontal plane, W/m2
/// altsol (α_sol): solar altitude, degrees
/// anglesolsurf (θ_sol;ic): solar angle of incidence on the inclined surface, degrees
fn get_diffuse_params(
    nday: u32,
    gsolbeam: f32,
    gsoldiff: f32,
    altsol: f32,
    anglesolsurf: f32,
) -> DiffuseParams {
    let zenith_sol = 90.0 - altsol;
    let a = f32::max(0.0, cosd(anglesolsurf));
    let b = f32::max(cosd(85.0), cosd(zenith_sol));
    let clearness = clearness(gsolbeam, gsoldiff, altsol);
    let c = brightness_coefficients(clearness);
    let skybr = airmass(altsol) * gsoldiff / I_ext(nday); // sky brightness param
    let F1 = f32::max(0.0, c.f11 + c.f12 * skybr + c.f13 * zenith_sol.to_radians());
    let F2 = c.f21 + c.f22 * skybr + c.f23 * zenith_sol.to_radians();

    DiffuseParams { a, b, F1, F2 }
}

/// Diffuse irradiance (without ground reflection) I_dif -> W/m2
///
/// I_dif, eq (34)
/// gsoldiff: solar diffuse radiation on an horizontal plane, W/m2
fn I_dif_eq(gsoldiff: f32, F1: f32, F2: f32, a: f32, b: f32, betasurf: f32) -> f32 {
    gsoldiff * ((1.0 - F1) * 0.5 * (1.0 + cosd(betasurf)) + F1 * a / b + F2 * sind(betasurf))
}

/// Diffuse irradiance due to ground reflection, I_dif;grnd  -> W/m2
///
/// eqs. (35)
///
/// gsolbeam (G_sol;b): solar direct (beam) radiation, W/m2
/// gsoldiff: solar diffuse radiation on an horizontal plane, W/m2
/// altsol (α_sol): solar altitude, degrees
/// betasurf (β_ic): surface tilt angle, degrees [0, 180]
/// albedo: solar reflectivity of the ground [0.0, 1.0]
pub fn I_dif_grnd(gsolbeam: f32, gsoldiff: f32, altsol: f32, betasurf: f32, albedo: f32) -> f32 {
    (gsoldiff + gsolbeam * sind(altsol)) * albedo * (1.0 - cosd(betasurf)) / 2.0
}

/// Circumsolar irradiance, I_circum -> W/m2
///
/// eqs. (36)
/// nday: day of the year (1<= n <= 366)
/// gsolbeam (G_sol;b): solar direct (beam) radiation, W/m2
/// gsoldiff: solar diffuse radiation on an horizontal plane, W/m2
/// altsol (α_sol): solar altitude, degrees
/// anglesolsurf (θ_sol;ic): solar angle of incidence on the inclined surface, degrees
pub fn I_circum(nday: u32, gsolbeam: f32, gsoldiff: f32, altsol: f32, anglesolsurf: f32) -> f32 {
    let DiffuseParams { a, b, F1, F2: _ } =
        get_diffuse_params(nday, gsolbeam, gsoldiff, altsol, anglesolsurf);
    I_circum_eq(gsoldiff, F1, a, b)
}

/// Circumsolar irradiance, I_circum -> W/m2
///
/// eqs. (36)
/// gsoldiff: solar diffuse radiation on an horizontal plane, W/m2
fn I_circum_eq(gsoldiff: f32, F1: f32, a: f32, b: f32) -> f32 {
    gsoldiff * F1 * a / b
}

/// Total direct solar irradiance, I_dir;tot -> W/m2
///
/// eqs. (37)
/// month: month of the year [1, 12]
/// day: day of the month [1, 31]
/// hour: solar hour [1, 24]
/// gsolbeam (G_sol;b): solar direct (beam) radiation (G_sol;b), W/m2
/// gsoldiff (G_sol;d): solar diffuse radiation on an horizontal surface, W/m2
/// altsol (α_sol): solar altitude, degrees
/// latitude (φ_w): latitude of the weather station, degrees [-90, +90]
/// betasurf (β_ic): surface tilt angle, degrees [0, 180]
/// gammasurf (γ_ic): surface orientation (deviation from south, E+, W-), degrees [-180, 180]
pub fn I_dir_tot(
    month: u32,
    day: u32,
    hour: f32,
    gsolbeam: f32,
    gsoldiff: f32,
    altsol: f32,
    latitude: f32,
    betasurf: f32,
    gammasurf: f32,
) -> f32 {
    let nday = nday_from_ymd(2001, month, day);
    let declination = declination_from_nday(nday);
    let hourangle_sol = hourangle_from_tsol(hour);
    // anglesolsurf: surface incidence angle
    let anglesolsurf = angle_sol_surf(declination, hourangle_sol, latitude, betasurf, gammasurf);
    // idir: direct irradiance on the inclined surface, W/m2
    // icircum: circumsolar irradiance, W/m2
    let idir = I_dir(gsolbeam, anglesolsurf);
    let icircum = I_circum(nday, gsolbeam, gsoldiff, altsol, anglesolsurf);
    // idirtotval
    I_dir_tot_eq(idir, icircum)
}

/// Total direct solar irradiance, I_dir;tot -> W/m2
///
/// eqs. (37)
#[inline]
fn I_dir_tot_eq(idir: f32, icircum: f32) -> f32 {
    idir + icircum
}

/// Total diffuse solar irradiance, I_dif;tot -> W/m2
///
/// eqs. (38)
/// month: month of the year [1, 12]
/// day: day of the month [1, 31]
/// hour: solar hour [1, 24]
/// gsolbeam (G_sol;b): solar direct (beam) radiation, W/m2
/// gsoldiff (G_sol;d): solar diffuse radiation on an horizontal surface, W/m2
/// altsol (α_sol): solar altitude, degrees
/// latitude (φ_w): latitude of the weather station, degrees [-90, +90]
/// betasurf (β_ic): surface tilt angle, degrees [0, 180]
/// gammasurf (γ_ic): surface orientation (deviation from south, E+, W-), degrees [-180, 180]
/// albedo (ρ_sol;grnd): solar reflectivity of the ground [0.0, 1.0]
pub fn I_dif_tot(
    month: u32,
    day: u32,
    hour: f32,
    gsolbeam: f32,
    gsoldiff: f32,
    altsol: f32,
    latitude: f32,
    betasurf: f32,
    gammasurf: f32,
    albedo: f32,
) -> f32 {
    let nday = nday_from_ymd(2001, month, day);
    let declination = declination_from_nday(nday);
    let hourangle_sol = hourangle_from_tsol(hour);
    let anglesolsurf = angle_sol_surf(declination, hourangle_sol, latitude, betasurf, gammasurf);
    // idif: diffuse irradiance on the inclined surface, W/m2
    let idif = I_dif(nday, gsolbeam, gsoldiff, altsol, anglesolsurf, betasurf);
    // icircum: circumsolar irradiance, W/m2
    let icircum = I_circum(nday, gsolbeam, gsoldiff, altsol, anglesolsurf);
    // idifgrnd: irradiance on the inclined surface by ground reflection, W/m2
    let idifgrnd = I_dif_grnd(gsolbeam, gsoldiff, altsol, betasurf, albedo);
    // idiftotval
    idif - icircum + idifgrnd
}

/// Total diffuse solar irradiance, I_dif;tot -> W/m2
///
/// eqs. (38)
#[inline]
fn I_dif_tot_eq(idif: f32, icircum: f32, idifgrnd: f32) -> f32 {
    idif - icircum + idifgrnd
}

/// Total solar irradiance, I_tot -> W/m2
///
/// eqs. (39)
/// month: month of the year [1, 12]
/// day: day of the month [1, 31]
/// hour: solar hour [1, 24]
/// gsolbeam (G_sol;b): solar direct (beam) radiation (G_sol;b), W/m2
/// gsoldiff (G_sol;d): solar diffuse radiation on an horizontal plane, W/m2
/// altsol (α_sol): solar altitude, degrees
/// latitude (φ_w): latitude of the weather station, degrees [-90, +90]
/// betasurf (β_ic): surface tilt angle, degrees [0, 180]
/// gammasurf (γ_ic): surface orientation (deviation from south, E+, W-), degrees [-180, 180]
/// albedo (ρ_sol;grnd): solar reflectivity of the ground [0.0, 1.0]
pub fn I_tot(
    month: u32,
    day: u32,
    hour: f32,
    gsolbeam: f32,
    gsoldiff: f32,
    altsol: f32,
    latitude: f32,
    betasurf: f32,
    gammasurf: f32,
    albedo: f32,
) -> f32 {
    I_dir_tot(
        month, day, hour, gsolbeam, gsoldiff, altsol, latitude, betasurf, gammasurf,
    ) + I_dif_tot(
        month, day, hour, gsolbeam, gsoldiff, altsol, latitude, betasurf, gammasurf, albedo,
    )
}
