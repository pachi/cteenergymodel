// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Elementos geométricos:
//! - Polígono (POLYGON)
//! - Vector

use std::{convert::TryFrom, f32::consts::FRAC_PI_2};

use anyhow::{bail, Error};

use nalgebra::{point, Matrix2, Point2, Point3, Rotation2, Vector2};

use crate::bdl::BdlBlock;
use crate::utils::normalize;

/// Polígono - conjunto de vértices 2D
#[derive(Debug, Clone, Default)]
pub struct Polygon(pub Vec<Point2<f32>>);

impl Polygon {
    /// Área del polígono definido por vértices (m2)
    pub fn area(&self) -> f32 {
        // https://www.mathopenref.com/coordpolygonarea2.html
        // https://www.mathopenref.com/coordpolygonarea.html
        // 0.5 * ( \SUM( x_i * y_i+1 - y_i * x_i+1)_(i = de 1 a n) + (x_n * y_1 - y_n * x_1) )
        let area = match self.0.len() {
            0 => 0.0,
            1 => 0.0,
            n => self
                .0
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    Matrix2::from_columns(&[v.coords, self.0[(i + 1) % n].coords]).determinant()
                })
                .sum(),
        };
        f32::abs(0.5 * area)
    }

    /// Perímetro de un polígono (m)
    pub fn perimeter(&self) -> f32 {
        match self.0.len() {
            0 => 0.0,
            1 => 0.0,
            n => self
                .0
                .iter()
                .enumerate()
                .map(|(i, v)| (v - self.0[(i + 1) % n]).magnitude())
                .sum(),
        }
    }

    /// Longitud del lado que empieza en el vértice con el nombre indicado
    pub fn edge_length(&self, vertexname: &str) -> f32 {
        self.edge_vertices(vertexname)
            .map(|[p_n, p_m]| (p_n - p_m).magnitude())
            .unwrap_or(0.0)
    }

    /// Vértices del lado que empieza en el vértice con el nombre indicdo (Vnn)
    /// El lado que empieza en el último vértice continua en el vértice inicial
    pub fn edge_vertices(&self, vertexname: &str) -> Option<[&Point2<f32>; 2]> {
        let num_vertex: usize = vertexname
            .strip_prefix('V')
            .map(str::parse::<usize>)
            .unwrap_or_else(|| panic!("Vértice {} desconocido de polígono", vertexname))
            .ok()?
            - 1;
        Some([
            &self.0[num_vertex],
            &self.0[(num_vertex + 1) % self.0.len()],
        ])
    }

    /// Ángulo con el norte (Y+) de la normal del lado definido por el vértice
    /// Los ángulos se dan en grados sexagesimales, sentido horario desde Y+ (E+, W-)
    pub fn edge_normal_to_y(&self, vertexname: &str) -> f32 {
        self.edge_vertices(vertexname)
            .map(|[p_n, p_m]| {
                // normal al vector director del lado (hay dos, (dy, -dx) y (-dy, dx) y cogemos (dy, -dx), un giro de -90º)
                let n = Rotation2::new(-FRAC_PI_2) * (p_m - p_n);
                // vector del sur (0, -1)
                let s = Vector2::y();
                // ángulo entre la normal y el sur
                let angle = n.angle(&s);
                // Para las normales en el semiplano nx <= 0 cogemos el ángulo largo
                normalize(f32::signum(n.x) * angle.to_degrees(), 0.0, 360.0)
            })
            .unwrap_or(0.0)
    }

    /// Devuelve copia como Vec<Point2<f32>>
    pub fn as_vec(&self) -> Vec<Point2<f32>> {
        self.0.clone()
    }

    /// Devuelve un polígono que es un espejo respecto al eje X
    pub fn mirror_y(&self) -> Self {
        let mirror: Vec<_> = self.0.iter().map(|p| point![p.x, -p.y]).collect();
        let mut counterclockwise = vec![mirror[0]];
        counterclockwise.extend(mirror[1..].iter().rev());
        Self(counterclockwise)
    }

    /// Devuelve un polígono rotado angle radianes respecto al origen
    pub fn rotate(&self, angle: f32) -> Self {
        let rot = Rotation2::new(angle);
        Self(self.0.iter().map(|p| rot * p).collect())
    }
}

impl TryFrom<BdlBlock> for Polygon {
    type Error = Error;

    /// Convierte de bloque BDL a polígono
    ///
    /// Define la geometría, mediante el atributo POLYGON de:
    /// - EXTERIOR-WALL, INTERIOR-WALL, UNDERGROUND-WALL, FLOOR y SPACE
    ///
    /// Ejemplo:
    /// ```text
    ///     "P01_E01_Pol2" = POLYGON
    ///     V1   =( 14.97, 11.39 )
    ///     V2   =( 10.84, 11.39 )
    ///     V3   =( 10.86, 0 )
    ///     V4   =( 18.22, 0 )
    ///     V5   =( 18.22, 9.04 )
    ///     V6   =( 14.97, 9.04 )
    ///     ..
    /// ```
    fn try_from(value: BdlBlock) -> Result<Self, Self::Error> {
        let BdlBlock { mut attrs, .. } = value;
        let mut vertices = Vec::new();
        for i in 1.. {
            let name = format!("V{}", i);
            if let Ok(vdata) = attrs.remove_str(&name) {
                vertices.push(point2_from_str(&vdata)?);
            } else {
                break;
            }
        }
        Ok(Self(vertices))
    }
}

/// Convierte de cadena a Point2 de coordenadas
///
/// Ejemplo:
/// ```text
///     ( 14.97, 11.39 )
/// ```
pub fn point2_from_str(s: &str) -> Result<Point2<f32>, Error> {
    if let [x, y] = s
        .split(',')
        .map(|v| v.trim_matches(&[' ', '(', ')'] as &[_]))
        .collect::<Vec<_>>()
        .as_slice()
    {
        Ok(point![x.parse::<f32>()?, y.parse::<f32>()?])
    } else {
        bail!("Fallo al generar punto 2D con los datos '{}'", s)
    }
}

/// Convierte de cadena a Point3 de coordenadas
///
/// Ejemplo:
/// ```text
///     ( 14.97, 11.39, 2.0 )
/// ```
pub fn point3_from_str(s: &str) -> Result<Point3<f32>, Error> {
    if let [x, y, z] = s
        .split(',')
        .map(|v| v.trim_matches(&[' ', '(', ')'] as &[_]))
        .collect::<Vec<_>>()
        .as_slice()
    {
        Ok(point![
            x.parse::<f32>()?,
            y.parse::<f32>()?,
            z.parse::<f32>()?
        ])
    } else {
        bail!("Fallo al generar punto 3D con los datos '{}'", s)
    }
}
