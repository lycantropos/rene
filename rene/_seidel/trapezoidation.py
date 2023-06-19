from __future__ import annotations

import typing as t
from itertools import repeat

import typing_extensions as te

from rene import (Location,
                  Orientation,
                  hints)
from rene._utils import (permute,
                         to_arg_min,
                         to_contour_orientation)
from .edge import Edge
from .leaf import Leaf
from .node import Node
from .trapezoid import Trapezoid
from .x_node import XNode
from .y_node import YNode


class Trapezoidation(t.Generic[hints.Scalar]):
    @classmethod
    def from_multisegment(cls,
                          multisegment: hints.Multisegment[hints.Scalar],
                          seed: int,
                          /) -> te.Self:
        assert seed >= 0, f'Seed should be non-negative, but got {seed}.'
        endpoints: t.List[hints.Point[hints.Scalar]] = []
        edges: t.List[Edge[hints.Scalar]] = []
        for segment in multisegment.segments:
            start, end = segment.start, segment.end
            start_index = len(endpoints)
            endpoints.append(start)
            end_index = len(endpoints)
            endpoints.append(end)
            edges.append(Edge.from_endpoints(start_index, end_index, False)
                         if start < end
                         else Edge.from_endpoints(end_index, start_index,
                                                  False))
        permute(edges, seed)
        return cls._from_box(multisegment.bounding_box, edges, endpoints)

    @classmethod
    def from_polygon(cls,
                     polygon: hints.Polygon[hints.Scalar],
                     seed: int,
                     /) -> te.Self:
        edges: t.List[Edge[hints.Scalar]] = []
        endpoints: t.List[hints.Point[hints.Scalar]] = []
        _populate_from_contour(polygon.border, Orientation.COUNTERCLOCKWISE,
                               edges, endpoints)
        for hole in polygon.holes:
            _populate_from_contour(hole, Orientation.CLOCKWISE, edges,
                                   endpoints)
        permute(edges, seed)
        return cls._from_box(polygon.bounding_box, edges, endpoints)

    @property
    def height(self) -> int:
        return self._root.to_height(self._nodes)

    def locate(self, point: hints.Point[hints.Scalar], /) -> Location:
        return self._root.locate(point, self._edges, self._endpoints,
                                 self._nodes)

    @classmethod
    def _from_box(
            cls,
            box: hints.Box[hints.Scalar],
            edges: t.List[Edge[hints.Scalar]],
            endpoints: t.List[hints.Point[hints.Scalar]],
            /
    ) -> te.Self:
        nodes: t.List[Node[hints.Scalar]] = []
        edges_count = len(edges)
        _add_edge_to_single_trapezoid(
                0, _box_to_trapezoid(box, edges, endpoints, nodes),
                edges, endpoints, nodes
        )
        for edge_index in range(1, edges_count):
            _add_edge(edge_index, edges, endpoints, nodes)
        return cls(edges, endpoints, nodes)

    @property
    def _root(self) -> Node[hints.Scalar]:
        return self._nodes[0]

    _edges: t.Sequence[Edge[hints.Scalar]]
    _endpoints: t.Sequence[hints.Point[hints.Scalar]]
    _nodes: t.Sequence[Node[hints.Scalar]]

    __slots__ = '_edges', '_endpoints', '_nodes'

    def __new__(cls,
                edges: t.Sequence[Edge[hints.Scalar]],
                endpoints: t.Sequence[hints.Point[hints.Scalar]],
                nodes: t.Sequence[Node[hints.Scalar]],
                /) -> te.Self:
        self = super().__new__(cls)
        self._edges, self._endpoints, self._nodes = edges, endpoints, nodes
        return self

    def __contains__(self, point: hints.Point[hints.Scalar]) -> bool:
        return bool(self._root.locate(point, self._edges, self._endpoints,
                                      self._nodes))


