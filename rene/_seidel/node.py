from __future__ import annotations

import typing as t
from abc import ABC, abstractmethod

from rene import Location, hints

if t.TYPE_CHECKING:
    from .edge import Edge


class Node(ABC, t.Generic[hints.Scalar]):
    @abstractmethod
    def locate(
        self,
        point: hints.Point[hints.Scalar],
        edges: t.Sequence[Edge[hints.Scalar]],
        endpoints: t.Sequence[hints.Point[hints.Scalar]],
        nodes: t.Sequence[Node[hints.Scalar]],
    ) -> Location:
        """
        Finds location of given point relative to the contour.
        """

    @abstractmethod
    def search_edge_node(
        self,
        edge: Edge[hints.Scalar],
        edges: t.Sequence[Edge[hints.Scalar]],
        endpoints: t.Sequence[hints.Point[hints.Scalar]],
        nodes: t.Sequence[Node[hints.Scalar]],
    ) -> Node[hints.Scalar]:
        """
        Recursive search for the trapezoid
        which contains the left endpoint of the given segment.
        """

    @abstractmethod
    def to_height(self, nodes: t.Sequence[Node[hints.Scalar]]) -> int:
        """
        Returns height of the node.
        """
