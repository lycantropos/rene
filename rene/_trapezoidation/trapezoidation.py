from __future__ import annotations

import typing as _t

import typing_extensions as _te

from rene import (Location,
                  Orientation,
                  hints as _hints)
from rene._utils import (permute,
                         to_arg_min,
                         to_contour_orientation)
from .edge import Edge
from .leaf import Leaf
from .node import Node
from .trapezoid import Trapezoid
from .x_node import XNode
from .y_node import YNode


class Trapezoidation(_t.Generic[_hints.Scalar]):
    @classmethod
    def from_multisegment(cls,
                          multisegment: _hints.Multisegment[_hints.Scalar],
                          seed: int) -> _te.Self:
        assert seed >= 0, f'Seed should be non-negative, but got {seed}.'
        edges = [(Edge.from_endpoints(segment.start, segment.end, False)
                  if segment.start < segment.end
                  else Edge.from_endpoints(segment.end, segment.start, False))
                 for segment in multisegment.segments]
        permute(edges, seed)
        return cls._from_box_with_edges(multisegment.bounding_box, edges)

    @classmethod
    def from_polygon(cls,
                     polygon: _hints.Polygon[_hints.Scalar],
                     seed: int) -> _te.Self:
        border = polygon.border
        is_border_positively_oriented = (
                to_contour_orientation(border.vertices,
                                       to_arg_min(border.vertices))
                is Orientation.COUNTERCLOCKWISE
        )
        edges = [
            Edge.from_endpoints(segment.start, segment.end,
                                is_border_positively_oriented)
            if segment.start < segment.end
            else Edge.from_endpoints(segment.end, segment.start,
                                     not is_border_positively_oriented)
            for segment in border.segments
        ]
        for hole in polygon.holes:
            is_hole_negatively_oriented = (
                    to_contour_orientation(hole.vertices,
                                           to_arg_min(hole.vertices))
                    is Orientation.CLOCKWISE
            )
            edges.extend(
                    Edge.from_endpoints(segment.start, segment.end,
                                        is_hole_negatively_oriented)
                    if segment.start < segment.end
                    else
                    Edge.from_endpoints(segment.end, segment.start,
                                        not is_hole_negatively_oriented)
                    for segment in hole.segments
            )
        permute(edges, seed)
        return cls._from_box_with_edges(border.bounding_box, edges)

    @property
    def height(self) -> int:
        return self._root.to_height(self._nodes)

    def locate(self, point: _hints.Point[_hints.Scalar]) -> Location:
        """
        Finds location of point relative to decomposed geometry.

        Time complexity:
            ``O(self.height)``
        Memory complexity:
            ``O(1)``
        """
        return self._root.locate(point, self._edges, self._nodes)

    __slots__ = '_edges', '_nodes'

    def __init__(self,
                 _edges: _t.Sequence[Edge[_hints.Scalar]],
                 _nodes: _t.Sequence[Node[_hints.Scalar]]) -> None:
        """
        Initializes graph.

        Time complexity:
            ``O(1)``
        Memory complexity:
            ``O(1)``
        """
        self._edges, self._nodes = _edges, _nodes

    def __contains__(self, point: _hints.Point[_hints.Scalar]) -> bool:
        """
        Checks if point is contained in decomposed geometry.

        Time complexity:
            ``O(self.height)``
        Memory complexity:
            ``O(1)``
        """
        return (self._root.locate(point, self._edges, self._nodes)
                is not Location.EXTERIOR)

    @classmethod
    def _from_box_with_edges(cls,
                             box: _hints.Box[_hints.Scalar],
                             edges: _t.List[Edge[_hints.Scalar]]) -> _te.Self:
        nodes: _t.List[Node[_hints.Scalar]] = []
        edges_count = len(edges)
        add_edge_to_single_trapezoid(
                0, box_to_trapezoid(box, edges, nodes), edges, nodes
        )
        for edge_index in range(1, edges_count):
            _add_edge(edge_index, edges, nodes)
        return cls(edges, nodes)

    @property
    def _root(self) -> Node[_hints.Scalar]:
        return self._nodes[0]


