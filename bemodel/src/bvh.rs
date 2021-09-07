use super::{point, Point3, Ray};
use std::ops::Deref;

/// Elementos capaces de definir la AABB que los encierra
pub trait Bounded {
    fn aabb(&self) -> AABB;
}

/// Elementos para los que se puede comprobar la intersección con un rayo
pub trait Intersectable {
    fn intersects(&self, ray: &Ray) -> Option<f32>;
}

/// Axis aligned bounding box definida por puntos extremos
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    /// Constructor
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    /// Punto medio de la AABB
    pub fn center(self) -> Point3 {
        nalgebra::center(&self.max, &self.min)
    }

    /// Calcula AABB que incluye a este y otro elemento
    pub fn join(self, other: Self) -> Self {
        let minx: f32 = self.min.x.min(other.min.x);
        let miny: f32 = self.min.y.min(other.min.y);
        let minz: f32 = self.min.z.min(other.min.z);
        let maxx: f32 = self.max.x.max(other.max.x);
        let maxy: f32 = self.max.y.max(other.max.y);
        let maxz: f32 = self.max.z.max(other.max.z);
        Self {
            min: point![minx, miny, minz],
            max: point![maxx, maxy, maxz],
        }
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            min: point![f32::INFINITY, f32::INFINITY, f32::INFINITY],
            max: point![f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY],
        }
    }
}

impl Bounded for AABB {
    fn aabb(&self) -> AABB {
        *self
    }
}

impl<T: Bounded, U: Deref<Target = [T]>> Bounded for U {
    fn aabb(&self) -> AABB {
        self.iter()
            .fold(AABB::default(), |res, elem| res.join(elem.aabb()))
    }
}

impl Intersectable for AABB {
    /// Detecta si existe intersección de AABB y rayo usando el algoritmo de Cyrus-Beck
    /// https://gdbooks.gitbooks.io/3dcollisions/content/Chapter3/raycast_aabb.html
    /// NaN es siempre distinto, de modo que las comparaciones con NaN son correctas
    /// Las AABB deben tener ancho > 0 en todas las dimensiones
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        let idx = 1.0 / ray.dir.x;
        let idy = 1.0 / ray.dir.y;
        let idz = 1.0 / ray.dir.z;

        let t1 = (self.min.x - ray.origin.x) * idx;
        let t2 = (self.max.x - ray.origin.x) * idx;
        let t3 = (self.min.y - ray.origin.y) * idy;
        let t4 = (self.max.y - ray.origin.y) * idy;
        let t5 = (self.min.z - ray.origin.z) * idz;
        let t6 = (self.max.z - ray.origin.z) * idz;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        // Si tmax < 0 la línea interseca pero el AABB está detrás
        if tmax < 0.0 {
            // t = tmax;
            return None;
        }

        // Si tmin > tmax el rayo no corta AABB
        if tmin > tmax {
            // t = tmax;
            return None;
        }
        // t = tmin;
        Some(tmin)
    }
}

/// BVH - Bounding Volume Hierarchy

/// Bounding Volume Hierarchy (BVH)
///
/// Elemento raíz de una partición de la geometría por objetos, usando AABBs (axis aligned bounding boxes).
/// Diseñamos la estructura para acelerar el cálculo de si un rayo colisiona con alguno de sus elementos terminales.
/// https://gdbooks.gitbooks.io/3dcollisions/content/Chapter2/static_aabb_plane.html ???
/// https://gdbooks.gitbooks.io/3dcollisions/content/Chapter3/raycast_aabb.html
#[derive(Debug)]
pub struct BVH<T> {
    pub root: Option<BVHNode<T>>,
    pub max_num_elements: usize,
}

impl<T: Bounded> BVH<T> {
    pub fn new(root: Option<BVHNode<T>>, max_num_elements: usize) -> Self {
        BVH {
            root,
            max_num_elements,
        }
    }

    /// Construye una BVH a partir de un vector de elementos
    /// TODO: Este método recursivo tiende a colapsar la pila (stack overflow)
    pub fn build(elements: Vec<T>, max_num_elements: usize) -> Self {
        // let aabb = AABB::from_slice(&elements);
        let aabb = elements.aabb();
        let root = if !elements.is_empty() {
            Some(BVH::split_leaf_node(
                BVHNode::Leaf { aabb, elements },
                max_num_elements,
            ))
        } else {
            None
        };
        BVH::new(root, max_num_elements)
    }