def _add_edge(edge_index: int,
              edges: t.Sequence[Edge[hints.Scalar]],
              endpoints: t.Sequence[hints.Point[hints.Scalar]],
              nodes: t.List[Node[hints.Scalar]],
              /) -> None:
    trapezoids = _find_intersecting_trapezoids(edge_index, edges, endpoints,
                                               nodes)
    if len(trapezoids) == 1:
        _add_edge_to_single_trapezoid(edge_index, trapezoids[0], edges,
                                      endpoints, nodes)
    else:
        prev_above, prev_below = _add_edge_to_first_trapezoid(
                edge_index, trapezoids[0], edges, endpoints, nodes
        )
        for middle_trapezoid in trapezoids[1:-1]:
            prev_above, prev_below = _add_edge_to_middle_trapezoid(
                    edge_index, middle_trapezoid, prev_above, prev_below,
                    edges, nodes
            )
        _add_edge_to_last_trapezoid(edge_index, trapezoids[-1], prev_above,
                                    prev_below, edges, endpoints, nodes)


def _add_edge_to_first_trapezoid(
        edge_index: int,
        trapezoid: Trapezoid,
        edges: t.Sequence[Edge[hints.Scalar]],
        endpoints: t.Sequence[hints.Point[hints.Scalar]],
        nodes: t.List[Node[hints.Scalar]],
        /
) -> t.Tuple[Trapezoid, Trapezoid]:
    edge = edges[edge_index]
    above, below = (
        _create_trapezoid(edge.left_point_index, trapezoid.right_point_index,
                          edge_index, trapezoid.above_edge_index, edges,
                          nodes),
        _create_trapezoid(edge.left_point_index, trapezoid.right_point_index,
                          trapezoid.below_edge_index, edge_index, edges, nodes)
    )
    replacement_node_index = _create_y_node(edge_index, below.leaf_index,
                                            above.leaf_index, nodes)
    # set pairs of trapezoid neighbours
    if (endpoints[edge.left_point_index]
            == endpoints[trapezoid.left_point_index]):
        _maybe_set_as_upper_left(above, trapezoid.upper_left_node_index, nodes)
        _maybe_set_as_lower_left(below, trapezoid.lower_left_node_index, nodes)
    else:
        left = _create_trapezoid(trapezoid.left_point_index,
                                 edge.left_point_index,
                                 trapezoid.below_edge_index,
                                 trapezoid.above_edge_index, edges, nodes)
        _maybe_set_as_lower_left(left, trapezoid.lower_left_node_index, nodes)
        _maybe_set_as_upper_left(left, trapezoid.upper_left_node_index, nodes)
        left.set_as_lower_right(below)
        left.set_as_upper_right(above)
        replacement_node_index = _create_x_node(
                edge.left_point_index, left.leaf_index, replacement_node_index,
                nodes
        )
    _maybe_set_as_upper_right(above, trapezoid.upper_right_node_index, nodes)
    _maybe_set_as_lower_right(below, trapezoid.lower_right_node_index, nodes)
    _replace_node(trapezoid.leaf_index, replacement_node_index, nodes)
    return above, below


def _add_edge_to_last_trapezoid(
        edge_index: int,
        trapezoid: Trapezoid,
        prev_above: Trapezoid,
        prev_below: Trapezoid,
        edges: t.Sequence[Edge[hints.Scalar]],
        endpoints: t.Sequence[hints.Point[hints.Scalar]],
        nodes: t.List[Node[hints.Scalar]],
        /
) -> None:
    edge = edges[edge_index]
    if prev_above.above_edge_index is trapezoid.above_edge_index:
        above = prev_above
        above.right_point_index = edge.right_point_index
    else:
        above = _create_trapezoid(trapezoid.left_point_index,
                                  edge.right_point_index, edge_index,
                                  trapezoid.above_edge_index, edges, nodes)
        above.set_as_lower_left(prev_above)
        _maybe_set_as_upper_left(above, trapezoid.upper_left_node_index, nodes)
    if prev_below.below_edge_index is trapezoid.below_edge_index:
        below = prev_below
        below.right_point_index = edge.right_point_index
    else:
        below = _create_trapezoid(
                trapezoid.left_point_index, edge.right_point_index,
                trapezoid.below_edge_index, edge_index, edges, nodes
        )
        below.set_as_upper_left(prev_below)
        _maybe_set_as_lower_left(below, trapezoid.lower_left_node_index, nodes)
    replacement_node_index = _create_y_node(edge_index, below.leaf_index,
                                            above.leaf_index, nodes)
    # set pairs of trapezoid neighbours
    if (endpoints[edge.right_point_index]
            == endpoints[trapezoid.right_point_index]):
        _maybe_set_as_upper_right(above, trapezoid.upper_right_node_index,
                                  nodes)
        _maybe_set_as_lower_right(below, trapezoid.lower_right_node_index,
                                  nodes)
    else:
        right = _create_trapezoid(edge.right_point_index,
                                  trapezoid.right_point_index,
                                  trapezoid.below_edge_index,
                                  trapezoid.above_edge_index, edges, nodes)
        _maybe_set_as_lower_right(right, trapezoid.lower_right_node_index,
                                  nodes)
        _maybe_set_as_upper_right(right, trapezoid.upper_right_node_index,
                                  nodes)
        right.set_as_lower_left(below)
        right.set_as_upper_left(above)
        replacement_node_index = _create_x_node(edge.right_point_index,
                                                replacement_node_index,
                                                right.leaf_index, nodes)
    _replace_node(trapezoid.leaf_index, replacement_node_index, nodes)


