import typing as _t

from reprit.base import generate_repr

from rene import (Location,
                  hints as _hints)
from .edge import Edge
from .node import Node


class XNode(Node[_hints.Scalar]):
    def locate(self,
               point: _hints.Point[_hints.Scalar],
               edges: _t.Sequence[Edge[_hints.Scalar]],
               nodes: _t.Sequence[Node[_hints.Scalar]]) -> Location:
        return (nodes[self.left_node_index].locate(point, edges, nodes)
                if point < self.point
                else (nodes[self.right_node_index].locate(point, edges, nodes)
                      if self.point < point
                      else Location.BOUNDARY))

    def search_edge_node(
            self,
            edge: Edge[_hints.Scalar],
            edges: _t.Sequence[Edge[_hints.Scalar]],
            nodes: _t.Sequence[Node[_hints.Scalar]]
    ) -> Node[_hints.Scalar]:
        return nodes[
            self.left_node_index
            if edge.left_point < self.point
            else self.right_node_index
        ]

    def to_height(self, nodes: _t.Sequence[Node[_hints.Scalar]]) -> int:
        return max(nodes[self.left_node_index].to_height(nodes),
                   nodes[self.right_node_index].to_height(nodes)) + 1

    __slots__ = 'left_node_index', 'point', 'right_node_index'

    def __init__(self,
                 point: _hints.Point[_hints.Scalar],
                 left_node_index: int,
                 right_node_index: int) -> None:
        self.left_node_index, self.point, self.right_node_index = (
            left_node_index, point, right_node_index
        )

    __repr__ = generate_repr(__init__)
