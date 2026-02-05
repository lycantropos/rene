from collections.abc import Sequence

from typing_extensions import Self, override

from rene import hints
from rene.enums import Location

from .edge import Edge
from .node import Node


class XNode(Node[hints.ScalarT]):
    point_index: int
    left_node_index: int
    right_node_index: int

    @override
    def locate(
        self,
        point: hints.Point[hints.ScalarT],
        edges: Sequence[Edge[hints.ScalarT]],
        endpoints: Sequence[hints.Point[hints.ScalarT]],
        nodes: Sequence[Node[hints.ScalarT]],
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

    @override
    def search_edge_node(
        self,
        edge: Edge[hints.ScalarT],
        edges: Sequence[Edge[hints.ScalarT]],
        endpoints: Sequence[hints.Point[hints.ScalarT]],
        nodes: Sequence[Node[hints.ScalarT]],
        /,
    ) -> Node[hints.ScalarT]:
        return nodes[
            (
                self.left_node_index
                if (
                    endpoints[edge.left_point_index]
                    < endpoints[self.point_index]
                )
                else self.right_node_index
            )
        ]

    @override
    def to_height(self, nodes: Sequence[Node[hints.ScalarT]], /) -> int:
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
    ) -> Self:
        self = super().__new__(cls)
        self.left_node_index, self.point_index, self.right_node_index = (
            left_node_index,
            point_index,
            right_node_index,
        )
        return self
