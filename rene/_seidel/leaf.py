import typing as t

import typing_extensions as te
from reprit.base import generate_repr

from rene import (Location,
                  hints)
from .edge import Edge
from .node import Node
from .trapezoid import Trapezoid


class Leaf(Node[hints.Scalar]):
    def locate(self,
               point: hints.Point[hints.Scalar],
               edges: t.Sequence[Edge[hints.Scalar]],
               nodes: t.Sequence[Node[hints.Scalar]],
               /) -> Location:
        return (Location.INTERIOR
                if self.trapezoid.is_component(edges)
                else Location.EXTERIOR)

    def search_edge_node(self,
                         edge: Edge[hints.Scalar],
                         edges: t.Sequence[Edge[hints.Scalar]],
                         nodes: t.Sequence[Node[hints.Scalar]],
                         /) -> te.Self:
        return self

    def to_height(self, nodes: t.Sequence[Node[hints.Scalar]], /) -> int:
        return 0

    __slots__ = 'trapezoid',

    def __init__(self, trapezoid: Trapezoid[hints.Scalar], /) -> None:
        self.trapezoid = trapezoid

    __repr__ = generate_repr(__init__)