def _find_intersecting_trapezoids(
        edge_index: int,
        edges: _t.Sequence[Edge[_hints.Scalar]],
        nodes: _t.Sequence[Node[_hints.Scalar]]
) -> _t.List[Trapezoid[_hints.Scalar]]:
    edge = edges[edge_index]
    trapezoid = nodes[0].search_edge(edge, edges, nodes)
    result = [trapezoid]
    right = edge.right_point
    while trapezoid.right_point < right:
        candidate_index = (
            (trapezoid.upper_right_leaf_index
             or trapezoid.lower_right_leaf_index)
            if (edge.orientation_of(trapezoid.right_point)
                is Orientation.CLOCKWISE)
            else (trapezoid.lower_right_leaf_index
                  or trapezoid.upper_right_leaf_index)
        )
        assert candidate_index is not None, (
            'Expected neighbour trapezoid, but none found.'
        )
        trapezoid = get_trapezoid(candidate_index, nodes)
        result.append(trapezoid)
    return result


def _add_edge(edge_index: int,
              edges: _t.Sequence[Edge[_hints.Scalar]],
              nodes: _t.List[Node[_hints.Scalar]]) -> None:
    trapezoids = _find_intersecting_trapezoids(edge_index, edges, nodes)
    if len(trapezoids) == 1:
        add_edge_to_single_trapezoid(edge_index, trapezoids[0], edges,
                                     nodes)
    else:
        prev_above, prev_below = add_edge_to_first_trapezoid(
                edge_index, trapezoids[0], edges, nodes
        )
        for middle_trapezoid in trapezoids[1:-1]:
            prev_above, prev_below = add_edge_to_middle_trapezoid(
                    edge_index, middle_trapezoid, prev_above, prev_below,
                    nodes
            )
        add_edge_to_last_trapezoid(edge_index, trapezoids[-1], prev_above,
                                   prev_below, edges, nodes)


def create_trapezoid(
        left_point: _hints.Point[_hints.Scalar],
        right_point: _hints.Point[_hints.Scalar],
        below_edge_index: int,
        above_edge_index: int,
        nodes: _t.List[Node[_hints.Scalar]]
) -> Trapezoid[_hints.Scalar]:
    leaf = Leaf(Trapezoid(left_point, right_point, below_edge_index,
                          above_edge_index, len(nodes)))
    nodes.append(leaf)
    return leaf.trapezoid


def create_x_node(point: _hints.Point[_hints.Scalar],
                  left_node_index: int,
                  right_node_index: int,
                  nodes: _t.List[Node[_hints.Scalar]]) -> int:
    result = len(nodes)
    nodes.append(XNode(point, left_node_index, right_node_index))
    return result


def create_y_node(edge_index: int,
                  below_index: int,
                  above_index: int,
                  nodes: _t.List[Node[_hints.Scalar]]) -> int:
    result = len(nodes)
    nodes.append(YNode(edge_index, below_index, above_index))
    return result


def add_edge_to_first_trapezoid(
        edge_index: int,
        trapezoid: Trapezoid[_hints.Scalar],
        edges: _t.Sequence[Edge[_hints.Scalar]],
        nodes: _t.List[Node[_hints.Scalar]]
) -> _t.Tuple[Trapezoid[_hints.Scalar], Trapezoid[_hints.Scalar]]:
    edge = edges[edge_index]
    above, below = (
        create_trapezoid(edge.left_point, trapezoid.right_point,
                         edge_index,
                         trapezoid.above_edge_index, nodes),
        create_trapezoid(edge.left_point, trapezoid.right_point,
                         trapezoid.below_edge_index, edge_index, nodes)
    )
    replacement_node_index = create_y_node(edge_index, below.leaf_index,
                                           above.leaf_index, nodes)
    # set pairs of trapezoid neighbours
    if edge.left_point == trapezoid.left_point:
        upper_left = maybe_get_trapezoid(trapezoid.upper_left_leaf_index,
                                         nodes)
        above.set_as_upper_left(upper_left)
        lower_left = maybe_get_trapezoid(trapezoid.lower_left_leaf_index,
                                         nodes)
        below.set_as_lower_left(lower_left)
    else:
        left = create_trapezoid(trapezoid.left_point, edge.left_point,
                                trapezoid.below_edge_index,
                                trapezoid.above_edge_index, nodes)
        left.set_as_lower_left(
                maybe_get_trapezoid(trapezoid.lower_left_leaf_index, nodes)
        )
        left.set_as_upper_left(
                maybe_get_trapezoid(trapezoid.upper_left_leaf_index, nodes)
        )
        left.set_as_lower_right(below)
        left.set_as_upper_right(above)
        replacement_node_index = create_x_node(
                edge.left_point, left.leaf_index, replacement_node_index,
                nodes
        )
    above.set_as_upper_right(
            maybe_get_trapezoid(trapezoid.upper_right_leaf_index, nodes)
    )
    below.set_as_lower_right(
            maybe_get_trapezoid(trapezoid.lower_right_leaf_index, nodes)
    )
    replace_node(trapezoid.leaf_index, replacement_node_index, nodes)
    return above, below


