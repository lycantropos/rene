from __future__ import annotations

from abc import ABC, abstractmethod
from collections.abc import Sequence
from typing import Generic, TYPE_CHECKING

from rene import hints

if TYPE_CHECKING:
    from rene.enums import Location

    from .edge import Edge


class Node(ABC, Generic[hints.ScalarT]):
    @abstractmethod
    def locate(
        self,
        point: hints.Point[hints.ScalarT],
        edges: Sequence[Edge[hints.ScalarT]],
        endpoints: Sequence[hints.Point[hints.ScalarT]],
        nodes: Sequence[Node[hints.ScalarT]],
        /,
    ) -> Location:
        """
        Finds location of given point relative to the contour.
        """

    @abstractmethod
    def search_edge_node(
        self,
        edge: Edge[hints.ScalarT],
        edges: Sequence[Edge[hints.ScalarT]],
        endpoints: Sequence[hints.Point[hints.ScalarT]],
        nodes: Sequence[Node[hints.ScalarT]],
        /,
    ) -> Node[hints.ScalarT]:
        """
        Recursive search for the trapezoid
        which contains the left endpoint of the given segment.
        """

    @abstractmethod
    def to_height(self, nodes: Sequence[Node[hints.ScalarT]], /) -> int:
        """
        Returns height of the node.
        """