    /// Itera sobre los nodos con los que colisiona el rayo
    /// Devuelve tanto nodos intermedios (Node) como finales (Leaf) para los que hay colisión,
    /// bien con su AABB o sus elementos
    pub fn iter_with_ray(&self, ray: &Ray) -> PreorderIter<T> {
        PreorderIter::new(self.root.as_ref(), *ray)
    }

    /// Divide nodo final en nodo intermedio con dos nodos finales separados por eje mayor
    fn split_leaf_node(node: BVHNode<T>, max_num_elements: usize) -> BVHNode<T> {
        if let BVHNode::Leaf { aabb, elements } = node {
            if elements.len() > max_num_elements {
                let (left_elems, right_elems) = BVH::partition_elements(elements, aabb);

                let left = BVHNode::Leaf {
                    aabb: left_elems.aabb(),
                    elements: left_elems,
                };
                let right = BVHNode::Leaf {
                    aabb: right_elems.aabb(),
                    elements: right_elems,
                };

                BVHNode::Node {
                    aabb,
                    left: Some(Box::new(BVH::split_leaf_node(left, max_num_elements))),
                    right: Some(Box::new(BVH::split_leaf_node(right, max_num_elements))),
                }
            } else {
                BVHNode::Leaf { aabb, elements }
            }
        } else {
            unreachable!();
        }
    }

    /// Divide lista de elementos en dos partes
    fn partition_elements(elements: Vec<T>, aabb: AABB) -> (Vec<T>, Vec<T>) {
        let center = aabb.center();
        let (dimx, dimy, dimz) = (
            aabb.max.x - aabb.min.x,
            aabb.max.y - aabb.min.y,
            aabb.max.z - aabb.min.z,
        );
        if dimx >= dimy && dimx >= dimz {
            // X es la dimensión mayor
            elements
                .into_iter()
                .partition(|e| e.aabb().center().x < center.x)
        } else if dimy >= dimz {
            // Y es la dimensión mayor
            elements
                .into_iter()
                .partition(|e| e.aabb().center().y < center.y)
        } else {
            // Z es la dimensión mayor
            elements
                .into_iter()
                .partition(|e| e.aabb().center().z < center.z)
        }
    }
}

impl<T> Intersectable for BVH<T>
where
    T: Bounded + Intersectable,
{
    fn intersects(&self, ray: &Ray) -> Option<f32> {
        let hits_iter = self
            .iter_with_ray(ray)
            .filter(|e| matches!(e, BVHNode::Leaf { .. }));
        for e in hits_iter {
            if e.is_leaf() {
                for occ in e.elements()? {
                    if let intersect_opt @ Some(_) = occ.intersects(ray) {
                        return intersect_opt;
                    }
                }
            }
        }
        None
    }
}

/// Nodos de la BVH. Puede ser un nodo terminal o intermedio
///
/// Los nodos incluyen información sobre la AABB que envuelven sus elementos
/// y pueden tener dos nodos hijos, en el caso de nodos intermedios, o una lista
/// de elementos asociados al nodo
#[derive(Debug)]
pub enum BVHNode<T> {
    Leaf {
        aabb: AABB,
        elements: Vec<T>,
    },
    Node {
        aabb: AABB,
        left: Option<Box<BVHNode<T>>>,
        right: Option<Box<BVHNode<T>>>,
    },
}

impl<T> BVHNode<T> {
    /// Rama izquierda de un nodo intermedio
    pub fn left(self) -> Option<Box<BVHNode<T>>> {
        match self {
            BVHNode::Node { left, .. } => left,
            _ => None,
        }
    }

    /// Rama derecha de un nodo intermedio
    pub fn right(self) -> Option<Box<BVHNode<T>>> {
        match self {
            BVHNode::Node { right, .. } => right,
            _ => None,
        }
    }

    /// ¿Es un nodo terminal?
    pub fn is_leaf(&self) -> bool {
        matches!(self, BVHNode::Leaf { .. })
    }

    /// Referencia a elementos
    pub fn elements(&self) -> Option<&Vec<T>> {
        match self {
            BVHNode::Leaf { elements, .. } => Some(elements),
            _ => None,
        }
    }
}

impl<T> Bounded for BVHNode<T> {
    fn aabb(&self) -> AABB {
        match *self {
            BVHNode::Leaf { aabb, .. } => aabb,
            BVHNode::Node { aabb, .. } => aabb,
        }
    }
}

// Implementación de iterador para recorrer el árbol (preorder traversal)
// Ver:
// - https://sachanganesh.com/programming/graph-tree-traversals-in-rust/
// - https://www.geeksforgeeks.org/tree-traversals-inorder-preorder-and-postorder/
#[derive(Debug, Clone)]
pub struct PreorderIter<'a, T> {
    stack: Vec<&'a BVHNode<T>>,
    ray: Ray,
}

