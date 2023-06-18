import typing as t

import typing_extensions as te

from rene import (Location,
                  Orientation,
                  hints)
from .edge import Edge
from .node import Node


class YNode(Node[hints.Scalar]):
    above_node_index: int
    below_node_index: int
    edge_index: int

    def locate(self,
               point: hints.Point[hints.Scalar],
               edges: t.Sequence[Edge[hints.Scalar]],
               endpoints: t.Sequence[hints.Point[hints.Scalar]],
               nodes: t.Sequence[Node[hints.Scalar]],
               /) -> Location:
        point_orientation = edges[self.edge_index].orientation_of(point,
                                                                  endpoints)
        return (nodes[self.above_node_index].locate(point, edges, endpoints,
                                                    nodes)
                if point_orientation is Orientation.COUNTERCLOCKWISE
                else (nodes[self.below_node_index].locate(point, edges,
                                                          endpoints, nodes)
                      if point_orientation is Orientation.CLOCKWISE
                      else Location.BOUNDARY))

    def search_edge_node(self,
                         edge: Edge[hints.Scalar],
                         edges: t.Sequence[Edge[hints.Scalar]],
                         endpoints: t.Sequence[hints.Point[hints.Scalar]],
                         nodes: t.Sequence[Node[hints.Scalar]],
                         /) -> Node[hints.Scalar]:
        return nodes[
            self.above_node_index
            if edges[self.edge_index].is_under(edge, endpoints)
            else self.below_node_index
        ]

    def to_height(self, nodes: t.Sequence[Node[hints.Scalar]], /) -> int:
        return max(nodes[self.below_node_index].to_height(nodes),
                   nodes[self.above_node_index].to_height(nodes)) + 1

    __slots__ = 'above_node_index', 'below_node_index', 'edge_index'

    def __new__(cls,
                edge_index: int,
                below_node_index: int,
                above_node_index: int,
                /) -> te.Self:
        self = super().__new__(cls)
        self.above_node_index, self.below_node_index, self.edge_index = (
            above_node_index, below_node_index, edge_index
        )
        return self
