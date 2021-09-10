use std::{collections::HashMap, fmt::Debug, ops::Deref};

use super::{point, Point3, Ray};

/// Elementos capaces de definir la AABB que los encierra
pub trait Bounded {
    fn aabb(&self) -> AABB;
}

/// Elementos para los que se puede comprobar la intersección con un rayo
pub trait Intersectable {
    fn intersects(&self, ray: &Ray) -> Option<f32>;
}

/// Axis aligned bounding box definida por puntos extremos
#[derive(Copy, Clone, PartialEq)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl Debug for AABB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min = self.min;
        let max = self.max;
        write!(
            f,
            "AABB (min: point![{}, {}, {}], max: point![{}, {}, {}])",
            min.x, min.y, min.z, max.x, max.y, max.z
        )
    }
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
}

#[derive(Debug)]
enum Side {
    L,
    R,
}

#[derive(Debug)]
enum NodeType {
    Leaf,
    Node,
}

type NodeId = usize;
struct TreeElement<T>(NodeId, NodeType, Side, Option<NodeId>, Option<Vec<T>>);

impl<T: Debug> Debug for TreeElement<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TreeElement ({:?}, {:?}, {:?}, {:?}, vec_with_elems: {:?}",
            &self.0,
            &self.1,
            &self.2,
            &self.3,
            &self.4.is_some()
        )
    }
}

impl<T: Bounded> BVH<T> {
    pub fn new(root: Option<BVHNode<T>>) -> Self {
        BVH { root }
    }

    /// Construye una BVH de forma recursiva a partir de un vector de elementos
    /// El método puede colapsar la pila (stack overflow) con muchos elementos
    pub fn build_recursive(elements: Vec<T>, max_num_elements: usize) -> Self {
        let aabb = elements.aabb();
        let root = if !elements.is_empty() {
            Some(BVH::split_leaf_node(
                BVHNode::Leaf { aabb, elements },
                max_num_elements,
            ))
        } else {
            None
        };
        BVH::new(root)
    }

    /// Construye una BVH de forma iterativa a partir de un vector de elementos
    pub fn build(elements: Vec<T>, max_num_elements: usize) -> Self {
        let node_list = BVH::generate_node_list(elements, max_num_elements);
        BVH::build_from_node_list(node_list)
    }

    /// Genera iterativamente lista de nodos del árbol a partir de la lista de elementos
    fn generate_node_list(elements: Vec<T>, max_num_elements: usize) -> Vec<TreeElement<T>> {
        use NodeType::*;
        use Side::*;

        // Nodos pendientes
        let mut pending: Vec<TreeElement<T>> = Vec::new();
        // Nodos procesados (2*n-1 nodos con n terminales)
        let expected_num_nodes = 2 * (elements.len() / max_num_elements) - 1;
        let mut node_list: Vec<TreeElement<T>> = Vec::with_capacity(expected_num_nodes);

        let mut id: NodeId = 0;
        let ll = elements.len();
        if ll > max_num_elements {
            let (left, right) = BVH::partition_elements(elements);
            // Guardamos nodo inicial (da igual el lado)
            node_list.push(TreeElement(0, Node, L, None, None));
            // Nodos pendientes
            pending.push(TreeElement(id + 2, Node, R, Some(id), Some(right)));
            pending.push(TreeElement(id + 1, Node, L, Some(id), Some(left)));
            id += 2;
            // Procesar stack de pendientes de dividir
            while !pending.is_empty() {
                let TreeElement(c_id, _c_type, c_side, c_maybe_parent_id, c_maybe_elems) =
                    pending.pop().unwrap();
                let c_elems = c_maybe_elems.unwrap();
                let cll = c_elems.len();
                if cll > max_num_elements {
                    // Completamos un nodo intermedio y dejamos pendientes sus ramas
                    let (left, right) = BVH::partition_elements(c_elems);
                    node_list.push(TreeElement(c_id, Node, c_side, c_maybe_parent_id, None));
                    pending.push(TreeElement(id + 2, Node, R, Some(c_id), Some(right)));
                    pending.push(TreeElement(id + 1, Node, L, Some(c_id), Some(left)));
                    id += 2;
                } else {
                    // Completamos un nodo terminal
                    node_list.push(TreeElement(
                        c_id,
                        Leaf,
                        c_side,
                        c_maybe_parent_id,
                        Some(c_elems),
                    ))
                }
            }
        } else {
            node_list.push(TreeElement(0, Leaf, L, None, Some(elements)))
        }
        node_list
    }

