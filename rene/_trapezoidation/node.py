from __future__ import annotations

import typing as _t
from abc import (ABC,
                 abstractmethod)

from rene import (Location,
                  hints as _hints)
from .edge import Edge
from .trapezoid import Trapezoid


class Node(ABC, _t.Generic[_hints.Scalar]):
    @abstractmethod
    def locate(self,
               point: _hints.Point[_hints.Scalar],
               edges: _t.Sequence[Edge[_hints.Scalar]],
               nodes: _t.Sequence[Node[_hints.Scalar]]) -> Location:
        """
        Finds location of given point relative to the contour.
        """

    @abstractmethod
    def search_edge(
            self,
            edge: Edge[_hints.Scalar],
            edges: _t.Sequence[Edge[_hints.Scalar]],
            nodes: _t.Sequence[Node[_hints.Scalar]]
    ) -> Trapezoid[_hints.Scalar]:
        """
        Recursive search for the trapezoid
        which contains the left endpoint of the given segment.
        """

    @abstractmethod
    def to_height(self, nodes: _t.Sequence[Node[_hints.Scalar]]) -> int:
        """
        Returns height of the node.
        """