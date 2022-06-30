from typing import NewType

QuadEdge = NewType('QuadEdge', int)


def to_opposite_edge(edge: QuadEdge) -> QuadEdge:
    return QuadEdge(((edge >> 2) << 2) + ((edge + 2) & 3))


def to_rotated_edge(edge: QuadEdge) -> QuadEdge:
    return QuadEdge(((edge >> 2) << 2) + ((edge + 1) & 3))