def maybe_get_trapezoid(
        index: _t.Optional[int], nodes: _t.Sequence[Node[_hints.Scalar]]
) -> _t.Optional[Trapezoid[_hints.Scalar]]:
    if index is None:
        return None
    node = nodes[index]
    assert isinstance(node, Leaf), node
    return node.trapezoid


def get_trapezoid(
        index: int, nodes: _t.Sequence[Node[_hints.Scalar]]
) -> Trapezoid[_hints.Scalar]:
    node = nodes[index]
    assert isinstance(node, Leaf), node
    return node.trapezoid


def add_edge_to_last_trapezoid(
        edge_index: int,
        trapezoid: Trapezoid[_hints.Scalar],
        prev_above: Trapezoid[_hints.Scalar],
        prev_below: Trapezoid[_hints.Scalar],
        edges: _t.Sequence[Edge[_hints.Scalar]],
        nodes: _t.List[Node[_hints.Scalar]]
) -> None:
    edge = edges[edge_index]
    if prev_above.above_edge_index is trapezoid.above_edge_index:
        above = prev_above
        above.right_point = edge.right_point
    else:
        above = create_trapezoid(trapezoid.left_point, edge.right_point,
                                 edge_index, trapezoid.above_edge_index,
                                 nodes)
        above.set_as_lower_left(prev_above)
        above.set_as_upper_left(
                maybe_get_trapezoid(trapezoid.upper_left_leaf_index, nodes)
        )
    if prev_below.below_edge_index is trapezoid.below_edge_index:
        below = prev_below
        below.right_point = edge.right_point
    else:
        below = create_trapezoid(trapezoid.left_point, edge.right_point,
                                 trapezoid.below_edge_index, edge_index,
                                 nodes)
        below.set_as_upper_left(prev_below)
        below.set_as_lower_left(
                maybe_get_trapezoid(trapezoid.lower_left_leaf_index, nodes)
        )
    replacement_node_index = create_y_node(edge_index, below.leaf_index,
                                           above.leaf_index, nodes)
    # set pairs of trapezoid neighbours
    if edge.right_point == trapezoid.right_point:
        above.set_as_upper_right(
                maybe_get_trapezoid(trapezoid.upper_right_leaf_index,
                                    nodes)
        )
        below.set_as_lower_right(
                maybe_get_trapezoid(trapezoid.lower_right_leaf_index,
                                    nodes)
        )
    else:
        right = create_trapezoid(edge.right_point, trapezoid.right_point,
                                 trapezoid.below_edge_index,
                                 trapezoid.above_edge_index, nodes)
        right.set_as_lower_right(
                maybe_get_trapezoid(trapezoid.lower_right_leaf_index,
                                    nodes)
        )
        right.set_as_upper_right(
                maybe_get_trapezoid(trapezoid.upper_right_leaf_index,
                                    nodes)
        )
        above.set_as_upper_right(right)
        below.set_as_lower_right(right)
        replacement_node_index = create_x_node(edge.right_point,
                                               replacement_node_index,
                                               right.leaf_index, nodes)
    replace_node(trapezoid.leaf_index, replacement_node_index, nodes)


def add_edge_to_middle_trapezoid(
        edge_index: int,
        trapezoid: Trapezoid[_hints.Scalar],
        prev_above: Trapezoid[_hints.Scalar],
        prev_below: Trapezoid[_hints.Scalar],
        nodes: _t.List[Node[_hints.Scalar]]
) -> _t.Tuple[Trapezoid[_hints.Scalar], Trapezoid[_hints.Scalar]]:
    if prev_above.above_edge_index is trapezoid.above_edge_index:
        above = prev_above
        above.right_point = trapezoid.right_point
    else:
        above = create_trapezoid(trapezoid.left_point,
                                 trapezoid.right_point,
                                 edge_index, trapezoid.above_edge_index,
                                 nodes)
        above.set_as_lower_left(prev_above)
        above.set_as_upper_left(
                maybe_get_trapezoid(trapezoid.upper_left_leaf_index, nodes)
        )
    if prev_below.below_edge_index is trapezoid.below_edge_index:
        below = prev_below
        below.right_point = trapezoid.right_point
    else:
        below = create_trapezoid(trapezoid.left_point,
                                 trapezoid.right_point,
                                 trapezoid.below_edge_index, edge_index,
                                 nodes)
        below.set_as_upper_left(prev_below)
        below.set_as_lower_left(
                maybe_get_trapezoid(trapezoid.lower_left_leaf_index, nodes)
        )
    above.set_as_upper_right(
            maybe_get_trapezoid(trapezoid.upper_right_leaf_index, nodes)
    )
    below.set_as_lower_right(
            maybe_get_trapezoid(trapezoid.lower_right_leaf_index, nodes)
    )
    replacement_node_index = create_y_node(edge_index, below.leaf_index,
                                           above.leaf_index, nodes)
    replace_node(trapezoid.leaf_index, replacement_node_index, nodes)
    return above, below


