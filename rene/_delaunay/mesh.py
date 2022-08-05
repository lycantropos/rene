from typing import (Generic,
                    Iterable,
                    List,
                    TypeVar)

from reprit.base import generate_repr

from rene._utils import is_even
from rene.hints import Point
from .quad_edge import (QuadEdge,
                        to_opposite_edge,
                        to_rotated_edge)
from .vertices import ContoursVertex

Endpoint = TypeVar('Endpoint', Point, ContoursVertex)


class Mesh(Generic[Endpoint]):
    endpoints: List[Endpoint]
    left_from_start: List[QuadEdge]
    starts_indices: List[int]

    @classmethod
    def from_points(cls, endpoints: List[Endpoint]) -> 'Mesh[Endpoint]':
        return cls(endpoints, [], [])

    def connect_edges(self, first: QuadEdge, second: QuadEdge) -> QuadEdge:
        result = self.create_edge(self.to_end_index(first),
                                  self.to_start_index(second))
        self.splice_edges(result, self.to_left_from_end(first))
        self.splice_edges(to_opposite_edge(result), second)
        return result

    def create_edge(self, start_index: int, end_index: int) -> QuadEdge:
        self.starts_indices.append(start_index)
        self.starts_indices.append(end_index)
        edge = QuadEdge(len(self.left_from_start))
        rotated_edge = QuadEdge(edge + 1)
        opposite_edge = QuadEdge(edge + 2)
        triple_rotated_edge = QuadEdge(edge + 3)
        self.left_from_start.append(edge)
        self.left_from_start.append(triple_rotated_edge)
        self.left_from_start.append(opposite_edge)
        self.left_from_start.append(rotated_edge)
        return edge

    def is_deleted_edge(self, edge: QuadEdge) -> bool:
        result = self.to_left_from_start(edge) == edge
        assert (self.to_right_from_start(edge) == edge) is result
        return result

    def delete_edge(self, edge: QuadEdge) -> None:
        self.splice_edges(edge, self.to_right_from_start(edge))
        opposite_edge = to_opposite_edge(edge)
        self.splice_edges(opposite_edge,
                          self.to_right_from_start(opposite_edge))

    def splice_edges(self, first: QuadEdge, second: QuadEdge) -> None:
        alpha = to_rotated_edge(self.to_left_from_start(first))
        beta = to_rotated_edge(self.to_left_from_start(second))
        self.left_from_start[first], self.left_from_start[second] = (
            self.to_left_from_start(second), self.to_left_from_start(first),
        )
        self.left_from_start[alpha], self.left_from_start[beta] = (
            self.to_left_from_start(beta), self.to_left_from_start(alpha),
        )

    def swap_diagonal(self, edge: QuadEdge) -> None:
        """
        Swaps diagonal in a quadrilateral formed by triangles
        in both clockwise and counterclockwise order around the start.
        """
        side = self.to_right_from_start(edge)
        opposite = to_opposite_edge(edge)
        opposite_side = self.to_right_from_start(opposite)
        self.splice_edges(edge, side)
        self.splice_edges(opposite, opposite_side)
        self.splice_edges(edge, self.to_left_from_end(side))
        self.splice_edges(opposite, self.to_left_from_end(opposite_side))
        self.starts_indices[edge // 2] = self.to_end_index(side)
        self.starts_indices[opposite // 2] = self.to_end_index(opposite_side)
        assert self.to_start(edge) == self.to_end(side)
        assert self.to_end(edge) == self.to_end(opposite_side)

    def to_edges(self) -> Iterable[QuadEdge]:
        candidates = [QuadEdge(index)
                      for index in range(0, len(self.left_from_start), 2)]
        return [candidate
                for candidate in candidates
                if not self.is_deleted_edge(candidate)]

    def to_end(self, edge: QuadEdge) -> Endpoint:
        """
        aka "Dest" in L. Guibas and J. Stolfi notation.
        """
        return self.endpoints[self.to_end_index(edge)]

    def to_end_index(self, edge: QuadEdge) -> int:
        return self.to_start_index(to_opposite_edge(edge))

    def to_left_from_end(self, edge: QuadEdge) -> QuadEdge:
        """
        aka "Lnext" in L. Guibas and J. Stolfi notation.
        """
        return to_rotated_edge(
                self.to_left_from_start(
                        to_opposite_edge(to_rotated_edge(edge))))

    def to_left_from_start(self, edge: QuadEdge) -> QuadEdge:
        """
        aka "Onext" in L. Guibas and J. Stolfi notation.
        """
        return self.left_from_start[edge]

    def to_right_from_end(self, edge: QuadEdge) -> QuadEdge:
        """
        aka "Rprev" in L. Guibas and J. Stolfi notation.
        """
        return self.to_left_from_start(to_opposite_edge(edge))

    def to_right_from_start(self, edge: QuadEdge) -> QuadEdge:
        """
        aka "Oprev" in L. Guibas and J. Stolfi notation.
        """
        return to_rotated_edge(self.to_left_from_start(to_rotated_edge(edge)))

    def to_start(self, edge: QuadEdge) -> Endpoint:
        """
        aka "Org" in L. Guibas and J. Stolfi notation.
        """
        return self.endpoints[self.to_start_index(edge)]

    def to_start_index(self, edge: QuadEdge) -> int:
        assert is_even(edge)
        return self.starts_indices[edge // 2]

    def to_unique_edges(self) -> Iterable[QuadEdge]:
        candidates = [QuadEdge(index)
                      for index in range(0, len(self.left_from_start), 4)]
        return [candidate
                for candidate in candidates
                if not self.is_deleted_edge(candidate)]

    __slots__ = 'endpoints', 'left_from_start', 'starts_indices'

    def __bool__(self) -> bool:
        return bool(self.left_from_start)

    def __init__(self,
                 endpoints: List[Endpoint],
                 left_from_start: List[QuadEdge],
                 starts_indices: List[int]) -> None:
        self.endpoints, self.left_from_start, self.starts_indices = (
            endpoints, left_from_start, starts_indices
        )

    __repr__ = generate_repr(__init__)
