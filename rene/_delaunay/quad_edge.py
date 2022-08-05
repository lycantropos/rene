import sys
from typing import NewType

QuadEdge = NewType('QuadEdge', int)

UNDEFINED_EDGE = QuadEdge(sys.maxsize)


def to_opposite_edge(edge: QuadEdge) -> QuadEdge:
    """
    aka "Sym" in L. Guibas and J. Stolfi notation.
    """
    return QuadEdge(((edge >> 2) << 2) + ((edge + 2) & 3))


def to_rotated_edge(edge: QuadEdge) -> QuadEdge:
    """
    aka "Rot" in L. Guibas and J. Stolfi notation.
    """
    return QuadEdge(((edge >> 2) << 2) + ((edge + 1) & 3))