def _add_edge_to_middle_trapezoid(
        edge_index: int,
        trapezoid: Trapezoid,
        prev_above: Trapezoid,
        prev_below: Trapezoid,
        edges: t.Sequence[Edge[hints.Scalar]],
        nodes: t.List[Node[hints.Scalar]],
        /
) -> t.Tuple[Trapezoid, Trapezoid]:
    if prev_above.above_edge_index == trapezoid.above_edge_index:
        above = prev_above
        above.right_point_index = trapezoid.right_point_index
    else:
        above = _create_trapezoid(trapezoid.left_point_index,
                                  trapezoid.right_point_index, edge_index,
                                  trapezoid.above_edge_index, edges, nodes)
        above.set_as_lower_left(prev_above)
        _maybe_set_as_upper_left(above, trapezoid.upper_left_node_index, nodes)
    if prev_below.below_edge_index == trapezoid.below_edge_index:
        below = prev_below
        below.right_point_index = trapezoid.right_point_index
    else:
        below = _create_trapezoid(
                trapezoid.left_point_index, trapezoid.right_point_index,
                trapezoid.below_edge_index, edge_index, edges, nodes
        )
        below.set_as_upper_left(prev_below)
        _maybe_set_as_lower_left(below, trapezoid.lower_left_node_index, nodes)
    _maybe_set_as_upper_right(above, trapezoid.upper_right_node_index, nodes)
    _maybe_set_as_lower_right(below, trapezoid.lower_right_node_index, nodes)
    replacement_node_index = _create_y_node(edge_index, below.leaf_index,
                                            above.leaf_index, nodes)
    _replace_node(trapezoid.leaf_index, replacement_node_index, nodes)
    return above, below


def _add_edge_to_single_trapezoid(
        edge_index: int,
        trapezoid: Trapezoid,
        edges: t.Sequence[Edge[hints.Scalar]],
        endpoints: t.Sequence[hints.Point[hints.Scalar]],
        nodes: t.List[Node[hints.Scalar]],
        /
) -> None:
    edge = edges[edge_index]
    above, below = (
        _create_trapezoid(edge.left_point_index, edge.right_point_index,
                          edge_index, trapezoid.above_edge_index, edges,
                          nodes),
        _create_trapezoid(edge.left_point_index, edge.right_point_index,
                          trapezoid.below_edge_index, edge_index, edges, nodes)
    )
    replacement_node_index = _create_y_node(edge_index, below.leaf_index,
                                            above.leaf_index, nodes)
    if (endpoints[edge.right_point_index]
            == endpoints[trapezoid.right_point_index]):
        _maybe_set_as_upper_right(above, trapezoid.upper_right_node_index,
                                  nodes)
        _maybe_set_as_lower_right(below, trapezoid.lower_right_node_index,
                                  nodes)
    else:
        right = _create_trapezoid(edge.right_point_index,
                                  trapezoid.right_point_index,
                                  trapezoid.below_edge_index,
                                  trapezoid.above_edge_index, edges, nodes)
        _maybe_set_as_lower_right(right, trapezoid.lower_right_node_index,
                                  nodes)
        _maybe_set_as_upper_right(right, trapezoid.upper_right_node_index,
                                  nodes)
        right.set_as_lower_left(below)
        right.set_as_upper_left(above)
        replacement_node_index = _create_x_node(edge.right_point_index,
                                                replacement_node_index,
                                                right.leaf_index, nodes)
    if (endpoints[edge.left_point_index]
            == endpoints[trapezoid.left_point_index]):
        _maybe_set_as_upper_left(above, trapezoid.upper_left_node_index, nodes)
        _maybe_set_as_lower_left(below, trapezoid.lower_left_node_index, nodes)
    else:
        left = _create_trapezoid(trapezoid.left_point_index,
                                 edge.left_point_index,
                                 trapezoid.below_edge_index,
                                 trapezoid.above_edge_index, edges, nodes)
        _maybe_set_as_lower_left(left, trapezoid.lower_left_node_index, nodes)
        _maybe_set_as_upper_left(left, trapezoid.upper_left_node_index, nodes)
        left.set_as_lower_right(below)
        left.set_as_upper_right(above)
        replacement_node_index = _create_x_node(
                edge.left_point_index, left.leaf_index, replacement_node_index,
                nodes
        )
    _replace_node(trapezoid.leaf_index, replacement_node_index, nodes)


