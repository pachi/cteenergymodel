// Copyright (c) 2018-2020 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Parser del Building Description Language (BDL) de DOE
//!
//! Elementos geométricos:
//! - Polígono (POLYGON)
//! - Vector

use std::{convert::TryFrom, f32::consts::FRAC_PI_2};

use anyhow::{bail, Error};

use na::{Point2, Rotation2, Vector2};

use crate::bdl::BdlBlock;
use crate::utils::normalize;

/// Polígono
/// Solo pueden ser polígonos con vértices 2D
#[derive(Debug, Clone, Default)]
pub struct Polygon {
    /// Nombre del polígono
    pub name: String,
    /// Lista de vectores que definen el polígono
    pub vertices: Vec<Vertex2D>,
}

impl Polygon {
    /// Área del polígono definido por vértices (m2)
    pub fn area(&self) -> f32 {
        // https://www.mathopenref.com/coordpolygonarea2.html
        // https://www.mathopenref.com/coordpolygonarea.html
        // 0.5 * ( \SUM( x_i * y_i+1 - y_i * x_i+1)_(i = de 1 a n) + (x_n * y_1 - y_n * x_1) )
        let vertices = &self.vertices;
        let nverts = vertices.len();

        let mut area = 0.0;
        for i in 0..nverts {
            let nexti = (i + 1) % nverts; // el último vértice vuelve a cero
            let vi = &vertices[i].vector;
            let vj = &vertices[nexti].vector;
            area += vi.x * vj.y - vi.y * vj.x;
        }

        f32::abs(0.5 * area)
    }

    /// Perímetro de un polígono (m)
    pub fn perimeter(&self) -> f32 {
        let nlen = self.vertices.len();
        match nlen {
            0 => 0.0,
            1 => 0.0,
            _ => {
                let first = self.vertices.get(0).unwrap().clone();
                let mut vecs = self.vertices.clone();
                vecs.push(first);
                vecs.as_slice()
                    .windows(2)
                    .map(|win| {
                        let vn = &win[0].vector;
                        let vm = &win[1].vector;
                        (vn.x - vm.x).hypot(vn.y - vm.y)
                    })
                    .sum()
            }
        }
    }

    /// Longitud del lado que empieza en el vértice con el nombre indicado
    pub fn edge_length(&self, vertexname: &str) -> f32 {
        self.edge_vertices(vertexname)
            .map(|[p_n, p_m]| (p_n.vector - p_m.vector).magnitude())
            .unwrap_or(0.0)
    }

    /// Vértices del lado que empieza en el vértice con el nombre indicdo (Vnn)
    /// El lado que empieza en el último vértice continua en el vértice inicial
    pub fn edge_vertices(&self, vertexname: &str) -> Option<[&Vertex2D; 2]> {
        let num_vertex: usize = vertexname.strip_prefix("V").map(str::parse::<usize>).unwrap().ok()? - 1;
        Some([&self.vertices[num_vertex], &self.vertices[(num_vertex + 1) % self.vertices.len()]])
    }

    /// Ángulo con el sur de la normal del lado definido por el vértice
    /// northangle es la desviación global respecto al norte
    /// Los ángulos se dan en grados sexagesimales
    pub fn edge_orient(&self, vertexname: &str, northangle: f32) -> f32 {
        self.edge_vertices(vertexname)
            .map(|[p_n, p_m]| {
                // normal al vector director del lado (hay dos, (dy, -dx) y (-dy, dx) y cogemos (dy, -dx), un giro de -90º)
                let n = Rotation2::new(-FRAC_PI_2) * (p_m.vector - p_n.vector);
                // vector del sur (0, -1)
                let s = -Vector2::y();
                // ángulo entre la normal y el sur
                let angle = n.angle(&s);
                // Para las normales en el semiplano nx <= 0 cogemos el ángulo largo
                normalize(
                    f32::signum(n.x) * angle.to_degrees() - northangle,
                    0.0,
                    360.0,
                )
            })
            .unwrap_or(0.0)
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
        let BdlBlock {
            name, mut attrs, ..
        } = value;
        let mut vertices = Vec::new();
        for i in 1.. {
            let name = format!("V{}", i);
            if let Ok(vdata) = attrs.remove_str(&name) {
                vertices.push(Vertex2D {
                    name,
                    vector: point2_from_str(&vdata)?,
                });
            } else {
                break;
            }
        }
        Ok(Self { name, vertices })
    }
}

/// Vertex2D - Vértice, conjunto de nombre y vector 2d (x, y)
#[derive(Debug, Clone)]
pub struct Vertex2D {
    /// Nombre del vértice
    pub name: String,
    /// Coordenadas del vértice
    pub vector: Point2<f32>,
}

/// Vector 2D (x,y)
#[derive(Debug, Copy, Clone, Default)]
pub struct Vector2D {
    /// Coordenada x
    pub x: f32,
    /// Coordenada y
    pub y: f32,
}

fn point2_from_str(s: &str) -> Result<Point2<f32>, Error> {
    if let [x, y] = s
        .split(',')
        .map(|v| v.trim_matches(&[' ', '(', ')'] as &[_]))
        .collect::<Vec<_>>()
        .as_slice()
    {
        Ok(Point2::new(x.parse::<f32>()?, y.parse::<f32>()?))
    } else {
        bail!("Fallo al generar vector 2D con los datos '{}'", s)
    }
}

/// Vertex3D - Vértice, conjunto de nombre y vector 3d (x, y, z)
/// Se usan para definir sombras
#[derive(Debug, Clone, Default)]
pub struct Vertex3D {
    /// Nombre del vértice
    pub name: String,
    /// Coordenadas del vértice
    pub vector: Vector3D,
}

/// Vector 3D (x,y,z)
#[derive(Debug, Copy, Clone, Default)]
pub struct Vector3D {
    /// Coordenada x
    pub x: f32,
    /// Coordenada y
    pub y: f32,
    /// Coordenada z
    pub z: f32,
}

impl std::str::FromStr for Vector3D {
    type Err = Error;

    /// Convierte de cadena a vector de coordenadas
    ///
    /// Ejemplo:
    /// ```text
    ///     ( 14.97, 11.39, 2.0 )
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [x, y, z] = s
            .split(',')
            .map(|v| v.trim_matches(&[' ', '(', ')'] as &[_]))
            .collect::<Vec<_>>()
            .as_slice()
        {
            Ok(Self {
                x: x.parse::<f32>()?,
                y: y.parse::<f32>()?,
                z: z.parse::<f32>()?,
            })
        } else {
            bail!("Fallo al generar vector 3D con los datos '{}'", s)
        }
    }
}
