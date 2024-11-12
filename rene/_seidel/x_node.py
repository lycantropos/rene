import typing as t

import typing_extensions as te

from rene import Location, hints

from .edge import Edge
from .node import Node


class XNode(Node[hints.Scalar]):
    point_index: int
    left_node_index: int
    right_node_index: int

    def locate(
        self,
        point: hints.Point[hints.Scalar],
        edges: t.Sequence[Edge[hints.Scalar]],
        endpoints: t.Sequence[hints.Point[hints.Scalar]],
        nodes: t.Sequence[Node[hints.Scalar]],
        /,
    ) -> Location:
        return (
            nodes[self.left_node_index].locate(point, edges, endpoints, nodes)
            if point < endpoints[self.point_index]
            else (
                nodes[self.right_node_index].locate(
                    point, edges, endpoints, nodes
                )
                if endpoints[self.point_index] < point
                else Location.BOUNDARY
            )
        )

    def search_edge_node(
        self,
        edge: Edge[hints.Scalar],
        edges: t.Sequence[Edge[hints.Scalar]],
        endpoints: t.Sequence[hints.Point[hints.Scalar]],
        nodes: t.Sequence[Node[hints.Scalar]],
        /,
    ) -> Node[hints.Scalar]:
        return nodes[
            (
                self.left_node_index
                if endpoints[edge.left_point_index]
                < endpoints[self.point_index]
                else self.right_node_index
            )
        ]

    def to_height(self, nodes: t.Sequence[Node[hints.Scalar]], /) -> int:
        return (
            max(
                nodes[self.left_node_index].to_height(nodes),
                nodes[self.right_node_index].to_height(nodes),
            )
            + 1
        )

    __slots__ = 'left_node_index', 'point_index', 'right_node_index'

    def __new__(
        cls, point_index: int, left_node_index: int, right_node_index: int, /
    ) -> te.Self:
        self = super().__new__(cls)
        self.left_node_index, self.point_index, self.right_node_index = (
            left_node_index,
            point_index,
            right_node_index,
        )
        return self
