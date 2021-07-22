// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Datos generales de zonas climáticas (latitud, longitud de referencia, nombre, etc)
//! Datos de radiación mensuales para superficies
//! Datos de radiación horaria por zona climática para el 21 de julio
//! Criterios de orientación UNE-EN ISO 52016-1, (S=0, E=+90, W=-90)
#![allow(clippy::clippy::approx_constant)]

use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

use crate::common::{
    ClimateZone::{self, *},
    Meta,
};

/// Diccionario con metadatos de zonas climáticas (20 climas canarios y 12 climas peninsulares)
pub static CLIMATEMETADATA: Lazy<Mutex<HashMap<ClimateZone, Meta>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        C4,
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
        Meta {
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
