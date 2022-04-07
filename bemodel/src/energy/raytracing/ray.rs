// Copyright (c) 2018-2021 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

//! Módulo que define una estructura de rayo (origen + dirección)

use crate::{Point2, Point3, Vector3};
use nalgebra::{point, IsometryMatrix3};

const EPSILON: f32 = 1e-5;

/// Estructura que define un rayo, con su origen y dirección
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    /// Origen del rayo.
    pub origin: Point3,
    /// Dirección del rayo.
    pub dir: Vector3,
}

impl Ray {
    /// Crea un nuevo rayo [`Ray`] a partir de un origen y una dirección.
    /// La dirección es normalizada.
    ///
    /// # Ejemplos
    /// ```
    /// use nalgebra::{point, vector};
    /// use bemodel::energy::Ray;
    /// use bemodel::{Point3, Vector3};
    ///
    /// let origin = point![0.0,0.0,0.0];
    /// let dir = vector![1.0,0.0,0.0];
    /// let ray = Ray::new(origin, dir);
    ///
    /// assert_eq!(ray.origin, origin);
    /// assert_eq!(ray.dir, dir);
    /// ```
    ///
    /// [`Ray`]: struct.Ray.html
    ///
    pub fn new(origin: Point3, dir: Vector3) -> Ray {
        let dir = dir.normalize();
        Ray { origin, dir }
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
        &self,
        polygon: &[Point2],
        global_to_poly_matrix: Option<&IsometryMatrix3<f32>>,
        n_p: &Vector3,
    ) -> Option<f32> {
        // Transform ray to the polygon coordinate space
        let trans_inv = global_to_poly_matrix?;
        let inv_ray_o = trans_inv * self.origin;
        let inv_ray_d = trans_inv * self.dir;

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
}

/// Test 2D de punto en polígono usando el método de Heines
/// http://erich.realtimerendering.com/ptinpoly/
///
/// Cuenta el número de cruces haciendo raycasting desde el punto para ver si está dentro (cruces impares) o fuera (cruces pares)
/// Evita el cálculo de las intersecciones y la división por cero viendo los cambios de signo
/// https://stackoverflow.com/questions/217578/how-can-i-determine-whether-a-2d-point-is-within-a-polygon/2922778#2922778
/// ver https://docs.rs/geo/0.2.6/src/geo/.cargo/registry/src/github.com-1ecc6299db9ec823/geo-0.2.6/src/algorithm/contains.rs.html#9-33
/// https://docs.rs/geo/0.18.0/geo/algorithm/contains/trait.Contains.html
/// Ver algunos casos límite en https://stackoverflow.com/a/63436180
/// Evita el cálculo del punto de intersección y una división localizando la condición de cruce
///
/// ```rust ignore
///     use nalgebra::point;
///     use bemodel::energy::ray::point_in_poly;
///     let poly = vec![
///         point![0.0, 0.0],
///         point![9.11, 0.0],
///         point![9.11, 3.0],
///         point![0.0, 3.0],
///     ];
///     assert!(!point_in_poly(point![-9.81, -7.3], &poly));
///     assert!(point_in_poly(point![2.0, 2.0], &poly));
/// ```
fn point_in_poly(pt: Point2, poly: &[Point2]) -> bool {
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
