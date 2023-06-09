import typing as _t

from reprit.base import generate_repr

from rene import (Location,
                  Orientation,
                  hints as _hints)
from .edge import Edge
from .node import Node


class YNode(Node[_hints.Scalar]):
    def locate(self,
               point: _hints.Point[_hints.Scalar],
               edges: _t.Sequence[Edge[_hints.Scalar]],
               nodes: _t.Sequence[Node[_hints.Scalar]]) -> Location:
        point_orientation = edges[self.edge_index].orientation_of(point)
        return (nodes[self.above_node_index].locate(point, edges, nodes)
                if point_orientation is Orientation.COUNTERCLOCKWISE
                else (nodes[self.below_node_index].locate(point, edges, nodes)
                      if point_orientation is Orientation.CLOCKWISE
                      else Location.BOUNDARY))

    def search_edge_node(
            self,
            edge: Edge[_hints.Scalar],
            edges: _t.Sequence[Edge[_hints.Scalar]],
            nodes: _t.Sequence[Node[_hints.Scalar]]
    ) -> Node[_hints.Scalar]:
        return nodes[
            self.above_node_index
            if edges[self.edge_index] < edge
            else self.below_node_index
        ]

    def to_height(self, nodes: _t.Sequence[Node[_hints.Scalar]]) -> int:
        return max(nodes[self.below_node_index].to_height(nodes),
                   nodes[self.above_node_index].to_height(nodes)) + 1

    __slots__ = 'above_node_index', 'below_node_index', 'edge_index'

    def __init__(self,
                 edge_index: int,
                 below_node_index: int,
                 above_node_index: int) -> None:
        self.above_node_index, self.below_node_index, self.edge_index = (
            above_node_index, below_node_index, edge_index
        )

    __repr__ = generate_repr(__init__)
