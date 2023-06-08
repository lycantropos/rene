import typing as _t

from reprit.base import generate_repr

from rene import (Location,
                  hints as _hints)
from .edge import Edge
from .node import Node
from .trapezoid import Trapezoid


class Leaf(Node[_hints.Scalar]):
    def locate(self,
               point: _hints.Point[_hints.Scalar],
               edges: _t.Sequence[Edge[_hints.Scalar]],
               nodes: _t.Sequence[Node[_hints.Scalar]]) -> Location:
        return (Location.INTERIOR
                if self.trapezoid.is_component(edges)
                else Location.EXTERIOR)

    def search_edge(
            self,
            edge: Edge[_hints.Scalar],
            edges: _t.Sequence[Edge[_hints.Scalar]],
            nodes: _t.Sequence[Node[_hints.Scalar]]
    ) -> Trapezoid[_hints.Scalar]:
        return self.trapezoid

    def to_height(self, nodes: _t.Sequence[Node[_hints.Scalar]]) -> int:
        return 0

    __slots__ = 'trapezoid',

    def __init__(self, trapezoid: Trapezoid[_hints.Scalar]) -> None:
        self.trapezoid = trapezoid

    __repr__ = generate_repr(__init__)
