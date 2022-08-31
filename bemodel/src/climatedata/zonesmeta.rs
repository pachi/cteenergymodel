// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See accompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Datos generales de zonas climáticas (latitud, longitud de referencia, nombre, etc)
#![allow(clippy::approx_constant)]

use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use super::ClimateZone::{self, *};

/// Metadatos de zonas climáticas .met
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetInfo {
    /// Climate file name. e.g. zonaD3.met
    pub metname: String,
    /// Climatic Zone (CTE). e.g. D3
    pub zc: ClimateZone,
    /// latitude, degrees
    pub latitude: f32,
    /// longitude, degrees
    pub longitude: f32,
    /// altitude, metres
    pub altitude: f32,
    /// reference longitude, degrees
    pub reflong: f32,
}

/// Diccionario con metadatos de zonas climáticas (20 climas canarios y 12 climas peninsulares)
pub static CLIMATEMETADATA: Lazy<Mutex<HashMap<ClimateZone, MetInfo>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        C4,
        MetInfo {
            metname: "zonaC4.met".to_string(),
            zc: C4,
            latitude: 40.68333,
            longitude: -4.133333,
            altitude: 667.0,
            reflong: 15.0,
        },
    );
    map.insert(
        B1c,
        MetInfo {
            metname: "zonaB1c.met".to_string(),
            zc: B1c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        D2,
        MetInfo {
            metname: "zonaD2.met".to_string(),
            zc: D2,
            latitude: 40.68333,
            longitude: -4.133333,
            altitude: 667.0,
            reflong: 15.0,
        },
    );
    map.insert(
        D1,
        MetInfo {
            metname: "zonaD1.met".to_string(),
            zc: D1,
            latitude: 40.68333,
            longitude: -4.133333,
            altitude: 667.0,
            reflong: 15.0,
        },
    );
    map.insert(
        E1c,
        MetInfo {
            metname: "zonaE1c.met".to_string(),
            zc: E1c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        D3c,
        MetInfo {
            metname: "zonaD3c.met".to_string(),
            zc: D3c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        Alfa2c,
        MetInfo {
            metname: "zonaAlfa2c.met".to_string(),
            zc: Alfa2c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        A3,
        MetInfo {
            metname: "zonaA3.met".to_string(),
            zc: A3,
            latitude: 40.68333,
            longitude: -4.133333,
            altitude: 667.0,
            reflong: 15.0,
        },
    );
    map.insert(
        C1c,
        MetInfo {
            metname: "zonaC1c.met".to_string(),
            zc: C1c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        Alfa4c,
        MetInfo {
            metname: "zonaAlfa4c.met".to_string(),
            zc: Alfa4c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        C4c,
        MetInfo {
            metname: "zonaC4c.met".to_string(),
            zc: C4c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        Alfa1c,
        MetInfo {
            metname: "zonaAlfa1c.met".to_string(),
            zc: Alfa1c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        E1,
        MetInfo {
            metname: "zonaE1.met".to_string(),
            zc: E1,
            latitude: 40.68333,
            longitude: -4.133333,
            altitude: 667.0,
            reflong: 15.0,
        },
    );
    map.insert(
        C2,
        MetInfo {
            metname: "zonaC2.met".to_string(),
            zc: C2,
            latitude: 40.68333,
            longitude: -4.133333,
            altitude: 667.0,
            reflong: 15.0,
        },
    );
    map.insert(
        B4,
        MetInfo {
            metname: "zonaB4.met".to_string(),
            zc: B4,
            latitude: 40.68333,
            longitude: -4.133333,
            altitude: 667.0,
            reflong: 15.0,
        },
    );
    map.insert(
        C1,
        MetInfo {
            metname: "zonaC1.met".to_string(),
            zc: C1,
            latitude: 40.68333,
            longitude: -4.133333,
            altitude: 667.0,
            reflong: 15.0,
        },
    );
    map.insert(
        C3c,
        MetInfo {
            metname: "zonaC3c.met".to_string(),
            zc: C3c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        B2c,
        MetInfo {
            metname: "zonaB2c.met".to_string(),
            zc: B2c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        Alfa3c,
        MetInfo {
            metname: "zonaAlfa3c.met".to_string(),
            zc: Alfa3c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        B3,
        MetInfo {
            metname: "zonaB3.met".to_string(),
            zc: B3,
            latitude: 40.68333,
            longitude: -4.133333,
            altitude: 667.0,
            reflong: 15.0,
        },
    );
    map.insert(
        B3c,
        MetInfo {
            metname: "zonaB3c.met".to_string(),
            zc: B3c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        C3,
        MetInfo {
            metname: "zonaC3.met".to_string(),
            zc: C3,
            latitude: 40.68333,
            longitude: -4.133333,
            altitude: 667.0,
            reflong: 15.0,
        },
    );
    map.insert(
        A1c,
        MetInfo {
            metname: "zonaA1c.met".to_string(),
            zc: A1c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        A4c,
        MetInfo {
            metname: "zonaA4c.met".to_string(),
            zc: A4c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        B4c,
        MetInfo {
            metname: "zonaB4c.met".to_string(),
            zc: B4c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        D1c,
        MetInfo {
            metname: "zonaD1c.met".to_string(),
            zc: D1c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        D2c,
        MetInfo {
            metname: "zonaD2c.met".to_string(),
            zc: D2c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        A3c,
        MetInfo {
            metname: "zonaA3c.met".to_string(),
            zc: A3c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        A4,
        MetInfo {
            metname: "zonaA4.met".to_string(),
            zc: A4,
            latitude: 40.68333,
            longitude: -4.133333,
            altitude: 667.0,
            reflong: 15.0,
        },
    );
    map.insert(
        C2c,
        MetInfo {
            metname: "zonaC2c.met".to_string(),
            zc: C2c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        A2c,
        MetInfo {
            metname: "zonaA2c.met".to_string(),
            zc: A2c,
            latitude: 28.325,
            longitude: -16.36666,
            altitude: 30.0,
            reflong: 0.0,
        },
    );
    map.insert(
        D3,
        MetInfo {
            metname: "zonaD3.met".to_string(),
            zc: D3,
            latitude: 40.68333,
            longitude: -4.133333,
            altitude: 667.0,
            reflong: 15.0,
        },
    );

    Mutex::new(map)
});
