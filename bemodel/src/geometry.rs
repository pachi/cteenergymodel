// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Rutinas de cálculo geométrico

use std::convert::From;

// use log::{debug, info, warn};
use na::{IsometryMatrix2, IsometryMatrix3, Rotation2, Rotation3, Translation2, Translation3};

use super::{
    bvh::{Bounded, Intersectable, AABB},
    point, vector, Geometry, Point2, Ray, Vector2, Vector3,
};

const EPSILON: f32 = 1e-5;

// -------------------------- Funciones auxiliares ---------------------------

impl Geometry {
    /// Vector unitario normal a la geometría, en coordenadas globales
    pub fn normal(&self) -> Vector3 {
        let n_p = poly_normal(&self.polygon);
        let zrot = Rotation3::new(Vector3::z() * self.azimuth.to_radians());
        let xrot = Rotation3::new(Vector3::x() * self.tilt.to_radians());
        zrot * xrot * n_p
    }

    /// Matriz de transformación de coordenadas locales a coordenadas globales
    /// Traslada de coordenadas de opaco / sombra a coordenadas globales (giros y desplazamientos)
    pub fn to_global_coords_matrix(&self) -> Option<IsometryMatrix3<f32>> {
        let trans = Translation3::from(self.position?);
        let zrot = Rotation3::new(Vector3::z() * self.azimuth.to_radians());
        let xrot = Rotation3::new(Vector3::x() * self.tilt.to_radians());

        Some(trans * zrot * xrot)
    }

    /// Matriz de transformación de coordenadas locales de la geometría a coordenadas de polígono interno 2D
    /// Se gira el eje X en la dirección del polígono de muro p1 - p0 y se traslada a p0 el origen
    pub fn to_polygon_coords_matrix(&self) -> Option<IsometryMatrix2<f32>> {
        if self.polygon.len() <= 2 {
            return None;
        };
        let v0 = self.polygon[0];
        let v1 = self.polygon[1];
        let dir_x = v1 - v0;
        let rot = Rotation2::rotation_between(&Vector2::x(), &dir_x);
        let trans = Translation2::from(v0);

        Some(trans * rot)
    }
}

impl Intersectable for Geometry {
    /// Calcula la intersección entre rayo y geometría, e indica el factor t en la dirección del rayo
    ///
    /// ray_origin: punto de origen del rayo en coordenadas globales (Vector3)
    /// ray_dir: dirección del rayo en coordenadas globales (Vector3)
    ///
    /// Si es un punto interior devuelve t tal que la intersección se produce en ray_origin + t * ray_dir
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        // Matrices de transformación de geometría
        let trans_inv = self.to_global_coords_matrix().map(|m| m.inverse());
        // Normal to the planar polygon
        let n_p = &poly_normal(&self.polygon);
        intersects_with_data(ray, &self.polygon, trans_inv.as_ref(), n_p)
    }
}

impl Bounded for Geometry {
    fn aabb(&self) -> AABB {
        if let Some(trans) = self.to_global_coords_matrix() {
            let mut min_x = f32::INFINITY;
            let mut min_y = f32::INFINITY;
            let mut min_z = f32::INFINITY;
            let mut max_x = f32::NEG_INFINITY;
            let mut max_y = f32::NEG_INFINITY;
            let mut max_z = f32::NEG_INFINITY;

            for p in self.polygon.iter().map(|p| trans * point![p.x, p.y, 0.0]) {
                min_x = min_x.min(p.x);
                max_x = max_x.max(p.x);
                min_y = min_y.min(p.y);
                max_y = max_y.max(p.y);
                min_z = min_z.min(p.z);
                max_z = max_z.max(p.z);
            }

            AABB {
                min: point![min_x, min_y, min_z],
                max: point![max_x, max_y, max_z],
            }
        } else {
            Default::default()
        }
    }
}

