from collections.abc import Sequence

from typing_extensions import Self, override

from rene import hints
from rene.enums import Location

from .edge import Edge
from .node import Node
from .trapezoid import Trapezoid


class Leaf(Node[hints.ScalarT]):
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
            Location.INTERIOR
            if self.trapezoid.is_component
            else Location.EXTERIOR
        )

    @override
    def search_edge_node(
        self,
        edge: Edge[hints.ScalarT],
        edges: Sequence[Edge[hints.ScalarT]],
        endpoints: Sequence[hints.Point[hints.ScalarT]],
        nodes: Sequence[Node[hints.ScalarT]],
        /,
    ) -> Self:
        return self

    @override
    def to_height(self, nodes: Sequence[Node[hints.ScalarT]], /) -> int:
        return 0

    trapezoid: Trapezoid

    __slots__ = ('trapezoid',)

    def __new__(
        cls,
        /,
        *,
        is_component: bool,
        left_point_index: int,
        right_point_index: int,
        below_edge_index: int,
        above_edge_index: int,
        index: int,
    ) -> Self:
        self = super().__new__(cls)
        self.trapezoid = Trapezoid(
            is_component=is_component,
            left_point_index=left_point_index,
            right_point_index=right_point_index,
            below_edge_index=below_edge_index,
            above_edge_index=above_edge_index,
            leaf_index=index,
        )
        return self
