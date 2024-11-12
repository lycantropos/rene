from collections.abc import Sequence

from typing_extensions import Self

from rene import Location, hints

from .edge import Edge
from .node import Node
from .trapezoid import Trapezoid


class Leaf(Node[hints.Scalar]):
    def locate(
        self,
        point: hints.Point[hints.Scalar],
        edges: Sequence[Edge[hints.Scalar]],
        endpoints: Sequence[hints.Point[hints.Scalar]],
        nodes: Sequence[Node[hints.Scalar]],
    ) -> Location:
        return (
            Location.INTERIOR
            if self.trapezoid.is_component
            else Location.EXTERIOR
        )

    def search_edge_node(
        self,
        edge: Edge[hints.Scalar],
        edges: Sequence[Edge[hints.Scalar]],
        endpoints: Sequence[hints.Point[hints.Scalar]],
        nodes: Sequence[Node[hints.Scalar]],
    ) -> Self:
        return self

    def to_height(self, nodes: Sequence[Node[hints.Scalar]]) -> int:
        return 0

    trapezoid: Trapezoid

    __slots__ = ('trapezoid',)

    def __new__(
        cls,
        is_component: bool,
        left_point_index: int,
        right_point_index: int,
        below_edge_index: int,
        above_edge_index: int,
        index: int,
    ) -> Self:
        self = super().__new__(cls)
        self.trapezoid = Trapezoid(
            is_component,
            left_point_index,
            right_point_index,
            below_edge_index,
            above_edge_index,
            index,
        )
        return self