/// Calcula la intersección entre rayo y polígono aportando matrices, e indica el factor t en la dirección del rayo
///
/// ray_origin: punto de origen del rayo en coordenadas globales (Vector3)
/// ray_dir: dirección del rayo en coordenadas globales (Vector3)
/// polygon: polígono 2D (en plano XY con transformación de coordenadas y normal dadas)
/// transInv: matriz de transformación desde coordenadas globales a locales del polígono
/// n_p: normal a la geometría
///
/// - Transforma el rayo al espacio del polígono
/// - Calcula el punto de intersección del rayo transformado con el plano XY
/// - Comprueba si el punto está en el interior del polígono
/// - Si es un punto interior devuelve t tal que la intersección se produce en ray_origin + t * ray_dir
pub fn intersects_with_data(
    ray: &Ray,
    polygon: &[Point2],
    global_to_poly_matrix: Option<&IsometryMatrix3<f32>>,
    n_p: &Vector3,
) -> Option<f32> {
    // Transform ray to the polygon coordinate space
    let trans_inv = global_to_poly_matrix?;
    let inv_ray_o = trans_inv * ray.origin;
    let inv_ray_d = trans_inv * ray.dir;

    // Check if ray is parallel to the polygon
    let denominator = n_p.dot(&inv_ray_d);
    if denominator.abs() < EPSILON {
        return None;
    }

    // Find intersection of ray with XY plane
    let poly_o_to_ray = point![polygon[0].x, polygon[0].y, 0.0] - inv_ray_o;
    let t = n_p.dot(&poly_o_to_ray) / denominator;

    // We only consider positive t (it's a ray!)
    if t < 0.0 {
        return None;
    }

    // Verify that the point falls inside the polygon
    let intersection_point = inv_ray_o + t * inv_ray_d;
    let point2d = intersection_point.xy();
    let point_is_inside = point_in_poly(point2d, polygon);

    if point_is_inside {
        // Intersection point is at t units in the ray direction from its origin
        // let intp = trans * intersection_point;
        // let intp = ray_origin + t * ray_dir;
        Some(t)
    } else {
        None
    }
}

/// Normal al polígono plano, en coordenadas locales
pub fn poly_normal(poly: &[Point2]) -> Vector3 {
    let v0 = poly[1] - poly[0];
    let v1 = poly[2] - poly[0];

    vector![v0.x, v0.y, 0.0]
        .cross(&vector![v1.x, v1.y, 0.0])
        .normalize()
}

/// Test 2D de punto en polígono usando el método de Heines
/// http://erich.realtimerendering.com/ptinpoly/
/// Cuenta el número de cruces haciendo raycasting desde el punto para ver si está dentro (cruces impares) o fuera (cruces pares)
/// Evita el cálculo de las intersecciones y la división por cero viendo los cambios de signo
/// https://stackoverflow.com/questions/217578/how-can-i-determine-whether-a-2d-point-is-within-a-polygon/2922778#2922778
/// ver https://docs.rs/geo/0.2.6/src/geo/.cargo/registry/src/github.com-1ecc6299db9ec823/geo-0.2.6/src/algorithm/contains.rs.html#9-33
/// https://docs.rs/geo/0.18.0/geo/algorithm/contains/trait.Contains.html
/// Ver algunos casos límite en https://stackoverflow.com/a/63436180
/// Evita el cálculo del punto de intersección y una división localizando la condición de cruce
pub fn point_in_poly(pt: Point2, poly: &[Point2]) -> bool {
    let x = pt.x;
    let y = pt.y;
    let mut inside = false;

    // Empezamos con el segmento que une el punto final con el inicial
    let mut v_j = poly[poly.len() - 1];
    let mut y_0 = v_j.y >= y;
    for &v_i in poly {
        let y_1 = v_i.y >= y;
        // primero se mira si el lado cruza la linea horizontal en pt.y
        // y, si es así, comprobamos si se cruza también en x para detectar que se produe el cruce
        if y_0 != y_1 && (((v_i.y - y) * (v_j.x - v_i.x) >= (v_i.x - x) * (v_j.y - v_i.y)) == y_1) {
            inside = !inside;
        }
        // Avanzamos al siguiente segmento
        y_0 = y_1;
        v_j = v_i;
    }

    inside
}

/// Elemento oclusor, con información geométrica e identificación
///
/// - el id permite excluir el muro de un hueco
/// - el origin_id permite excluir las geometrías de retranqueo que no son del hueco analizado
/// - normal y trans_matrix permiten cachear resultados para cálculo de intersecciones con el polígono 2D transformando un rayo
#[derive(Debug)]
pub struct Occluder {
    /// Id del elemento
    pub id: String,
    /// Id del elemento que genera este oclusor (si proviene de otro elemento, como sombras de retranqueos de huecos)
    pub linked_to_id: Option<String>,
    /// normal del polígono
    pub normal: Vector3,
    /// Matriz de transformación de coordenadas globales a locales de polígono
    pub trans_matrix: Option<IsometryMatrix3<f32>>,
    /// Polígono 2D
    pub polygon: Vec<Point2>,
    /// AABB (min, max)
    pub aabb: AABB,
}

impl Intersectable for &Occluder {
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        self.aabb.intersects(ray)?;
        intersects_with_data(ray, &self.polygon, self.trans_matrix.as_ref(), &self.normal)
    }
}

impl Bounded for &Occluder {
    fn aabb(&self) -> AABB {
        self.aabb
    }
}
