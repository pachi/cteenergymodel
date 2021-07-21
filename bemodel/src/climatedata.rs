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

use super::common::{ClimateZone, Meta, Orientation, RadData, SurfaceMonthlyRadiation};
use ClimateZone::*;
use Orientation::*;

/// Diccionario con el valor de la radiación total por orientación para el mes de julio
pub fn total_radiation_in_july_by_orientation(climate: &ClimateZone) -> HashMap<Orientation, f32> {
    RADDATA
        .lock()
        .unwrap()
        .iter()
        .filter(|e| &e.zone == climate)
        .map(|e| (e.orientation, e.dir[6] + e.dif[6]))
        .collect()
}

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

/// Datos de radiación mensual sobre una superficie orientada e inclinada
/// Array de (20 climas canarios y 12 climas peninsulares) * 9 orientaciones (N, S, E, W, NE, NW, SE, SW, HZ) con datos de radiación mensual
/// Estos datos nos permiten calcular de forma aproximada q_soljul
pub static RADDATA: Lazy<Mutex<Vec<SurfaceMonthlyRadiation>>> = Lazy::new(|| {
    let raddata = vec![
        SurfaceMonthlyRadiation {
            zone: A1c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                58.49, 70.11, 103.8, 128.87, 131.55, 134.4, 155.06, 137.02, 90.55, 93.43, 63.91,
                53.58,
            ],
            dif: vec![
                22.95, 24.57, 32.74, 33.8, 39.59, 45.2, 40.36, 36.98, 35.54, 28.05, 22.01, 20.37,
            ],
            f_shwith200: vec![
                0.82, 0.86, 0.91, 0.93, 0.91, 0.94, 0.95, 0.94, 0.89, 0.89, 0.87, 0.8,
            ],
            f_shwith300: vec![
                0.72, 0.76, 0.82, 0.87, 0.86, 0.87, 0.89, 0.87, 0.79, 0.77, 0.74, 0.67,
            ],
            f_shwith500: vec![
                0.47, 0.54, 0.64, 0.69, 0.69, 0.71, 0.73, 0.72, 0.6, 0.58, 0.48, 0.36,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A1c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                3.3, 6.61, 19.08, 29.89, 37.02, 40.61, 46.9, 31.21, 16.12, 11.17, 3.43, 3.0,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 41.09, 44.39, 45.04, 40.72, 33.21, 29.97, 22.74, 20.69,
            ],
            f_shwith200: vec![
                0.03, 0.2, 0.44, 0.58, 0.6, 0.68, 0.65, 0.57, 0.42, 0.28, 0.07, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.09, 0.3, 0.44, 0.51, 0.57, 0.56, 0.44, 0.25, 0.14, 0.01, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.07, 0.18, 0.29, 0.37, 0.36, 0.18, 0.07, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A1c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                29.39, 33.7, 50.51, 55.24, 53.33, 52.15, 62.59, 53.39, 40.1, 44.24, 27.76, 31.12,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 41.09, 44.39, 45.04, 40.72, 33.21, 29.97, 22.74, 20.69,
            ],
            f_shwith200: vec![
                0.69, 0.7, 0.72, 0.75, 0.72, 0.74, 0.75, 0.73, 0.7, 0.7, 0.67, 0.69,
            ],
            f_shwith300: vec![
                0.57, 0.62, 0.63, 0.66, 0.63, 0.61, 0.63, 0.62, 0.61, 0.62, 0.52, 0.57,
            ],
            f_shwith500: vec![
                0.35, 0.47, 0.49, 0.48, 0.45, 0.45, 0.49, 0.48, 0.38, 0.48, 0.35, 0.38,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A1c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                59.81, 56.41, 62.41, 53.09, 39.35, 33.13, 41.84, 47.92, 48.33, 67.76, 55.92, 63.74,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 41.09, 44.39, 45.04, 40.72, 33.21, 29.97, 22.74, 20.69,
            ],
            f_shwith200: vec![
                0.85, 0.85, 0.8, 0.77, 0.67, 0.67, 0.67, 0.73, 0.75, 0.84, 0.86, 0.86,
            ],
            f_shwith300: vec![
                0.8, 0.76, 0.74, 0.67, 0.56, 0.52, 0.57, 0.62, 0.67, 0.76, 0.79, 0.79,
            ],
            f_shwith500: vec![
                0.65, 0.62, 0.54, 0.39, 0.25, 0.16, 0.23, 0.36, 0.46, 0.61, 0.55, 0.57,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A1c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                80.11, 67.92, 59.39, 34.43, 13.0, 5.07, 9.22, 27.55, 44.71, 76.37, 79.16, 81.83,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 41.09, 44.39, 45.04, 40.72, 33.21, 29.97, 22.74, 20.69,
            ],
            f_shwith200: vec![
                0.9, 0.88, 0.82, 0.74, 0.51, 0.38, 0.45, 0.69, 0.75, 0.87, 0.92, 0.9,
            ],
            f_shwith300: vec![
                0.83, 0.79, 0.71, 0.56, 0.18, 0.0, 0.0, 0.45, 0.67, 0.77, 0.81, 0.82,
            ],
            f_shwith500: vec![
                0.68, 0.58, 0.45, 0.06, 0.0, 0.0, 0.0, 0.03, 0.36, 0.56, 0.59, 0.63,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A1c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                60.21, 52.2, 58.13, 52.37, 37.01, 32.33, 41.53, 49.28, 45.95, 61.26, 64.17, 57.17,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 41.09, 44.39, 45.04, 40.72, 33.21, 29.97, 22.74, 20.69,
            ],
            f_shwith200: vec![
                0.85, 0.82, 0.78, 0.78, 0.65, 0.67, 0.68, 0.75, 0.73, 0.82, 0.88, 0.85,
            ],
            f_shwith300: vec![
                0.8, 0.75, 0.71, 0.66, 0.56, 0.49, 0.57, 0.63, 0.64, 0.72, 0.8, 0.73,
            ],
            f_shwith500: vec![
                0.64, 0.54, 0.54, 0.43, 0.27, 0.13, 0.19, 0.37, 0.46, 0.54, 0.64, 0.52,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A1c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                29.76, 30.25, 46.33, 54.4, 48.79, 50.95, 61.21, 54.8, 37.7, 38.71, 34.49, 25.91,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 41.09, 44.39, 45.04, 40.72, 33.21, 29.97, 22.74, 20.69,
            ],
            f_shwith200: vec![
                0.69, 0.68, 0.71, 0.75, 0.69, 0.75, 0.74, 0.73, 0.65, 0.69, 0.72, 0.6,
            ],
            f_shwith300: vec![
                0.57, 0.58, 0.61, 0.65, 0.61, 0.59, 0.64, 0.63, 0.56, 0.55, 0.64, 0.51,
            ],
            f_shwith500: vec![
                0.37, 0.4, 0.48, 0.5, 0.46, 0.41, 0.48, 0.49, 0.41, 0.4, 0.43, 0.33,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A1c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                3.42, 5.95, 17.47, 29.43, 32.93, 39.72, 45.25, 31.85, 15.1, 9.85, 4.71, 2.2,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 41.09, 44.39, 45.04, 40.72, 33.21, 29.97, 22.74, 20.69,
            ],
            f_shwith200: vec![
                0.06, 0.16, 0.4, 0.56, 0.58, 0.68, 0.65, 0.58, 0.37, 0.25, 0.11, 0.01,
            ],
            f_shwith300: vec![
                0.0, 0.07, 0.3, 0.45, 0.48, 0.55, 0.56, 0.48, 0.27, 0.1, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.08, 0.23, 0.29, 0.31, 0.32, 0.24, 0.05, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A1c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.8, 8.47, 15.58, 15.43, 3.41, 0.12, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 41.09, 44.39, 45.04, 40.72, 33.21, 29.97, 22.74, 20.69,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.05, 0.33, 0.17, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: A2c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                58.49, 70.11, 103.8, 128.87, 168.37, 154.15, 184.52, 161.96, 108.14, 93.84, 64.38,
                53.12,
            ],
            dif: vec![
                22.95, 24.57, 32.74, 33.8, 34.12, 40.03, 32.22, 31.75, 31.13, 27.64, 21.54, 20.81,
            ],
            f_shwith200: vec![
                0.82, 0.86, 0.91, 0.93, 0.96, 0.95, 0.96, 0.95, 0.91, 0.89, 0.85, 0.81,
            ],
            f_shwith300: vec![
                0.72, 0.76, 0.82, 0.87, 0.92, 0.9, 0.91, 0.92, 0.83, 0.81, 0.73, 0.66,
            ],
            f_shwith500: vec![
                0.47, 0.54, 0.64, 0.69, 0.76, 0.72, 0.8, 0.74, 0.65, 0.59, 0.48, 0.37,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A2c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                3.3, 6.61, 19.08, 29.89, 43.47, 45.92, 55.41, 39.71, 20.12, 10.87, 4.19, 2.57,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.77, 44.47, 41.58, 33.35, 29.87, 22.65, 20.61,
            ],
            f_shwith200: vec![
                0.03, 0.2, 0.44, 0.58, 0.65, 0.64, 0.72, 0.64, 0.45, 0.28, 0.1, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.09, 0.3, 0.44, 0.56, 0.55, 0.65, 0.52, 0.38, 0.15, 0.04, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.07, 0.18, 0.32, 0.37, 0.45, 0.25, 0.13, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A2c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                29.39, 33.7, 50.51, 55.24, 63.04, 58.87, 75.22, 66.88, 46.88, 43.9, 32.95, 26.79,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.77, 44.47, 41.58, 33.35, 29.87, 22.65, 20.61,
            ],
            f_shwith200: vec![
                0.69, 0.7, 0.72, 0.75, 0.76, 0.74, 0.79, 0.78, 0.72, 0.73, 0.7, 0.62,
            ],
            f_shwith300: vec![
                0.57, 0.62, 0.63, 0.66, 0.65, 0.61, 0.7, 0.69, 0.63, 0.63, 0.6, 0.53,
            ],
            f_shwith500: vec![
                0.35, 0.47, 0.49, 0.48, 0.49, 0.46, 0.58, 0.54, 0.49, 0.51, 0.42, 0.35,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A2c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                59.81, 56.41, 62.41, 53.09, 46.81, 37.34, 51.38, 58.82, 53.26, 67.62, 63.69, 58.78,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.77, 44.47, 41.58, 33.35, 29.87, 22.65, 20.61,
            ],
            f_shwith200: vec![
                0.85, 0.85, 0.8, 0.77, 0.72, 0.64, 0.73, 0.79, 0.78, 0.84, 0.86, 0.84,
            ],
            f_shwith300: vec![
                0.8, 0.76, 0.74, 0.67, 0.6, 0.51, 0.67, 0.67, 0.69, 0.77, 0.8, 0.76,
            ],
            f_shwith500: vec![
                0.65, 0.62, 0.54, 0.39, 0.27, 0.19, 0.31, 0.4, 0.46, 0.64, 0.61, 0.56,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A2c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                80.11, 67.92, 59.39, 34.43, 15.99, 6.0, 12.19, 31.52, 45.93, 78.94, 78.72, 79.97,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.77, 44.47, 41.58, 33.35, 29.87, 22.65, 20.61,
            ],
            f_shwith200: vec![
                0.9, 0.88, 0.82, 0.74, 0.57, 0.38, 0.51, 0.74, 0.79, 0.89, 0.91, 0.9,
            ],
            f_shwith300: vec![
                0.83, 0.79, 0.71, 0.56, 0.14, 0.0, 0.07, 0.46, 0.65, 0.81, 0.82, 0.82,
            ],
            f_shwith500: vec![
                0.68, 0.58, 0.45, 0.06, 0.0, 0.0, 0.0, 0.03, 0.29, 0.58, 0.65, 0.65,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A2c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                60.21, 52.2, 58.13, 52.37, 50.53, 38.18, 47.45, 57.1, 50.75, 64.76, 55.42, 58.7,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.77, 44.47, 41.58, 33.35, 29.87, 22.65, 20.61,
            ],
            f_shwith200: vec![
                0.85, 0.82, 0.78, 0.78, 0.74, 0.65, 0.71, 0.78, 0.74, 0.85, 0.86, 0.85,
            ],
            f_shwith300: vec![
                0.8, 0.75, 0.71, 0.66, 0.64, 0.52, 0.65, 0.67, 0.66, 0.77, 0.75, 0.75,
            ],
            f_shwith500: vec![
                0.64, 0.54, 0.54, 0.43, 0.29, 0.16, 0.23, 0.4, 0.46, 0.58, 0.52, 0.61,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A2c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                29.76, 30.25, 46.33, 54.4, 68.39, 59.64, 68.65, 64.26, 44.52, 41.18, 26.69, 26.2,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.77, 44.47, 41.58, 33.35, 29.87, 22.65, 20.61,
            ],
            f_shwith200: vec![
                0.69, 0.68, 0.71, 0.75, 0.78, 0.74, 0.78, 0.78, 0.69, 0.72, 0.63, 0.65,
            ],
            f_shwith300: vec![
                0.57, 0.58, 0.61, 0.65, 0.68, 0.62, 0.67, 0.67, 0.58, 0.61, 0.5, 0.53,
            ],
            f_shwith500: vec![
                0.37, 0.4, 0.48, 0.5, 0.52, 0.48, 0.55, 0.53, 0.47, 0.42, 0.29, 0.34,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A2c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                3.42, 5.95, 17.47, 29.43, 47.32, 46.16, 50.05, 37.72, 19.31, 9.88, 3.6, 1.81,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.77, 44.47, 41.58, 33.35, 29.87, 22.65, 20.61,
            ],
            f_shwith200: vec![
                0.06, 0.16, 0.4, 0.56, 0.68, 0.65, 0.69, 0.63, 0.42, 0.26, 0.07, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.07, 0.3, 0.45, 0.6, 0.55, 0.61, 0.51, 0.34, 0.13, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.08, 0.23, 0.33, 0.35, 0.38, 0.24, 0.11, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A2c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.8, 11.35, 17.71, 16.88, 4.3, 0.27, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.77, 44.47, 41.58, 33.35, 29.87, 22.65, 20.61,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.08, 0.22, 0.19, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: A3c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                58.49, 70.11, 103.8, 128.87, 168.37, 169.41, 187.61, 168.07, 121.03, 94.58, 63.59,
                52.66,
            ],
            dif: vec![
                22.95, 24.57, 32.74, 33.8, 34.12, 35.1, 32.29, 30.06, 29.36, 26.91, 22.34, 21.27,
            ],
            f_shwith200: vec![
                0.82, 0.86, 0.91, 0.93, 0.96, 0.97, 0.97, 0.95, 0.94, 0.91, 0.83, 0.82,
            ],
            f_shwith300: vec![
                0.72, 0.76, 0.82, 0.87, 0.92, 0.91, 0.92, 0.91, 0.86, 0.81, 0.73, 0.7,
            ],
            f_shwith500: vec![
                0.47, 0.54, 0.64, 0.69, 0.76, 0.76, 0.77, 0.77, 0.67, 0.61, 0.53, 0.42,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                3.3, 6.61, 19.08, 29.89, 43.47, 53.67, 58.88, 45.43, 21.08, 11.34, 3.42, 2.86,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.14, 45.04, 41.33, 34.69, 29.85, 22.73, 20.61,
            ],
            f_shwith200: vec![
                0.03, 0.2, 0.44, 0.58, 0.65, 0.69, 0.71, 0.68, 0.46, 0.28, 0.08, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.09, 0.3, 0.44, 0.56, 0.62, 0.64, 0.57, 0.34, 0.17, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.07, 0.18, 0.32, 0.41, 0.41, 0.36, 0.04, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                29.39, 33.7, 50.51, 55.24, 63.04, 68.64, 78.26, 73.78, 51.04, 45.08, 27.82, 30.33,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.14, 45.04, 41.33, 34.69, 29.85, 22.73, 20.61,
            ],
            f_shwith200: vec![
                0.69, 0.7, 0.72, 0.75, 0.76, 0.78, 0.8, 0.8, 0.74, 0.75, 0.66, 0.69,
            ],
            f_shwith300: vec![
                0.57, 0.62, 0.63, 0.66, 0.65, 0.67, 0.7, 0.72, 0.64, 0.63, 0.53, 0.59,
            ],
            f_shwith500: vec![
                0.35, 0.47, 0.49, 0.48, 0.49, 0.51, 0.57, 0.62, 0.47, 0.48, 0.36, 0.36,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                59.81, 56.41, 62.41, 53.09, 46.81, 43.4, 52.0, 62.63, 60.36, 69.14, 58.55, 62.96,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.14, 45.04, 41.33, 34.69, 29.85, 22.73, 20.61,
            ],
            f_shwith200: vec![
                0.85, 0.85, 0.8, 0.77, 0.72, 0.68, 0.73, 0.82, 0.8, 0.87, 0.84, 0.86,
            ],
            f_shwith300: vec![
                0.8, 0.76, 0.74, 0.67, 0.6, 0.58, 0.64, 0.72, 0.72, 0.76, 0.77, 0.8,
            ],
            f_shwith500: vec![
                0.65, 0.62, 0.54, 0.39, 0.27, 0.18, 0.26, 0.44, 0.52, 0.64, 0.62, 0.63,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                80.11, 67.92, 59.39, 34.43, 15.99, 6.58, 10.77, 30.18, 56.38, 77.53, 80.33, 78.88,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.14, 45.04, 41.33, 34.69, 29.85, 22.73, 20.61,
            ],
            f_shwith200: vec![
                0.9, 0.88, 0.82, 0.74, 0.57, 0.39, 0.46, 0.73, 0.83, 0.89, 0.89, 0.91,
            ],
            f_shwith300: vec![
                0.83, 0.79, 0.71, 0.56, 0.14, 0.0, 0.01, 0.45, 0.68, 0.78, 0.82, 0.84,
            ],
            f_shwith500: vec![
                0.68, 0.58, 0.45, 0.06, 0.0, 0.0, 0.0, 0.0, 0.36, 0.57, 0.67, 0.64,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                60.21, 52.2, 58.13, 52.37, 50.53, 40.67, 50.08, 57.49, 61.62, 61.19, 62.06, 53.23,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.14, 45.04, 41.33, 34.69, 29.85, 22.73, 20.61,
            ],
            f_shwith200: vec![
                0.85, 0.82, 0.78, 0.78, 0.74, 0.68, 0.71, 0.8, 0.81, 0.85, 0.84, 0.84,
            ],
            f_shwith300: vec![
                0.8, 0.75, 0.71, 0.66, 0.64, 0.54, 0.63, 0.69, 0.72, 0.74, 0.77, 0.75,
            ],
            f_shwith500: vec![
                0.64, 0.54, 0.54, 0.43, 0.29, 0.2, 0.25, 0.41, 0.53, 0.58, 0.6, 0.56,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                29.76, 30.25, 46.33, 54.4, 68.39, 64.03, 74.87, 66.02, 52.14, 38.05, 30.4, 22.68,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.14, 45.04, 41.33, 34.69, 29.85, 22.73, 20.61,
            ],
            f_shwith200: vec![
                0.69, 0.68, 0.71, 0.75, 0.78, 0.76, 0.77, 0.78, 0.74, 0.71, 0.69, 0.63,
            ],
            f_shwith300: vec![
                0.57, 0.58, 0.61, 0.65, 0.68, 0.65, 0.69, 0.7, 0.64, 0.56, 0.53, 0.5,
            ],
            f_shwith500: vec![
                0.37, 0.4, 0.48, 0.5, 0.52, 0.49, 0.53, 0.56, 0.5, 0.39, 0.4, 0.24,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                3.42, 5.95, 17.47, 29.43, 47.32, 49.87, 56.01, 39.6, 21.36, 9.34, 3.58, 1.77,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.14, 45.04, 41.33, 34.69, 29.85, 22.73, 20.61,
            ],
            f_shwith200: vec![
                0.06, 0.16, 0.4, 0.56, 0.68, 0.67, 0.68, 0.65, 0.45, 0.24, 0.06, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.07, 0.3, 0.45, 0.6, 0.59, 0.64, 0.53, 0.37, 0.13, 0.01, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.08, 0.23, 0.33, 0.38, 0.4, 0.26, 0.07, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.8, 11.35, 20.34, 19.82, 5.36, 0.14, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 44.14, 45.04, 41.33, 34.69, 29.85, 22.73, 20.61,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.08, 0.26, 0.21, 0.01, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: A4c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                58.49, 70.11, 103.8, 128.87, 168.37, 179.71, 206.65, 183.88, 128.15, 94.44, 63.14,
                53.25,
            ],
            dif: vec![
                22.95, 24.57, 32.74, 33.8, 34.12, 35.06, 28.21, 27.58, 28.42, 27.06, 22.77, 20.7,
            ],
            f_shwith200: vec![
                0.82, 0.86, 0.91, 0.93, 0.96, 0.97, 0.96, 0.96, 0.94, 0.9, 0.83, 0.81,
            ],
            f_shwith300: vec![
                0.72, 0.76, 0.82, 0.87, 0.92, 0.92, 0.93, 0.93, 0.85, 0.81, 0.73, 0.68,
            ],
            f_shwith500: vec![
                0.47, 0.54, 0.64, 0.69, 0.76, 0.78, 0.81, 0.79, 0.72, 0.6, 0.47, 0.4,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                3.3, 6.61, 19.08, 29.89, 43.47, 55.64, 60.29, 45.82, 22.99, 10.57, 3.72, 2.26,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 45.61, 45.52, 41.96, 35.24, 29.67, 22.73, 20.7,
            ],
            f_shwith200: vec![
                0.03, 0.2, 0.44, 0.58, 0.65, 0.72, 0.71, 0.68, 0.49, 0.28, 0.09, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.09, 0.3, 0.44, 0.56, 0.62, 0.67, 0.57, 0.34, 0.14, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.07, 0.18, 0.32, 0.4, 0.41, 0.31, 0.06, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                29.39, 33.7, 50.51, 55.24, 63.04, 71.14, 81.64, 75.4, 55.7, 40.87, 29.35, 27.47,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 45.61, 45.52, 41.96, 35.24, 29.67, 22.73, 20.7,
            ],
            f_shwith200: vec![
                0.69, 0.7, 0.72, 0.75, 0.76, 0.78, 0.8, 0.81, 0.76, 0.71, 0.68, 0.65,
            ],
            f_shwith300: vec![
                0.57, 0.62, 0.63, 0.66, 0.65, 0.66, 0.71, 0.72, 0.67, 0.6, 0.56, 0.55,
            ],
            f_shwith500: vec![
                0.35, 0.47, 0.49, 0.48, 0.49, 0.53, 0.58, 0.61, 0.49, 0.43, 0.39, 0.3,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                59.81, 56.41, 62.41, 53.09, 46.81, 44.97, 55.57, 64.83, 64.67, 64.66, 57.97, 58.73,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 45.61, 45.52, 41.96, 35.24, 29.67, 22.73, 20.7,
            ],
            f_shwith200: vec![
                0.85, 0.85, 0.8, 0.77, 0.72, 0.69, 0.73, 0.81, 0.82, 0.84, 0.84, 0.85,
            ],
            f_shwith300: vec![
                0.8, 0.76, 0.74, 0.67, 0.6, 0.58, 0.67, 0.72, 0.74, 0.75, 0.76, 0.76,
            ],
            f_shwith500: vec![
                0.65, 0.62, 0.54, 0.39, 0.27, 0.19, 0.31, 0.45, 0.51, 0.56, 0.62, 0.56,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                80.11, 67.92, 59.39, 34.43, 15.99, 6.84, 13.35, 33.24, 57.8, 77.6, 78.34, 80.23,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 45.61, 45.52, 41.96, 35.24, 29.67, 22.73, 20.7,
            ],
            f_shwith200: vec![
                0.9, 0.88, 0.82, 0.74, 0.57, 0.41, 0.53, 0.77, 0.83, 0.89, 0.9, 0.9,
            ],
            f_shwith300: vec![
                0.83, 0.79, 0.71, 0.56, 0.14, 0.0, 0.04, 0.49, 0.71, 0.8, 0.81, 0.82,
            ],
            f_shwith500: vec![
                0.68, 0.58, 0.45, 0.06, 0.0, 0.0, 0.0, 0.0, 0.37, 0.55, 0.65, 0.63,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                60.21, 52.2, 58.13, 52.37, 50.53, 45.28, 57.05, 63.43, 61.77, 66.01, 60.32, 59.54,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 45.61, 45.52, 41.96, 35.24, 29.67, 22.73, 20.7,
            ],
            f_shwith200: vec![
                0.85, 0.82, 0.78, 0.78, 0.74, 0.7, 0.74, 0.81, 0.81, 0.83, 0.84, 0.86,
            ],
            f_shwith300: vec![
                0.8, 0.75, 0.71, 0.66, 0.64, 0.57, 0.69, 0.73, 0.72, 0.75, 0.77, 0.77,
            ],
            f_shwith500: vec![
                0.64, 0.54, 0.54, 0.43, 0.29, 0.22, 0.29, 0.4, 0.5, 0.58, 0.6, 0.56,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                29.76, 30.25, 46.33, 54.4, 68.39, 72.03, 84.08, 72.79, 52.93, 41.68, 31.06, 28.25,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 45.61, 45.52, 41.96, 35.24, 29.67, 22.73, 20.7,
            ],
            f_shwith200: vec![
                0.69, 0.68, 0.71, 0.75, 0.78, 0.79, 0.81, 0.8, 0.75, 0.72, 0.69, 0.67,
            ],
            f_shwith300: vec![
                0.57, 0.58, 0.61, 0.65, 0.68, 0.67, 0.72, 0.71, 0.65, 0.57, 0.57, 0.55,
            ],
            f_shwith500: vec![
                0.37, 0.4, 0.48, 0.5, 0.52, 0.52, 0.59, 0.56, 0.46, 0.46, 0.37, 0.39,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                3.42, 5.95, 17.47, 29.43, 47.32, 56.59, 62.27, 43.54, 21.97, 10.37, 3.79, 2.55,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 45.61, 45.52, 41.96, 35.24, 29.67, 22.73, 20.7,
            ],
            f_shwith200: vec![
                0.06, 0.16, 0.4, 0.56, 0.68, 0.72, 0.73, 0.67, 0.48, 0.26, 0.07, 0.01,
            ],
            f_shwith300: vec![
                0.0, 0.07, 0.3, 0.45, 0.6, 0.63, 0.67, 0.55, 0.33, 0.16, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.08, 0.23, 0.33, 0.42, 0.43, 0.24, 0.06, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.8, 11.35, 22.39, 20.38, 5.73, 0.18, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                22.28, 24.68, 33.88, 38.23, 43.93, 45.61, 45.52, 41.96, 35.24, 29.67, 22.73, 20.7,
            ],
            f_shwith200: vec![0.0, 0.0, 0.0, 0.0, 0.08, 0.28, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa1c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                91.6, 104.6, 137.12, 154.56, 177.09, 133.79, 155.42, 135.58, 90.23, 126.39, 95.55,
                84.26,
            ],
            dif: vec![
                23.33, 21.95, 29.45, 31.66, 31.25, 45.81, 39.99, 38.44, 35.83, 24.41, 23.04, 22.94,
            ],
            f_shwith200: vec![
                0.92, 0.94, 0.95, 0.95, 0.96, 0.94, 0.95, 0.93, 0.9, 0.94, 0.93, 0.91,
            ],
            f_shwith300: vec![
                0.84, 0.88, 0.87, 0.92, 0.91, 0.86, 0.9, 0.88, 0.8, 0.89, 0.86, 0.79,
            ],
            f_shwith500: vec![
                0.54, 0.66, 0.74, 0.77, 0.76, 0.68, 0.74, 0.71, 0.63, 0.7, 0.6, 0.48,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa1c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                5.1, 9.74, 21.2, 34.4, 50.05, 43.79, 47.09, 32.19, 17.06, 13.64, 5.43, 3.53,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 44.91, 45.11, 40.82, 33.22, 34.1, 28.48, 26.86,
            ],
            f_shwith200: vec![
                0.05, 0.27, 0.45, 0.61, 0.69, 0.66, 0.67, 0.59, 0.43, 0.3, 0.06, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.14, 0.31, 0.51, 0.6, 0.54, 0.56, 0.48, 0.29, 0.11, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.02, 0.23, 0.37, 0.37, 0.33, 0.26, 0.04, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa1c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                45.85, 48.36, 60.34, 63.84, 72.28, 55.44, 63.33, 55.5, 42.15, 56.04, 44.86, 42.21,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 44.91, 45.11, 40.82, 33.22, 34.1, 28.48, 26.86,
            ],
            f_shwith200: vec![
                0.77, 0.77, 0.77, 0.78, 0.79, 0.72, 0.76, 0.74, 0.7, 0.79, 0.78, 0.77,
            ],
            f_shwith300: vec![
                0.64, 0.69, 0.68, 0.69, 0.69, 0.6, 0.64, 0.64, 0.6, 0.67, 0.65, 0.63,
            ],
            f_shwith500: vec![
                0.36, 0.51, 0.51, 0.54, 0.57, 0.47, 0.5, 0.54, 0.47, 0.46, 0.38, 0.31,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa1c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                95.4, 82.3, 78.24, 61.71, 53.45, 34.61, 42.81, 50.1, 49.84, 88.45, 91.01, 93.59,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 44.91, 45.11, 40.82, 33.22, 34.1, 28.48, 26.86,
            ],
            f_shwith200: vec![
                0.91, 0.92, 0.85, 0.82, 0.74, 0.63, 0.69, 0.77, 0.76, 0.91, 0.92, 0.92,
            ],
            f_shwith300: vec![
                0.88, 0.86, 0.8, 0.71, 0.65, 0.48, 0.58, 0.64, 0.69, 0.84, 0.88, 0.88,
            ],
            f_shwith500: vec![
                0.71, 0.69, 0.61, 0.46, 0.31, 0.19, 0.25, 0.4, 0.5, 0.63, 0.69, 0.72,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa1c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                125.52, 101.91, 79.84, 41.47, 17.24, 4.83, 9.81, 28.39, 44.99, 103.59, 120.1,
                126.12,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 44.91, 45.11, 40.82, 33.22, 34.1, 28.48, 26.86,
            ],
            f_shwith200: vec![
                0.96, 0.96, 0.89, 0.82, 0.57, 0.34, 0.49, 0.71, 0.8, 0.95, 0.96, 0.96,
            ],
            f_shwith300: vec![
                0.9, 0.89, 0.81, 0.62, 0.19, 0.0, 0.06, 0.47, 0.67, 0.86, 0.91, 0.9,
            ],
            f_shwith500: vec![
                0.76, 0.69, 0.47, 0.09, 0.0, 0.0, 0.0, 0.01, 0.34, 0.62, 0.75, 0.77,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa1c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                91.71, 80.22, 76.98, 64.47, 51.32, 33.47, 40.78, 47.2, 47.03, 84.22, 89.66, 91.84,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 44.91, 45.11, 40.82, 33.22, 34.1, 28.48, 26.86,
            ],
            f_shwith200: vec![
                0.92, 0.92, 0.84, 0.83, 0.73, 0.62, 0.67, 0.76, 0.76, 0.91, 0.92, 0.91,
            ],
            f_shwith300: vec![
                0.88, 0.85, 0.78, 0.73, 0.62, 0.48, 0.59, 0.6, 0.7, 0.82, 0.88, 0.87,
            ],
            f_shwith500: vec![
                0.69, 0.71, 0.59, 0.49, 0.31, 0.18, 0.18, 0.33, 0.51, 0.61, 0.7, 0.68,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa1c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                42.82, 46.13, 59.39, 67.34, 69.57, 53.1, 59.84, 51.48, 39.65, 52.26, 43.88, 40.98,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 44.91, 45.11, 40.82, 33.22, 34.1, 28.48, 26.86,
            ],
            f_shwith200: vec![
                0.77, 0.79, 0.76, 0.79, 0.78, 0.72, 0.73, 0.73, 0.7, 0.77, 0.77, 0.76,
            ],
            f_shwith300: vec![
                0.62, 0.68, 0.66, 0.7, 0.67, 0.59, 0.63, 0.6, 0.59, 0.65, 0.64, 0.61,
            ],
            f_shwith500: vec![
                0.3, 0.51, 0.51, 0.57, 0.53, 0.44, 0.49, 0.43, 0.47, 0.4, 0.37, 0.31,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa1c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                4.51, 8.66, 21.11, 36.59, 48.34, 41.63, 44.19, 29.4, 16.33, 12.54, 5.39, 3.54,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 44.91, 45.11, 40.82, 33.22, 34.1, 28.48, 26.86,
            ],
            f_shwith200: vec![
                0.03, 0.23, 0.44, 0.63, 0.67, 0.65, 0.64, 0.58, 0.42, 0.3, 0.06, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.09, 0.29, 0.52, 0.57, 0.54, 0.58, 0.43, 0.31, 0.11, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.04, 0.23, 0.35, 0.32, 0.32, 0.19, 0.02, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa1c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 2.44, 12.73, 17.09, 15.24, 3.14, 0.1, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 44.91, 45.11, 40.82, 33.22, 34.1, 28.48, 26.86,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.09, 0.25, 0.14, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa2c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                91.6, 104.6, 137.12, 154.56, 177.09, 175.4, 186.04, 161.8, 127.21, 123.8, 96.22,
                84.0,
            ],
            dif: vec![
                23.33, 21.95, 29.45, 31.66, 31.25, 37.16, 30.7, 31.89, 29.36, 27.0, 22.36, 23.22,
            ],
            f_shwith200: vec![
                0.92, 0.94, 0.95, 0.95, 0.96, 0.96, 0.96, 0.95, 0.93, 0.94, 0.94, 0.92,
            ],
            f_shwith300: vec![
                0.84, 0.88, 0.87, 0.92, 0.91, 0.92, 0.92, 0.91, 0.86, 0.9, 0.87, 0.79,
            ],
            f_shwith500: vec![
                0.54, 0.66, 0.74, 0.77, 0.76, 0.77, 0.78, 0.77, 0.73, 0.72, 0.58, 0.49,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa2c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                5.1, 9.74, 21.2, 34.4, 50.05, 56.1, 54.34, 37.89, 24.06, 13.34, 5.35, 3.7,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 46.58, 44.49, 41.4, 35.24, 34.33, 28.52, 26.87,
            ],
            f_shwith200: vec![
                0.05, 0.27, 0.45, 0.61, 0.69, 0.71, 0.69, 0.64, 0.5, 0.33, 0.07, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.14, 0.31, 0.51, 0.6, 0.61, 0.62, 0.51, 0.38, 0.12, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.02, 0.23, 0.37, 0.38, 0.38, 0.27, 0.06, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa2c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                45.85, 48.36, 60.34, 63.84, 72.28, 71.44, 73.54, 63.77, 56.97, 55.74, 44.49, 42.68,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 46.58, 44.49, 41.4, 35.24, 34.33, 28.52, 26.87,
            ],
            f_shwith200: vec![
                0.77, 0.77, 0.77, 0.78, 0.79, 0.78, 0.78, 0.78, 0.76, 0.78, 0.77, 0.77,
            ],
            f_shwith300: vec![
                0.64, 0.69, 0.68, 0.69, 0.69, 0.66, 0.69, 0.67, 0.67, 0.69, 0.64, 0.61,
            ],
            f_shwith500: vec![
                0.36, 0.51, 0.51, 0.54, 0.57, 0.53, 0.55, 0.54, 0.53, 0.53, 0.33, 0.3,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa2c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                95.4, 82.3, 78.24, 61.71, 53.45, 44.93, 50.01, 56.13, 65.27, 88.11, 89.83, 94.24,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 46.58, 44.49, 41.4, 35.24, 34.33, 28.52, 26.87,
            ],
            f_shwith200: vec![
                0.91, 0.92, 0.85, 0.82, 0.74, 0.7, 0.71, 0.79, 0.8, 0.9, 0.92, 0.92,
            ],
            f_shwith300: vec![
                0.88, 0.86, 0.8, 0.71, 0.65, 0.56, 0.64, 0.68, 0.76, 0.83, 0.88, 0.85,
            ],
            f_shwith500: vec![
                0.71, 0.69, 0.61, 0.46, 0.31, 0.15, 0.24, 0.42, 0.58, 0.66, 0.67, 0.71,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa2c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                125.52, 101.91, 79.84, 41.47, 17.24, 6.64, 11.89, 30.28, 56.28, 104.3, 118.9, 126.0,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 46.58, 44.49, 41.4, 35.24, 34.33, 28.52, 26.87,
            ],
            f_shwith200: vec![
                0.96, 0.96, 0.89, 0.82, 0.57, 0.43, 0.48, 0.73, 0.84, 0.95, 0.97, 0.95,
            ],
            f_shwith300: vec![
                0.9, 0.89, 0.81, 0.62, 0.19, 0.0, 0.04, 0.48, 0.73, 0.86, 0.89, 0.89,
            ],
            f_shwith500: vec![
                0.76, 0.69, 0.47, 0.09, 0.0, 0.0, 0.0, 0.02, 0.32, 0.62, 0.71, 0.75,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa2c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                91.71, 80.22, 76.98, 64.47, 51.32, 43.93, 50.7, 55.76, 60.21, 85.51, 89.12, 91.09,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 46.58, 44.49, 41.4, 35.24, 34.33, 28.52, 26.87,
            ],
            f_shwith200: vec![
                0.92, 0.92, 0.84, 0.83, 0.73, 0.68, 0.72, 0.78, 0.79, 0.91, 0.93, 0.91,
            ],
            f_shwith300: vec![
                0.88, 0.85, 0.78, 0.73, 0.62, 0.55, 0.65, 0.67, 0.72, 0.82, 0.88, 0.87,
            ],
            f_shwith500: vec![
                0.69, 0.71, 0.59, 0.49, 0.31, 0.23, 0.23, 0.4, 0.53, 0.65, 0.66, 0.67,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa2c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                42.82, 46.13, 59.39, 67.34, 69.57, 69.26, 74.6, 63.4, 52.1, 53.52, 44.06, 40.27,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 46.58, 44.49, 41.4, 35.24, 34.33, 28.52, 26.87,
            ],
            f_shwith200: vec![
                0.77, 0.79, 0.76, 0.79, 0.78, 0.77, 0.78, 0.78, 0.73, 0.78, 0.78, 0.76,
            ],
            f_shwith300: vec![
                0.62, 0.68, 0.66, 0.7, 0.67, 0.65, 0.69, 0.67, 0.64, 0.66, 0.64, 0.6,
            ],
            f_shwith500: vec![
                0.3, 0.51, 0.51, 0.57, 0.53, 0.5, 0.56, 0.52, 0.5, 0.45, 0.31, 0.28,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa2c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                4.51, 8.66, 21.11, 36.59, 48.34, 54.02, 55.15, 37.75, 22.22, 12.78, 5.44, 3.44,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 46.58, 44.49, 41.4, 35.24, 34.33, 28.52, 26.87,
            ],
            f_shwith200: vec![
                0.03, 0.23, 0.44, 0.63, 0.67, 0.69, 0.7, 0.64, 0.46, 0.31, 0.05, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.09, 0.29, 0.52, 0.57, 0.6, 0.62, 0.5, 0.37, 0.1, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.04, 0.23, 0.35, 0.38, 0.36, 0.21, 0.1, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa2c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 2.44, 12.73, 21.68, 18.11, 4.65, 0.27, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 46.58, 44.49, 41.4, 35.24, 34.33, 28.52, 26.87,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.09, 0.25, 0.16, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa3c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                91.6, 104.6, 137.12, 154.56, 177.09, 179.72, 188.25, 169.64, 127.5, 125.21, 96.09,
                84.31,
            ],
            dif: vec![
                23.33, 21.95, 29.45, 31.66, 31.25, 34.42, 31.66, 28.49, 29.07, 25.61, 22.5, 22.92,
            ],
            f_shwith200: vec![
                0.92, 0.94, 0.95, 0.95, 0.96, 0.96, 0.96, 0.95, 0.94, 0.94, 0.94, 0.93,
            ],
            f_shwith300: vec![
                0.84, 0.88, 0.87, 0.92, 0.91, 0.92, 0.92, 0.92, 0.88, 0.88, 0.87, 0.78,
            ],
            f_shwith500: vec![
                0.54, 0.66, 0.74, 0.77, 0.76, 0.78, 0.78, 0.78, 0.7, 0.71, 0.6, 0.46,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa3c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                5.1, 9.74, 21.2, 34.4, 50.05, 55.76, 56.91, 40.65, 21.4, 13.21, 5.55, 3.2,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.33, 44.9, 41.09, 35.22, 34.18, 28.55, 26.93,
            ],
            f_shwith200: vec![
                0.05, 0.27, 0.45, 0.61, 0.69, 0.72, 0.7, 0.65, 0.45, 0.32, 0.05, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.14, 0.31, 0.51, 0.6, 0.64, 0.64, 0.55, 0.35, 0.13, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.02, 0.23, 0.37, 0.44, 0.41, 0.24, 0.1, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa3c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                45.85, 48.36, 60.34, 63.84, 72.28, 71.47, 76.67, 68.97, 50.55, 54.42, 44.76, 41.28,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.33, 44.9, 41.09, 35.22, 34.18, 28.55, 26.93,
            ],
            f_shwith200: vec![
                0.77, 0.77, 0.77, 0.78, 0.79, 0.78, 0.79, 0.79, 0.75, 0.79, 0.78, 0.76,
            ],
            f_shwith300: vec![
                0.64, 0.69, 0.68, 0.69, 0.69, 0.68, 0.7, 0.71, 0.63, 0.66, 0.64, 0.62,
            ],
            f_shwith500: vec![
                0.36, 0.51, 0.51, 0.54, 0.57, 0.55, 0.58, 0.58, 0.45, 0.47, 0.35, 0.27,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa3c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                95.4, 82.3, 78.24, 61.71, 53.45, 45.32, 51.86, 61.27, 58.92, 86.85, 89.41, 93.0,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.33, 44.9, 41.09, 35.22, 34.18, 28.55, 26.93,
            ],
            f_shwith200: vec![
                0.91, 0.92, 0.85, 0.82, 0.74, 0.71, 0.73, 0.8, 0.82, 0.91, 0.92, 0.91,
            ],
            f_shwith300: vec![
                0.88, 0.86, 0.8, 0.71, 0.65, 0.6, 0.65, 0.71, 0.72, 0.83, 0.88, 0.88,
            ],
            f_shwith500: vec![
                0.71, 0.69, 0.61, 0.46, 0.31, 0.21, 0.27, 0.45, 0.49, 0.66, 0.68, 0.67,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa3c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                125.52, 101.91, 79.84, 41.47, 17.24, 6.74, 11.79, 33.79, 54.57, 103.23, 118.37,
                126.57,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.33, 44.9, 41.09, 35.22, 34.18, 28.55, 26.93,
            ],
            f_shwith200: vec![
                0.96, 0.96, 0.89, 0.82, 0.57, 0.42, 0.48, 0.75, 0.84, 0.95, 0.97, 0.97,
            ],
            f_shwith300: vec![
                0.9, 0.89, 0.81, 0.62, 0.19, 0.0, 0.05, 0.5, 0.67, 0.86, 0.9, 0.89,
            ],
            f_shwith500: vec![
                0.76, 0.69, 0.47, 0.09, 0.0, 0.0, 0.0, 0.02, 0.31, 0.64, 0.71, 0.77,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa3c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                91.71, 80.22, 76.98, 64.47, 51.32, 45.64, 50.89, 59.86, 63.51, 84.62, 88.92, 92.99,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.33, 44.9, 41.09, 35.22, 34.18, 28.55, 26.93,
            ],
            f_shwith200: vec![
                0.92, 0.92, 0.84, 0.83, 0.73, 0.71, 0.71, 0.79, 0.82, 0.9, 0.92, 0.91,
            ],
            f_shwith300: vec![
                0.88, 0.85, 0.78, 0.73, 0.62, 0.59, 0.64, 0.7, 0.73, 0.81, 0.88, 0.87,
            ],
            f_shwith500: vec![
                0.69, 0.71, 0.59, 0.49, 0.31, 0.19, 0.26, 0.38, 0.52, 0.65, 0.67, 0.65,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa3c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                42.82, 46.13, 59.39, 67.34, 69.57, 72.07, 74.93, 66.8, 55.85, 52.17, 44.3, 41.7,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.33, 44.9, 41.09, 35.22, 34.18, 28.55, 26.93,
            ],
            f_shwith200: vec![
                0.77, 0.79, 0.76, 0.79, 0.78, 0.79, 0.78, 0.79, 0.76, 0.77, 0.79, 0.75,
            ],
            f_shwith300: vec![
                0.62, 0.68, 0.66, 0.7, 0.67, 0.68, 0.68, 0.69, 0.67, 0.65, 0.65, 0.61,
            ],
            f_shwith500: vec![
                0.3, 0.51, 0.51, 0.57, 0.53, 0.54, 0.57, 0.56, 0.5, 0.45, 0.32, 0.33,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa3c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                4.51, 8.66, 21.11, 36.59, 48.34, 56.28, 55.42, 39.0, 24.3, 12.26, 5.39, 3.79,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.33, 44.9, 41.09, 35.22, 34.18, 28.55, 26.93,
            ],
            f_shwith200: vec![
                0.03, 0.23, 0.44, 0.63, 0.67, 0.72, 0.69, 0.64, 0.5, 0.28, 0.06, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.09, 0.29, 0.52, 0.57, 0.63, 0.62, 0.53, 0.39, 0.1, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.04, 0.23, 0.35, 0.41, 0.4, 0.27, 0.11, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa3c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 2.44, 12.73, 21.65, 18.57, 4.46, 0.32, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.33, 44.9, 41.09, 35.22, 34.18, 28.55, 26.93,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.09, 0.29, 0.19, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa4c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                91.6, 104.6, 137.12, 154.56, 177.09, 179.63, 207.76, 186.24, 129.37, 125.4, 96.25,
                84.37,
            ],
            dif: vec![
                23.33, 21.95, 29.45, 31.66, 31.25, 35.15, 27.11, 25.2, 27.19, 25.42, 22.34, 22.84,
            ],
            f_shwith200: vec![
                0.92, 0.94, 0.95, 0.95, 0.96, 0.96, 0.97, 0.96, 0.94, 0.94, 0.93, 0.92,
            ],
            f_shwith300: vec![
                0.84, 0.88, 0.87, 0.92, 0.91, 0.92, 0.94, 0.94, 0.88, 0.89, 0.86, 0.8,
            ],
            f_shwith500: vec![
                0.54, 0.66, 0.74, 0.77, 0.76, 0.78, 0.81, 0.81, 0.71, 0.69, 0.58, 0.47,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa4c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                5.1, 9.74, 21.2, 34.4, 50.05, 55.65, 58.66, 45.17, 24.93, 12.96, 5.43, 3.36,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.74, 45.36, 41.68, 35.05, 34.21, 28.53, 26.97,
            ],
            f_shwith200: vec![
                0.05, 0.27, 0.45, 0.61, 0.69, 0.72, 0.71, 0.69, 0.51, 0.33, 0.09, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.14, 0.31, 0.51, 0.6, 0.65, 0.66, 0.58, 0.39, 0.13, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.02, 0.23, 0.37, 0.44, 0.42, 0.31, 0.1, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa4c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                45.85, 48.36, 60.34, 63.84, 72.28, 71.35, 80.07, 75.68, 58.07, 54.6, 44.2, 40.21,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.74, 45.36, 41.68, 35.05, 34.21, 28.53, 26.97,
            ],
            f_shwith200: vec![
                0.77, 0.77, 0.77, 0.78, 0.79, 0.78, 0.8, 0.82, 0.78, 0.78, 0.76, 0.76,
            ],
            f_shwith300: vec![
                0.64, 0.69, 0.68, 0.69, 0.69, 0.68, 0.71, 0.73, 0.68, 0.67, 0.65, 0.61,
            ],
            f_shwith500: vec![
                0.36, 0.51, 0.51, 0.54, 0.57, 0.55, 0.59, 0.59, 0.51, 0.48, 0.37, 0.24,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa4c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                95.4, 82.3, 78.24, 61.71, 53.45, 45.25, 54.98, 66.31, 66.05, 86.6, 87.9, 91.67,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.74, 45.36, 41.68, 35.05, 34.21, 28.53, 26.97,
            ],
            f_shwith200: vec![
                0.91, 0.92, 0.85, 0.82, 0.74, 0.71, 0.74, 0.83, 0.83, 0.9, 0.92, 0.91,
            ],
            f_shwith300: vec![
                0.88, 0.86, 0.8, 0.71, 0.65, 0.59, 0.67, 0.73, 0.76, 0.82, 0.87, 0.88,
            ],
            f_shwith500: vec![
                0.71, 0.69, 0.61, 0.46, 0.31, 0.22, 0.3, 0.47, 0.5, 0.65, 0.69, 0.66,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa4c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                125.52, 101.91, 79.84, 41.47, 17.24, 6.74, 13.49, 35.39, 56.57, 104.39, 118.3,
                125.91,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.74, 45.36, 41.68, 35.05, 34.21, 28.53, 26.97,
            ],
            f_shwith200: vec![
                0.96, 0.96, 0.89, 0.82, 0.57, 0.42, 0.52, 0.79, 0.84, 0.95, 0.97, 0.96,
            ],
            f_shwith300: vec![
                0.9, 0.89, 0.81, 0.62, 0.19, 0.0, 0.05, 0.54, 0.72, 0.87, 0.9, 0.9,
            ],
            f_shwith500: vec![
                0.76, 0.69, 0.47, 0.09, 0.0, 0.0, 0.0, 0.0, 0.26, 0.62, 0.7, 0.74,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa4c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                91.71, 80.22, 76.98, 64.47, 51.32, 45.6, 55.75, 66.29, 61.37, 87.78, 91.1, 92.96,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.74, 45.36, 41.68, 35.05, 34.21, 28.53, 26.97,
            ],
            f_shwith200: vec![
                0.92, 0.92, 0.84, 0.83, 0.73, 0.71, 0.74, 0.83, 0.81, 0.91, 0.93, 0.92,
            ],
            f_shwith300: vec![
                0.88, 0.85, 0.78, 0.73, 0.62, 0.58, 0.67, 0.75, 0.71, 0.82, 0.89, 0.87,
            ],
            f_shwith500: vec![
                0.69, 0.71, 0.59, 0.49, 0.31, 0.2, 0.29, 0.41, 0.52, 0.64, 0.68, 0.68,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa4c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                42.82, 46.13, 59.39, 67.34, 69.57, 72.0, 81.4, 75.79, 53.28, 56.01, 47.07, 41.02,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.74, 45.36, 41.68, 35.05, 34.21, 28.53, 26.97,
            ],
            f_shwith200: vec![
                0.77, 0.79, 0.76, 0.79, 0.78, 0.79, 0.81, 0.81, 0.75, 0.79, 0.78, 0.77,
            ],
            f_shwith300: vec![
                0.62, 0.68, 0.66, 0.7, 0.67, 0.68, 0.72, 0.73, 0.65, 0.68, 0.66, 0.6,
            ],
            f_shwith500: vec![
                0.3, 0.51, 0.51, 0.57, 0.53, 0.54, 0.59, 0.6, 0.48, 0.48, 0.38, 0.28,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa4c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                4.51, 8.66, 21.11, 36.59, 48.34, 56.22, 59.78, 45.35, 22.85, 13.78, 6.28, 3.21,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.74, 45.36, 41.68, 35.05, 34.21, 28.53, 26.97,
            ],
            f_shwith200: vec![
                0.03, 0.23, 0.44, 0.63, 0.67, 0.73, 0.72, 0.69, 0.48, 0.31, 0.09, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.09, 0.29, 0.52, 0.57, 0.64, 0.67, 0.59, 0.33, 0.11, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.04, 0.23, 0.35, 0.41, 0.41, 0.26, 0.1, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: Alfa4c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 2.44, 12.73, 21.61, 18.94, 5.62, 0.26, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                28.44, 28.48, 37.06, 40.79, 43.5, 45.74, 45.36, 41.68, 35.05, 34.21, 28.53, 26.97,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.09, 0.29, 0.19, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: B1c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                51.7, 63.86, 96.89, 121.23, 156.45, 165.14, 154.88, 137.74, 92.15, 86.52, 55.99,
                45.57,
            ],
            dif: vec![
                22.32, 25.2, 33.81, 35.68, 37.93, 41.02, 40.52, 36.28, 33.91, 27.43, 23.18, 20.53,
            ],
            f_shwith200: vec![
                0.79, 0.86, 0.9, 0.92, 0.94, 0.96, 0.95, 0.93, 0.91, 0.89, 0.82, 0.77,
            ],
            f_shwith300: vec![
                0.69, 0.72, 0.81, 0.86, 0.9, 0.91, 0.88, 0.89, 0.81, 0.79, 0.71, 0.63,
            ],
            f_shwith500: vec![
                0.46, 0.51, 0.58, 0.67, 0.72, 0.75, 0.72, 0.71, 0.64, 0.59, 0.51, 0.41,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B1c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                2.24, 5.62, 16.49, 28.57, 43.94, 49.63, 49.33, 32.68, 16.33, 9.14, 3.46, 2.26,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 46.98, 45.23, 40.59, 32.93, 29.07, 21.65, 19.17,
            ],
            f_shwith200: vec![
                0.04, 0.18, 0.38, 0.55, 0.64, 0.7, 0.67, 0.59, 0.43, 0.24, 0.09, 0.01,
            ],
            f_shwith300: vec![
                0.0, 0.08, 0.26, 0.44, 0.57, 0.59, 0.58, 0.46, 0.31, 0.09, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.03, 0.18, 0.33, 0.37, 0.36, 0.24, 0.04, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B1c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                22.78, 28.98, 43.82, 53.89, 62.8, 63.89, 66.07, 54.72, 39.87, 38.33, 27.1, 25.14,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 46.98, 45.23, 40.59, 32.93, 29.07, 21.65, 19.17,
            ],
            f_shwith200: vec![
                0.63, 0.66, 0.68, 0.75, 0.75, 0.76, 0.76, 0.74, 0.71, 0.69, 0.66, 0.65,
            ],
            f_shwith300: vec![
                0.49, 0.57, 0.6, 0.66, 0.65, 0.64, 0.65, 0.65, 0.61, 0.59, 0.57, 0.48,
            ],
            f_shwith500: vec![
                0.35, 0.39, 0.41, 0.5, 0.52, 0.48, 0.5, 0.48, 0.47, 0.37, 0.39, 0.32,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B1c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                50.2, 49.29, 55.25, 52.24, 45.84, 40.73, 44.36, 48.02, 47.07, 61.09, 52.56, 54.09,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 46.98, 45.23, 40.59, 32.93, 29.07, 21.65, 19.17,
            ],
            f_shwith200: vec![
                0.82, 0.81, 0.78, 0.78, 0.7, 0.68, 0.68, 0.76, 0.77, 0.84, 0.82, 0.83,
            ],
            f_shwith300: vec![
                0.74, 0.74, 0.69, 0.68, 0.61, 0.55, 0.58, 0.63, 0.71, 0.75, 0.78, 0.75,
            ],
            f_shwith500: vec![
                0.58, 0.56, 0.47, 0.39, 0.26, 0.21, 0.24, 0.33, 0.5, 0.55, 0.63, 0.57,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B1c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                71.33, 63.0, 56.09, 33.39, 14.62, 6.3, 9.41, 26.12, 43.18, 72.69, 67.91, 68.16,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 46.98, 45.23, 40.59, 32.93, 29.07, 21.65, 19.17,
            ],
            f_shwith200: vec![
                0.88, 0.87, 0.82, 0.75, 0.56, 0.42, 0.45, 0.71, 0.79, 0.88, 0.88, 0.87,
            ],
            f_shwith300: vec![
                0.81, 0.76, 0.67, 0.53, 0.15, 0.0, 0.04, 0.43, 0.66, 0.78, 0.8, 0.78,
            ],
            f_shwith500: vec![
                0.66, 0.55, 0.39, 0.09, 0.0, 0.0, 0.0, 0.02, 0.36, 0.54, 0.64, 0.64,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B1c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                55.81, 51.63, 57.49, 46.83, 47.34, 40.69, 41.52, 48.6, 45.98, 59.58, 50.25, 45.89,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 46.98, 45.23, 40.59, 32.93, 29.07, 21.65, 19.17,
            ],
            f_shwith200: vec![
                0.84, 0.82, 0.78, 0.74, 0.72, 0.67, 0.66, 0.76, 0.76, 0.85, 0.8, 0.81,
            ],
            f_shwith300: vec![
                0.77, 0.75, 0.67, 0.61, 0.64, 0.55, 0.55, 0.63, 0.69, 0.78, 0.76, 0.73,
            ],
            f_shwith500: vec![
                0.59, 0.6, 0.5, 0.38, 0.26, 0.14, 0.22, 0.33, 0.48, 0.54, 0.64, 0.53,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B1c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                27.22, 31.04, 45.71, 48.11, 64.63, 64.27, 62.16, 55.12, 38.76, 36.96, 25.35, 18.7,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 46.98, 45.23, 40.59, 32.93, 29.07, 21.65, 19.17,
            ],
            f_shwith200: vec![
                0.66, 0.68, 0.69, 0.71, 0.75, 0.77, 0.74, 0.75, 0.69, 0.71, 0.65, 0.54,
            ],
            f_shwith300: vec![
                0.55, 0.6, 0.61, 0.59, 0.68, 0.65, 0.62, 0.62, 0.58, 0.59, 0.55, 0.43,
            ],
            f_shwith500: vec![
                0.39, 0.42, 0.44, 0.43, 0.5, 0.48, 0.49, 0.47, 0.44, 0.36, 0.37, 0.22,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B1c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                2.9, 6.2, 16.92, 25.81, 45.04, 50.21, 46.65, 32.67, 15.86, 8.73, 3.31, 1.33,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 46.98, 45.23, 40.59, 32.93, 29.07, 21.65, 19.17,
            ],
            f_shwith200: vec![
                0.02, 0.19, 0.39, 0.5, 0.65, 0.7, 0.65, 0.59, 0.41, 0.22, 0.09, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.1, 0.25, 0.37, 0.59, 0.6, 0.55, 0.47, 0.29, 0.13, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.08, 0.18, 0.31, 0.35, 0.34, 0.24, 0.03, 0.01, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B1c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.79, 11.65, 19.33, 16.55, 4.0, 0.15, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 46.98, 45.23, 40.59, 32.93, 29.07, 21.65, 19.17,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.09, 0.27, 0.18, 0.02, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: B2c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                51.7, 63.86, 96.89, 121.23, 156.45, 153.71, 184.38, 161.5, 109.03, 85.52, 57.8,
                45.2,
            ],
            dif: vec![
                22.32, 25.2, 33.81, 35.68, 37.93, 40.47, 32.34, 32.18, 30.24, 28.44, 21.35, 20.89,
            ],
            f_shwith200: vec![
                0.79, 0.86, 0.9, 0.92, 0.94, 0.95, 0.96, 0.95, 0.92, 0.89, 0.83, 0.76,
            ],
            f_shwith300: vec![
                0.69, 0.72, 0.81, 0.86, 0.9, 0.9, 0.91, 0.9, 0.84, 0.76, 0.7, 0.6,
            ],
            f_shwith500: vec![
                0.46, 0.51, 0.58, 0.67, 0.72, 0.75, 0.78, 0.76, 0.63, 0.55, 0.47, 0.41,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B2c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                2.24, 5.62, 16.49, 28.57, 43.94, 45.48, 53.55, 42.59, 20.38, 10.34, 3.09, 2.83,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.79, 44.49, 41.53, 33.44, 29.29, 21.61, 19.25,
            ],
            f_shwith200: vec![
                0.04, 0.18, 0.38, 0.55, 0.64, 0.66, 0.69, 0.65, 0.45, 0.27, 0.02, 0.01,
            ],
            f_shwith300: vec![
                0.0, 0.08, 0.26, 0.44, 0.57, 0.56, 0.62, 0.56, 0.34, 0.12, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.03, 0.18, 0.33, 0.35, 0.41, 0.27, 0.16, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B2c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                22.78, 28.98, 43.82, 53.89, 62.8, 58.6, 72.1, 69.68, 48.02, 40.5, 27.38, 27.18,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.79, 44.49, 41.53, 33.44, 29.29, 21.61, 19.25,
            ],
            f_shwith200: vec![
                0.63, 0.66, 0.68, 0.75, 0.75, 0.74, 0.77, 0.79, 0.72, 0.7, 0.68, 0.66,
            ],
            f_shwith300: vec![
                0.49, 0.57, 0.6, 0.66, 0.65, 0.61, 0.69, 0.7, 0.64, 0.6, 0.54, 0.56,
            ],
            f_shwith500: vec![
                0.35, 0.39, 0.41, 0.5, 0.52, 0.46, 0.57, 0.58, 0.48, 0.44, 0.25, 0.38,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B2c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                50.2, 49.29, 55.25, 52.24, 45.84, 37.4, 48.69, 59.6, 55.62, 62.65, 55.46, 55.09,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.79, 44.49, 41.53, 33.44, 29.29, 21.61, 19.25,
            ],
            f_shwith200: vec![
                0.82, 0.81, 0.78, 0.78, 0.7, 0.66, 0.71, 0.79, 0.78, 0.84, 0.85, 0.84,
            ],
            f_shwith300: vec![
                0.74, 0.74, 0.69, 0.68, 0.61, 0.52, 0.63, 0.68, 0.69, 0.75, 0.76, 0.76,
            ],
            f_shwith500: vec![
                0.58, 0.56, 0.47, 0.39, 0.26, 0.18, 0.26, 0.4, 0.5, 0.57, 0.57, 0.61,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B2c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                71.33, 63.0, 56.09, 33.39, 14.62, 5.89, 11.04, 29.17, 51.59, 73.26, 71.72, 69.82,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.79, 44.49, 41.53, 33.44, 29.29, 21.61, 19.25,
            ],
            f_shwith200: vec![
                0.88, 0.87, 0.82, 0.75, 0.56, 0.39, 0.49, 0.72, 0.82, 0.88, 0.89, 0.88,
            ],
            f_shwith300: vec![
                0.81, 0.76, 0.67, 0.53, 0.15, 0.0, 0.02, 0.45, 0.67, 0.76, 0.8, 0.78,
            ],
            f_shwith500: vec![
                0.66, 0.55, 0.39, 0.09, 0.0, 0.0, 0.0, 0.01, 0.37, 0.52, 0.59, 0.61,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B2c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                55.81, 51.63, 57.49, 46.83, 47.34, 38.7, 49.8, 52.7, 56.2, 60.7, 52.33, 48.57,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.79, 44.49, 41.53, 33.44, 29.29, 21.61, 19.25,
            ],
            f_shwith200: vec![
                0.84, 0.82, 0.78, 0.74, 0.72, 0.66, 0.72, 0.76, 0.78, 0.82, 0.83, 0.8,
            ],
            f_shwith300: vec![
                0.77, 0.75, 0.67, 0.61, 0.64, 0.53, 0.64, 0.65, 0.68, 0.73, 0.76, 0.71,
            ],
            f_shwith500: vec![
                0.59, 0.6, 0.5, 0.38, 0.26, 0.16, 0.25, 0.37, 0.51, 0.56, 0.58, 0.55,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B2c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                27.22, 31.04, 45.71, 48.11, 64.63, 61.16, 73.78, 60.12, 47.26, 38.47, 25.29, 22.06,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.79, 44.49, 41.53, 33.44, 29.29, 21.61, 19.25,
            ],
            f_shwith200: vec![
                0.66, 0.68, 0.69, 0.71, 0.75, 0.75, 0.79, 0.76, 0.72, 0.68, 0.64, 0.59,
            ],
            f_shwith300: vec![
                0.55, 0.6, 0.61, 0.59, 0.68, 0.64, 0.7, 0.65, 0.61, 0.59, 0.49, 0.48,
            ],
            f_shwith500: vec![
                0.39, 0.42, 0.44, 0.43, 0.5, 0.47, 0.55, 0.51, 0.47, 0.46, 0.32, 0.24,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B2c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                2.9, 6.2, 16.92, 25.81, 45.04, 47.8, 54.83, 35.96, 18.74, 9.42, 3.27, 2.1,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.79, 44.49, 41.53, 33.44, 29.29, 21.61, 19.25,
            ],
            f_shwith200: vec![
                0.02, 0.19, 0.39, 0.5, 0.65, 0.68, 0.7, 0.61, 0.43, 0.25, 0.09, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.1, 0.25, 0.37, 0.59, 0.58, 0.63, 0.48, 0.32, 0.14, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.08, 0.18, 0.31, 0.35, 0.42, 0.24, 0.06, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B2c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.79, 11.65, 18.03, 18.03, 5.3, 0.18, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.79, 44.49, 41.53, 33.44, 29.29, 21.61, 19.25,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.09, 0.24, 0.22, 0.02, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: B3c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                51.7, 63.86, 96.89, 121.23, 156.45, 169.13, 188.2, 168.72, 121.57, 85.47, 57.0,
                44.82,
            ],
            dif: vec![
                22.32, 25.2, 33.81, 35.68, 37.93, 35.4, 31.71, 29.4, 28.81, 28.5, 22.12, 21.29,
            ],
            f_shwith200: vec![
                0.79, 0.86, 0.9, 0.92, 0.94, 0.95, 0.96, 0.95, 0.93, 0.88, 0.81, 0.78,
            ],
            f_shwith300: vec![
                0.69, 0.72, 0.81, 0.86, 0.9, 0.9, 0.91, 0.92, 0.86, 0.78, 0.66, 0.66,
            ],
            f_shwith500: vec![
                0.46, 0.51, 0.58, 0.67, 0.72, 0.77, 0.79, 0.78, 0.68, 0.6, 0.45, 0.39,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                2.24, 5.62, 16.49, 28.57, 43.94, 50.88, 58.18, 41.03, 22.33, 10.2, 3.93, 1.9,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.27, 44.8, 41.24, 34.72, 29.21, 21.61, 19.23,
            ],
            f_shwith200: vec![
                0.04, 0.18, 0.38, 0.55, 0.64, 0.68, 0.7, 0.66, 0.48, 0.27, 0.09, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.08, 0.26, 0.44, 0.57, 0.59, 0.64, 0.55, 0.36, 0.14, 0.01, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.03, 0.18, 0.33, 0.39, 0.45, 0.26, 0.06, 0.01, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                22.78, 28.98, 43.82, 53.89, 62.8, 65.49, 77.71, 68.82, 51.97, 39.85, 29.24, 21.79,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.27, 44.8, 41.24, 34.72, 29.21, 21.61, 19.23,
            ],
            f_shwith200: vec![
                0.63, 0.66, 0.68, 0.75, 0.75, 0.77, 0.78, 0.79, 0.75, 0.7, 0.67, 0.64,
            ],
            f_shwith300: vec![
                0.49, 0.57, 0.6, 0.66, 0.65, 0.65, 0.69, 0.7, 0.64, 0.59, 0.57, 0.5,
            ],
            f_shwith500: vec![
                0.35, 0.39, 0.41, 0.5, 0.52, 0.5, 0.57, 0.58, 0.47, 0.43, 0.38, 0.24,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                50.2, 49.29, 55.25, 52.24, 45.84, 41.74, 52.04, 60.35, 59.64, 60.75, 55.71, 49.0,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.27, 44.8, 41.24, 34.72, 29.21, 21.61, 19.23,
            ],
            f_shwith200: vec![
                0.82, 0.81, 0.78, 0.78, 0.7, 0.68, 0.71, 0.8, 0.82, 0.83, 0.83, 0.84,
            ],
            f_shwith300: vec![
                0.74, 0.74, 0.69, 0.68, 0.61, 0.55, 0.65, 0.7, 0.71, 0.74, 0.76, 0.78,
            ],
            f_shwith500: vec![
                0.58, 0.56, 0.47, 0.39, 0.26, 0.23, 0.27, 0.43, 0.48, 0.57, 0.56, 0.57,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                71.33, 63.0, 56.09, 33.39, 14.62, 6.46, 11.54, 32.32, 54.52, 71.38, 71.38, 67.71,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.27, 44.8, 41.24, 34.72, 29.21, 21.61, 19.23,
            ],
            f_shwith200: vec![
                0.88, 0.87, 0.82, 0.75, 0.56, 0.4, 0.5, 0.75, 0.84, 0.87, 0.87, 0.89,
            ],
            f_shwith300: vec![
                0.81, 0.76, 0.67, 0.53, 0.15, 0.0, 0.03, 0.49, 0.69, 0.77, 0.78, 0.82,
            ],
            f_shwith500: vec![
                0.66, 0.55, 0.39, 0.09, 0.0, 0.0, 0.0, 0.0, 0.29, 0.56, 0.58, 0.66,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                55.81, 51.63, 57.49, 46.83, 47.34, 41.66, 50.76, 59.38, 62.94, 60.08, 52.76, 50.64,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.27, 44.8, 41.24, 34.72, 29.21, 21.61, 19.23,
            ],
            f_shwith200: vec![
                0.84, 0.82, 0.78, 0.74, 0.72, 0.67, 0.72, 0.79, 0.83, 0.82, 0.8, 0.83,
            ],
            f_shwith300: vec![
                0.77, 0.75, 0.67, 0.61, 0.64, 0.57, 0.63, 0.71, 0.72, 0.74, 0.72, 0.79,
            ],
            f_shwith500: vec![
                0.59, 0.6, 0.5, 0.38, 0.26, 0.16, 0.27, 0.42, 0.52, 0.6, 0.59, 0.61,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                27.22, 31.04, 45.71, 48.11, 64.63, 65.81, 75.29, 67.24, 55.13, 39.01, 26.91, 23.01,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.27, 44.8, 41.24, 34.72, 29.21, 21.61, 19.23,
            ],
            f_shwith200: vec![
                0.66, 0.68, 0.69, 0.71, 0.75, 0.76, 0.79, 0.78, 0.77, 0.7, 0.63, 0.65,
            ],
            f_shwith300: vec![
                0.55, 0.6, 0.61, 0.59, 0.68, 0.65, 0.69, 0.7, 0.65, 0.59, 0.54, 0.58,
            ],
            f_shwith500: vec![
                0.39, 0.42, 0.44, 0.43, 0.5, 0.51, 0.54, 0.55, 0.5, 0.45, 0.4, 0.29,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                2.9, 6.2, 16.92, 25.81, 45.04, 51.42, 56.03, 39.76, 23.51, 9.69, 3.59, 1.98,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.27, 44.8, 41.24, 34.72, 29.21, 21.61, 19.23,
            ],
            f_shwith200: vec![
                0.02, 0.19, 0.39, 0.5, 0.65, 0.68, 0.7, 0.64, 0.48, 0.25, 0.09, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.1, 0.25, 0.37, 0.59, 0.61, 0.65, 0.55, 0.39, 0.11, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.08, 0.18, 0.31, 0.38, 0.41, 0.25, 0.09, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.79, 11.65, 19.82, 19.61, 4.79, 0.25, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 44.27, 44.8, 41.24, 34.72, 29.21, 21.61, 19.23,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.09, 0.24, 0.22, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: B4c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                51.7, 63.86, 96.89, 121.23, 156.45, 179.93, 206.46, 183.9, 129.37, 85.91, 56.82,
                45.35,
            ],
            dif: vec![
                22.32, 25.2, 33.81, 35.68, 37.93, 34.83, 28.41, 27.55, 27.21, 28.05, 22.31, 20.75,
            ],
            f_shwith200: vec![
                0.79, 0.86, 0.9, 0.92, 0.94, 0.97, 0.97, 0.96, 0.94, 0.88, 0.8, 0.77,
            ],
            f_shwith300: vec![
                0.69, 0.72, 0.81, 0.86, 0.9, 0.91, 0.93, 0.93, 0.88, 0.77, 0.68, 0.63,
            ],
            f_shwith500: vec![
                0.46, 0.51, 0.58, 0.67, 0.72, 0.77, 0.82, 0.79, 0.68, 0.55, 0.46, 0.41,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                2.24, 5.62, 16.49, 28.57, 43.94, 61.62, 61.14, 45.58, 24.75, 11.24, 2.89, 2.29,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 45.67, 45.55, 41.89, 35.09, 29.25, 21.55, 19.24,
            ],
            f_shwith200: vec![
                0.04, 0.18, 0.38, 0.55, 0.64, 0.73, 0.73, 0.68, 0.5, 0.29, 0.05, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.08, 0.26, 0.44, 0.57, 0.68, 0.66, 0.57, 0.38, 0.15, 0.01, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.03, 0.18, 0.33, 0.46, 0.42, 0.32, 0.09, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                22.78, 28.98, 43.82, 53.89, 62.8, 77.54, 82.57, 75.9, 58.3, 42.53, 25.35, 23.17,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 45.67, 45.55, 41.89, 35.09, 29.25, 21.55, 19.24,
            ],
            f_shwith200: vec![
                0.63, 0.66, 0.68, 0.75, 0.75, 0.8, 0.81, 0.8, 0.77, 0.69, 0.63, 0.61,
            ],
            f_shwith300: vec![
                0.49, 0.57, 0.6, 0.66, 0.65, 0.71, 0.71, 0.73, 0.68, 0.63, 0.51, 0.49,
            ],
            f_shwith500: vec![
                0.35, 0.39, 0.41, 0.5, 0.52, 0.56, 0.59, 0.63, 0.48, 0.46, 0.33, 0.34,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                50.2, 49.29, 55.25, 52.24, 45.84, 48.04, 55.98, 66.01, 66.97, 63.38, 51.94, 50.4,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 45.67, 45.55, 41.89, 35.09, 29.25, 21.55, 19.24,
            ],
            f_shwith200: vec![
                0.82, 0.81, 0.78, 0.78, 0.7, 0.72, 0.74, 0.82, 0.84, 0.83, 0.81, 0.83,
            ],
            f_shwith300: vec![
                0.74, 0.74, 0.69, 0.68, 0.61, 0.61, 0.67, 0.73, 0.74, 0.72, 0.74, 0.76,
            ],
            f_shwith500: vec![
                0.58, 0.56, 0.47, 0.39, 0.26, 0.19, 0.28, 0.5, 0.53, 0.58, 0.54, 0.52,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                71.33, 63.0, 56.09, 33.39, 14.62, 6.74, 12.76, 34.01, 59.04, 71.45, 70.92, 69.14,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 45.67, 45.55, 41.89, 35.09, 29.25, 21.55, 19.24,
            ],
            f_shwith200: vec![
                0.88, 0.87, 0.82, 0.75, 0.56, 0.41, 0.53, 0.76, 0.86, 0.88, 0.86, 0.87,
            ],
            f_shwith300: vec![
                0.81, 0.76, 0.67, 0.53, 0.15, 0.0, 0.04, 0.52, 0.7, 0.75, 0.8, 0.8,
            ],
            f_shwith500: vec![
                0.66, 0.55, 0.39, 0.09, 0.0, 0.0, 0.0, 0.0, 0.3, 0.49, 0.58, 0.64,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                55.81, 51.63, 57.49, 46.83, 47.34, 45.18, 54.72, 61.41, 64.16, 57.95, 54.9, 51.74,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 45.67, 45.55, 41.89, 35.09, 29.25, 21.55, 19.24,
            ],
            f_shwith200: vec![
                0.84, 0.82, 0.78, 0.74, 0.72, 0.69, 0.73, 0.8, 0.81, 0.82, 0.81, 0.83,
            ],
            f_shwith300: vec![
                0.77, 0.75, 0.67, 0.61, 0.64, 0.58, 0.66, 0.72, 0.73, 0.73, 0.78, 0.77,
            ],
            f_shwith500: vec![
                0.59, 0.6, 0.5, 0.38, 0.26, 0.19, 0.28, 0.41, 0.53, 0.58, 0.56, 0.6,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                27.22, 31.04, 45.71, 48.11, 64.63, 71.64, 80.92, 69.7, 55.16, 37.14, 27.98, 23.96,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 45.67, 45.55, 41.89, 35.09, 29.25, 21.55, 19.24,
            ],
            f_shwith200: vec![
                0.66, 0.68, 0.69, 0.71, 0.75, 0.79, 0.8, 0.8, 0.75, 0.68, 0.66, 0.65,
            ],
            f_shwith300: vec![
                0.55, 0.6, 0.61, 0.59, 0.68, 0.69, 0.7, 0.7, 0.64, 0.59, 0.54, 0.53,
            ],
            f_shwith500: vec![
                0.39, 0.42, 0.44, 0.43, 0.5, 0.53, 0.57, 0.58, 0.47, 0.44, 0.41, 0.36,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                2.9, 6.2, 16.92, 25.81, 45.04, 56.14, 60.07, 41.42, 23.12, 9.05, 3.66, 2.07,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 45.67, 45.55, 41.89, 35.09, 29.25, 21.55, 19.24,
            ],
            f_shwith200: vec![
                0.02, 0.19, 0.39, 0.5, 0.65, 0.72, 0.72, 0.66, 0.47, 0.24, 0.09, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.1, 0.25, 0.37, 0.59, 0.65, 0.65, 0.54, 0.36, 0.13, 0.02, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.08, 0.18, 0.31, 0.4, 0.4, 0.24, 0.12, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.79, 11.65, 24.08, 20.19, 5.43, 0.17, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                20.65, 24.17, 33.6, 38.26, 44.5, 45.67, 45.55, 41.89, 35.09, 29.25, 21.55, 19.24,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.09, 0.31, 0.2, 0.01, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: C1c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                34.16, 44.65, 77.17, 91.69, 125.97, 134.08, 157.66, 136.28, 90.85, 66.2, 39.18,
                27.72,
            ],
            dif: vec![
                22.6, 26.32, 35.43, 44.32, 45.41, 45.52, 37.74, 37.72, 35.23, 30.03, 23.07, 21.5,
            ],
            f_shwith200: vec![
                0.75, 0.77, 0.87, 0.89, 0.94, 0.94, 0.95, 0.93, 0.9, 0.83, 0.71, 0.64,
            ],
            f_shwith300: vec![
                0.61, 0.61, 0.78, 0.82, 0.88, 0.87, 0.9, 0.89, 0.79, 0.72, 0.6, 0.52,
            ],
            f_shwith500: vec![
                0.35, 0.47, 0.56, 0.62, 0.66, 0.7, 0.74, 0.7, 0.59, 0.51, 0.37, 0.36,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                2.07, 3.65, 13.91, 21.13, 36.74, 40.64, 46.55, 32.84, 17.23, 7.77, 2.35, 1.57,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.72, 44.91, 40.72, 33.17, 27.2, 19.28, 16.38,
            ],
            f_shwith200: vec![
                0.05, 0.11, 0.36, 0.49, 0.59, 0.62, 0.65, 0.61, 0.41, 0.24, 0.06, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.05, 0.24, 0.38, 0.48, 0.52, 0.55, 0.47, 0.32, 0.14, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.06, 0.15, 0.26, 0.31, 0.29, 0.24, 0.06, 0.01, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                17.24, 19.41, 38.82, 39.71, 51.79, 52.18, 62.31, 54.5, 42.71, 28.81, 19.56, 15.42,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.72, 44.91, 40.72, 33.17, 27.2, 19.28, 16.38,
            ],
            f_shwith200: vec![
                0.59, 0.6, 0.68, 0.67, 0.7, 0.7, 0.74, 0.74, 0.69, 0.63, 0.57, 0.55,
            ],
            f_shwith300: vec![
                0.48, 0.47, 0.58, 0.59, 0.6, 0.57, 0.63, 0.64, 0.6, 0.52, 0.49, 0.44,
            ],
            f_shwith500: vec![
                0.31, 0.25, 0.43, 0.44, 0.4, 0.41, 0.45, 0.49, 0.48, 0.37, 0.33, 0.32,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                35.08, 33.99, 49.67, 38.7, 37.18, 33.15, 41.81, 47.47, 50.5, 43.72, 38.59, 32.38,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.72, 44.91, 40.72, 33.17, 27.2, 19.28, 16.38,
            ],
            f_shwith200: vec![
                0.77, 0.76, 0.77, 0.71, 0.66, 0.59, 0.66, 0.74, 0.76, 0.77, 0.78, 0.74,
            ],
            f_shwith300: vec![
                0.71, 0.67, 0.67, 0.61, 0.53, 0.49, 0.56, 0.62, 0.68, 0.66, 0.69, 0.63,
            ],
            f_shwith500: vec![
                0.51, 0.45, 0.47, 0.37, 0.19, 0.16, 0.18, 0.35, 0.49, 0.5, 0.5, 0.53,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                46.07, 43.85, 48.59, 26.19, 11.06, 5.18, 9.59, 25.66, 45.01, 52.41, 48.72, 42.33,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.72, 44.91, 40.72, 33.17, 27.2, 19.28, 16.38,
            ],
            f_shwith200: vec![
                0.84, 0.8, 0.8, 0.68, 0.49, 0.36, 0.46, 0.68, 0.78, 0.8, 0.82, 0.78,
            ],
            f_shwith300: vec![
                0.74, 0.66, 0.67, 0.5, 0.09, 0.0, 0.03, 0.43, 0.64, 0.68, 0.71, 0.7,
            ],
            f_shwith500: vec![
                0.58, 0.49, 0.42, 0.11, 0.0, 0.0, 0.0, 0.02, 0.39, 0.44, 0.52, 0.55,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                33.76, 36.54, 45.91, 36.75, 38.8, 33.57, 43.51, 47.81, 43.92, 47.19, 35.03, 30.33,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.72, 44.91, 40.72, 33.17, 27.2, 19.28, 16.38,
            ],
            f_shwith200: vec![
                0.78, 0.76, 0.76, 0.67, 0.68, 0.63, 0.68, 0.75, 0.74, 0.79, 0.76, 0.73,
            ],
            f_shwith300: vec![
                0.68, 0.66, 0.64, 0.56, 0.53, 0.46, 0.57, 0.64, 0.64, 0.66, 0.65, 0.62,
            ],
            f_shwith500: vec![
                0.48, 0.5, 0.48, 0.34, 0.16, 0.2, 0.17, 0.36, 0.43, 0.51, 0.42, 0.5,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                15.97, 22.07, 35.48, 36.89, 54.25, 52.32, 64.45, 53.96, 35.55, 32.14, 17.05, 13.76,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.72, 44.91, 40.72, 33.17, 27.2, 19.28, 16.38,
            ],
            f_shwith200: vec![
                0.56, 0.59, 0.67, 0.65, 0.72, 0.71, 0.75, 0.74, 0.66, 0.66, 0.51, 0.5,
            ],
            f_shwith300: vec![
                0.42, 0.48, 0.55, 0.53, 0.6, 0.57, 0.65, 0.62, 0.52, 0.54, 0.41, 0.39,
            ],
            f_shwith500: vec![
                0.34, 0.38, 0.39, 0.4, 0.43, 0.42, 0.45, 0.5, 0.38, 0.41, 0.28, 0.25,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.61, 4.86, 12.95, 19.1, 38.6, 40.42, 47.87, 31.73, 13.69, 9.02, 2.37, 1.28,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.72, 44.91, 40.72, 33.17, 27.2, 19.28, 16.38,
            ],
            f_shwith200: vec![
                0.03, 0.18, 0.35, 0.45, 0.61, 0.62, 0.66, 0.59, 0.35, 0.25, 0.06, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.11, 0.18, 0.35, 0.49, 0.51, 0.57, 0.45, 0.25, 0.16, 0.02, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.04, 0.13, 0.25, 0.31, 0.29, 0.26, 0.03, 0.03, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.28, 10.61, 15.32, 16.02, 3.93, 0.1, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.72, 44.91, 40.72, 33.17, 27.2, 19.28, 16.38,
            ],
            f_shwith200: vec![0.0, 0.0, 0.0, 0.01, 0.1, 0.2, 0.14, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: C2c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                34.16, 44.65, 77.17, 91.69, 125.97, 154.54, 185.75, 161.38, 108.32, 65.06, 38.94,
                27.01,
            ],
            dif: vec![
                22.6, 26.32, 35.43, 44.32, 45.41, 39.62, 30.98, 32.32, 30.95, 31.15, 23.31, 22.22,
            ],
            f_shwith200: vec![
                0.75, 0.77, 0.87, 0.89, 0.94, 0.96, 0.96, 0.95, 0.92, 0.84, 0.76, 0.64,
            ],
            f_shwith300: vec![
                0.61, 0.61, 0.78, 0.82, 0.88, 0.9, 0.91, 0.91, 0.83, 0.69, 0.62, 0.5,
            ],
            f_shwith500: vec![
                0.35, 0.47, 0.56, 0.62, 0.66, 0.76, 0.79, 0.74, 0.63, 0.48, 0.38, 0.37,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                2.07, 3.65, 13.91, 21.13, 36.74, 52.69, 56.5, 40.35, 21.58, 8.29, 2.55, 1.17,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.86, 44.37, 41.48, 33.55, 27.23, 19.33, 16.36,
            ],
            f_shwith200: vec![
                0.05, 0.11, 0.36, 0.49, 0.59, 0.7, 0.7, 0.66, 0.48, 0.23, 0.05, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.05, 0.24, 0.38, 0.48, 0.6, 0.64, 0.54, 0.34, 0.13, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.06, 0.15, 0.26, 0.39, 0.42, 0.29, 0.07, 0.03, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                17.24, 19.41, 38.82, 39.71, 51.79, 66.51, 75.64, 66.69, 51.44, 30.59, 19.61, 14.01,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.86, 44.37, 41.48, 33.55, 27.23, 19.33, 16.36,
            ],
            f_shwith200: vec![
                0.59, 0.6, 0.68, 0.67, 0.7, 0.78, 0.79, 0.79, 0.74, 0.62, 0.58, 0.52,
            ],
            f_shwith300: vec![
                0.48, 0.47, 0.58, 0.59, 0.6, 0.65, 0.7, 0.68, 0.67, 0.53, 0.47, 0.42,
            ],
            f_shwith500: vec![
                0.31, 0.25, 0.43, 0.44, 0.4, 0.5, 0.55, 0.54, 0.44, 0.38, 0.31, 0.29,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                35.08, 33.99, 49.67, 38.7, 37.18, 41.38, 50.77, 57.53, 59.06, 46.75, 38.26, 29.16,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.86, 44.37, 41.48, 33.55, 27.23, 19.33, 16.36,
            ],
            f_shwith200: vec![
                0.77, 0.76, 0.77, 0.71, 0.66, 0.68, 0.73, 0.8, 0.8, 0.76, 0.77, 0.7,
            ],
            f_shwith300: vec![
                0.71, 0.67, 0.67, 0.61, 0.53, 0.56, 0.65, 0.68, 0.72, 0.66, 0.68, 0.64,
            ],
            f_shwith500: vec![
                0.51, 0.45, 0.47, 0.37, 0.19, 0.19, 0.29, 0.39, 0.49, 0.47, 0.51, 0.52,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                46.07, 43.85, 48.59, 26.19, 11.06, 5.65, 11.32, 28.54, 50.35, 55.61, 50.08, 41.22,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.86, 44.37, 41.48, 33.55, 27.23, 19.33, 16.36,
            ],
            f_shwith200: vec![
                0.84, 0.8, 0.8, 0.68, 0.49, 0.39, 0.51, 0.74, 0.8, 0.81, 0.84, 0.79,
            ],
            f_shwith300: vec![
                0.74, 0.66, 0.67, 0.5, 0.09, 0.0, 0.03, 0.41, 0.65, 0.66, 0.72, 0.71,
            ],
            f_shwith500: vec![
                0.58, 0.49, 0.42, 0.11, 0.0, 0.0, 0.0, 0.0, 0.35, 0.47, 0.57, 0.6,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                33.76, 36.54, 45.91, 36.75, 38.8, 37.45, 47.85, 52.61, 50.99, 47.94, 37.28, 31.98,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.86, 44.37, 41.48, 33.55, 27.23, 19.33, 16.36,
            ],
            f_shwith200: vec![
                0.78, 0.76, 0.76, 0.67, 0.68, 0.66, 0.71, 0.77, 0.75, 0.77, 0.78, 0.73,
            ],
            f_shwith300: vec![
                0.68, 0.66, 0.64, 0.56, 0.53, 0.53, 0.64, 0.62, 0.65, 0.65, 0.71, 0.68,
            ],
            f_shwith500: vec![
                0.48, 0.5, 0.48, 0.34, 0.16, 0.13, 0.22, 0.34, 0.47, 0.5, 0.52, 0.56,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                15.97, 22.07, 35.48, 36.89, 54.25, 59.24, 70.52, 60.3, 42.87, 31.05, 18.63, 16.36,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.86, 44.37, 41.48, 33.55, 27.23, 19.33, 16.36,
            ],
            f_shwith200: vec![
                0.56, 0.59, 0.67, 0.65, 0.72, 0.75, 0.78, 0.77, 0.69, 0.62, 0.61, 0.56,
            ],
            f_shwith300: vec![
                0.42, 0.48, 0.55, 0.53, 0.6, 0.63, 0.68, 0.63, 0.59, 0.52, 0.48, 0.47,
            ],
            f_shwith500: vec![
                0.34, 0.38, 0.39, 0.4, 0.43, 0.42, 0.55, 0.51, 0.41, 0.37, 0.32, 0.39,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.61, 4.86, 12.95, 19.1, 38.6, 46.32, 52.17, 36.22, 17.54, 7.75, 2.16, 1.68,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.86, 44.37, 41.48, 33.55, 27.23, 19.33, 16.36,
            ],
            f_shwith200: vec![
                0.03, 0.18, 0.35, 0.45, 0.61, 0.67, 0.68, 0.59, 0.4, 0.19, 0.04, 0.02,
            ],
            f_shwith300: vec![
                0.0, 0.11, 0.18, 0.35, 0.49, 0.57, 0.61, 0.48, 0.27, 0.14, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.04, 0.13, 0.25, 0.32, 0.4, 0.24, 0.09, 0.03, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.28, 10.61, 19.91, 18.42, 4.8, 0.19, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.86, 44.37, 41.48, 33.55, 27.23, 19.33, 16.36,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.01, 0.1, 0.25, 0.18, 0.01, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: C3c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                34.16, 44.65, 77.17, 91.69, 125.97, 168.57, 189.63, 167.92, 121.17, 66.87, 37.93,
                27.6,
            ],
            dif: vec![
                22.6, 26.32, 35.43, 44.32, 45.41, 35.95, 30.27, 30.2, 29.23, 29.34, 24.31, 21.63,
            ],
            f_shwith200: vec![
                0.75, 0.77, 0.87, 0.89, 0.94, 0.96, 0.96, 0.95, 0.94, 0.85, 0.72, 0.64,
            ],
            f_shwith300: vec![
                0.61, 0.61, 0.78, 0.82, 0.88, 0.91, 0.92, 0.92, 0.85, 0.72, 0.62, 0.54,
            ],
            f_shwith500: vec![
                0.35, 0.47, 0.56, 0.62, 0.66, 0.77, 0.79, 0.77, 0.72, 0.49, 0.4, 0.29,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                2.07, 3.65, 13.91, 21.13, 36.74, 52.24, 55.35, 41.47, 22.63, 8.6, 2.18, 1.42,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.33, 44.76, 41.31, 34.79, 27.19, 19.3, 16.37,
            ],
            f_shwith200: vec![
                0.05, 0.11, 0.36, 0.49, 0.59, 0.69, 0.7, 0.66, 0.48, 0.26, 0.09, 0.01,
            ],
            f_shwith300: vec![
                0.0, 0.05, 0.24, 0.38, 0.48, 0.61, 0.62, 0.55, 0.36, 0.15, 0.01, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.06, 0.15, 0.26, 0.38, 0.38, 0.27, 0.06, 0.03, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                17.24, 19.41, 38.82, 39.71, 51.79, 66.98, 74.66, 69.35, 54.29, 33.42, 17.29, 14.93,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.33, 44.76, 41.31, 34.79, 27.19, 19.3, 16.37,
            ],
            f_shwith200: vec![
                0.59, 0.6, 0.68, 0.67, 0.7, 0.77, 0.79, 0.79, 0.75, 0.65, 0.56, 0.52,
            ],
            f_shwith300: vec![
                0.48, 0.47, 0.58, 0.59, 0.6, 0.66, 0.68, 0.71, 0.66, 0.56, 0.46, 0.42,
            ],
            f_shwith500: vec![
                0.31, 0.25, 0.43, 0.44, 0.4, 0.51, 0.55, 0.58, 0.52, 0.41, 0.34, 0.3,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                35.08, 33.99, 49.67, 38.7, 37.18, 42.48, 50.57, 60.26, 62.98, 49.67, 34.12, 31.42,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.33, 44.76, 41.31, 34.79, 27.19, 19.3, 16.37,
            ],
            f_shwith200: vec![
                0.77, 0.76, 0.77, 0.71, 0.66, 0.68, 0.72, 0.8, 0.81, 0.78, 0.76, 0.74,
            ],
            f_shwith300: vec![
                0.71, 0.67, 0.67, 0.61, 0.53, 0.58, 0.64, 0.71, 0.72, 0.69, 0.68, 0.66,
            ],
            f_shwith500: vec![
                0.51, 0.45, 0.47, 0.37, 0.19, 0.18, 0.24, 0.41, 0.53, 0.51, 0.5, 0.49,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                46.07, 43.85, 48.59, 26.19, 11.06, 6.6, 11.85, 30.41, 56.23, 57.69, 45.95, 42.4,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.33, 44.76, 41.31, 34.79, 27.19, 19.3, 16.37,
            ],
            f_shwith200: vec![
                0.84, 0.8, 0.8, 0.68, 0.49, 0.4, 0.51, 0.74, 0.81, 0.83, 0.83, 0.79,
            ],
            f_shwith300: vec![
                0.74, 0.66, 0.67, 0.5, 0.09, 0.0, 0.02, 0.43, 0.73, 0.7, 0.73, 0.7,
            ],
            f_shwith500: vec![
                0.58, 0.49, 0.42, 0.11, 0.0, 0.0, 0.0, 0.02, 0.36, 0.46, 0.56, 0.61,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                33.76, 36.54, 45.91, 36.75, 38.8, 41.49, 51.73, 56.36, 59.06, 48.51, 35.27, 31.26,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.33, 44.76, 41.31, 34.79, 27.19, 19.3, 16.37,
            ],
            f_shwith200: vec![
                0.78, 0.76, 0.76, 0.67, 0.68, 0.68, 0.73, 0.77, 0.79, 0.77, 0.76, 0.75,
            ],
            f_shwith300: vec![
                0.68, 0.66, 0.64, 0.56, 0.53, 0.56, 0.62, 0.68, 0.73, 0.65, 0.71, 0.66,
            ],
            f_shwith500: vec![
                0.48, 0.5, 0.48, 0.34, 0.16, 0.18, 0.24, 0.39, 0.48, 0.46, 0.56, 0.53,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                15.97, 22.07, 35.48, 36.89, 54.25, 65.28, 75.79, 65.03, 49.74, 32.17, 18.13, 14.73,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.33, 44.76, 41.31, 34.79, 27.19, 19.3, 16.37,
            ],
            f_shwith200: vec![
                0.56, 0.59, 0.67, 0.65, 0.72, 0.76, 0.79, 0.77, 0.74, 0.62, 0.58, 0.54,
            ],
            f_shwith300: vec![
                0.42, 0.48, 0.55, 0.53, 0.6, 0.65, 0.7, 0.68, 0.63, 0.55, 0.52, 0.46,
            ],
            f_shwith500: vec![
                0.34, 0.38, 0.39, 0.4, 0.43, 0.49, 0.55, 0.55, 0.46, 0.38, 0.35, 0.32,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.61, 4.86, 12.95, 19.1, 38.6, 50.82, 55.78, 39.25, 20.13, 8.0, 2.22, 1.3,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.33, 44.76, 41.31, 34.79, 27.19, 19.3, 16.37,
            ],
            f_shwith200: vec![
                0.03, 0.18, 0.35, 0.45, 0.61, 0.67, 0.71, 0.64, 0.44, 0.22, 0.08, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.11, 0.18, 0.35, 0.49, 0.59, 0.64, 0.52, 0.33, 0.13, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.04, 0.13, 0.25, 0.37, 0.37, 0.26, 0.01, 0.03, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.28, 10.61, 20.11, 18.09, 5.03, 0.18, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 44.33, 44.76, 41.31, 34.79, 27.19, 19.3, 16.37,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.01, 0.1, 0.25, 0.17, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: C4c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                34.16, 44.65, 77.17, 91.69, 125.97, 181.06, 207.0, 185.12, 127.95, 65.96, 38.03,
                27.87,
            ],
            dif: vec![
                22.6, 26.32, 35.43, 44.32, 45.41, 33.7, 27.84, 26.34, 28.61, 30.25, 24.22, 21.36,
            ],
            f_shwith200: vec![
                0.75, 0.77, 0.87, 0.89, 0.94, 0.96, 0.97, 0.96, 0.94, 0.83, 0.75, 0.66,
            ],
            f_shwith300: vec![
                0.61, 0.61, 0.78, 0.82, 0.88, 0.92, 0.94, 0.94, 0.86, 0.73, 0.62, 0.54,
            ],
            f_shwith500: vec![
                0.35, 0.47, 0.56, 0.62, 0.66, 0.79, 0.82, 0.81, 0.71, 0.49, 0.43, 0.31,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                2.07, 3.65, 13.91, 21.13, 36.74, 57.81, 62.36, 46.2, 22.75, 6.8, 2.49, 1.83,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 45.56, 45.39, 41.73, 35.21, 27.11, 19.25, 16.38,
            ],
            f_shwith200: vec![
                0.05, 0.11, 0.36, 0.49, 0.59, 0.71, 0.72, 0.68, 0.48, 0.23, 0.09, 0.02,
            ],
            f_shwith300: vec![
                0.0, 0.05, 0.24, 0.38, 0.48, 0.65, 0.67, 0.58, 0.35, 0.13, 0.03, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.06, 0.15, 0.26, 0.4, 0.47, 0.36, 0.09, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                17.24, 19.41, 38.82, 39.71, 51.79, 73.53, 83.7, 77.09, 53.72, 28.01, 18.36, 17.72,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 45.56, 45.39, 41.73, 35.21, 27.11, 19.25, 16.38,
            ],
            f_shwith200: vec![
                0.59, 0.6, 0.68, 0.67, 0.7, 0.78, 0.81, 0.81, 0.75, 0.62, 0.56, 0.58,
            ],
            f_shwith300: vec![
                0.48, 0.47, 0.58, 0.59, 0.6, 0.68, 0.72, 0.73, 0.66, 0.52, 0.51, 0.46,
            ],
            f_shwith500: vec![
                0.31, 0.25, 0.43, 0.44, 0.4, 0.54, 0.6, 0.6, 0.48, 0.38, 0.36, 0.31,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                35.08, 33.99, 49.67, 38.7, 37.18, 46.17, 56.32, 67.34, 62.56, 44.52, 35.73, 35.0,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 45.56, 45.39, 41.73, 35.21, 27.11, 19.25, 16.38,
            ],
            f_shwith200: vec![
                0.77, 0.76, 0.77, 0.71, 0.66, 0.7, 0.75, 0.82, 0.8, 0.78, 0.75, 0.76,
            ],
            f_shwith300: vec![
                0.71, 0.67, 0.67, 0.61, 0.53, 0.6, 0.69, 0.71, 0.73, 0.67, 0.7, 0.69,
            ],
            f_shwith500: vec![
                0.51, 0.45, 0.47, 0.37, 0.19, 0.15, 0.29, 0.5, 0.54, 0.52, 0.53, 0.52,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                46.07, 43.85, 48.59, 26.19, 11.06, 6.9, 12.67, 35.69, 58.19, 54.32, 45.56, 42.51,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 45.56, 45.39, 41.73, 35.21, 27.11, 19.25, 16.38,
            ],
            f_shwith200: vec![
                0.84, 0.8, 0.8, 0.68, 0.49, 0.42, 0.51, 0.79, 0.83, 0.84, 0.8, 0.81,
            ],
            f_shwith300: vec![
                0.74, 0.66, 0.67, 0.5, 0.09, 0.0, 0.02, 0.54, 0.72, 0.69, 0.74, 0.73,
            ],
            f_shwith500: vec![
                0.58, 0.49, 0.42, 0.11, 0.0, 0.0, 0.0, 0.01, 0.37, 0.46, 0.56, 0.54,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                33.76, 36.54, 45.91, 36.75, 38.8, 44.26, 52.88, 63.3, 64.62, 48.06, 33.01, 27.93,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 45.56, 45.39, 41.73, 35.21, 27.11, 19.25, 16.38,
            ],
            f_shwith200: vec![
                0.78, 0.76, 0.76, 0.67, 0.68, 0.69, 0.74, 0.81, 0.81, 0.81, 0.74, 0.71,
            ],
            f_shwith300: vec![
                0.68, 0.66, 0.64, 0.56, 0.53, 0.57, 0.67, 0.72, 0.75, 0.68, 0.62, 0.63,
            ],
            f_shwith500: vec![
                0.48, 0.5, 0.48, 0.34, 0.16, 0.14, 0.24, 0.45, 0.52, 0.5, 0.48, 0.45,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                15.97, 22.07, 35.48, 36.89, 54.25, 69.55, 76.88, 70.59, 54.96, 32.05, 15.96, 12.11,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 45.56, 45.39, 41.73, 35.21, 27.11, 19.25, 16.38,
            ],
            f_shwith200: vec![
                0.56, 0.59, 0.67, 0.65, 0.72, 0.77, 0.8, 0.8, 0.76, 0.67, 0.53, 0.5,
            ],
            f_shwith300: vec![
                0.42, 0.48, 0.55, 0.53, 0.6, 0.67, 0.71, 0.71, 0.64, 0.55, 0.43, 0.4,
            ],
            f_shwith500: vec![
                0.34, 0.38, 0.39, 0.4, 0.43, 0.5, 0.54, 0.56, 0.49, 0.36, 0.31, 0.24,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.61, 4.86, 12.95, 19.1, 38.6, 54.1, 56.16, 41.04, 22.45, 8.96, 1.82, 0.98,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 45.56, 45.39, 41.73, 35.21, 27.11, 19.25, 16.38,
            ],
            f_shwith200: vec![
                0.03, 0.18, 0.35, 0.45, 0.61, 0.69, 0.71, 0.67, 0.47, 0.27, 0.06, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.11, 0.18, 0.35, 0.49, 0.63, 0.67, 0.56, 0.35, 0.15, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.04, 0.13, 0.25, 0.34, 0.36, 0.22, 0.08, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.28, 10.61, 22.09, 19.26, 4.99, 0.23, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                18.03, 21.84, 31.73, 37.84, 44.42, 45.56, 45.39, 41.73, 35.21, 27.11, 19.25, 16.38,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.01, 0.1, 0.25, 0.21, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: D1c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                31.63, 49.13, 82.99, 104.7, 139.61, 135.44, 152.66, 136.86, 90.96, 72.96, 40.21,
                25.8,
            ],
            dif: vec![
                22.46, 24.74, 32.81, 38.52, 40.78, 44.15, 42.76, 37.15, 35.11, 26.78, 21.47, 21.14,
            ],
            f_shwith200: vec![
                0.69, 0.82, 0.89, 0.9, 0.93, 0.94, 0.95, 0.93, 0.9, 0.85, 0.76, 0.63,
            ],
            f_shwith300: vec![
                0.57, 0.67, 0.79, 0.82, 0.88, 0.86, 0.88, 0.89, 0.82, 0.74, 0.66, 0.48,
            ],
            f_shwith500: vec![
                0.37, 0.45, 0.61, 0.62, 0.69, 0.7, 0.72, 0.7, 0.64, 0.54, 0.42, 0.37,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                1.82, 4.56, 14.69, 22.66, 35.53, 47.9, 44.04, 32.94, 15.93, 8.31, 1.9, 1.15,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.76, 45.38, 40.69, 32.98, 26.72, 18.51, 15.62,
            ],
            f_shwith200: vec![
                0.02, 0.18, 0.39, 0.47, 0.57, 0.66, 0.63, 0.6, 0.4, 0.23, 0.02, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.11, 0.22, 0.36, 0.5, 0.58, 0.55, 0.49, 0.28, 0.16, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.04, 0.14, 0.3, 0.38, 0.39, 0.26, 0.05, 0.03, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                15.73, 22.6, 39.17, 43.85, 52.62, 60.27, 60.43, 54.29, 38.68, 32.12, 17.43, 13.26,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.76, 45.38, 40.69, 32.98, 26.72, 18.51, 15.62,
            ],
            f_shwith200: vec![
                0.59, 0.6, 0.71, 0.68, 0.71, 0.73, 0.73, 0.74, 0.69, 0.66, 0.58, 0.52,
            ],
            f_shwith300: vec![
                0.43, 0.49, 0.58, 0.58, 0.59, 0.63, 0.61, 0.63, 0.6, 0.55, 0.46, 0.45,
            ],
            f_shwith500: vec![
                0.26, 0.4, 0.42, 0.41, 0.48, 0.48, 0.52, 0.5, 0.42, 0.37, 0.3, 0.29,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                32.88, 38.01, 49.07, 43.2, 39.96, 37.34, 41.81, 46.81, 45.56, 49.01, 36.24, 29.03,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.76, 45.38, 40.69, 32.98, 26.72, 18.51, 15.62,
            ],
            f_shwith200: vec![
                0.75, 0.76, 0.79, 0.71, 0.67, 0.64, 0.66, 0.73, 0.75, 0.78, 0.79, 0.72,
            ],
            f_shwith300: vec![
                0.7, 0.67, 0.66, 0.6, 0.55, 0.54, 0.58, 0.63, 0.67, 0.7, 0.73, 0.64,
            ],
            f_shwith500: vec![
                0.5, 0.54, 0.46, 0.36, 0.31, 0.16, 0.27, 0.36, 0.44, 0.52, 0.5, 0.56,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                43.43, 48.1, 49.1, 29.97, 14.21, 5.16, 10.56, 24.12, 42.16, 58.63, 50.9, 39.09,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.76, 45.38, 40.69, 32.98, 26.72, 18.51, 15.62,
            ],
            f_shwith200: vec![
                0.83, 0.84, 0.82, 0.72, 0.52, 0.37, 0.45, 0.67, 0.8, 0.84, 0.85, 0.79,
            ],
            f_shwith300: vec![
                0.73, 0.72, 0.69, 0.48, 0.18, 0.0, 0.05, 0.41, 0.64, 0.73, 0.76, 0.7,
            ],
            f_shwith500: vec![
                0.6, 0.53, 0.38, 0.16, 0.0, 0.0, 0.0, 0.02, 0.34, 0.47, 0.64, 0.55,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                32.0, 39.49, 48.57, 43.75, 40.24, 33.27, 40.52, 46.24, 45.04, 51.65, 40.16, 28.62,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.76, 45.38, 40.69, 32.98, 26.72, 18.51, 15.62,
            ],
            f_shwith200: vec![
                0.76, 0.78, 0.78, 0.72, 0.68, 0.59, 0.65, 0.73, 0.74, 0.8, 0.8, 0.7,
            ],
            f_shwith300: vec![
                0.69, 0.72, 0.69, 0.56, 0.55, 0.47, 0.56, 0.63, 0.65, 0.71, 0.75, 0.63,
            ],
            f_shwith500: vec![
                0.52, 0.55, 0.49, 0.35, 0.28, 0.13, 0.22, 0.35, 0.46, 0.53, 0.57, 0.52,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                14.98, 23.9, 37.98, 45.19, 53.45, 52.78, 58.62, 53.33, 37.86, 34.79, 20.64, 13.02,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.76, 45.38, 40.69, 32.98, 26.72, 18.51, 15.62,
            ],
            f_shwith200: vec![
                0.55, 0.64, 0.7, 0.69, 0.73, 0.71, 0.72, 0.73, 0.67, 0.68, 0.64, 0.48,
            ],
            f_shwith300: vec![
                0.47, 0.54, 0.6, 0.59, 0.59, 0.57, 0.59, 0.62, 0.57, 0.6, 0.53, 0.42,
            ],
            f_shwith500: vec![
                0.26, 0.38, 0.44, 0.43, 0.45, 0.38, 0.49, 0.53, 0.42, 0.39, 0.31, 0.32,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.64, 4.92, 13.51, 24.0, 36.43, 41.37, 42.77, 32.15, 15.29, 9.45, 2.52, 1.22,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.76, 45.38, 40.69, 32.98, 26.72, 18.51, 15.62,
            ],
            f_shwith200: vec![
                0.05, 0.18, 0.38, 0.51, 0.61, 0.63, 0.63, 0.58, 0.37, 0.3, 0.04, 0.01,
            ],
            f_shwith300: vec![
                0.0, 0.11, 0.25, 0.35, 0.48, 0.52, 0.53, 0.48, 0.29, 0.16, 0.02, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.01, 0.17, 0.28, 0.28, 0.33, 0.26, 0.05, 0.01, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.48, 8.38, 18.34, 13.72, 4.35, 0.17, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.76, 45.38, 40.69, 32.98, 26.72, 18.51, 15.62,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.06, 0.23, 0.15, 0.01, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: D2c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                31.63, 49.13, 82.99, 104.7, 139.61, 154.51, 184.84, 161.33, 108.18, 71.61, 39.96,
                26.84,
            ],
            dif: vec![
                22.46, 24.74, 32.81, 38.52, 40.78, 39.66, 31.9, 32.36, 31.09, 28.12, 21.73, 20.12,
            ],
            f_shwith200: vec![
                0.69, 0.82, 0.89, 0.9, 0.93, 0.95, 0.96, 0.94, 0.91, 0.85, 0.73, 0.65,
            ],
            f_shwith300: vec![
                0.57, 0.67, 0.79, 0.82, 0.88, 0.9, 0.91, 0.9, 0.84, 0.73, 0.6, 0.48,
            ],
            f_shwith500: vec![
                0.37, 0.45, 0.61, 0.62, 0.69, 0.75, 0.8, 0.76, 0.69, 0.55, 0.43, 0.34,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                1.82, 4.56, 14.69, 22.66, 35.53, 51.2, 53.68, 39.89, 18.5, 8.19, 2.92, 1.4,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.88, 44.51, 41.49, 33.3, 26.85, 18.57, 15.67,
            ],
            f_shwith200: vec![
                0.02, 0.18, 0.39, 0.47, 0.57, 0.68, 0.68, 0.64, 0.41, 0.23, 0.11, 0.04,
            ],
            f_shwith300: vec![
                0.0, 0.11, 0.22, 0.36, 0.5, 0.6, 0.62, 0.52, 0.32, 0.16, 0.04, 0.02,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.04, 0.14, 0.3, 0.41, 0.4, 0.3, 0.13, 0.02, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                15.73, 22.6, 39.17, 43.85, 52.62, 64.83, 72.33, 66.55, 44.23, 33.99, 22.25, 15.76,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.88, 44.51, 41.49, 33.3, 26.85, 18.57, 15.67,
            ],
            f_shwith200: vec![
                0.59, 0.6, 0.71, 0.68, 0.71, 0.75, 0.77, 0.78, 0.71, 0.67, 0.63, 0.59,
            ],
            f_shwith300: vec![
                0.43, 0.49, 0.58, 0.58, 0.59, 0.66, 0.69, 0.69, 0.6, 0.57, 0.55, 0.48,
            ],
            f_shwith500: vec![
                0.26, 0.4, 0.42, 0.41, 0.48, 0.51, 0.57, 0.58, 0.44, 0.44, 0.38, 0.35,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                32.88, 38.01, 49.07, 43.2, 39.96, 40.49, 48.91, 57.82, 52.11, 53.34, 41.16, 32.27,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.88, 44.51, 41.49, 33.3, 26.85, 18.57, 15.67,
            ],
            f_shwith200: vec![
                0.75, 0.76, 0.79, 0.71, 0.67, 0.67, 0.71, 0.78, 0.77, 0.81, 0.8, 0.74,
            ],
            f_shwith300: vec![
                0.7, 0.67, 0.66, 0.6, 0.55, 0.57, 0.62, 0.68, 0.69, 0.71, 0.71, 0.67,
            ],
            f_shwith500: vec![
                0.5, 0.54, 0.46, 0.36, 0.31, 0.17, 0.26, 0.45, 0.45, 0.55, 0.6, 0.55,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                43.43, 48.1, 49.1, 29.97, 14.21, 5.77, 11.0, 29.18, 48.27, 60.94, 49.85, 40.52,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.88, 44.51, 41.49, 33.3, 26.85, 18.57, 15.67,
            ],
            f_shwith200: vec![
                0.83, 0.84, 0.82, 0.72, 0.52, 0.42, 0.49, 0.72, 0.81, 0.85, 0.83, 0.79,
            ],
            f_shwith300: vec![
                0.73, 0.72, 0.69, 0.48, 0.18, 0.0, 0.03, 0.45, 0.7, 0.74, 0.75, 0.7,
            ],
            f_shwith500: vec![
                0.6, 0.53, 0.38, 0.16, 0.0, 0.0, 0.0, 0.0, 0.31, 0.52, 0.58, 0.53,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                32.0, 39.49, 48.57, 43.75, 40.24, 37.95, 50.09, 52.32, 52.87, 48.09, 34.16, 27.36,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.88, 44.51, 41.49, 33.3, 26.85, 18.57, 15.67,
            ],
            f_shwith200: vec![
                0.76, 0.78, 0.78, 0.72, 0.68, 0.65, 0.72, 0.77, 0.79, 0.79, 0.73, 0.69,
            ],
            f_shwith300: vec![
                0.69, 0.72, 0.69, 0.56, 0.55, 0.52, 0.63, 0.66, 0.7, 0.71, 0.66, 0.6,
            ],
            f_shwith500: vec![
                0.52, 0.55, 0.49, 0.35, 0.28, 0.13, 0.25, 0.37, 0.46, 0.52, 0.47, 0.42,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                14.98, 23.9, 37.98, 45.19, 53.45, 59.68, 74.44, 59.63, 44.68, 29.47, 16.58, 11.94,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.88, 44.51, 41.49, 33.3, 26.85, 18.57, 15.67,
            ],
            f_shwith200: vec![
                0.55, 0.64, 0.7, 0.69, 0.73, 0.74, 0.79, 0.75, 0.72, 0.64, 0.56, 0.47,
            ],
            f_shwith300: vec![
                0.47, 0.54, 0.6, 0.59, 0.59, 0.61, 0.68, 0.67, 0.63, 0.55, 0.43, 0.38,
            ],
            f_shwith500: vec![
                0.26, 0.38, 0.44, 0.43, 0.45, 0.43, 0.53, 0.51, 0.44, 0.38, 0.28, 0.24,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.64, 4.92, 13.51, 24.0, 36.43, 46.46, 55.49, 35.62, 18.39, 7.05, 1.89, 0.92,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.88, 44.51, 41.49, 33.3, 26.85, 18.57, 15.67,
            ],
            f_shwith200: vec![
                0.05, 0.18, 0.38, 0.51, 0.61, 0.67, 0.7, 0.62, 0.44, 0.2, 0.01, 0.03,
            ],
            f_shwith300: vec![
                0.0, 0.11, 0.25, 0.35, 0.48, 0.55, 0.62, 0.51, 0.33, 0.08, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.01, 0.17, 0.28, 0.33, 0.38, 0.22, 0.07, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.48, 8.38, 19.36, 18.19, 4.69, 0.13, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.88, 44.51, 41.49, 33.3, 26.85, 18.57, 15.67,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.06, 0.25, 0.18, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: D3c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                31.63, 49.13, 82.99, 104.7, 139.61, 169.22, 189.31, 166.77, 121.39, 72.11, 40.19,
                26.37,
            ],
            dif: vec![
                22.46, 24.74, 32.81, 38.52, 40.78, 35.31, 30.6, 31.36, 28.99, 27.62, 21.5, 20.57,
            ],
            f_shwith200: vec![
                0.69, 0.82, 0.89, 0.9, 0.93, 0.96, 0.96, 0.95, 0.94, 0.85, 0.76, 0.65,
            ],
            f_shwith300: vec![
                0.57, 0.67, 0.79, 0.82, 0.88, 0.9, 0.92, 0.91, 0.86, 0.73, 0.64, 0.56,
            ],
            f_shwith500: vec![
                0.37, 0.45, 0.61, 0.62, 0.69, 0.76, 0.8, 0.75, 0.69, 0.56, 0.41, 0.33,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                1.82, 4.56, 14.69, 22.66, 35.53, 54.93, 57.38, 37.94, 21.5, 7.62, 2.53, 1.51,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.4, 44.7, 41.34, 34.73, 26.7, 18.54, 15.67,
            ],
            f_shwith200: vec![
                0.02, 0.18, 0.39, 0.47, 0.57, 0.7, 0.7, 0.62, 0.47, 0.2, 0.1, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.11, 0.22, 0.36, 0.5, 0.62, 0.63, 0.49, 0.35, 0.13, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.04, 0.14, 0.3, 0.41, 0.39, 0.27, 0.08, 0.02, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                15.73, 22.6, 39.17, 43.85, 52.62, 69.93, 76.69, 64.76, 51.72, 34.07, 18.29, 16.31,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.4, 44.7, 41.34, 34.73, 26.7, 18.54, 15.67,
            ],
            f_shwith200: vec![
                0.59, 0.6, 0.71, 0.68, 0.71, 0.78, 0.79, 0.77, 0.75, 0.69, 0.58, 0.6,
            ],
            f_shwith300: vec![
                0.43, 0.49, 0.58, 0.58, 0.59, 0.68, 0.69, 0.66, 0.65, 0.58, 0.46, 0.47,
            ],
            f_shwith500: vec![
                0.26, 0.4, 0.42, 0.41, 0.48, 0.52, 0.55, 0.55, 0.49, 0.39, 0.33, 0.33,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                32.88, 38.01, 49.07, 43.2, 39.96, 43.96, 51.37, 57.84, 60.28, 54.69, 37.61, 33.45,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.4, 44.7, 41.34, 34.73, 26.7, 18.54, 15.67,
            ],
            f_shwith200: vec![
                0.75, 0.76, 0.79, 0.71, 0.67, 0.7, 0.72, 0.78, 0.8, 0.82, 0.76, 0.78,
            ],
            f_shwith300: vec![
                0.7, 0.67, 0.66, 0.6, 0.55, 0.56, 0.62, 0.66, 0.75, 0.74, 0.69, 0.71,
            ],
            f_shwith500: vec![
                0.5, 0.54, 0.46, 0.36, 0.31, 0.2, 0.23, 0.41, 0.52, 0.57, 0.5, 0.55,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                43.43, 48.1, 49.1, 29.97, 14.21, 6.39, 11.46, 33.01, 55.07, 62.31, 52.08, 39.87,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.4, 44.7, 41.34, 34.73, 26.7, 18.54, 15.67,
            ],
            f_shwith200: vec![
                0.83, 0.84, 0.82, 0.72, 0.52, 0.4, 0.49, 0.74, 0.84, 0.85, 0.85, 0.79,
            ],
            f_shwith300: vec![
                0.73, 0.72, 0.69, 0.48, 0.18, 0.0, 0.02, 0.51, 0.69, 0.75, 0.78, 0.7,
            ],
            f_shwith500: vec![
                0.6, 0.53, 0.38, 0.16, 0.0, 0.0, 0.0, 0.0, 0.31, 0.57, 0.6, 0.59,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                32.0, 39.49, 48.57, 43.75, 40.24, 41.18, 50.47, 60.17, 61.48, 46.98, 40.92, 25.32,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.4, 44.7, 41.34, 34.73, 26.7, 18.54, 15.67,
            ],
            f_shwith200: vec![
                0.76, 0.78, 0.78, 0.72, 0.68, 0.67, 0.72, 0.79, 0.79, 0.79, 0.8, 0.71,
            ],
            f_shwith300: vec![
                0.69, 0.72, 0.69, 0.56, 0.55, 0.54, 0.64, 0.69, 0.73, 0.66, 0.75, 0.6,
            ],
            f_shwith500: vec![
                0.52, 0.55, 0.49, 0.35, 0.28, 0.18, 0.26, 0.42, 0.52, 0.49, 0.59, 0.42,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                14.98, 23.9, 37.98, 45.19, 53.45, 65.01, 74.26, 67.65, 53.38, 27.42, 20.5, 10.11,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.4, 44.7, 41.34, 34.73, 26.7, 18.54, 15.67,
            ],
            f_shwith200: vec![
                0.55, 0.64, 0.7, 0.69, 0.73, 0.76, 0.78, 0.79, 0.74, 0.61, 0.63, 0.42,
            ],
            f_shwith300: vec![
                0.47, 0.54, 0.6, 0.59, 0.59, 0.66, 0.69, 0.69, 0.66, 0.49, 0.56, 0.3,
            ],
            f_shwith500: vec![
                0.26, 0.38, 0.44, 0.43, 0.45, 0.51, 0.54, 0.57, 0.48, 0.36, 0.4, 0.18,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.64, 4.92, 13.51, 24.0, 36.43, 50.75, 54.84, 39.69, 22.66, 5.93, 2.34, 0.87,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.4, 44.7, 41.34, 34.73, 26.7, 18.54, 15.67,
            ],
            f_shwith200: vec![
                0.05, 0.18, 0.38, 0.51, 0.61, 0.68, 0.69, 0.65, 0.47, 0.16, 0.07, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.11, 0.25, 0.35, 0.48, 0.61, 0.62, 0.52, 0.38, 0.1, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.01, 0.17, 0.28, 0.37, 0.37, 0.26, 0.08, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.48, 8.38, 20.92, 18.8, 4.46, 0.19, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                17.56, 21.36, 31.09, 37.28, 43.06, 44.4, 44.7, 41.34, 34.73, 26.7, 18.54, 15.67,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.06, 0.25, 0.18, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: E1c,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                31.66, 45.13, 76.63, 99.68, 137.34, 135.1, 154.54, 137.26, 91.93, 67.46, 39.31,
                26.39,
            ],
            dif: vec![
                21.53, 24.27, 35.01, 37.85, 40.28, 44.5, 40.88, 36.72, 34.14, 28.17, 20.22, 19.94,
            ],
            f_shwith200: vec![
                0.68, 0.77, 0.85, 0.9, 0.93, 0.94, 0.94, 0.94, 0.91, 0.83, 0.74, 0.65,
            ],
            f_shwith300: vec![
                0.59, 0.66, 0.74, 0.83, 0.87, 0.87, 0.89, 0.88, 0.81, 0.73, 0.62, 0.53,
            ],
            f_shwith500: vec![
                0.37, 0.45, 0.54, 0.64, 0.7, 0.71, 0.73, 0.73, 0.65, 0.57, 0.36, 0.35,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1c,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                1.82, 4.94, 15.39, 22.86, 39.82, 46.75, 45.81, 32.74, 16.48, 7.37, 2.92, 1.03,
            ],
            dif: vec![
                17.06, 20.44, 31.07, 35.88, 42.91, 44.69, 45.07, 40.58, 32.93, 25.75, 17.77, 15.14,
            ],
            f_shwith200: vec![
                0.02, 0.2, 0.36, 0.51, 0.61, 0.68, 0.66, 0.59, 0.41, 0.25, 0.1, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.12, 0.23, 0.38, 0.53, 0.56, 0.57, 0.48, 0.28, 0.15, 0.05, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.02, 0.05, 0.19, 0.31, 0.31, 0.34, 0.23, 0.07, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1c,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                15.72, 24.45, 41.11, 41.95, 56.55, 58.77, 61.08, 54.96, 40.53, 30.66, 20.09, 12.29,
            ],
            dif: vec![
                17.06, 20.44, 31.07, 35.88, 42.91, 44.69, 45.07, 40.58, 32.93, 25.75, 17.77, 15.14,
            ],
            f_shwith200: vec![
                0.59, 0.64, 0.68, 0.7, 0.72, 0.74, 0.74, 0.75, 0.71, 0.67, 0.61, 0.47,
            ],
            f_shwith300: vec![
                0.45, 0.58, 0.58, 0.61, 0.63, 0.62, 0.63, 0.63, 0.6, 0.57, 0.51, 0.42,
            ],
            f_shwith500: vec![
                0.27, 0.43, 0.43, 0.44, 0.47, 0.46, 0.46, 0.46, 0.43, 0.46, 0.38, 0.29,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1c,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                32.88, 39.04, 50.14, 39.93, 41.12, 36.37, 40.78, 48.15, 48.22, 47.99, 37.75, 27.54,
            ],
            dif: vec![
                17.06, 20.44, 31.07, 35.88, 42.91, 44.69, 45.07, 40.58, 32.93, 25.75, 17.77, 15.14,
            ],
            f_shwith200: vec![
                0.76, 0.78, 0.75, 0.72, 0.68, 0.65, 0.67, 0.76, 0.77, 0.79, 0.8, 0.73,
            ],
            f_shwith300: vec![
                0.7, 0.7, 0.64, 0.59, 0.57, 0.52, 0.56, 0.63, 0.7, 0.73, 0.71, 0.65,
            ],
            f_shwith500: vec![
                0.52, 0.57, 0.47, 0.38, 0.26, 0.12, 0.21, 0.34, 0.43, 0.62, 0.53, 0.49,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1c,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                43.45, 46.81, 46.43, 25.54, 13.15, 5.2, 9.17, 25.01, 44.11, 55.93, 48.27, 39.1,
            ],
            dif: vec![
                17.06, 20.44, 31.07, 35.88, 42.91, 44.69, 45.07, 40.58, 32.93, 25.75, 17.77, 15.14,
            ],
            f_shwith200: vec![
                0.83, 0.82, 0.77, 0.7, 0.49, 0.39, 0.46, 0.68, 0.78, 0.83, 0.85, 0.81,
            ],
            f_shwith300: vec![
                0.74, 0.73, 0.62, 0.49, 0.13, 0.0, 0.03, 0.4, 0.68, 0.74, 0.78, 0.74,
            ],
            f_shwith500: vec![
                0.61, 0.51, 0.38, 0.07, 0.0, 0.0, 0.0, 0.02, 0.31, 0.57, 0.53, 0.57,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1c,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                32.01, 36.55, 43.72, 39.59, 39.7, 32.53, 40.82, 45.97, 44.35, 46.24, 36.14, 30.08,
            ],
            dif: vec![
                17.06, 20.44, 31.07, 35.88, 42.91, 44.69, 45.07, 40.58, 32.93, 25.75, 17.77, 15.14,
            ],
            f_shwith200: vec![
                0.76, 0.76, 0.73, 0.7, 0.67, 0.6, 0.68, 0.74, 0.73, 0.78, 0.78, 0.76,
            ],
            f_shwith300: vec![
                0.7, 0.66, 0.63, 0.6, 0.54, 0.5, 0.57, 0.62, 0.64, 0.71, 0.71, 0.71,
            ],
            f_shwith500: vec![
                0.53, 0.55, 0.43, 0.39, 0.23, 0.18, 0.21, 0.33, 0.46, 0.55, 0.5, 0.56,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1c,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                14.97, 22.34, 34.75, 41.79, 53.49, 50.95, 60.29, 52.76, 35.93, 29.7, 18.79, 14.28,
            ],
            dif: vec![
                17.06, 20.44, 31.07, 35.88, 42.91, 44.69, 45.07, 40.58, 32.93, 25.75, 17.77, 15.14,
            ],
            f_shwith200: vec![
                0.55, 0.61, 0.64, 0.69, 0.73, 0.69, 0.75, 0.73, 0.66, 0.65, 0.57, 0.57,
            ],
            f_shwith300: vec![
                0.48, 0.55, 0.56, 0.59, 0.61, 0.57, 0.64, 0.63, 0.54, 0.56, 0.49, 0.49,
            ],
            f_shwith500: vec![
                0.27, 0.42, 0.4, 0.48, 0.43, 0.44, 0.47, 0.47, 0.41, 0.39, 0.34, 0.35,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1c,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.63, 4.45, 12.81, 22.99, 36.91, 39.53, 44.66, 31.82, 13.85, 7.77, 2.69, 1.29,
            ],
            dif: vec![
                17.06, 20.44, 31.07, 35.88, 42.91, 44.69, 45.07, 40.58, 32.93, 25.75, 17.77, 15.14,
            ],
            f_shwith200: vec![
                0.05, 0.19, 0.34, 0.51, 0.62, 0.62, 0.66, 0.58, 0.33, 0.24, 0.11, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.11, 0.23, 0.41, 0.49, 0.52, 0.56, 0.48, 0.25, 0.12, 0.07, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.02, 0.07, 0.17, 0.24, 0.34, 0.32, 0.22, 0.02, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1c,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.73, 10.25, 17.48, 15.45, 4.11, 0.1, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                17.06, 20.44, 31.07, 35.88, 42.91, 44.69, 45.07, 40.58, 32.93, 25.75, 17.77, 15.14,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.1, 0.25, 0.16, 0.01, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: A3,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                58.07, 70.7, 104.79, 126.64, 165.59, 168.7, 189.06, 167.6, 122.07, 94.85, 63.35,
                51.86,
            ],
            dif: vec![
                23.27, 24.01, 31.75, 36.04, 36.9, 35.84, 30.9, 30.51, 28.31, 26.64, 22.53, 22.01,
            ],
            f_shwith200: vec![
                0.86, 0.92, 0.93, 0.93, 0.95, 0.95, 0.96, 0.95, 0.95, 0.93, 0.88, 0.88,
            ],
            f_shwith300: vec![
                0.72, 0.82, 0.85, 0.86, 0.88, 0.89, 0.9, 0.91, 0.88, 0.85, 0.8, 0.66,
            ],
            f_shwith500: vec![
                0.04, 0.4, 0.57, 0.67, 0.74, 0.75, 0.77, 0.76, 0.66, 0.54, 0.11, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                1.66, 5.15, 15.19, 25.55, 40.65, 46.82, 50.91, 37.88, 19.82, 8.66, 2.34, 0.69,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 45.07, 45.65, 42.37, 35.63, 31.06, 24.12, 22.0,
            ],
            f_shwith200: vec![
                0.0, 0.07, 0.31, 0.51, 0.59, 0.68, 0.64, 0.59, 0.43, 0.19, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.0, 0.21, 0.36, 0.52, 0.56, 0.58, 0.48, 0.26, 0.03, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.08, 0.25, 0.33, 0.32, 0.21, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                33.63, 39.5, 53.4, 60.26, 70.61, 72.15, 82.2, 76.82, 58.37, 50.23, 33.91, 30.11,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 45.07, 45.65, 42.37, 35.63, 31.06, 24.12, 22.0,
            ],
            f_shwith200: vec![
                0.71, 0.78, 0.75, 0.77, 0.77, 0.78, 0.8, 0.8, 0.79, 0.79, 0.74, 0.7,
            ],
            f_shwith300: vec![
                0.6, 0.65, 0.64, 0.65, 0.67, 0.7, 0.71, 0.71, 0.67, 0.69, 0.57, 0.59,
            ],
            f_shwith500: vec![
                0.08, 0.31, 0.49, 0.54, 0.51, 0.53, 0.55, 0.57, 0.44, 0.37, 0.12, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                89.56, 81.73, 82.89, 73.09, 67.08, 60.59, 72.46, 83.25, 83.73, 94.12, 87.37, 87.67,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 45.07, 45.65, 42.37, 35.63, 31.06, 24.12, 22.0,
            ],
            f_shwith200: vec![
                0.91, 0.92, 0.88, 0.83, 0.78, 0.78, 0.81, 0.84, 0.9, 0.92, 0.92, 0.93,
            ],
            f_shwith300: vec![
                0.89, 0.88, 0.79, 0.76, 0.7, 0.67, 0.7, 0.8, 0.79, 0.87, 0.88, 0.91,
            ],
            f_shwith500: vec![
                0.72, 0.67, 0.62, 0.53, 0.48, 0.36, 0.46, 0.56, 0.6, 0.66, 0.7, 0.72,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                123.94, 107.12, 96.92, 67.15, 47.26, 33.46, 44.34, 68.42, 91.15, 118.75, 121.54,
                121.92,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 45.07, 45.65, 42.37, 35.63, 31.06, 24.12, 22.0,
            ],
            f_shwith200: vec![
                0.97, 0.95, 0.93, 0.85, 0.8, 0.69, 0.78, 0.87, 0.92, 0.96, 0.96, 0.98,
            ],
            f_shwith300: vec![
                0.95, 0.9, 0.83, 0.75, 0.66, 0.54, 0.61, 0.75, 0.84, 0.89, 0.92, 0.94,
            ],
            f_shwith500: vec![
                0.78, 0.69, 0.58, 0.45, 0.15, 0.01, 0.03, 0.38, 0.59, 0.69, 0.77, 0.81,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                89.01, 80.04, 84.83, 73.16, 71.31, 57.59, 70.5, 82.54, 83.52, 90.87, 89.58, 86.14,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 45.07, 45.65, 42.37, 35.63, 31.06, 24.12, 22.0,
            ],
            f_shwith200: vec![
                0.91, 0.91, 0.9, 0.83, 0.8, 0.76, 0.81, 0.84, 0.91, 0.93, 0.91, 0.93,
            ],
            f_shwith300: vec![
                0.89, 0.86, 0.82, 0.75, 0.73, 0.63, 0.7, 0.8, 0.81, 0.89, 0.89, 0.89,
            ],
            f_shwith500: vec![
                0.71, 0.68, 0.6, 0.54, 0.49, 0.34, 0.4, 0.54, 0.59, 0.65, 0.7, 0.7,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                33.21, 38.29, 54.97, 61.77, 76.08, 68.56, 79.11, 75.83, 57.33, 47.75, 35.73, 29.04,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 45.07, 45.65, 42.37, 35.63, 31.06, 24.12, 22.0,
            ],
            f_shwith200: vec![
                0.73, 0.76, 0.78, 0.76, 0.78, 0.77, 0.8, 0.8, 0.78, 0.79, 0.74, 0.69,
            ],
            f_shwith300: vec![
                0.59, 0.64, 0.66, 0.66, 0.71, 0.69, 0.7, 0.71, 0.67, 0.66, 0.59, 0.56,
            ],
            f_shwith500: vec![
                0.12, 0.28, 0.43, 0.53, 0.54, 0.5, 0.55, 0.58, 0.42, 0.34, 0.18, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.62, 5.14, 15.47, 27.63, 44.16, 44.75, 48.5, 37.19, 18.57, 8.4, 2.72, 0.7,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 45.07, 45.65, 42.37, 35.63, 31.06, 24.12, 22.0,
            ],
            f_shwith200: vec![
                0.0, 0.07, 0.34, 0.51, 0.6, 0.66, 0.64, 0.59, 0.41, 0.19, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.0, 0.17, 0.39, 0.57, 0.54, 0.58, 0.49, 0.24, 0.03, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.17, 0.28, 0.28, 0.29, 0.13, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A3,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.34, 9.37, 14.64, 13.54, 4.27, 0.04, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 45.07, 45.65, 42.37, 35.63, 31.06, 24.12, 22.0,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.01, 0.13, 0.06, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: A4,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                58.07, 70.7, 104.79, 126.64, 165.59, 179.15, 207.3, 185.56, 128.39, 94.97, 63.43,
                51.85,
            ],
            dif: vec![
                23.27, 24.01, 31.75, 36.04, 36.9, 35.63, 27.61, 25.86, 28.17, 26.52, 22.45, 22.0,
            ],
            f_shwith200: vec![
                0.86, 0.92, 0.93, 0.93, 0.95, 0.97, 0.97, 0.96, 0.94, 0.92, 0.89, 0.88,
            ],
            f_shwith300: vec![
                0.72, 0.82, 0.85, 0.86, 0.88, 0.91, 0.92, 0.94, 0.89, 0.85, 0.8, 0.65,
            ],
            f_shwith500: vec![
                0.04, 0.4, 0.57, 0.67, 0.74, 0.74, 0.8, 0.8, 0.7, 0.53, 0.07, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                1.66, 5.15, 15.19, 25.55, 40.65, 52.37, 52.72, 39.97, 19.63, 7.54, 2.81, 0.71,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 46.5, 46.36, 42.89, 36.08, 31.09, 24.2, 21.9,
            ],
            f_shwith200: vec![
                0.0, 0.07, 0.31, 0.51, 0.59, 0.67, 0.66, 0.61, 0.41, 0.18, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.0, 0.21, 0.36, 0.52, 0.6, 0.61, 0.5, 0.28, 0.01, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.08, 0.25, 0.36, 0.29, 0.21, 0.01, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                33.63, 39.5, 53.4, 60.26, 70.61, 79.37, 86.54, 82.39, 60.35, 46.49, 37.3, 29.7,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 46.5, 46.36, 42.89, 36.08, 31.09, 24.2, 21.9,
            ],
            f_shwith200: vec![
                0.71, 0.78, 0.75, 0.77, 0.77, 0.79, 0.81, 0.82, 0.78, 0.77, 0.76, 0.71,
            ],
            f_shwith300: vec![
                0.6, 0.65, 0.64, 0.65, 0.67, 0.7, 0.74, 0.74, 0.69, 0.66, 0.6, 0.57,
            ],
            f_shwith500: vec![
                0.08, 0.31, 0.49, 0.54, 0.51, 0.56, 0.56, 0.6, 0.49, 0.33, 0.17, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                89.56, 81.73, 82.89, 73.09, 67.08, 65.21, 77.66, 90.3, 87.41, 90.68, 90.42, 86.67,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 46.5, 46.36, 42.89, 36.08, 31.09, 24.2, 21.9,
            ],
            f_shwith200: vec![
                0.91, 0.92, 0.88, 0.83, 0.78, 0.78, 0.82, 0.86, 0.9, 0.92, 0.92, 0.92,
            ],
            f_shwith300: vec![
                0.89, 0.88, 0.79, 0.76, 0.7, 0.67, 0.73, 0.83, 0.81, 0.87, 0.88, 0.9,
            ],
            f_shwith500: vec![
                0.72, 0.67, 0.62, 0.53, 0.48, 0.43, 0.48, 0.61, 0.61, 0.67, 0.71, 0.69,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                123.94, 107.12, 96.92, 67.15, 47.26, 34.53, 48.92, 74.73, 94.89, 118.5, 122.4,
                122.53,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 46.5, 46.36, 42.89, 36.08, 31.09, 24.2, 21.9,
            ],
            f_shwith200: vec![
                0.97, 0.95, 0.93, 0.85, 0.8, 0.69, 0.81, 0.9, 0.93, 0.95, 0.96, 0.99,
            ],
            f_shwith300: vec![
                0.95, 0.9, 0.83, 0.75, 0.66, 0.52, 0.69, 0.77, 0.86, 0.9, 0.93, 0.95,
            ],
            f_shwith500: vec![
                0.78, 0.69, 0.58, 0.45, 0.15, 0.0, 0.04, 0.37, 0.62, 0.69, 0.77, 0.79,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                89.01, 80.04, 84.83, 73.16, 71.31, 62.83, 77.8, 87.05, 85.48, 92.87, 88.29, 88.1,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 46.5, 46.36, 42.89, 36.08, 31.09, 24.2, 21.9,
            ],
            f_shwith200: vec![
                0.91, 0.91, 0.9, 0.83, 0.8, 0.79, 0.82, 0.85, 0.9, 0.92, 0.91, 0.93,
            ],
            f_shwith300: vec![
                0.89, 0.86, 0.82, 0.75, 0.73, 0.64, 0.73, 0.82, 0.81, 0.88, 0.88, 0.91,
            ],
            f_shwith500: vec![
                0.71, 0.68, 0.6, 0.54, 0.49, 0.4, 0.44, 0.59, 0.61, 0.67, 0.69, 0.71,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                33.21, 38.29, 54.97, 61.77, 76.08, 76.07, 87.65, 78.66, 58.62, 48.66, 35.78, 30.75,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 46.5, 46.36, 42.89, 36.08, 31.09, 24.2, 21.9,
            ],
            f_shwith200: vec![
                0.73, 0.76, 0.78, 0.76, 0.78, 0.79, 0.82, 0.81, 0.78, 0.78, 0.74, 0.68,
            ],
            f_shwith300: vec![
                0.59, 0.64, 0.66, 0.66, 0.71, 0.7, 0.75, 0.73, 0.67, 0.66, 0.61, 0.59,
            ],
            f_shwith500: vec![
                0.12, 0.28, 0.43, 0.53, 0.54, 0.55, 0.56, 0.57, 0.46, 0.36, 0.16, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.62, 5.14, 15.47, 27.63, 44.16, 50.08, 54.16, 37.96, 19.12, 8.42, 2.79, 0.77,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 46.5, 46.36, 42.89, 36.08, 31.09, 24.2, 21.9,
            ],
            f_shwith200: vec![
                0.0, 0.07, 0.34, 0.51, 0.6, 0.67, 0.68, 0.6, 0.4, 0.17, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.0, 0.17, 0.39, 0.57, 0.57, 0.62, 0.51, 0.24, 0.04, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.17, 0.28, 0.35, 0.3, 0.12, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: A4,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.34, 9.37, 16.44, 14.56, 4.43, 0.04, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                23.66, 25.9, 34.75, 39.28, 45.02, 46.5, 46.36, 42.89, 36.08, 31.09, 24.2, 21.9,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.01, 0.17, 0.07, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: B3,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                52.45, 64.45, 97.73, 121.14, 158.27, 169.86, 187.11, 168.21, 121.78, 86.85, 57.21,
                45.54,
            ],
            dif: vec![
                21.51, 24.62, 32.93, 35.76, 36.12, 34.66, 32.83, 29.92, 28.6, 27.12, 21.9, 20.53,
            ],
            f_shwith200: vec![
                0.85, 0.91, 0.93, 0.93, 0.95, 0.95, 0.96, 0.95, 0.94, 0.92, 0.86, 0.85,
            ],
            f_shwith300: vec![
                0.7, 0.79, 0.84, 0.85, 0.89, 0.89, 0.9, 0.9, 0.87, 0.82, 0.74, 0.6,
            ],
            f_shwith500: vec![
                0.06, 0.37, 0.54, 0.63, 0.73, 0.76, 0.76, 0.76, 0.67, 0.49, 0.12, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                1.45, 4.81, 14.32, 26.47, 40.0, 45.01, 46.32, 39.34, 18.69, 8.29, 2.23, 0.56,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 44.9, 45.58, 42.32, 35.57, 30.17, 22.62, 20.29,
            ],
            f_shwith200: vec![
                0.0, 0.07, 0.33, 0.51, 0.58, 0.64, 0.64, 0.61, 0.41, 0.2, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.0, 0.19, 0.4, 0.49, 0.55, 0.55, 0.51, 0.27, 0.04, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.12, 0.19, 0.29, 0.31, 0.2, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                29.79, 36.08, 50.32, 59.1, 69.36, 69.57, 76.33, 78.65, 55.84, 45.33, 32.87, 25.33,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 44.9, 45.58, 42.32, 35.57, 30.17, 22.62, 20.29,
            ],
            f_shwith200: vec![
                0.69, 0.75, 0.74, 0.76, 0.77, 0.77, 0.79, 0.81, 0.78, 0.75, 0.73, 0.62,
            ],
            f_shwith300: vec![
                0.53, 0.62, 0.61, 0.66, 0.67, 0.68, 0.71, 0.72, 0.66, 0.64, 0.55, 0.47,
            ],
            f_shwith500: vec![
                0.13, 0.29, 0.44, 0.52, 0.5, 0.51, 0.53, 0.58, 0.43, 0.38, 0.24, 0.01,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                79.84, 74.95, 78.43, 69.31, 65.42, 58.72, 69.17, 84.46, 81.26, 85.42, 81.73, 75.24,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 44.9, 45.58, 42.32, 35.57, 30.17, 22.62, 20.29,
            ],
            f_shwith200: vec![
                0.91, 0.9, 0.89, 0.83, 0.8, 0.77, 0.8, 0.83, 0.9, 0.91, 0.91, 0.92,
            ],
            f_shwith300: vec![
                0.86, 0.86, 0.78, 0.76, 0.71, 0.63, 0.71, 0.8, 0.79, 0.86, 0.86, 0.86,
            ],
            f_shwith500: vec![
                0.64, 0.64, 0.59, 0.51, 0.41, 0.36, 0.41, 0.55, 0.58, 0.62, 0.67, 0.59,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                111.23, 98.6, 90.43, 61.31, 45.36, 34.0, 44.61, 68.25, 88.91, 109.03, 111.36,
                107.22,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 44.9, 45.58, 42.32, 35.57, 30.17, 22.62, 20.29,
            ],
            f_shwith200: vec![
                0.96, 0.94, 0.92, 0.85, 0.83, 0.71, 0.78, 0.87, 0.91, 0.95, 0.95, 0.97,
            ],
            f_shwith300: vec![
                0.92, 0.88, 0.81, 0.72, 0.65, 0.52, 0.65, 0.75, 0.83, 0.88, 0.92, 0.91,
            ],
            f_shwith500: vec![
                0.73, 0.7, 0.57, 0.38, 0.09, 0.0, 0.06, 0.36, 0.59, 0.64, 0.7, 0.71,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                80.39, 73.93, 77.42, 66.79, 67.19, 59.83, 69.36, 78.25, 81.59, 83.8, 80.35, 77.6,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 44.9, 45.58, 42.32, 35.57, 30.17, 22.62, 20.29,
            ],
            f_shwith200: vec![
                0.91, 0.9, 0.87, 0.81, 0.79, 0.78, 0.8, 0.82, 0.89, 0.91, 0.91, 0.92,
            ],
            f_shwith300: vec![
                0.86, 0.86, 0.77, 0.74, 0.71, 0.65, 0.69, 0.78, 0.79, 0.84, 0.86, 0.87,
            ],
            f_shwith500: vec![
                0.63, 0.64, 0.6, 0.49, 0.43, 0.37, 0.42, 0.55, 0.59, 0.65, 0.66, 0.6,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                30.19, 35.22, 49.12, 56.35, 71.44, 70.48, 76.48, 69.53, 55.93, 43.09, 31.99, 27.05,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 44.9, 45.58, 42.32, 35.57, 30.17, 22.62, 20.29,
            ],
            f_shwith200: vec![
                0.7, 0.74, 0.74, 0.74, 0.78, 0.78, 0.79, 0.78, 0.78, 0.76, 0.72, 0.64,
            ],
            f_shwith300: vec![
                0.54, 0.61, 0.61, 0.64, 0.69, 0.68, 0.69, 0.68, 0.68, 0.62, 0.57, 0.52,
            ],
            f_shwith500: vec![
                0.11, 0.28, 0.39, 0.48, 0.48, 0.48, 0.52, 0.53, 0.45, 0.28, 0.13, 0.01,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.48, 4.63, 13.64, 25.1, 41.19, 45.19, 46.34, 32.65, 18.48, 6.74, 2.36, 0.64,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 44.9, 45.58, 42.32, 35.57, 30.17, 22.62, 20.29,
            ],
            f_shwith200: vec![
                0.0, 0.07, 0.29, 0.48, 0.59, 0.66, 0.63, 0.56, 0.41, 0.14, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.01, 0.13, 0.34, 0.52, 0.53, 0.55, 0.45, 0.25, 0.01, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.14, 0.23, 0.28, 0.3, 0.1, 0.01, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B3,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.54, 9.0, 13.95, 12.18, 4.11, 0.05, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 44.9, 45.58, 42.32, 35.57, 30.17, 22.62, 20.29,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.01, 0.13, 0.07, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: B4,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                52.45, 64.45, 97.73, 121.14, 158.27, 181.43, 205.92, 184.76, 128.91, 85.82, 57.09,
                45.38,
            ],
            dif: vec![
                21.51, 24.62, 32.93, 35.76, 36.12, 33.37, 28.96, 26.66, 27.66, 28.13, 22.01, 20.7,
            ],
            f_shwith200: vec![
                0.85, 0.91, 0.93, 0.93, 0.95, 0.97, 0.97, 0.96, 0.95, 0.9, 0.85, 0.86,
            ],
            f_shwith300: vec![
                0.7, 0.79, 0.84, 0.85, 0.89, 0.9, 0.91, 0.93, 0.89, 0.81, 0.75, 0.62,
            ],
            f_shwith500: vec![
                0.06, 0.37, 0.54, 0.63, 0.73, 0.75, 0.79, 0.8, 0.69, 0.48, 0.1, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                1.45, 4.81, 14.32, 26.47, 40.0, 50.98, 55.08, 40.52, 20.18, 7.67, 2.54, 0.53,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 46.32, 46.45, 42.93, 36.2, 30.21, 22.66, 20.32,
            ],
            f_shwith200: vec![
                0.0, 0.07, 0.33, 0.51, 0.58, 0.69, 0.68, 0.61, 0.44, 0.15, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.0, 0.19, 0.4, 0.49, 0.58, 0.63, 0.52, 0.26, 0.03, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.12, 0.19, 0.33, 0.37, 0.17, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                29.79, 36.08, 50.32, 59.1, 69.36, 77.76, 89.24, 82.86, 61.09, 44.58, 32.46, 25.08,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 46.32, 46.45, 42.93, 36.2, 30.21, 22.66, 20.32,
            ],
            f_shwith200: vec![
                0.69, 0.75, 0.74, 0.76, 0.77, 0.8, 0.82, 0.82, 0.79, 0.76, 0.7, 0.63,
            ],
            f_shwith300: vec![
                0.53, 0.62, 0.61, 0.66, 0.67, 0.71, 0.75, 0.73, 0.69, 0.62, 0.57, 0.47,
            ],
            f_shwith500: vec![
                0.13, 0.29, 0.44, 0.52, 0.5, 0.54, 0.58, 0.58, 0.47, 0.35, 0.15, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                79.84, 74.95, 78.43, 69.31, 65.42, 64.67, 78.97, 90.26, 88.36, 83.48, 79.72, 75.11,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 46.32, 46.45, 42.93, 36.2, 30.21, 22.66, 20.32,
            ],
            f_shwith200: vec![
                0.91, 0.9, 0.89, 0.83, 0.8, 0.8, 0.82, 0.85, 0.9, 0.9, 0.91, 0.92,
            ],
            f_shwith300: vec![
                0.86, 0.86, 0.78, 0.76, 0.71, 0.67, 0.74, 0.83, 0.81, 0.82, 0.86, 0.86,
            ],
            f_shwith500: vec![
                0.64, 0.64, 0.59, 0.51, 0.41, 0.37, 0.48, 0.59, 0.61, 0.66, 0.65, 0.62,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                111.23, 98.6, 90.43, 61.31, 45.36, 36.16, 48.18, 74.76, 96.73, 106.4, 108.92,
                106.59,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 46.32, 46.45, 42.93, 36.2, 30.21, 22.66, 20.32,
            ],
            f_shwith200: vec![
                0.96, 0.94, 0.92, 0.85, 0.83, 0.71, 0.8, 0.9, 0.92, 0.94, 0.94, 0.96,
            ],
            f_shwith300: vec![
                0.92, 0.88, 0.81, 0.72, 0.65, 0.54, 0.68, 0.78, 0.86, 0.86, 0.9, 0.92,
            ],
            f_shwith500: vec![
                0.73, 0.7, 0.57, 0.38, 0.09, 0.0, 0.02, 0.37, 0.58, 0.62, 0.72, 0.72,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                80.39, 73.93, 77.42, 66.79, 67.19, 63.47, 75.81, 87.75, 88.15, 82.75, 79.55, 76.73,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 46.32, 46.45, 42.93, 36.2, 30.21, 22.66, 20.32,
            ],
            f_shwith200: vec![
                0.91, 0.9, 0.87, 0.81, 0.79, 0.79, 0.81, 0.85, 0.9, 0.9, 0.91, 0.92,
            ],
            f_shwith300: vec![
                0.86, 0.86, 0.77, 0.74, 0.71, 0.66, 0.73, 0.83, 0.81, 0.84, 0.86, 0.87,
            ],
            f_shwith500: vec![
                0.63, 0.64, 0.6, 0.49, 0.43, 0.39, 0.47, 0.6, 0.62, 0.63, 0.67, 0.62,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                30.19, 35.22, 49.12, 56.35, 71.44, 74.96, 84.75, 79.26, 60.54, 44.36, 32.45, 26.26,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 46.32, 46.45, 42.93, 36.2, 30.21, 22.66, 20.32,
            ],
            f_shwith200: vec![
                0.7, 0.74, 0.74, 0.74, 0.78, 0.79, 0.81, 0.81, 0.79, 0.74, 0.71, 0.64,
            ],
            f_shwith300: vec![
                0.54, 0.61, 0.61, 0.64, 0.69, 0.69, 0.72, 0.73, 0.69, 0.64, 0.56, 0.49,
            ],
            f_shwith500: vec![
                0.11, 0.28, 0.39, 0.48, 0.48, 0.51, 0.58, 0.57, 0.44, 0.34, 0.16, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.48, 4.63, 13.64, 25.1, 41.19, 48.2, 51.88, 37.93, 19.61, 8.08, 2.69, 0.57,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 46.32, 46.45, 42.93, 36.2, 30.21, 22.66, 20.32,
            ],
            f_shwith200: vec![
                0.0, 0.07, 0.29, 0.48, 0.59, 0.67, 0.66, 0.6, 0.41, 0.17, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.01, 0.13, 0.34, 0.52, 0.58, 0.6, 0.49, 0.24, 0.03, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.14, 0.23, 0.27, 0.31, 0.15, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: B4,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.54, 9.0, 15.68, 14.36, 4.35, 0.06, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                21.84, 25.07, 34.34, 39.08, 45.04, 46.32, 46.45, 42.93, 36.2, 30.21, 22.66, 20.32,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.01, 0.14, 0.09, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: C1,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                34.72, 45.39, 76.95, 93.85, 122.34, 135.39, 156.91, 138.49, 91.2, 66.92, 39.73,
                28.82,
            ],
            dif: vec![
                22.02, 25.59, 35.62, 42.12, 49.06, 44.24, 38.54, 35.51, 34.87, 29.3, 22.5, 20.41,
            ],
            f_shwith200: vec![
                0.74, 0.84, 0.89, 0.89, 0.92, 0.94, 0.95, 0.94, 0.9, 0.87, 0.76, 0.72,
            ],
            f_shwith300: vec![
                0.49, 0.68, 0.75, 0.79, 0.83, 0.86, 0.88, 0.88, 0.79, 0.76, 0.58, 0.39,
            ],
            f_shwith500: vec![
                0.08, 0.25, 0.45, 0.54, 0.62, 0.66, 0.69, 0.66, 0.58, 0.42, 0.13, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                1.18, 2.89, 11.79, 21.45, 37.19, 38.62, 42.78, 30.53, 15.89, 5.2, 1.87, 0.33,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.24, 45.58, 41.41, 33.81, 27.93, 20.0, 16.99,
            ],
            f_shwith200: vec![
                0.0, 0.05, 0.27, 0.46, 0.57, 0.59, 0.62, 0.53, 0.35, 0.1, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.0, 0.15, 0.34, 0.47, 0.48, 0.51, 0.43, 0.23, 0.0, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.08, 0.22, 0.28, 0.22, 0.13, 0.01, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                20.75, 22.99, 41.01, 46.84, 61.75, 58.37, 68.93, 62.69, 45.67, 32.17, 23.17, 16.05,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.24, 45.58, 41.41, 33.81, 27.93, 20.0, 16.99,
            ],
            f_shwith200: vec![
                0.6, 0.6, 0.7, 0.7, 0.73, 0.72, 0.77, 0.77, 0.72, 0.69, 0.62, 0.51,
            ],
            f_shwith300: vec![
                0.45, 0.47, 0.54, 0.61, 0.63, 0.61, 0.67, 0.67, 0.6, 0.54, 0.47, 0.38,
            ],
            f_shwith500: vec![
                0.13, 0.19, 0.41, 0.4, 0.45, 0.43, 0.47, 0.46, 0.43, 0.26, 0.15, 0.05,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                52.08, 49.65, 62.95, 53.65, 55.61, 48.17, 60.79, 68.53, 63.78, 63.7, 55.07, 47.44,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.24, 45.58, 41.41, 33.81, 27.93, 20.0, 16.99,
            ],
            f_shwith200: vec![
                0.84, 0.83, 0.83, 0.77, 0.74, 0.72, 0.78, 0.81, 0.82, 0.86, 0.85, 0.83,
            ],
            f_shwith300: vec![
                0.73, 0.74, 0.7, 0.67, 0.63, 0.55, 0.64, 0.75, 0.72, 0.78, 0.77, 0.74,
            ],
            f_shwith500: vec![
                0.51, 0.5, 0.53, 0.4, 0.37, 0.28, 0.35, 0.44, 0.55, 0.57, 0.5, 0.46,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                73.0, 68.08, 72.0, 47.0, 33.27, 26.51, 36.5, 55.35, 67.06, 83.98, 75.9, 68.23,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.24, 45.58, 41.41, 33.81, 27.93, 20.0, 16.99,
            ],
            f_shwith200: vec![
                0.91, 0.88, 0.88, 0.77, 0.71, 0.64, 0.71, 0.82, 0.84, 0.91, 0.9, 0.91,
            ],
            f_shwith300: vec![
                0.82, 0.8, 0.73, 0.63, 0.49, 0.44, 0.54, 0.68, 0.75, 0.83, 0.84, 0.81,
            ],
            f_shwith500: vec![
                0.55, 0.58, 0.47, 0.29, 0.13, 0.02, 0.04, 0.3, 0.51, 0.6, 0.56, 0.57,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                53.73, 53.08, 62.37, 54.15, 45.04, 49.13, 56.19, 63.89, 61.53, 65.97, 56.19, 49.75,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.24, 45.58, 41.41, 33.81, 27.93, 20.0, 16.99,
            ],
            f_shwith200: vec![
                0.84, 0.85, 0.82, 0.76, 0.68, 0.72, 0.75, 0.79, 0.83, 0.87, 0.84, 0.84,
            ],
            f_shwith300: vec![
                0.74, 0.77, 0.71, 0.67, 0.56, 0.57, 0.6, 0.72, 0.69, 0.81, 0.78, 0.73,
            ],
            f_shwith500: vec![
                0.56, 0.55, 0.5, 0.43, 0.34, 0.32, 0.36, 0.41, 0.51, 0.6, 0.52, 0.49,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                22.06, 25.88, 40.53, 47.53, 46.69, 59.31, 62.51, 58.33, 43.21, 34.14, 24.09, 17.71,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.24, 45.58, 41.41, 33.81, 27.93, 20.0, 16.99,
            ],
            f_shwith200: vec![
                0.61, 0.67, 0.69, 0.69, 0.66, 0.73, 0.73, 0.74, 0.71, 0.7, 0.65, 0.53,
            ],
            f_shwith300: vec![
                0.48, 0.5, 0.55, 0.57, 0.53, 0.61, 0.62, 0.64, 0.55, 0.56, 0.51, 0.37,
            ],
            f_shwith500: vec![
                0.13, 0.25, 0.36, 0.41, 0.38, 0.47, 0.41, 0.4, 0.4, 0.3, 0.17, 0.01,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.38, 3.55, 11.7, 21.93, 26.46, 39.0, 38.29, 29.0, 14.66, 5.71, 2.05, 0.36,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.24, 45.58, 41.41, 33.81, 27.93, 20.0, 16.99,
            ],
            f_shwith200: vec![
                0.0, 0.06, 0.26, 0.43, 0.45, 0.59, 0.57, 0.52, 0.32, 0.13, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.02, 0.16, 0.33, 0.35, 0.48, 0.44, 0.38, 0.21, 0.02, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.01, 0.1, 0.19, 0.29, 0.21, 0.13, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C1,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.45, 7.11, 12.6, 11.11, 3.81, 0.06, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.24, 45.58, 41.41, 33.81, 27.93, 20.0, 16.99,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.04, 0.14, 0.07, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: C2,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                34.72, 45.39, 76.95, 93.85, 122.34, 154.09, 184.86, 163.4, 108.57, 66.02, 39.15,
                28.77,
            ],
            dif: vec![
                22.02, 25.59, 35.62, 42.12, 49.06, 40.1, 31.93, 30.29, 30.69, 30.2, 23.08, 20.44,
            ],
            f_shwith200: vec![
                0.74, 0.84, 0.89, 0.89, 0.92, 0.94, 0.96, 0.95, 0.93, 0.85, 0.77, 0.65,
            ],
            f_shwith300: vec![
                0.49, 0.68, 0.75, 0.79, 0.83, 0.89, 0.9, 0.91, 0.85, 0.71, 0.57, 0.4,
            ],
            f_shwith500: vec![
                0.08, 0.25, 0.45, 0.54, 0.62, 0.71, 0.76, 0.77, 0.63, 0.45, 0.13, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                1.18, 2.89, 11.79, 21.45, 37.19, 44.06, 51.14, 34.38, 15.45, 6.97, 1.62, 0.38,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.49, 45.31, 42.38, 34.32, 27.9, 20.01, 16.95,
            ],
            f_shwith200: vec![
                0.0, 0.05, 0.27, 0.46, 0.57, 0.66, 0.66, 0.56, 0.34, 0.15, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.0, 0.15, 0.34, 0.47, 0.55, 0.58, 0.45, 0.21, 0.05, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.08, 0.22, 0.3, 0.34, 0.09, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                20.75, 22.99, 41.01, 46.84, 61.75, 67.68, 82.78, 70.25, 50.06, 37.42, 23.53, 17.62,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.49, 45.31, 42.38, 34.32, 27.9, 20.01, 16.95,
            ],
            f_shwith200: vec![
                0.6, 0.6, 0.7, 0.7, 0.73, 0.77, 0.8, 0.79, 0.75, 0.7, 0.64, 0.55,
            ],
            f_shwith300: vec![
                0.45, 0.47, 0.54, 0.61, 0.63, 0.69, 0.72, 0.69, 0.64, 0.57, 0.52, 0.34,
            ],
            f_shwith500: vec![
                0.13, 0.19, 0.41, 0.4, 0.45, 0.49, 0.57, 0.52, 0.46, 0.33, 0.14, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                52.08, 49.65, 62.95, 53.65, 55.61, 56.46, 72.88, 76.93, 74.7, 68.38, 56.16, 49.71,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.49, 45.31, 42.38, 34.32, 27.9, 20.01, 16.95,
            ],
            f_shwith200: vec![
                0.84, 0.83, 0.83, 0.77, 0.74, 0.77, 0.8, 0.82, 0.86, 0.85, 0.84, 0.83,
            ],
            f_shwith300: vec![
                0.73, 0.74, 0.7, 0.67, 0.63, 0.62, 0.71, 0.78, 0.77, 0.78, 0.77, 0.75,
            ],
            f_shwith500: vec![
                0.51, 0.5, 0.53, 0.4, 0.37, 0.37, 0.47, 0.53, 0.59, 0.6, 0.57, 0.48,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                73.0, 68.08, 72.0, 47.0, 33.27, 30.62, 43.3, 65.3, 83.93, 82.89, 74.47, 68.35,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.49, 45.31, 42.38, 34.32, 27.9, 20.01, 16.95,
            ],
            f_shwith200: vec![
                0.91, 0.88, 0.88, 0.77, 0.71, 0.67, 0.77, 0.87, 0.9, 0.89, 0.88, 0.9,
            ],
            f_shwith300: vec![
                0.82, 0.8, 0.73, 0.63, 0.49, 0.53, 0.6, 0.75, 0.82, 0.78, 0.82, 0.78,
            ],
            f_shwith500: vec![
                0.55, 0.58, 0.47, 0.29, 0.13, 0.0, 0.05, 0.34, 0.56, 0.59, 0.62, 0.58,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                53.73, 53.08, 62.37, 54.15, 45.04, 52.64, 66.18, 79.75, 76.1, 61.82, 52.52, 47.67,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.49, 45.31, 42.38, 34.32, 27.9, 20.01, 16.95,
            ],
            f_shwith200: vec![
                0.84, 0.85, 0.82, 0.76, 0.68, 0.74, 0.78, 0.83, 0.87, 0.83, 0.83, 0.81,
            ],
            f_shwith300: vec![
                0.74, 0.77, 0.71, 0.67, 0.56, 0.61, 0.68, 0.78, 0.77, 0.76, 0.75, 0.71,
            ],
            f_shwith500: vec![
                0.56, 0.55, 0.5, 0.43, 0.34, 0.35, 0.41, 0.54, 0.58, 0.53, 0.55, 0.51,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                22.06, 25.88, 40.53, 47.53, 46.69, 62.66, 73.5, 73.45, 51.94, 32.1, 21.04, 16.15,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.49, 45.31, 42.38, 34.32, 27.9, 20.01, 16.95,
            ],
            f_shwith200: vec![
                0.61, 0.67, 0.69, 0.69, 0.66, 0.75, 0.77, 0.79, 0.76, 0.65, 0.58, 0.5,
            ],
            f_shwith300: vec![
                0.48, 0.5, 0.55, 0.57, 0.53, 0.64, 0.67, 0.69, 0.65, 0.51, 0.46, 0.38,
            ],
            f_shwith500: vec![
                0.13, 0.25, 0.36, 0.41, 0.38, 0.49, 0.5, 0.53, 0.44, 0.3, 0.15, 0.07,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.38, 3.55, 11.7, 21.93, 26.46, 40.79, 44.71, 36.08, 16.71, 6.01, 1.75, 0.34,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.49, 45.31, 42.38, 34.32, 27.9, 20.01, 16.95,
            ],
            f_shwith200: vec![
                0.0, 0.06, 0.26, 0.43, 0.45, 0.62, 0.61, 0.58, 0.36, 0.12, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.02, 0.16, 0.33, 0.35, 0.5, 0.52, 0.46, 0.21, 0.02, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.01, 0.1, 0.19, 0.28, 0.25, 0.16, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C2,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.45, 7.11, 13.47, 12.74, 4.33, 0.04, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.49, 45.31, 42.38, 34.32, 27.9, 20.01, 16.95,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.04, 0.11, 0.08, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: C3,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                34.72, 45.39, 76.95, 93.85, 122.34, 168.38, 189.3, 168.12, 122.11, 67.12, 39.6,
                28.85,
            ],
            dif: vec![
                22.02, 25.59, 35.62, 42.12, 49.06, 36.16, 30.64, 29.98, 28.26, 29.11, 22.63, 20.37,
            ],
            f_shwith200: vec![
                0.74, 0.84, 0.89, 0.89, 0.92, 0.96, 0.96, 0.96, 0.94, 0.86, 0.77, 0.7,
            ],
            f_shwith300: vec![
                0.49, 0.68, 0.75, 0.79, 0.83, 0.88, 0.91, 0.91, 0.88, 0.75, 0.55, 0.39,
            ],
            f_shwith500: vec![
                0.08, 0.25, 0.45, 0.54, 0.62, 0.75, 0.77, 0.77, 0.66, 0.4, 0.16, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                1.18, 2.89, 11.79, 21.45, 37.19, 47.1, 51.27, 36.94, 20.55, 6.51, 2.05, 0.37,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.05, 45.62, 42.17, 35.55, 27.87, 20.06, 17.03,
            ],
            f_shwith200: vec![
                0.0, 0.05, 0.27, 0.46, 0.57, 0.67, 0.65, 0.6, 0.43, 0.14, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.0, 0.15, 0.34, 0.47, 0.56, 0.59, 0.48, 0.3, 0.03, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.08, 0.22, 0.33, 0.31, 0.16, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                20.75, 22.99, 41.01, 46.84, 61.75, 72.67, 83.04, 76.2, 59.96, 37.05, 24.21, 17.11,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.05, 45.62, 42.17, 35.55, 27.87, 20.06, 17.03,
            ],
            f_shwith200: vec![
                0.6, 0.6, 0.7, 0.7, 0.73, 0.78, 0.81, 0.81, 0.79, 0.7, 0.64, 0.54,
            ],
            f_shwith300: vec![
                0.45, 0.47, 0.54, 0.61, 0.63, 0.69, 0.73, 0.72, 0.69, 0.59, 0.48, 0.37,
            ],
            f_shwith500: vec![
                0.13, 0.19, 0.41, 0.4, 0.45, 0.53, 0.56, 0.56, 0.49, 0.37, 0.16, 0.06,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                52.08, 49.65, 62.95, 53.65, 55.61, 61.02, 73.37, 83.33, 84.06, 67.53, 57.59, 48.93,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.05, 45.62, 42.17, 35.55, 27.87, 20.06, 17.03,
            ],
            f_shwith200: vec![
                0.84, 0.83, 0.83, 0.77, 0.74, 0.78, 0.81, 0.84, 0.89, 0.87, 0.86, 0.84,
            ],
            f_shwith300: vec![
                0.73, 0.74, 0.7, 0.67, 0.63, 0.65, 0.71, 0.8, 0.81, 0.78, 0.79, 0.74,
            ],
            f_shwith500: vec![
                0.51, 0.5, 0.53, 0.4, 0.37, 0.38, 0.44, 0.56, 0.63, 0.58, 0.53, 0.5,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                73.0, 68.08, 72.0, 47.0, 33.27, 33.76, 44.2, 68.49, 87.94, 82.8, 75.88, 68.58,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.05, 45.62, 42.17, 35.55, 27.87, 20.06, 17.03,
            ],
            f_shwith200: vec![
                0.91, 0.88, 0.88, 0.77, 0.71, 0.69, 0.77, 0.88, 0.91, 0.9, 0.89, 0.91,
            ],
            f_shwith300: vec![
                0.82, 0.8, 0.73, 0.63, 0.49, 0.53, 0.64, 0.76, 0.85, 0.81, 0.79, 0.81,
            ],
            f_shwith500: vec![
                0.55, 0.58, 0.47, 0.29, 0.13, 0.0, 0.02, 0.36, 0.58, 0.57, 0.59, 0.57,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                53.73, 53.08, 62.37, 54.15, 45.04, 57.45, 70.37, 77.03, 80.35, 61.92, 53.54, 48.81,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.05, 45.62, 42.17, 35.55, 27.87, 20.06, 17.03,
            ],
            f_shwith200: vec![
                0.84, 0.85, 0.82, 0.76, 0.68, 0.76, 0.8, 0.82, 0.89, 0.85, 0.84, 0.83,
            ],
            f_shwith300: vec![
                0.74, 0.77, 0.71, 0.67, 0.56, 0.62, 0.7, 0.78, 0.79, 0.75, 0.73, 0.72,
            ],
            f_shwith500: vec![
                0.56, 0.55, 0.5, 0.43, 0.34, 0.37, 0.39, 0.52, 0.59, 0.52, 0.46, 0.48,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                22.06, 25.88, 40.53, 47.53, 46.69, 67.57, 79.65, 68.23, 56.65, 32.63, 21.15, 17.03,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.05, 45.62, 42.17, 35.55, 27.87, 20.06, 17.03,
            ],
            f_shwith200: vec![
                0.61, 0.67, 0.69, 0.69, 0.66, 0.76, 0.8, 0.78, 0.78, 0.68, 0.56, 0.54,
            ],
            f_shwith300: vec![
                0.48, 0.5, 0.55, 0.57, 0.53, 0.65, 0.7, 0.69, 0.66, 0.52, 0.37, 0.34,
            ],
            f_shwith500: vec![
                0.13, 0.25, 0.36, 0.41, 0.38, 0.5, 0.54, 0.52, 0.43, 0.26, 0.13, 0.05,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.38, 3.55, 11.7, 21.93, 26.46, 43.45, 49.47, 31.97, 19.57, 5.86, 1.76, 0.38,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.05, 45.62, 42.17, 35.55, 27.87, 20.06, 17.03,
            ],
            f_shwith200: vec![
                0.0, 0.06, 0.26, 0.43, 0.45, 0.64, 0.64, 0.55, 0.42, 0.12, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.02, 0.16, 0.33, 0.35, 0.52, 0.55, 0.44, 0.27, 0.03, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.01, 0.1, 0.19, 0.3, 0.25, 0.1, 0.01, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C3,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.45, 7.11, 14.02, 13.79, 3.82, 0.06, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 45.05, 45.62, 42.17, 35.55, 27.87, 20.06, 17.03,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.04, 0.13, 0.08, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: C4,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                34.72, 45.39, 76.95, 93.85, 122.34, 180.04, 207.14, 185.25, 127.65, 66.36, 39.51,
                28.65,
            ],
            dif: vec![
                22.02, 25.59, 35.62, 42.12, 49.06, 34.74, 27.77, 26.2, 28.91, 29.88, 22.69, 20.58,
            ],
            f_shwith200: vec![
                0.74, 0.84, 0.89, 0.89, 0.92, 0.96, 0.97, 0.96, 0.95, 0.88, 0.76, 0.67,
            ],
            f_shwith300: vec![
                0.49, 0.68, 0.75, 0.79, 0.83, 0.91, 0.92, 0.93, 0.87, 0.74, 0.59, 0.4,
            ],
            f_shwith500: vec![
                0.08, 0.25, 0.45, 0.54, 0.62, 0.77, 0.8, 0.79, 0.7, 0.42, 0.15, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                1.18, 2.89, 11.79, 21.45, 37.19, 50.34, 55.25, 40.99, 21.42, 4.87, 1.2, 0.42,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 46.41, 46.35, 42.9, 36.27, 27.87, 19.99, 16.93,
            ],
            f_shwith200: vec![
                0.0, 0.05, 0.27, 0.46, 0.57, 0.69, 0.67, 0.61, 0.47, 0.08, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.0, 0.15, 0.34, 0.47, 0.58, 0.63, 0.54, 0.3, 0.01, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.08, 0.22, 0.29, 0.32, 0.19, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                20.75, 22.99, 41.01, 46.84, 61.75, 76.85, 90.11, 83.89, 62.6, 30.24, 20.88, 17.9,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 46.41, 46.35, 42.9, 36.27, 27.87, 19.99, 16.93,
            ],
            f_shwith200: vec![
                0.6, 0.6, 0.7, 0.7, 0.73, 0.8, 0.82, 0.82, 0.79, 0.66, 0.56, 0.56,
            ],
            f_shwith300: vec![
                0.45, 0.47, 0.54, 0.61, 0.63, 0.71, 0.75, 0.75, 0.7, 0.51, 0.42, 0.41,
            ],
            f_shwith500: vec![
                0.13, 0.19, 0.41, 0.4, 0.45, 0.53, 0.61, 0.6, 0.51, 0.25, 0.14, 0.06,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                52.08, 49.65, 62.95, 53.65, 55.61, 63.91, 80.0, 91.49, 88.52, 62.04, 55.43, 49.48,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 46.41, 46.35, 42.9, 36.27, 27.87, 19.99, 16.93,
            ],
            f_shwith200: vec![
                0.84, 0.83, 0.83, 0.77, 0.74, 0.8, 0.83, 0.85, 0.9, 0.86, 0.85, 0.84,
            ],
            f_shwith300: vec![
                0.73, 0.74, 0.7, 0.67, 0.63, 0.66, 0.75, 0.83, 0.81, 0.76, 0.76, 0.77,
            ],
            f_shwith500: vec![
                0.51, 0.5, 0.53, 0.4, 0.37, 0.41, 0.49, 0.61, 0.66, 0.55, 0.51, 0.48,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                73.0, 68.08, 72.0, 47.0, 33.27, 35.13, 49.0, 75.22, 93.55, 84.49, 77.66, 67.77,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 46.41, 46.35, 42.9, 36.27, 27.87, 19.99, 16.93,
            ],
            f_shwith200: vec![
                0.91, 0.88, 0.88, 0.77, 0.71, 0.7, 0.8, 0.89, 0.92, 0.9, 0.9, 0.9,
            ],
            f_shwith300: vec![
                0.82, 0.8, 0.73, 0.63, 0.49, 0.54, 0.68, 0.79, 0.86, 0.82, 0.83, 0.81,
            ],
            f_shwith500: vec![
                0.55, 0.58, 0.47, 0.29, 0.13, 0.0, 0.04, 0.38, 0.64, 0.6, 0.63, 0.57,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                53.73, 53.08, 62.37, 54.15, 45.04, 63.69, 75.65, 87.97, 84.03, 67.81, 57.2, 47.14,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 46.41, 46.35, 42.9, 36.27, 27.87, 19.99, 16.93,
            ],
            f_shwith200: vec![
                0.84, 0.85, 0.82, 0.76, 0.68, 0.78, 0.82, 0.85, 0.88, 0.87, 0.83, 0.81,
            ],
            f_shwith300: vec![
                0.74, 0.77, 0.71, 0.67, 0.56, 0.67, 0.73, 0.83, 0.8, 0.8, 0.76, 0.72,
            ],
            f_shwith500: vec![
                0.56, 0.55, 0.5, 0.43, 0.34, 0.38, 0.44, 0.59, 0.64, 0.59, 0.54, 0.52,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                22.06, 25.88, 40.53, 47.53, 46.69, 76.76, 83.78, 79.74, 57.65, 34.77, 22.42, 16.19,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 46.41, 46.35, 42.9, 36.27, 27.87, 19.99, 16.93,
            ],
            f_shwith200: vec![
                0.61, 0.67, 0.69, 0.69, 0.66, 0.79, 0.81, 0.82, 0.77, 0.7, 0.6, 0.51,
            ],
            f_shwith300: vec![
                0.48, 0.5, 0.55, 0.57, 0.53, 0.71, 0.73, 0.73, 0.67, 0.56, 0.45, 0.4,
            ],
            f_shwith500: vec![
                0.13, 0.25, 0.36, 0.41, 0.38, 0.54, 0.55, 0.56, 0.45, 0.27, 0.14, 0.09,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.38, 3.55, 11.7, 21.93, 26.46, 50.43, 50.64, 38.65, 18.9, 5.5, 1.6, 0.35,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 46.41, 46.35, 42.9, 36.27, 27.87, 19.99, 16.93,
            ],
            f_shwith200: vec![
                0.0, 0.06, 0.26, 0.43, 0.45, 0.68, 0.66, 0.61, 0.41, 0.11, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.02, 0.16, 0.33, 0.35, 0.6, 0.59, 0.49, 0.26, 0.01, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.01, 0.1, 0.19, 0.33, 0.26, 0.13, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: C4,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.45, 7.11, 16.16, 13.81, 4.63, 0.05, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                18.82, 22.5, 32.34, 38.5, 45.39, 46.41, 46.35, 42.9, 36.27, 27.87, 19.99, 16.93,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.04, 0.14, 0.07, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: D1,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                33.0, 50.72, 82.43, 105.38, 138.4, 133.86, 155.21, 137.09, 92.48, 72.51, 40.72,
                26.87,
            ],
            dif: vec![
                21.07, 23.14, 33.37, 37.85, 41.99, 45.75, 40.26, 36.91, 33.59, 27.21, 20.95, 20.06,
            ],
            f_shwith200: vec![
                0.7, 0.84, 0.88, 0.91, 0.93, 0.93, 0.95, 0.94, 0.92, 0.89, 0.74, 0.7,
            ],
            f_shwith300: vec![
                0.46, 0.67, 0.77, 0.83, 0.85, 0.84, 0.88, 0.88, 0.78, 0.75, 0.55, 0.46,
            ],
            f_shwith500: vec![
                0.12, 0.24, 0.53, 0.59, 0.64, 0.68, 0.74, 0.69, 0.58, 0.43, 0.18, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                0.93, 3.41, 11.78, 20.8, 40.3, 36.94, 42.74, 28.76, 16.19, 6.95, 2.0, 0.34,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.24, 45.65, 41.52, 33.62, 27.75, 19.32, 16.21,
            ],
            f_shwith200: vec![
                0.0, 0.06, 0.3, 0.43, 0.59, 0.58, 0.61, 0.52, 0.36, 0.13, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.01, 0.14, 0.3, 0.49, 0.49, 0.52, 0.4, 0.23, 0.07, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.1, 0.24, 0.27, 0.3, 0.15, 0.01, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                16.97, 29.11, 43.32, 48.05, 67.8, 56.6, 68.94, 60.09, 46.15, 38.13, 25.17, 15.34,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.24, 45.65, 41.52, 33.62, 27.75, 19.32, 16.21,
            ],
            f_shwith200: vec![
                0.53, 0.68, 0.71, 0.72, 0.77, 0.72, 0.76, 0.76, 0.73, 0.72, 0.62, 0.53,
            ],
            f_shwith300: vec![
                0.37, 0.58, 0.6, 0.59, 0.66, 0.61, 0.67, 0.65, 0.61, 0.6, 0.5, 0.38,
            ],
            f_shwith500: vec![
                0.13, 0.28, 0.45, 0.41, 0.45, 0.47, 0.51, 0.46, 0.41, 0.35, 0.15, 0.02,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                47.01, 60.51, 68.52, 58.06, 61.72, 47.37, 60.78, 66.62, 64.25, 72.02, 59.17, 45.06,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.24, 45.65, 41.52, 33.62, 27.75, 19.32, 16.21,
            ],
            f_shwith200: vec![
                0.8, 0.88, 0.84, 0.8, 0.76, 0.71, 0.78, 0.81, 0.84, 0.88, 0.86, 0.84,
            ],
            f_shwith300: vec![
                0.7, 0.8, 0.77, 0.7, 0.65, 0.58, 0.67, 0.73, 0.74, 0.81, 0.79, 0.75,
            ],
            f_shwith500: vec![
                0.49, 0.58, 0.63, 0.42, 0.41, 0.32, 0.37, 0.44, 0.49, 0.61, 0.54, 0.5,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                69.03, 79.64, 79.5, 54.83, 38.31, 26.6, 36.69, 56.85, 67.23, 92.31, 77.25, 63.4,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.24, 45.65, 41.52, 33.62, 27.75, 19.32, 16.21,
            ],
            f_shwith200: vec![
                0.89, 0.92, 0.87, 0.83, 0.73, 0.62, 0.75, 0.83, 0.85, 0.92, 0.89, 0.91,
            ],
            f_shwith300: vec![
                0.8, 0.81, 0.79, 0.69, 0.53, 0.47, 0.59, 0.7, 0.76, 0.84, 0.81, 0.84,
            ],
            f_shwith500: vec![
                0.55, 0.63, 0.55, 0.36, 0.19, 0.0, 0.05, 0.31, 0.5, 0.61, 0.57, 0.63,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                52.81, 60.11, 67.3, 61.39, 54.79, 46.52, 55.9, 68.05, 61.01, 72.15, 53.96, 45.28,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.24, 45.65, 41.52, 33.62, 27.75, 19.32, 16.21,
            ],
            f_shwith200: vec![
                0.85, 0.88, 0.84, 0.79, 0.72, 0.7, 0.76, 0.8, 0.83, 0.88, 0.83, 0.84,
            ],
            f_shwith300: vec![
                0.73, 0.78, 0.73, 0.73, 0.63, 0.56, 0.64, 0.71, 0.71, 0.82, 0.73, 0.77,
            ],
            f_shwith500: vec![
                0.51, 0.49, 0.56, 0.49, 0.4, 0.33, 0.34, 0.45, 0.49, 0.59, 0.46, 0.57,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                21.32, 29.67, 42.33, 51.67, 58.72, 55.02, 61.56, 62.23, 42.36, 38.02, 21.41, 15.51,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.24, 45.65, 41.52, 33.62, 27.75, 19.32, 16.21,
            ],
            f_shwith200: vec![
                0.59, 0.68, 0.7, 0.72, 0.7, 0.71, 0.75, 0.76, 0.72, 0.73, 0.6, 0.51,
            ],
            f_shwith300: vec![
                0.43, 0.54, 0.59, 0.62, 0.61, 0.59, 0.64, 0.65, 0.57, 0.58, 0.41, 0.41,
            ],
            f_shwith500: vec![
                0.14, 0.24, 0.34, 0.49, 0.46, 0.43, 0.45, 0.47, 0.35, 0.27, 0.16, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.28, 4.59, 11.6, 22.6, 34.38, 35.55, 37.19, 30.35, 14.08, 6.68, 1.89, 0.35,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.24, 45.65, 41.52, 33.62, 27.75, 19.32, 16.21,
            ],
            f_shwith200: vec![
                0.0, 0.05, 0.25, 0.45, 0.51, 0.57, 0.59, 0.51, 0.31, 0.13, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.01, 0.1, 0.37, 0.45, 0.44, 0.47, 0.38, 0.19, 0.0, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.11, 0.21, 0.25, 0.19, 0.16, 0.01, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D1,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.06, 8.73, 11.48, 10.7, 3.42, 0.06, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.24, 45.65, 41.52, 33.62, 27.75, 19.32, 16.21,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.04, 0.12, 0.06, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: D2,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                33.0, 50.72, 82.43, 105.38, 138.4, 154.61, 186.01, 161.01, 108.56, 71.27, 40.88,
                27.35,
            ],
            dif: vec![
                21.07, 23.14, 33.37, 37.85, 41.99, 39.58, 30.78, 32.69, 30.71, 28.45, 20.78, 19.58,
            ],
            f_shwith200: vec![
                0.7, 0.84, 0.88, 0.91, 0.93, 0.95, 0.96, 0.95, 0.93, 0.88, 0.76, 0.67,
            ],
            f_shwith300: vec![
                0.46, 0.67, 0.77, 0.83, 0.85, 0.88, 0.91, 0.91, 0.84, 0.74, 0.58, 0.4,
            ],
            f_shwith500: vec![
                0.12, 0.24, 0.53, 0.59, 0.64, 0.71, 0.77, 0.74, 0.63, 0.43, 0.12, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                0.93, 3.41, 11.78, 20.8, 40.3, 44.98, 49.38, 37.89, 17.31, 5.51, 1.89, 0.34,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.53, 45.22, 42.6, 34.25, 27.77, 19.4, 16.21,
            ],
            f_shwith200: vec![
                0.0, 0.06, 0.3, 0.43, 0.59, 0.64, 0.65, 0.59, 0.38, 0.1, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.01, 0.14, 0.3, 0.49, 0.55, 0.56, 0.49, 0.22, 0.0, 0.0, 0.0,
            ],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.1, 0.24, 0.3, 0.3, 0.2, 0.02, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: D2,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                16.97, 29.11, 43.32, 48.05, 67.8, 67.67, 80.3, 76.08, 51.43, 36.52, 24.24, 15.6,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.53, 45.22, 42.6, 34.25, 27.77, 19.4, 16.21,
            ],
            f_shwith200: vec![
                0.53, 0.68, 0.71, 0.72, 0.77, 0.77, 0.8, 0.8, 0.76, 0.72, 0.63, 0.51,
            ],
            f_shwith300: vec![
                0.37, 0.58, 0.6, 0.59, 0.66, 0.66, 0.72, 0.7, 0.62, 0.59, 0.49, 0.35,
            ],
            f_shwith500: vec![
                0.13, 0.28, 0.45, 0.41, 0.45, 0.51, 0.53, 0.58, 0.44, 0.33, 0.14, 0.08,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                47.01, 60.51, 68.52, 58.06, 61.72, 55.49, 71.39, 82.12, 73.92, 71.83, 57.84, 46.07,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.53, 45.22, 42.6, 34.25, 27.77, 19.4, 16.21,
            ],
            f_shwith200: vec![
                0.8, 0.88, 0.84, 0.8, 0.76, 0.76, 0.81, 0.84, 0.87, 0.87, 0.85, 0.84,
            ],
            f_shwith300: vec![
                0.7, 0.8, 0.77, 0.7, 0.65, 0.63, 0.71, 0.8, 0.75, 0.81, 0.78, 0.72,
            ],
            f_shwith500: vec![
                0.49, 0.58, 0.63, 0.42, 0.41, 0.35, 0.44, 0.54, 0.55, 0.63, 0.53, 0.49,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                69.03, 79.64, 79.5, 54.83, 38.31, 30.25, 43.73, 66.95, 80.4, 95.05, 79.61, 64.65,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.53, 45.22, 42.6, 34.25, 27.77, 19.4, 16.21,
            ],
            f_shwith200: vec![
                0.89, 0.92, 0.87, 0.83, 0.73, 0.67, 0.77, 0.86, 0.89, 0.91, 0.9, 0.9,
            ],
            f_shwith300: vec![
                0.8, 0.81, 0.79, 0.69, 0.53, 0.52, 0.61, 0.74, 0.81, 0.83, 0.82, 0.82,
            ],
            f_shwith500: vec![
                0.55, 0.63, 0.55, 0.36, 0.19, 0.0, 0.07, 0.38, 0.54, 0.65, 0.6, 0.6,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                52.81, 60.11, 67.3, 61.39, 54.79, 55.22, 68.46, 79.31, 73.1, 74.13, 58.95, 46.01,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.53, 45.22, 42.6, 34.25, 27.77, 19.4, 16.21,
            ],
            f_shwith200: vec![
                0.85, 0.88, 0.84, 0.79, 0.72, 0.76, 0.79, 0.82, 0.87, 0.87, 0.85, 0.85,
            ],
            f_shwith300: vec![
                0.73, 0.78, 0.73, 0.73, 0.63, 0.63, 0.71, 0.76, 0.76, 0.82, 0.75, 0.75,
            ],
            f_shwith500: vec![
                0.51, 0.49, 0.56, 0.49, 0.4, 0.32, 0.41, 0.51, 0.54, 0.65, 0.53, 0.46,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                21.32, 29.67, 42.33, 51.67, 58.72, 66.27, 76.41, 71.93, 49.99, 38.51, 25.32, 15.55,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.53, 45.22, 42.6, 34.25, 27.77, 19.4, 16.21,
            ],
            f_shwith200: vec![
                0.59, 0.68, 0.7, 0.72, 0.7, 0.77, 0.79, 0.79, 0.75, 0.71, 0.61, 0.51,
            ],
            f_shwith300: vec![
                0.43, 0.54, 0.59, 0.62, 0.61, 0.67, 0.7, 0.67, 0.64, 0.59, 0.45, 0.35,
            ],
            f_shwith500: vec![
                0.14, 0.24, 0.34, 0.49, 0.46, 0.47, 0.52, 0.52, 0.44, 0.33, 0.18, 0.02,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.28, 4.59, 11.6, 22.6, 34.38, 43.27, 46.81, 34.84, 16.08, 6.02, 2.32, 0.32,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.53, 45.22, 42.6, 34.25, 27.77, 19.4, 16.21,
            ],
            f_shwith200: vec![
                0.0, 0.05, 0.25, 0.45, 0.51, 0.65, 0.63, 0.56, 0.36, 0.11, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.01, 0.1, 0.37, 0.45, 0.54, 0.56, 0.43, 0.22, 0.01, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.11, 0.21, 0.26, 0.26, 0.14, 0.01, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D2,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.06, 8.73, 14.38, 12.86, 4.23, 0.06, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.53, 45.22, 42.6, 34.25, 27.77, 19.4, 16.21,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.04, 0.12, 0.06, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: D3,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                33.0, 50.72, 82.43, 105.38, 138.4, 167.72, 188.25, 168.36, 121.77, 72.39, 41.0,
                27.48,
            ],
            dif: vec![
                21.07, 23.14, 33.37, 37.85, 41.99, 36.81, 31.68, 29.77, 28.61, 27.32, 20.66, 19.46,
            ],
            f_shwith200: vec![
                0.7, 0.84, 0.88, 0.91, 0.93, 0.95, 0.96, 0.96, 0.95, 0.88, 0.77, 0.66,
            ],
            f_shwith300: vec![
                0.46, 0.67, 0.77, 0.83, 0.85, 0.89, 0.91, 0.91, 0.86, 0.76, 0.58, 0.35,
            ],
            f_shwith500: vec![
                0.12, 0.24, 0.53, 0.59, 0.64, 0.77, 0.77, 0.76, 0.67, 0.43, 0.17, 0.01,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                0.93, 3.41, 11.78, 20.8, 40.3, 43.85, 48.45, 36.54, 19.83, 6.71, 1.38, 0.34,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.06, 45.61, 42.32, 35.72, 27.79, 19.35, 16.24,
            ],
            f_shwith200: vec![
                0.0, 0.06, 0.3, 0.43, 0.59, 0.63, 0.64, 0.58, 0.4, 0.15, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.01, 0.14, 0.3, 0.49, 0.53, 0.58, 0.47, 0.27, 0.04, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.1, 0.24, 0.32, 0.32, 0.15, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                16.97, 29.11, 43.32, 48.05, 67.8, 68.29, 79.24, 75.13, 59.75, 36.49, 21.78, 17.53,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.06, 45.61, 42.32, 35.72, 27.79, 19.35, 16.24,
            ],
            f_shwith200: vec![
                0.53, 0.68, 0.71, 0.72, 0.77, 0.76, 0.79, 0.79, 0.79, 0.7, 0.6, 0.58,
            ],
            f_shwith300: vec![
                0.37, 0.58, 0.6, 0.59, 0.66, 0.67, 0.71, 0.7, 0.67, 0.55, 0.44, 0.4,
            ],
            f_shwith500: vec![
                0.13, 0.28, 0.45, 0.41, 0.45, 0.53, 0.56, 0.57, 0.48, 0.32, 0.14, 0.06,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                47.01, 60.51, 68.52, 58.06, 61.72, 58.03, 70.71, 81.91, 85.36, 69.03, 56.05, 48.2,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.06, 45.61, 42.32, 35.72, 27.79, 19.35, 16.24,
            ],
            f_shwith200: vec![
                0.8, 0.88, 0.84, 0.8, 0.76, 0.76, 0.8, 0.83, 0.89, 0.86, 0.86, 0.83,
            ],
            f_shwith300: vec![
                0.7, 0.8, 0.77, 0.7, 0.65, 0.65, 0.71, 0.79, 0.8, 0.79, 0.75, 0.74,
            ],
            f_shwith500: vec![
                0.49, 0.58, 0.63, 0.42, 0.41, 0.41, 0.45, 0.57, 0.6, 0.58, 0.52, 0.46,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                69.03, 79.64, 79.5, 54.83, 38.31, 33.29, 43.99, 67.87, 91.27, 91.24, 77.92, 65.46,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.06, 45.61, 42.32, 35.72, 27.79, 19.35, 16.24,
            ],
            f_shwith200: vec![
                0.89, 0.92, 0.87, 0.83, 0.73, 0.67, 0.78, 0.87, 0.92, 0.91, 0.9, 0.9,
            ],
            f_shwith300: vec![
                0.8, 0.81, 0.79, 0.69, 0.53, 0.53, 0.63, 0.75, 0.84, 0.83, 0.84, 0.77,
            ],
            f_shwith500: vec![
                0.55, 0.63, 0.55, 0.36, 0.19, 0.0, 0.02, 0.34, 0.56, 0.61, 0.62, 0.55,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                52.81, 60.11, 67.3, 61.39, 54.79, 58.53, 70.6, 81.0, 82.78, 74.02, 57.32, 45.06,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.06, 45.61, 42.32, 35.72, 27.79, 19.35, 16.24,
            ],
            f_shwith200: vec![
                0.85, 0.88, 0.84, 0.79, 0.72, 0.76, 0.81, 0.83, 0.89, 0.89, 0.86, 0.8,
            ],
            f_shwith300: vec![
                0.73, 0.78, 0.73, 0.73, 0.63, 0.64, 0.7, 0.79, 0.79, 0.82, 0.77, 0.66,
            ],
            f_shwith500: vec![
                0.51, 0.49, 0.56, 0.49, 0.4, 0.4, 0.43, 0.53, 0.57, 0.58, 0.51, 0.45,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                21.32, 29.67, 42.33, 51.67, 58.72, 68.65, 79.7, 74.22, 57.53, 40.44, 22.97, 15.3,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.06, 45.61, 42.32, 35.72, 27.79, 19.35, 16.24,
            ],
            f_shwith200: vec![
                0.59, 0.68, 0.7, 0.72, 0.7, 0.77, 0.8, 0.8, 0.79, 0.74, 0.6, 0.5,
            ],
            f_shwith300: vec![
                0.43, 0.54, 0.59, 0.62, 0.61, 0.67, 0.73, 0.71, 0.67, 0.64, 0.45, 0.35,
            ],
            f_shwith500: vec![
                0.14, 0.24, 0.34, 0.49, 0.46, 0.53, 0.53, 0.55, 0.44, 0.29, 0.09, 0.08,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.28, 4.59, 11.6, 22.6, 34.38, 43.86, 49.21, 36.16, 19.28, 7.31, 1.79, 0.34,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.06, 45.61, 42.32, 35.72, 27.79, 19.35, 16.24,
            ],
            f_shwith200: vec![
                0.0, 0.05, 0.25, 0.45, 0.51, 0.65, 0.65, 0.59, 0.41, 0.14, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.01, 0.1, 0.37, 0.45, 0.54, 0.59, 0.47, 0.23, 0.02, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.11, 0.21, 0.31, 0.28, 0.12, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: D3,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.06, 8.73, 12.89, 13.13, 4.08, 0.03, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                18.18, 22.2, 31.99, 37.79, 44.08, 45.06, 45.61, 42.32, 35.72, 27.79, 19.35, 16.24,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.04, 0.12, 0.08, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
        SurfaceMonthlyRadiation {
            zone: E1,
            orientation: HZ,
            beta: 0.0,
            gamma: 0.0,
            dir: vec![
                31.89, 46.33, 77.32, 98.34, 134.55, 133.78, 154.75, 137.97, 90.97, 69.95, 38.93,
                27.62,
            ],
            dif: vec![
                21.32, 23.08, 34.3, 39.16, 43.11, 45.85, 40.71, 36.03, 35.09, 25.68, 20.58, 18.7,
            ],
            f_shwith200: vec![
                0.69, 0.81, 0.88, 0.9, 0.93, 0.93, 0.95, 0.94, 0.91, 0.87, 0.74, 0.68,
            ],
            f_shwith300: vec![
                0.48, 0.64, 0.76, 0.82, 0.84, 0.85, 0.86, 0.88, 0.8, 0.73, 0.56, 0.4,
            ],
            f_shwith500: vec![
                0.06, 0.31, 0.48, 0.59, 0.67, 0.69, 0.71, 0.71, 0.57, 0.41, 0.16, 0.03,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1,
            orientation: NE,
            beta: 90.0,
            gamma: -135.0,
            dir: vec![
                0.93, 3.03, 9.21, 20.94, 33.25, 36.8, 42.94, 31.17, 14.67, 5.74, 1.53, 0.44,
            ],
            dif: vec![
                17.58, 21.04, 31.42, 36.74, 43.74, 45.31, 45.88, 41.29, 33.76, 26.5, 18.44, 15.88,
            ],
            f_shwith200: vec![
                0.0, 0.03, 0.25, 0.44, 0.52, 0.6, 0.6, 0.54, 0.33, 0.09, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.0, 0.11, 0.34, 0.45, 0.47, 0.5, 0.45, 0.23, 0.01, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.15, 0.26, 0.26, 0.26, 0.15, 0.01, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1,
            orientation: E,
            beta: 90.0,
            gamma: -90.0,
            dir: vec![
                18.75, 23.96, 34.99, 47.89, 58.14, 56.45, 69.04, 62.62, 44.69, 36.14, 22.02, 17.97,
            ],
            dif: vec![
                17.58, 21.04, 31.42, 36.74, 43.74, 45.31, 45.88, 41.29, 33.76, 26.5, 18.44, 15.88,
            ],
            f_shwith200: vec![
                0.56, 0.63, 0.66, 0.7, 0.71, 0.71, 0.76, 0.77, 0.7, 0.71, 0.6, 0.58,
            ],
            f_shwith300: vec![
                0.48, 0.47, 0.54, 0.61, 0.61, 0.6, 0.65, 0.67, 0.58, 0.55, 0.48, 0.49,
            ],
            f_shwith500: vec![
                0.19, 0.27, 0.36, 0.49, 0.48, 0.45, 0.48, 0.52, 0.45, 0.28, 0.2, 0.08,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1,
            orientation: SE,
            beta: 90.0,
            gamma: -45.0,
            dir: vec![
                48.53, 52.1, 57.19, 56.53, 55.7, 47.42, 60.72, 67.52, 64.2, 70.51, 54.24, 48.08,
            ],
            dif: vec![
                17.58, 21.04, 31.42, 36.74, 43.74, 45.31, 45.88, 41.29, 33.76, 26.5, 18.44, 15.88,
            ],
            f_shwith200: vec![
                0.82, 0.82, 0.82, 0.77, 0.73, 0.7, 0.76, 0.82, 0.82, 0.88, 0.82, 0.84,
            ],
            f_shwith300: vec![
                0.76, 0.73, 0.73, 0.68, 0.64, 0.57, 0.64, 0.75, 0.72, 0.79, 0.74, 0.78,
            ],
            f_shwith500: vec![
                0.58, 0.54, 0.52, 0.49, 0.44, 0.34, 0.39, 0.48, 0.56, 0.55, 0.6, 0.56,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1,
            orientation: S,
            beta: 90.0,
            gamma: 0.0,
            dir: vec![
                68.07, 70.87, 71.87, 50.86, 39.34, 26.66, 36.6, 54.87, 67.41, 91.17, 74.26, 65.91,
            ],
            dif: vec![
                17.58, 21.04, 31.42, 36.74, 43.74, 45.31, 45.88, 41.29, 33.76, 26.5, 18.44, 15.88,
            ],
            f_shwith200: vec![
                0.89, 0.88, 0.87, 0.81, 0.75, 0.63, 0.72, 0.85, 0.85, 0.92, 0.88, 0.91,
            ],
            f_shwith300: vec![
                0.8, 0.78, 0.78, 0.67, 0.57, 0.46, 0.57, 0.69, 0.74, 0.81, 0.81, 0.84,
            ],
            f_shwith500: vec![
                0.62, 0.6, 0.53, 0.37, 0.2, 0.0, 0.06, 0.29, 0.52, 0.61, 0.63, 0.58,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1,
            orientation: SW,
            beta: 90.0,
            gamma: 45.0,
            dir: vec![
                49.76, 54.99, 65.6, 54.88, 56.95, 47.83, 55.51, 64.35, 59.92, 70.27, 53.93, 45.97,
            ],
            dif: vec![
                17.58, 21.04, 31.42, 36.74, 43.74, 45.31, 45.88, 41.29, 33.76, 26.5, 18.44, 15.88,
            ],
            f_shwith200: vec![
                0.82, 0.85, 0.83, 0.78, 0.74, 0.72, 0.73, 0.81, 0.81, 0.88, 0.83, 0.84,
            ],
            f_shwith300: vec![
                0.75, 0.76, 0.76, 0.68, 0.65, 0.57, 0.62, 0.73, 0.7, 0.79, 0.76, 0.71,
            ],
            f_shwith500: vec![
                0.56, 0.55, 0.58, 0.43, 0.42, 0.32, 0.38, 0.49, 0.51, 0.6, 0.56, 0.46,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1,
            orientation: W,
            beta: 90.0,
            gamma: 90.0,
            dir: vec![
                19.73, 26.57, 42.88, 46.18, 59.96, 57.28, 60.85, 57.98, 41.31, 36.24, 21.86, 16.44,
            ],
            dif: vec![
                17.58, 21.04, 31.42, 36.74, 43.74, 45.31, 45.88, 41.29, 33.76, 26.5, 18.44, 15.88,
            ],
            f_shwith200: vec![
                0.6, 0.66, 0.72, 0.7, 0.73, 0.72, 0.73, 0.75, 0.68, 0.7, 0.61, 0.53,
            ],
            f_shwith300: vec![
                0.47, 0.52, 0.61, 0.59, 0.63, 0.64, 0.6, 0.63, 0.56, 0.58, 0.46, 0.38,
            ],
            f_shwith500: vec![
                0.13, 0.27, 0.44, 0.44, 0.46, 0.46, 0.46, 0.49, 0.37, 0.35, 0.19, 0.1,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1,
            orientation: NW,
            beta: 90.0,
            gamma: 135.0,
            dir: vec![
                1.1, 3.83, 11.95, 20.19, 34.57, 37.56, 36.57, 27.77, 14.17, 6.12, 1.61, 0.4,
            ],
            dif: vec![
                17.58, 21.04, 31.42, 36.74, 43.74, 45.31, 45.88, 41.29, 33.76, 26.5, 18.44, 15.88,
            ],
            f_shwith200: vec![
                0.0, 0.06, 0.29, 0.44, 0.55, 0.6, 0.56, 0.5, 0.33, 0.13, 0.0, 0.0,
            ],
            f_shwith300: vec![
                0.0, 0.03, 0.17, 0.31, 0.45, 0.49, 0.45, 0.4, 0.2, 0.03, 0.0, 0.0,
            ],
            f_shwith500: vec![
                0.0, 0.0, 0.0, 0.09, 0.25, 0.29, 0.21, 0.16, 0.0, 0.0, 0.0, 0.0,
            ],
        },
        SurfaceMonthlyRadiation {
            zone: E1,
            orientation: N,
            beta: 90.0,
            gamma: 180.0,
            dir: vec![
                0.0, 0.0, 0.0, 1.17, 7.64, 11.88, 10.63, 3.3, 0.03, 0.0, 0.0, 0.0,
            ],
            dif: vec![
                17.58, 21.04, 31.42, 36.74, 43.74, 45.31, 45.88, 41.29, 33.76, 26.5, 18.44, 15.88,
            ],
            f_shwith200: vec![
                0.0, 0.0, 0.0, 0.0, 0.03, 0.11, 0.04, 0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            f_shwith300: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            f_shwith500: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        },
    ];

    Mutex::new(raddata)
});

/// Datos de radiación horarios para el día 21 de julio
pub static JULYRADDATA: Lazy<Mutex<HashMap<ClimateZone, Vec<RadData>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(
        A3c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 14.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 76.0,
                dif: 79.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 156.0,
                dif: 156.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 465.0,
                dif: 153.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 598.0,
                dif: 177.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 607.0,
                dif: 252.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 570.0,
                dif: 294.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 423.0,
                dif: 330.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 447.0,
                dif: 283.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 425.0,
                dif: 237.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 454.0,
                dif: 157.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 201.0,
                dif: 136.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 113.0,
                dif: 64.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 4.0,
                dif: 19.0,
            },
        ],
    );
    map.insert(
        C2,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 110.2,
                altitude: 7.8,
                dir: 6.0,
                dif: 37.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 101.2,
                altitude: 18.7,
                dir: 34.0,
                dif: 106.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 92.1,
                altitude: 30.0,
                dir: 25.0,
                dif: 165.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 81.5,
                altitude: 41.4,
                dir: 213.0,
                dif: 224.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 69.0,
                altitude: 52.4,
                dir: 139.0,
                dif: 283.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 50.1,
                altitude: 62.2,
                dir: 49.0,
                dif: 318.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 19.5,
                altitude: 68.8,
                dir: 138.0,
                dif: 337.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -19.5,
                altitude: 68.8,
                dir: 187.0,
                dif: 335.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -50.1,
                altitude: 62.2,
                dir: 343.0,
                dif: 306.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -69.0,
                altitude: 52.4,
                dir: 413.0,
                dif: 220.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -81.5,
                altitude: 41.4,
                dir: 186.0,
                dif: 225.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -92.1,
                altitude: 30.0,
                dir: 233.0,
                dif: 133.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -101.2,
                altitude: 18.7,
                dir: 178.0,
                dif: 62.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -110.2,
                altitude: 7.8,
                dir: 45.0,
                dif: 39.0,
            },
        ],
    );
    map.insert(
        Alfa3c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 7.0,
                dif: 19.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 197.0,
                dif: 37.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 391.0,
                dif: 61.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 586.0,
                dif: 90.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 800.0,
                dif: 104.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 854.0,
                dif: 138.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 782.0,
                dif: 210.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 757.0,
                dif: 222.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 612.0,
                dif: 248.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 592.0,
                dif: 183.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 426.0,
                dif: 154.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 239.0,
                dif: 123.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 132.0,
                dif: 70.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 10.0,
                dif: 19.0,
            },
        ],
    );
    map.insert(
        D2c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 10.0,
                dif: 18.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 154.0,
                dif: 52.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 276.0,
                dif: 109.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 551.0,
                dif: 113.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 695.0,
                dif: 135.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 682.0,
                dif: 216.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 619.0,
                dif: 280.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 674.0,
                dif: 240.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 627.0,
                dif: 241.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 485.0,
                dif: 213.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 473.0,
                dif: 129.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 290.0,
                dif: 110.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 59.0,
                dif: 79.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 5.0,
                dif: 18.0,
            },
        ],
    );
    map.insert(
        D2,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 110.2,
                altitude: 7.8,
                dir: 42.0,
                dif: 31.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 101.2,
                altitude: 18.7,
                dir: 247.0,
                dif: 44.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 92.1,
                altitude: 30.0,
                dir: 471.0,
                dif: 49.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 81.5,
                altitude: 41.4,
                dir: 609.0,
                dif: 81.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 69.0,
                altitude: 52.4,
                dir: 782.0,
                dif: 94.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 50.1,
                altitude: 62.2,
                dir: 957.0,
                dif: 78.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 19.5,
                altitude: 68.8,
                dir: 985.0,
                dif: 93.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -19.5,
                altitude: 68.8,
                dir: 951.0,
                dif: 104.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -50.1,
                altitude: 62.2,
                dir: 876.0,
                dif: 113.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -69.0,
                altitude: 52.4,
                dir: 702.0,
                dif: 121.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -81.5,
                altitude: 41.4,
                dir: 525.0,
                dif: 118.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -92.1,
                altitude: 30.0,
                dir: 316.0,
                dif: 100.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -101.2,
                altitude: 18.7,
                dir: 205.0,
                dif: 67.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -110.2,
                altitude: 7.8,
                dir: 37.0,
                dif: 31.0,
            },
        ],
    );
    map.insert(
        E1,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 110.2,
                altitude: 7.8,
                dir: 9.0,
                dif: 38.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 101.2,
                altitude: 18.7,
                dir: 171.0,
                dif: 73.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 92.1,
                altitude: 30.0,
                dir: 401.0,
                dif: 73.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 81.5,
                altitude: 41.4,
                dir: 490.0,
                dif: 132.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 69.0,
                altitude: 52.4,
                dir: 648.0,
                dif: 150.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 50.1,
                altitude: 62.2,
                dir: 807.0,
                dif: 142.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 19.5,
                altitude: 68.8,
                dir: 938.0,
                dif: 111.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -19.5,
                altitude: 68.8,
                dir: 955.0,
                dif: 105.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -50.1,
                altitude: 62.2,
                dir: 812.0,
                dif: 140.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -69.0,
                altitude: 52.4,
                dir: 751.0,
                dif: 108.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -81.5,
                altitude: 41.4,
                dir: 518.0,
                dif: 129.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -92.1,
                altitude: 30.0,
                dir: 287.0,
                dif: 128.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -101.2,
                altitude: 18.7,
                dir: 208.0,
                dif: 72.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -110.2,
                altitude: 7.8,
                dir: 53.0,
                dif: 30.0,
            },
        ],
    );
    map.insert(
        C3,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 110.2,
                altitude: 7.8,
                dir: 53.0,
                dif: 33.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 101.2,
                altitude: 18.7,
                dir: 229.0,
                dif: 59.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 92.1,
                altitude: 30.0,
                dir: 315.0,
                dif: 112.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 81.5,
                altitude: 41.4,
                dir: 167.0,
                dif: 221.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 69.0,
                altitude: 52.4,
                dir: 278.0,
                dif: 258.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 50.1,
                altitude: 62.2,
                dir: 424.0,
                dif: 263.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 19.5,
                altitude: 68.8,
                dir: 319.0,
                dif: 313.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -19.5,
                altitude: 68.8,
                dir: 581.0,
                dif: 266.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -50.1,
                altitude: 62.2,
                dir: 525.0,
                dif: 237.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -69.0,
                altitude: 52.4,
                dir: 619.0,
                dif: 158.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -81.5,
                altitude: 41.4,
                dir: 429.0,
                dif: 150.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -92.1,
                altitude: 30.0,
                dir: 130.0,
                dif: 164.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -101.2,
                altitude: 18.7,
                dir: 130.0,
                dif: 80.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -110.2,
                altitude: 7.8,
                dir: 19.0,
                dif: 35.0,
            },
        ],
    );
    map.insert(
        D1c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 9.0,
                dif: 24.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 129.0,
                dif: 64.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 230.0,
                dif: 134.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 320.0,
                dif: 210.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 525.0,
                dif: 223.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 589.0,
                dif: 261.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 685.0,
                dif: 275.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 680.0,
                dif: 274.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 687.0,
                dif: 223.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 649.0,
                dif: 161.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 517.0,
                dif: 123.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 257.0,
                dif: 135.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 57.0,
                dif: 94.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 16.0,
            },
        ],
    );
    map.insert(
        D3c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 3.0,
                dif: 17.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 122.0,
                dif: 69.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 120.0,
                dif: 150.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 25.0,
                dif: 202.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 84.0,
                dif: 274.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 110.0,
                dif: 316.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 263.0,
                dif: 336.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 95.0,
                dif: 333.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 101.0,
                dif: 311.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 88.0,
                dif: 276.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 7.0,
                dif: 159.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 13.0,
                dif: 137.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 28.0,
                dif: 79.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 14.0,
            },
        ],
    );
    map.insert(
        C1,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 110.2,
                altitude: 7.8,
                dir: 0.0,
                dif: 20.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 101.2,
                altitude: 18.7,
                dir: 57.0,
                dif: 96.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 92.1,
                altitude: 30.0,
                dir: 94.0,
                dif: 174.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 81.5,
                altitude: 41.4,
                dir: 78.0,
                dif: 215.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 69.0,
                altitude: 52.4,
                dir: 155.0,
                dif: 268.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 50.1,
                altitude: 62.2,
                dir: 31.0,
                dif: 269.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 19.5,
                altitude: 68.8,
                dir: 24.0,
                dif: 209.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -19.5,
                altitude: 68.8,
                dir: 89.0,
                dif: 299.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -50.1,
                altitude: 62.2,
                dir: 156.0,
                dif: 308.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -69.0,
                altitude: 52.4,
                dir: 86.0,
                dif: 259.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -81.5,
                altitude: 41.4,
                dir: 107.0,
                dif: 220.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -92.1,
                altitude: 30.0,
                dir: 16.0,
                dif: 135.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -101.2,
                altitude: 18.7,
                dir: 56.0,
                dif: 96.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -110.2,
                altitude: 7.8,
                dir: 36.0,
                dif: 32.0,
            },
        ],
    );
    map.insert(
        B4,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 110.2,
                altitude: 7.8,
                dir: 9.0,
                dif: 45.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 101.2,
                altitude: 18.7,
                dir: 57.0,
                dif: 116.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 92.1,
                altitude: 30.0,
                dir: 270.0,
                dif: 137.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 81.5,
                altitude: 41.4,
                dir: 444.0,
                dif: 156.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 69.0,
                altitude: 52.4,
                dir: 680.0,
                dif: 121.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 50.1,
                altitude: 62.2,
                dir: 829.0,
                dif: 105.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 19.5,
                altitude: 68.8,
                dir: 850.0,
                dif: 127.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -19.5,
                altitude: 68.8,
                dir: 843.0,
                dif: 130.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -50.1,
                altitude: 62.2,
                dir: 804.0,
                dif: 116.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -69.0,
                altitude: 52.4,
                dir: 681.0,
                dif: 113.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -81.5,
                altitude: 41.4,
                dir: 554.0,
                dif: 89.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -92.1,
                altitude: 30.0,
                dir: 417.0,
                dif: 56.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -101.2,
                altitude: 18.7,
                dir: 169.0,
                dif: 96.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -110.2,
                altitude: 7.8,
                dir: 9.0,
                dif: 40.0,
            },
        ],
    );
    map.insert(
        C3c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 5.0,
                dif: 17.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 164.0,
                dif: 45.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 361.0,
                dif: 79.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 410.0,
                dif: 154.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 533.0,
                dif: 197.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 506.0,
                dif: 249.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 551.0,
                dif: 272.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 520.0,
                dif: 276.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 585.0,
                dif: 233.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 503.0,
                dif: 205.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 526.0,
                dif: 113.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 377.0,
                dif: 67.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 190.0,
                dif: 42.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 3.0,
                dif: 18.0,
            },
        ],
    );
    map.insert(
        A3,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 110.2,
                altitude: 7.8,
                dir: 3.0,
                dif: 35.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 101.2,
                altitude: 18.7,
                dir: 138.0,
                dif: 79.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 92.1,
                altitude: 30.0,
                dir: 273.0,
                dif: 115.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 81.5,
                altitude: 41.4,
                dir: 438.0,
                dif: 143.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 69.0,
                altitude: 52.4,
                dir: 609.0,
                dif: 155.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 50.1,
                altitude: 62.2,
                dir: 778.0,
                dif: 143.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 19.5,
                altitude: 68.8,
                dir: 733.0,
                dif: 192.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -19.5,
                altitude: 68.8,
                dir: 740.0,
                dif: 187.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -50.1,
                altitude: 62.2,
                dir: 749.0,
                dif: 151.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -69.0,
                altitude: 52.4,
                dir: 684.0,
                dif: 120.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -81.5,
                altitude: 41.4,
                dir: 573.0,
                dif: 91.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -92.1,
                altitude: 30.0,
                dir: 358.0,
                dif: 84.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -101.2,
                altitude: 18.7,
                dir: 196.0,
                dif: 56.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -110.2,
                altitude: 7.8,
                dir: 42.0,
                dif: 30.0,
            },
        ],
    );
    map.insert(
        Alfa4c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 11.0,
                dif: 25.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 173.0,
                dif: 45.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 389.0,
                dif: 61.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 497.0,
                dif: 124.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 610.0,
                dif: 170.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 684.0,
                dif: 211.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 724.0,
                dif: 238.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 580.0,
                dif: 313.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 583.0,
                dif: 250.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 562.0,
                dif: 205.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 390.0,
                dif: 171.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 211.0,
                dif: 134.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 87.0,
                dif: 79.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 17.0,
            },
        ],
    );
    map.insert(
        C2c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 13.0,
                dif: 20.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 158.0,
                dif: 74.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 131.0,
                dif: 163.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 111.0,
                dif: 246.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 213.0,
                dif: 297.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 193.0,
                dif: 372.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 211.0,
                dif: 366.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 231.0,
                dif: 399.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 293.0,
                dif: 331.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 187.0,
                dif: 301.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 160.0,
                dif: 244.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 161.0,
                dif: 152.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 140.0,
                dif: 67.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 10.0,
                dif: 22.0,
            },
        ],
    );
    map.insert(
        B4c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 4.0,
                dif: 19.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 34.0,
                dif: 100.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 89.0,
                dif: 189.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 284.0,
                dif: 222.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 225.0,
                dif: 307.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 359.0,
                dif: 330.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 533.0,
                dif: 307.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 575.0,
                dif: 312.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 583.0,
                dif: 264.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 469.0,
                dif: 235.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 183.0,
                dif: 245.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 221.0,
                dif: 141.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 143.0,
                dif: 57.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 6.0,
                dif: 19.0,
            },
        ],
    );
    map.insert(
        A4,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 110.2,
                altitude: 7.8,
                dir: 53.0,
                dif: 32.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 101.2,
                altitude: 18.7,
                dir: 183.0,
                dif: 66.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 92.1,
                altitude: 30.0,
                dir: 411.0,
                dif: 76.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 81.5,
                altitude: 41.4,
                dir: 641.0,
                dif: 57.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 69.0,
                altitude: 52.4,
                dir: 789.0,
                dif: 72.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 50.1,
                altitude: 62.2,
                dir: 896.0,
                dif: 84.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 19.5,
                altitude: 68.8,
                dir: 941.0,
                dif: 92.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -19.5,
                altitude: 68.8,
                dir: 895.0,
                dif: 108.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -50.1,
                altitude: 62.2,
                dir: 828.0,
                dif: 121.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -69.0,
                altitude: 52.4,
                dir: 694.0,
                dif: 115.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -81.5,
                altitude: 41.4,
                dir: 492.0,
                dif: 122.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -92.1,
                altitude: 30.0,
                dir: 350.0,
                dif: 86.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -101.2,
                altitude: 18.7,
                dir: 185.0,
                dif: 60.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -110.2,
                altitude: 7.8,
                dir: 35.0,
                dif: 34.0,
            },
        ],
    );
    map.insert(
        Alfa1c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 11.0,
                dif: 20.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 154.0,
                dif: 67.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 192.0,
                dif: 153.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 227.0,
                dif: 231.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 268.0,
                dif: 303.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 186.0,
                dif: 358.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 143.0,
                dif: 419.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 201.0,
                dif: 422.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 285.0,
                dif: 338.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 72.0,
                dif: 303.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 134.0,
                dif: 252.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 192.0,
                dif: 152.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 44.0,
                dif: 89.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 12.0,
            },
        ],
    );
    map.insert(
        E1c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 7.0,
                dif: 20.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 72.0,
                dif: 84.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 258.0,
                dif: 132.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 203.0,
                dif: 248.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 249.0,
                dif: 312.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 524.0,
                dif: 277.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 308.0,
                dif: 379.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 364.0,
                dif: 376.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 282.0,
                dif: 352.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 366.0,
                dif: 283.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 150.0,
                dif: 266.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 102.0,
                dif: 182.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 9.0,
                dif: 89.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 1.0,
                dif: 20.0,
            },
        ],
    );
    map.insert(
        C4c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 17.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 169.0,
                dif: 48.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 397.0,
                dif: 80.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 592.0,
                dif: 86.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 745.0,
                dif: 103.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 857.0,
                dif: 121.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 870.0,
                dif: 157.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 878.0,
                dif: 152.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 823.0,
                dif: 138.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 761.0,
                dif: 96.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 611.0,
                dif: 69.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 378.0,
                dif: 65.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 171.0,
                dif: 46.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 12.0,
                dif: 22.0,
            },
        ],
    );
    map.insert(
        B1c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 10.0,
                dif: 25.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 118.0,
                dif: 69.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 393.0,
                dif: 76.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 614.0,
                dif: 104.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 689.0,
                dif: 158.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 681.0,
                dif: 231.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 718.0,
                dif: 252.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 676.0,
                dif: 273.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 687.0,
                dif: 224.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 596.0,
                dif: 188.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 479.0,
                dif: 141.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 383.0,
                dif: 78.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 195.0,
                dif: 46.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 15.0,
            },
        ],
    );
    map.insert(
        C4,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 110.2,
                altitude: 7.8,
                dir: 54.0,
                dif: 34.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 101.2,
                altitude: 18.7,
                dir: 230.0,
                dif: 53.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 92.1,
                altitude: 30.0,
                dir: 311.0,
                dif: 104.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 81.5,
                altitude: 41.4,
                dir: 387.0,
                dif: 160.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 69.0,
                altitude: 52.4,
                dir: 511.0,
                dif: 206.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 50.1,
                altitude: 62.2,
                dir: 676.0,
                dif: 186.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 19.5,
                altitude: 68.8,
                dir: 767.0,
                dif: 170.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -19.5,
                altitude: 68.8,
                dir: 688.0,
                dif: 214.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -50.1,
                altitude: 62.2,
                dir: 746.0,
                dif: 151.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -69.0,
                altitude: 52.4,
                dir: 718.0,
                dif: 107.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -81.5,
                altitude: 41.4,
                dir: 502.0,
                dif: 117.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -92.1,
                altitude: 30.0,
                dir: 327.0,
                dif: 102.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -101.2,
                altitude: 18.7,
                dir: 145.0,
                dif: 81.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -110.2,
                altitude: 7.8,
                dir: 17.0,
                dif: 38.0,
            },
        ],
    );
    map.insert(
        A2c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 18.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 73.0,
                dif: 82.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 243.0,
                dif: 127.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 373.0,
                dif: 179.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 505.0,
                dif: 210.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 582.0,
                dif: 243.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 622.0,
                dif: 281.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 647.0,
                dif: 280.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 662.0,
                dif: 231.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 531.0,
                dif: 215.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 478.0,
                dif: 151.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 219.0,
                dif: 136.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 96.0,
                dif: 72.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 9.0,
            },
        ],
    );
    map.insert(
        C1c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 6.0,
                dif: 19.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 17.0,
                dif: 75.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 173.0,
                dif: 142.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 119.0,
                dif: 229.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 197.0,
                dif: 278.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 109.0,
                dif: 308.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 216.0,
                dif: 340.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 76.0,
                dif: 275.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 271.0,
                dif: 292.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 22.0,
                dif: 233.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 41.0,
                dif: 214.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 68.0,
                dif: 164.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 26.0,
                dif: 85.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 7.0,
                dif: 19.0,
            },
        ],
    );
    map.insert(
        D1,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 110.2,
                altitude: 7.8,
                dir: 0.0,
                dif: 26.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 101.2,
                altitude: 18.7,
                dir: 0.0,
                dif: 43.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 92.1,
                altitude: 30.0,
                dir: 0.0,
                dif: 60.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 81.5,
                altitude: 41.4,
                dir: 7.0,
                dif: 176.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 69.0,
                altitude: 52.4,
                dir: 16.0,
                dif: 250.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 50.1,
                altitude: 62.2,
                dir: 11.0,
                dif: 239.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 19.5,
                altitude: 68.8,
                dir: 8.0,
                dif: 237.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -19.5,
                altitude: 68.8,
                dir: 6.0,
                dif: 212.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -50.1,
                altitude: 62.2,
                dir: 68.0,
                dif: 307.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -69.0,
                altitude: 52.4,
                dir: 3.0,
                dif: 156.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -81.5,
                altitude: 41.4,
                dir: 0.0,
                dif: 131.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -92.1,
                altitude: 30.0,
                dir: 0.0,
                dif: 85.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -101.2,
                altitude: 18.7,
                dir: 0.0,
                dif: 20.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -110.2,
                altitude: 7.8,
                dir: 0.0,
                dif: 34.0,
            },
        ],
    );
    map.insert(
        Alfa2c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 9.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 30.0,
                dif: 79.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 168.0,
                dif: 141.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 476.0,
                dif: 145.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 755.0,
                dif: 117.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 879.0,
                dif: 129.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 961.0,
                dif: 133.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 1019.0,
                dif: 111.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 961.0,
                dif: 98.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 853.0,
                dif: 95.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 693.0,
                dif: 73.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 422.0,
                dif: 66.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 180.0,
                dif: 43.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 9.0,
            },
        ],
    );
    map.insert(
        A4c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 3.0,
                dif: 21.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 206.0,
                dif: 36.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 419.0,
                dif: 58.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 551.0,
                dif: 112.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 569.0,
                dif: 202.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 505.0,
                dif: 281.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 554.0,
                dif: 294.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 555.0,
                dif: 307.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 605.0,
                dif: 264.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 467.0,
                dif: 235.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 416.0,
                dif: 155.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 257.0,
                dif: 118.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 114.0,
                dif: 67.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 8.0,
                dif: 20.0,
            },
        ],
    );
    map.insert(
        B3,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 110.2,
                altitude: 7.8,
                dir: 19.0,
                dif: 41.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 101.2,
                altitude: 18.7,
                dir: 201.0,
                dif: 71.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 92.1,
                altitude: 30.0,
                dir: 436.0,
                dif: 56.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 81.5,
                altitude: 41.4,
                dir: 635.0,
                dif: 75.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 69.0,
                altitude: 52.4,
                dir: 798.0,
                dif: 73.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 50.1,
                altitude: 62.2,
                dir: 887.0,
                dif: 91.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 19.5,
                altitude: 68.8,
                dir: 927.0,
                dif: 103.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -19.5,
                altitude: 68.8,
                dir: 985.0,
                dif: 85.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -50.1,
                altitude: 62.2,
                dir: 928.0,
                dif: 80.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -69.0,
                altitude: 52.4,
                dir: 754.0,
                dif: 99.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -81.5,
                altitude: 41.4,
                dir: 664.0,
                dif: 54.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -92.1,
                altitude: 30.0,
                dir: 426.0,
                dif: 72.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -101.2,
                altitude: 18.7,
                dir: 159.0,
                dif: 76.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -110.2,
                altitude: 7.8,
                dir: 27.0,
                dif: 36.0,
            },
        ],
    );
    map.insert(
        D3,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 110.2,
                altitude: 7.8,
                dir: 0.0,
                dif: 27.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 101.2,
                altitude: 18.7,
                dir: 5.0,
                dif: 93.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 92.1,
                altitude: 30.0,
                dir: 186.0,
                dif: 169.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 81.5,
                altitude: 41.4,
                dir: 455.0,
                dif: 148.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 69.0,
                altitude: 52.4,
                dir: 650.0,
                dif: 137.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 50.1,
                altitude: 62.2,
                dir: 809.0,
                dif: 129.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 19.5,
                altitude: 68.8,
                dir: 902.0,
                dif: 110.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -19.5,
                altitude: 68.8,
                dir: 915.0,
                dif: 104.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -50.1,
                altitude: 62.2,
                dir: 889.0,
                dif: 88.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -69.0,
                altitude: 52.4,
                dir: 755.0,
                dif: 86.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -81.5,
                altitude: 41.4,
                dir: 632.0,
                dif: 61.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -92.1,
                altitude: 30.0,
                dir: 443.0,
                dif: 55.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -101.2,
                altitude: 18.7,
                dir: 180.0,
                dif: 79.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -110.2,
                altitude: 7.8,
                dir: 20.0,
                dif: 42.0,
            },
        ],
    );
    map.insert(
        A1c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 12.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 77.0,
                dif: 79.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 290.0,
                dif: 109.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 427.0,
                dif: 155.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 641.0,
                dif: 173.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 680.0,
                dif: 232.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 847.0,
                dif: 195.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 779.0,
                dif: 224.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 760.0,
                dif: 194.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 764.0,
                dif: 121.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 656.0,
                dif: 66.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 396.0,
                dif: 74.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 126.0,
                dif: 66.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 0.0,
                dif: 16.0,
            },
        ],
    );
    map.insert(
        B2c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 2.0,
                dif: 19.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 93.0,
                dif: 75.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 336.0,
                dif: 91.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 467.0,
                dif: 135.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 545.0,
                dif: 202.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 610.0,
                dif: 250.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 625.0,
                dif: 286.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 657.0,
                dif: 272.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 655.0,
                dif: 233.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 703.0,
                dif: 134.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 597.0,
                dif: 80.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 368.0,
                dif: 79.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 180.0,
                dif: 60.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 15.0,
                dif: 20.0,
            },
        ],
    );
    map.insert(
        B3c,
        vec![
            RadData {
                month: 7,
                day: 21,
                hour: 6.0,
                azimuth: 111.3,
                altitude: 3.4,
                dir: 7.0,
                dif: 21.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 7.0,
                azimuth: 105.1,
                altitude: 15.9,
                dir: 104.0,
                dif: 67.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 8.0,
                azimuth: 99.1,
                altitude: 28.8,
                dir: 252.0,
                dif: 116.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 9.0,
                azimuth: 92.6,
                altitude: 41.9,
                dir: 403.0,
                dif: 157.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 10.0,
                azimuth: 85.3,
                altitude: 55.1,
                dir: 531.0,
                dif: 209.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 11.0,
                azimuth: 74.1,
                altitude: 68.1,
                dir: 640.0,
                dif: 233.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 12.0,
                azimuth: 42.5,
                altitude: 79.6,
                dir: 699.0,
                dif: 248.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 13.0,
                azimuth: -42.5,
                altitude: 79.6,
                dir: 749.0,
                dif: 223.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 14.0,
                azimuth: -74.1,
                altitude: 68.1,
                dir: 688.0,
                dif: 210.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 15.0,
                azimuth: -85.3,
                altitude: 55.1,
                dir: 659.0,
                dif: 148.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 16.0,
                azimuth: -92.6,
                altitude: 41.9,
                dir: 561.0,
                dif: 95.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 17.0,
                azimuth: -99.1,
                altitude: 28.8,
                dir: 361.0,
                dif: 71.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 18.0,
                azimuth: -105.1,
                altitude: 15.9,
                dir: 164.0,
                dif: 48.0,
            },
            RadData {
                month: 7,
                day: 21,
                hour: 19.0,
                azimuth: -111.3,
                altitude: 3.4,
                dir: 10.0,
                dif: 19.0,
            },
        ],
    );
    Mutex::new(map)
});