def _box_to_trapezoid(box: hints.Box[hints.Scalar],
                      edges: t.List[Edge[hints.Scalar]],
                      endpoints: t.List[hints.Point[hints.Scalar]],
                      nodes: t.List[Node[hints.Scalar]],
                      /) -> Trapezoid:
    min_x, min_y, max_x, max_y = box.min_x, box.min_y, box.max_x, box.max_y
    delta_x, delta_y = (max_x - min_x) or 1, (max_y - min_y) or 1
    min_x, min_y, max_x, max_y = (min_x - delta_x, min_y - delta_y,
                                  max_x + delta_x, max_y + delta_y)
    point_cls = type(endpoints[0])
    above_edge_left_point_index = len(endpoints)
    endpoints.append(point_cls(min_x, max_y))
    above_edge_right_point_index = len(endpoints)
    endpoints.append(point_cls(max_x, max_y))
    above_edge_index = len(edges)
    edges.append(Edge.from_endpoints(above_edge_left_point_index,
                                     above_edge_right_point_index, True))
    below_edge_left_point_index = len(endpoints)
    endpoints.append(point_cls(min_x, min_y))
    below_edge_right_point_index = len(endpoints)
    endpoints.append(point_cls(max_x, min_y))
    below_edge_index = len(edges)
    edges.append(Edge.from_endpoints(below_edge_left_point_index,
                                     below_edge_right_point_index, False))
    return _create_trapezoid(below_edge_left_point_index,
                             below_edge_right_point_index, below_edge_index,
                             above_edge_index, edges, nodes)


def _create_trapezoid(left_point_index: int,
                      right_point_index: int,
                      below_edge_index: int,
                      above_edge_index: int,
                      edges: t.Sequence[Edge[hints.Scalar]],
                      nodes: t.List[Node[hints.Scalar]],
                      /) -> Trapezoid:
    is_component = (edges[below_edge_index].interior_to_left
                    and not edges[above_edge_index].interior_to_left)
    leaf: Leaf[hints.Scalar] = Leaf(is_component, left_point_index,
                                    right_point_index, below_edge_index,
                                    above_edge_index, len(nodes))
    nodes.append(leaf)
    return leaf.trapezoid


def _create_x_node(point_index: int,
                   left_node_index: int,
                   right_node_index: int,
                   nodes: t.List[Node[hints.Scalar]],
                   /) -> int:
    result = len(nodes)
    nodes.append(XNode(point_index, left_node_index, right_node_index))
    return result


def _create_y_node(edge_index: int,
                   below_index: int,
                   above_index: int,
                   nodes: t.List[Node[hints.Scalar]],
                   /) -> int:
    result = len(nodes)
    nodes.append(YNode(edge_index, below_index, above_index))
    return result


