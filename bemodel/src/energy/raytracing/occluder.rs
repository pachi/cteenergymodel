// Copyright (c) 2018-2022 Rafael Villar Burke <pachi@ietcc.csic.es>
// Distributed under the MIT License
// (See acoompanying LICENSE file or a copy at http://opensource.org/licenses/MIT)

use nalgebra::IsometryMatrix3;

use super::{Bounded, Intersectable, Ray, AABB};

use crate::{Polygon, Vector3, Uuid};

/// Elemento oclusor, con información geométrica e identificación
///
/// - el id permite excluir el muro de un hueco
/// - el origin_id permite excluir las geometrías de retranqueo que no son del hueco analizado
/// - normal y trans_matrix permiten cachear resultados para cálculo de intersecciones con el polígono 2D transformando un rayo
pub struct Occluder {
    /// Id del elemento
    pub id: Uuid,
    /// Id del elemento que genera este oclusor (si proviene de otro elemento, como sombras de retranqueos de huecos)
    pub linked_to_id: Option<Uuid>,
    /// normal del polígono
    pub normal: Vector3,
    /// Matriz de transformación de coordenadas globales a locales de polígono
    pub trans_matrix: Option<IsometryMatrix3<f32>>,
    /// Polígono 2D
    pub polygon: Polygon,
    /// AABB (min, max)
    pub aabb: AABB,
}

impl Intersectable for &Occluder {
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        self.aabb.intersects(ray)?;
        ray.intersects_with_data(&self.polygon, self.trans_matrix.as_ref(), &self.normal)
    }
}

impl Bounded for &Occluder {
    fn aabb(&self) -> AABB {
        self.aabb
    }
}

impl std::fmt::Debug for Occluder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Occluder (id: {}, linked_to_id: {:?}",
            &self.id, &self.linked_to_id
        )
    }
}