impl<'a, T> PreorderIter<'a, T> {
    pub fn new(root: Option<&'a BVHNode<T>>, ray: Ray) -> Self {
        if let Some(node) = root {
            PreorderIter {
                stack: vec![node],
                ray,
            }
        } else {
            PreorderIter { stack: vec![], ray }
        }
    }
}

// Iteradores en Rust y recorrer un grafo
// https://aloso.github.io/2021/03/09/creating-an-iterator
// https://sachanganesh.com/programming/graph-tree-traversals-in-rust/
// https://www.geeksforgeeks.org/tree-traversals-inorder-preorder-and-postorder/
impl<'a, T> Iterator for PreorderIter<'a, T> {
    type Item = &'a BVHNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let hits_node = node.aabb().intersects(&self.ray).is_some();
            if hits_node {
                if let BVHNode::Node { right, left, .. } = node {
                    if let Some(r_node) = &right {
                        self.stack.push(r_node)
                    }
                    if let Some(l_node) = &left {
                        self.stack.push(l_node)
                    };
                };
                return Some(node);
            };
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{BVHNode, Bounded, Intersectable, Ray, AABB, BVH};
    use nalgebra::{point, vector};

    /// Prueba la unión de dos AABBs
    #[test]
    fn aabb_join() {
        let a = AABB::new(point![1.0, 1.0, 1.0], point![5.0, 5.0, 5.0]);
        let b = AABB::new(point![0.0, 2.0, 1.0], point![5.0, 7.0, 4.0]);
        let res = a.join(b);

        assert!(res.min == point![0.0, 1.0, 1.0] && res.max == point![5.0, 7.0, 5.0]);
    }

    /// Prueba la construcción de una BVH
    #[test]
    fn bvh_build_from_elements() {
        let elements = vec![
            AABB::new(point![1.0, 1.0, 1.0], point![5.0, 5.0, 5.0]),
            AABB::new(point![1.0, 5.0, 5.0], point![5.0, 9.0, 9.0]),
            AABB::new(point![5.0, 5.0, 1.0], point![9.0, 9.0, 5.0]),
            AABB::new(point![5.0, 1.0, 5.0], point![9.0, 5.0, 9.0]),
        ];

        let bvh = BVH::build(elements, 2);
        let root = bvh.root.unwrap();
        let aabb = root.aabb();
        assert_eq!(aabb.min, point![1.0, 1.0, 1.0]);
        assert_eq!(aabb.max, point![9.0, 9.0, 9.0]);
        let left = root.left().unwrap();
        let left_aabb = left.aabb();
        assert_eq!(left_aabb.min, point![1.0, 1.0, 1.0]);
        assert_eq!(left_aabb.max, point![5.0, 9.0, 9.0]);
    }

    /// Prueba la construcción de una BVH
    #[test]
    fn bvh_build_tree() {
        let a = AABB::new(point![1.0, 1.0, 1.0], point![5.0, 5.0, 5.0]);
        let b = AABB::new(point![1.0, 5.0, 5.0], point![5.0, 9.0, 9.0]);
        let c = AABB::new(point![5.0, 5.0, 1.0], point![9.0, 9.0, 5.0]);
        let d = AABB::new(point![5.0, 1.0, 5.0], point![9.0, 5.0, 9.0]);

        let left = BVHNode::Leaf {
            aabb: a.join(b),
            elements: vec![a, b],
        };
        let right = BVHNode::Leaf {
            aabb: c.join(d),
            elements: vec![c, d],
        };
        let root = BVHNode::Node {
            aabb: left.aabb().join(right.aabb()),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
        let bvh = BVH::new(Some(root), 2);
        let aabb = bvh.root.as_ref().unwrap().aabb();
        assert_eq!(aabb.min, point![1.0, 1.0, 1.0]);
        assert_eq!(aabb.max, point![9.0, 9.0, 9.0]);

        // Rayo que colisiona solo con elemento a
        let ray = Ray::new(point![2.5, 0.0, 2.5], vector![0.0, 1.0, 0.0]);

        let hits_iter = bvh
            .iter_with_ray(&ray)
            .filter(|e| matches!(e, BVHNode::Leaf { .. }));
        assert_eq!(hits_iter.clone().count(), 1);
        let hit_node = hits_iter.clone().collect::<Vec<_>>()[0];
        let elem = match hit_node {
            BVHNode::Leaf { aabb: _, elements } => elements[0],
            _ => panic!(),
        };
        assert_eq!(elem, a);
        assert_eq!(bvh.intersects(&ray), Some(1.0));
    }
}