def add_edge_to_single_trapezoid(edge_index: int,
                                 trapezoid: Trapezoid[_hints.Scalar],
                                 edges: _t.Sequence[Edge[_hints.Scalar]],
                                 nodes: _t.List[Node[_hints.Scalar]]) -> None:
    edge = edges[edge_index]
    above, below = (
        create_trapezoid(edge.left_point, edge.right_point, edge_index,
                         trapezoid.above_edge_index, nodes),
        create_trapezoid(edge.left_point, edge.right_point,
                         trapezoid.below_edge_index, edge_index, nodes)
    )
    replacement_node_index = create_y_node(edge_index, below.leaf_index,
                                           above.leaf_index, nodes)
    if edge.right_point == trapezoid.right_point:
        above.set_as_upper_right(
                maybe_get_trapezoid(trapezoid.upper_right_leaf_index,
                                    nodes)
        )
        below.set_as_lower_right(
                maybe_get_trapezoid(trapezoid.lower_right_leaf_index,
                                    nodes)
        )
    else:
        right = create_trapezoid(edge.right_point, trapezoid.right_point,
                                 trapezoid.below_edge_index,
                                 trapezoid.above_edge_index, nodes)
        right.set_as_lower_right(
                maybe_get_trapezoid(trapezoid.lower_right_leaf_index,
                                    nodes)
        )
        right.set_as_upper_right(
                maybe_get_trapezoid(trapezoid.upper_right_leaf_index,
                                    nodes)
        )
        below.set_as_lower_right(right)
        above.set_as_upper_right(right)
        replacement_node_index = create_x_node(edge.right_point,
                                               replacement_node_index,
                                               right.leaf_index, nodes)
    if edge.left_point == trapezoid.left_point:
        above.set_as_upper_left(
                maybe_get_trapezoid(trapezoid.upper_left_leaf_index, nodes)
        )
        below.set_as_lower_left(
                maybe_get_trapezoid(trapezoid.lower_left_leaf_index, nodes)
        )
    else:
        left = create_trapezoid(trapezoid.left_point, edge.left_point,
                                trapezoid.below_edge_index,
                                trapezoid.above_edge_index, nodes)
        left.set_as_lower_left(
                maybe_get_trapezoid(trapezoid.lower_left_leaf_index, nodes)
        )
        left.set_as_upper_left(
                maybe_get_trapezoid(trapezoid.upper_left_leaf_index, nodes)
        )
        left.set_as_lower_right(below)
        left.set_as_upper_right(above)
        replacement_node_index = create_x_node(
                edge.left_point, left.leaf_index, replacement_node_index,
                nodes
        )
    replace_node(trapezoid.leaf_index, replacement_node_index, nodes)


def box_to_trapezoid(
        box: _hints.Box[_hints.Scalar],
        edges: _t.List[Edge[_hints.Scalar]],
        nodes: _t.List[Node[_hints.Scalar]]
) -> Trapezoid[_hints.Scalar]:
    min_x, min_y, max_x, max_y = box.min_x, box.min_y, box.max_x, box.max_y
    delta_x, delta_y = (max_x - min_x) or 1, (max_y - min_y) or 1
    min_x, min_y, max_x, max_y = (min_x - delta_x, min_y - delta_y,
                                  max_x + delta_x, max_y + delta_y)
    point_cls = type(edges[0].left_point)
    below_edge_index = len(edges)
    edges.append(Edge.from_endpoints(point_cls(min_x, min_y),
                                     point_cls(max_x, min_y), False))
    above_edge_index = len(edges)
    edges.append(Edge.from_endpoints(point_cls(min_x, max_y),
                                     point_cls(max_x, max_y), True))
    return create_trapezoid(point_cls(min_x, min_y),
                            point_cls(max_x, min_y),
                            below_edge_index, above_edge_index, nodes)


def replace_node(original_index: int,
                 replacement_index: int,
                 nodes: _t.List[Node[_hints.Scalar]]) -> None:
    assert replacement_index == len(nodes) - 1, (replacement_index,
                                                 len(nodes) - 1)
    nodes[original_index] = nodes.pop()
