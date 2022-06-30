from typing import (Iterable,
                    List)

from reprit.base import generate_repr

from rene._utils import is_even
from rene.hints import Point
from .quad_edge import (QuadEdge,
                        to_opposite_edge,
                        to_rotated_edge)


class Mesh:
    @classmethod
    def from_points(cls, endpoints: List[Point]) -> 'Mesh':
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

    def to_edges(self) -> Iterable[QuadEdge]:
        return [QuadEdge(index * 2)
                for index in range(0, len(self.left_from_start) // 2)]

    def to_end(self, edge: QuadEdge) -> Point:
        return self.endpoints[self.to_start_index(to_opposite_edge(edge))]

    def to_end_index(self, edge: QuadEdge) -> int:
        return self.to_start_index(to_opposite_edge(edge))

    def to_left_from_end(self, edge: QuadEdge) -> QuadEdge:
        return to_rotated_edge(
                self.to_left_from_start(
                        to_opposite_edge(to_rotated_edge(edge))))

    def to_left_from_start(self, edge: QuadEdge) -> QuadEdge:
        return self.left_from_start[edge]

    def to_right_from_end(self, edge: QuadEdge) -> QuadEdge:
        return self.to_left_from_start(to_opposite_edge(edge))

    def to_right_from_start(self, edge: QuadEdge) -> QuadEdge:
        return to_rotated_edge(self.to_left_from_start(to_rotated_edge(edge)))

    def to_start(self, edge: QuadEdge) -> Point:
        return self.endpoints[self.to_start_index(edge)]

    def to_start_index(self, edge: QuadEdge) -> int:
        assert is_even(edge)
        return self.starts_indices[edge // 2]

    __slots__ = 'endpoints', 'left_from_start', 'starts_indices'

    def __bool__(self) -> bool:
        return bool(self.left_from_start)

    def __init__(self,
                 endpoints: List[Point],
                 left_from_start: List[QuadEdge],
                 starts_indices: List[int]) -> None:
        self.endpoints, self.left_from_start, self.starts_indices = (
            endpoints, left_from_start, starts_indices
        )

    __repr__ = generate_repr(__init__)
