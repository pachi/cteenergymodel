# encoding: utf-8
import pprint

MAX_ELEMS = 2


class Tree:
    def __init__(self, node=None):
        self.root = node

    def build(elems=None, max_elems=MAX_ELEMS):
        return Tree(split_leaf(Leaf(elems), max_elems))

    def __str__(self):
        return self.root.other_repr(0)


class Leaf:
    def __init__(self, elems=None):
        if elems is None:
            self.elems = []
        else:
            self.elems = elems

    def other_repr(self, level=0):
        ret = "\t"*level+self.elems.__str__()+"\n"
        return ret


class Node:
    def __init__(self, left=None, right=None):
        self.left = left
        self.right = right

    def build(left=None, right=None, max_elems=MAX_ELEMS):
        return Node(split_leaf(Leaf(left), max_elems), split_leaf(Leaf(right), max_elems))

    def other_repr(self, level=0):
        ret = "\n"+"\t"*level+"@L:("+str(level)+")" + \
            self.left.other_repr(level+1)
        ret += "\n"+"\t"*level+"@R:("+str(level)+")" + \
            self.right.other_repr(level+1)
        return ret


def split_leaf(leaf, max_elems):
    if leaf.elems is not None:
        ll = len(leaf.elems)
        if ll > max_elems:
            left = leaf.elems[:ll//2]
            right = leaf.elems[ll//2:]
            return Node.build(left, right, max_elems)
        else:
            return leaf
    return leaf


def generate_node_list(elems, max_elems=MAX_ELEMS):
    """Genera lista de nodos intermedios y terminales a partir de lista de elementos y criterio de partición"""
    # Dos pilas de datos:
    #   (id: usize, tipo: node|leaf, lado: L|R, parent_id: usize, elems: Option<Vec<T>>)
    # Nodos pendientes
    pending = []
    # Nodos procesados
    node_list = []

    id = 0
    ll = len(elems)
    if ll > max_elems:
        left = elems[:ll//2]
        right = elems[ll//2:]
        # guardamos nodo inicial (da igual el lado)
        node_list.append([0, "node", "L", None, None])
        # Nodos pendientes
        pending.append([id+2, "node", "R", id, right])
        pending.append([id+1, "node", "L", id, left])
        id += 2
        # procesar stack de pendientes de dividir
        while len(pending) > 0:
            [c_id, c_type, c_side, c_parent_id, c_elems] = pending.pop()
            cll = len(c_elems)
            if cll > max_elems:
                # Completamos un nodo intermedio y dejamos pendientes sus ramas
                left = c_elems[:cll//2]
                right = c_elems[cll//2:]
                node_list.append(
                    [c_id, "node", c_side, c_parent_id, None])
                pending.append(
                    [id+2, "node", "R", c_id, right])
                pending.append(
                    [id+1, "node", "L", c_id, left])
                id += 2
            else:
                # Completamos nodo terminal
                node_list.append(
                    [c_id, "leaf", c_side, c_parent_id, c_elems])
    else:
        # guardamos el nodo terminal (da igual el lado)
        node_list.append([0, "leaf", "L", None, elems])
    return node_list


def build_from_node_list(node_list):
    """Reconstruye árbol a partir de lista de nodos intermedios y terminales"""
    # Diccionario de elementos pendientes de acabar (sin dos nodos hijos), indexados por padre
    pending = {}
    # Diccionario de elementos completos, listos para insertar en sus padres, indexados por padre
    completed = {}

    # Vamos añadiendo los nodos que tenemos a sus elementos padre y
    # a medida que los completamos los añadimos a sus respectivos padres
    while len(node_list) > 1:
        # Con nodo intermedio elems es None, y tiene datos en nodos terminales
        [id, _type, side, parent_id, elements] = node_list.pop()
        parent_node = pending.get(parent_id, Node(None, None))
        if side == "L":
            if _type == 'leaf':
                parent_node.left = Leaf(elements)
            else:
                parent_node.left = completed[id]
                del completed[id]
        else:
            if _type == 'leaf':
                parent_node.right = Leaf(elements)
            else:
                parent_node.right = completed[id]
                del completed[id]
        # Está completo y disponible para insertar en otro nodo
        if parent_node.left is not None and parent_node.right is not None:
            completed[parent_id] = parent_node
            del pending[parent_id]
        else:
            pending[parent_id] = parent_node
    # Devolvemos el único nodo que debe quedar en complete
    return list(completed.values())[0]


if __name__ == "__main__":
    max_elems = 2
    elems = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    tree = Tree.build(elems, max_elems)
    print(tree)

    node_list = generate_node_list(elems, max_elems)
    print("tree node_list:")
    pprint.pprint(node_list)

    tree2 = build_from_node_list(node_list)
    print("tree (from node list):")
    print(tree2.other_repr())