def _find_intersecting_trapezoids(
        edge_index: int,
        edges: t.Sequence[Edge[hints.Scalar]],
        endpoints: t.Sequence[hints.Point[hints.Scalar]],
        nodes: t.Sequence[Node[hints.Scalar]],
        /
) -> t.List[Trapezoid]:
    edge = edges[edge_index]
    cursor = nodes[0]
    for _ in repeat(None, len(nodes) - 1):
        candidate = cursor.search_edge_node(edge, edges, endpoints, nodes)
        if candidate is cursor:
            break
        cursor = candidate
    assert isinstance(cursor, Leaf), cursor
    trapezoid = cursor.trapezoid
    result = [trapezoid]
    right = endpoints[edge.right_point_index]
    while endpoints[trapezoid.right_point_index] < right:
        candidate_index = (
            (trapezoid.upper_right_node_index
             or trapezoid.lower_right_node_index)
            if (edge.orientation_of(endpoints[trapezoid.right_point_index],
                                    endpoints)
                is Orientation.CLOCKWISE)
            else (trapezoid.lower_right_node_index
                  or trapezoid.upper_right_node_index)
        )
        assert candidate_index is not None, (
            'Expected neighbour trapezoid, but none found.'
        )
        trapezoid = _get_trapezoid(candidate_index, nodes)
        result.append(trapezoid)
    return result


def _get_trapezoid(index: int,
                   nodes: t.Sequence[Node[hints.Scalar]],
                   /) -> Trapezoid:
    node = nodes[index]
    assert isinstance(node, Leaf), node
    return node.trapezoid


def _maybe_set_as_lower_left(trapezoid: Trapezoid,
                             maybe_node_index: t.Optional[int],
                             nodes: t.Sequence[Node[hints.Scalar]],
                             /) -> None:
    if maybe_node_index is None:
        trapezoid.reset_lower_left()
    else:
        trapezoid.set_as_lower_left(_get_trapezoid(maybe_node_index, nodes))


def _maybe_set_as_lower_right(trapezoid: Trapezoid,
                              maybe_node_index: t.Optional[int],
                              nodes: t.Sequence[Node[hints.Scalar]],
                              /) -> None:
    if maybe_node_index is None:
        trapezoid.reset_lower_right()
    else:
        trapezoid.set_as_lower_right(_get_trapezoid(maybe_node_index, nodes))


def _maybe_set_as_upper_left(trapezoid: Trapezoid,
                             maybe_node_index: t.Optional[int],
                             nodes: t.Sequence[Node[hints.Scalar]],
                             /) -> None:
    if maybe_node_index is None:
        trapezoid.reset_upper_left()
    else:
        trapezoid.set_as_upper_left(_get_trapezoid(maybe_node_index, nodes))


def _maybe_set_as_upper_right(trapezoid: Trapezoid,
                              maybe_node_index: t.Optional[int],
                              nodes: t.Sequence[Node[hints.Scalar]],
                              /) -> None:
    if maybe_node_index is None:
        trapezoid.reset_upper_right()
    else:
        trapezoid.set_as_upper_right(_get_trapezoid(maybe_node_index, nodes))


def _populate_from_contour(
        contour: hints.Contour[hints.Scalar],
        correct_orientation: Orientation,
        edges: t.List[Edge[hints.Scalar]],
        endpoints: t.List[hints.Point[hints.Scalar]],
        /
) -> None:
    contour_vertices = contour.vertices
    is_contour_correctly_oriented = (
            to_contour_orientation(contour_vertices,
                                   to_arg_min(contour_vertices))
            is correct_orientation
    )
    first_start = start = contour_vertices[0]
    first_start_index = start_index = len(endpoints)
    endpoints.extend(contour_vertices)
    for end_index, end in enumerate(contour_vertices[1:],
                                    start=first_start_index + 1):
        edges.append(
                Edge.from_endpoints(start_index, end_index,
                                    is_contour_correctly_oriented)
                if start < end
                else Edge.from_endpoints(end_index, start_index,
                                         not is_contour_correctly_oriented)
        )
        start, start_index = end, end_index
    last_end_index, last_end = len(endpoints) - 1, endpoints[-1]
    assert last_end_index == first_start_index + contour.vertices_count - 1
    edges.append(
            Edge.from_endpoints(first_start_index, last_end_index,
                                is_contour_correctly_oriented)
            if first_start < last_end
            else Edge.from_endpoints(last_end_index, first_start_index,
                                     not is_contour_correctly_oriented)
    )


def _replace_node(original_index: int,
                  replacement_index: int,
                  nodes: t.List[Node[hints.Scalar]],
                  /) -> None:
    assert replacement_index == len(nodes) - 1, (replacement_index,
                                                 len(nodes) - 1)
    nodes[original_index] = nodes.pop()