    /// Reconstruye árbol a partir de lista de nodos intermedios y terminales
    fn build_from_node_list(mut node_list: Vec<TreeElement<T>>) -> Self {
        use NodeType::*;
        use Side::*;

        // Diccionario de elementos pendientes de acabar (sin dos nodos hijos), indexados por padre
        let mut pending: HashMap<NodeId, BVHNode<T>> = HashMap::new();
        // Diccionario de nodos completos, listos para insertar en sus padres e indexados por padre
        // Al final del proceso contiene el nodo raíz
        let mut completed: HashMap<NodeId, BVHNode<T>> = HashMap::new();

        // Vamos añadiendo los nodos que tenemos a sus elementos padre y
        // a medida que los completamos los añadimos a sus respectivos padres
        while node_list.len() > 1 {
            // Con nodo intermedio elems es None, y tiene datos en nodos terminales
            let TreeElement(id, _type, side, maybe_parent_id, elems) = node_list.pop().unwrap();
            let parent_id = maybe_parent_id.unwrap();
            let parent_node = pending.entry(parent_id).or_insert(BVHNode::Node {
                aabb: AABB::default(),
                left: None,
                right: None,
            });
            match (side, _type) {
                (L, Leaf) => {
                    let elements = elems.unwrap();
                    let aabb = elements.aabb();
                    parent_node.set_left(BVHNode::Leaf { aabb, elements })
                }
                (L, Node) => {
                    let left = completed.remove(&id).unwrap();
                    parent_node.set_left(left)
                }
                (R, Leaf) => {
                    let elements = elems.unwrap();
                    let aabb = elements.aabb();
                    parent_node.set_right(BVHNode::Leaf { aabb, elements })
                }
                (R, Node) => {
                    let right = completed.remove(&id).unwrap();
                    parent_node.set_right(right)
                }
            };
            // Está completo y disponible para insertar en otro nodo
            // Lo eliminamos de pending y añadimos a completed
            if parent_node.is_complete_node() {
                let mut parent_node = pending.remove(&parent_id).unwrap();
                parent_node.set_aabb_from_children();
                completed.insert(parent_id, parent_node);
            }
        }
        let root = completed.remove(&0_usize).unwrap();
        Self::new(Some(root))
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
                let (left_elems, right_elems) = BVH::partition_elements_with_aabb(elements, aabb);

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
    fn partition_elements_with_aabb(elements: Vec<T>, aabb: AABB) -> (Vec<T>, Vec<T>) {
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

    /// Divide lista de elementos en dos partes
    fn partition_elements(elements: Vec<T>) -> (Vec<T>, Vec<T>) {
        let aabb = &elements.aabb();
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
            for occ in e.elements()? {
                if let intersect_opt @ Some(_) = occ.intersects(ray) {
                    return intersect_opt;
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

impl<T: Bounded> BVHNode<T> {
    /// Define la AABB de un nodo
    pub fn set_aabb_from_children(&mut self) {
        match self {
            BVHNode::Node {
                aabb, left, right, ..
            } => {
                let left_aabb = left.as_ref().unwrap().aabb();
                let right_aabb = right.as_ref().unwrap().aabb();
                *aabb = left_aabb.join(right_aabb);
            }
            BVHNode::Leaf { aabb, elements } => *aabb = elements.aabb(),
        }
    }

    /// Rama izquierda de un nodo intermedio
    pub fn take_left(self) -> Option<Box<BVHNode<T>>> {
        match self {
            BVHNode::Node { left, .. } => left,
            _ => None,
        }
    }

    /// Define la rama izquierda de un nodo intermedio
    /// No actualiza su aabb
    pub fn set_left(&mut self, node: BVHNode<T>) {
        match self {
            BVHNode::Node { left, .. } => *left = Some(Box::new(node)),
            _ => panic!(),
        }
    }

    /// Rama derecha de un nodo intermedio
    pub fn take_right(self) -> Option<Box<BVHNode<T>>> {
        match self {
            BVHNode::Node { right, .. } => right,
            _ => None,
        }
    }

    /// Define la rama derecha de un nodo intermedio
    /// No actualiza el aabb
    pub fn set_right(&mut self, node: BVHNode<T>) {
        match self {
            BVHNode::Node { right, .. } => *right = Some(Box::new(node)),
            _ => panic!(),
        }
    }

    /// Comprueba si es un nodo intermedio y ambos nodos hijos tienen nodo
    pub fn is_complete_node(&self) -> bool {
        match self {
            BVHNode::Node { left, right, .. } => left.is_some() && right.is_some(),
            BVHNode::Leaf { .. } => false,
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
impl<'a, T: Bounded> Iterator for PreorderIter<'a, T> {
    type Item = &'a BVHNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            if node.aabb().intersects(&self.ray).is_some() {
                if let BVHNode::Node { right, left, .. } = node {
                    if let Some(r_node) = &right {
                        self.stack.push(r_node.deref())
                    }
                    if let Some(l_node) = &left {
                        self.stack.push(l_node.deref())
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
    use core::panic;

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

        // Método recursivo
        let bvh = BVH::build_recursive(elements.clone(), 2);
        let root = bvh.root.unwrap();
        let aabb = root.aabb();
        assert_eq!(aabb.min, point![1.0, 1.0, 1.0]);
        assert_eq!(aabb.max, point![9.0, 9.0, 9.0]);
        let left = root.take_left().unwrap();
        let left_aabb = left.aabb();
        assert_eq!(left_aabb.min, point![1.0, 1.0, 1.0]);
        assert_eq!(left_aabb.max, point![5.0, 9.0, 9.0]);

        // Método iterativo
        let bvh = BVH::build(elements.clone(), 2);
        let root = bvh.root.unwrap();
        let aabb = root.aabb();
        assert_eq!(aabb.min, point![1.0, 1.0, 1.0]);
        assert_eq!(aabb.max, point![9.0, 9.0, 9.0]);
        let left = root.take_left().unwrap();
        let left_aabb = left.aabb();
        assert_eq!(left_aabb.min, point![1.0, 1.0, 1.0]);
        assert_eq!(left_aabb.max, point![5.0, 9.0, 9.0]);

        // Rayo que colisiona solo con elemento a
        let bvh = BVH::build(elements, 2);
        let ray = Ray::new(point![2.5, 0.0, 2.5], vector![0.0, 1.0, 0.0]);
        assert_eq!(bvh.intersects(&ray), Some(1.0));
    }

    /// Prueba la construcción de una BVH
    #[test]
    fn bvh_tree_intersects() {
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
        let bvh = BVH::new(Some(root));
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
