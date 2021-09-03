//! Módulo que define una estructura de rayo (origen + dirección)

use super::{Point3, Vector3};

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
    /// use bemodel::Ray;
    /// use bemodel::{point, vector, Point3, Vector3};
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
}
