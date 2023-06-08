from __future__ import annotations

import typing as _t
from typing import Any

import typing_extensions as _te
from reprit.base import generate_repr

from rene import (Orientation,
                  hints as _hints)
from rene._utils import orient


class Edge(_t.Generic[_hints.Scalar]):
    @classmethod
    def from_endpoints(cls,
                       left_point: _hints.Point[_hints.Scalar],
                       right_point: _hints.Point[_hints.Scalar],
                       interior_to_left: bool) -> _te.Self:
        return cls(left_point, right_point, interior_to_left)

    def orientation_of(self,
                       point: _hints.Point[_hints.Scalar]) -> Orientation:
        return orient(self.left_point, self.right_point, point)

    __slots__ = 'interior_to_left', 'left_point', 'right_point'

    def __init__(self,
                 left_point: _hints.Point[_hints.Scalar],
                 right_point: _hints.Point[_hints.Scalar],
                 interior_to_left: bool) -> None:
        assert left_point < right_point, 'Incorrect endpoints order'
        (
            self.interior_to_left, self.left_point, self.right_point
        ) = interior_to_left, left_point, right_point

    def __lt__(self, other: _te.Self) -> Any:
        """Checks if the edge is lower than the other."""
        other_left_orientation = self.orientation_of(other.left_point)
        other_right_orientation = self.orientation_of(other.right_point)
        if other_left_orientation is other_right_orientation:
            return other_left_orientation is Orientation.COUNTERCLOCKWISE
        elif other_left_orientation is Orientation.COLLINEAR:
            return other_right_orientation is Orientation.COUNTERCLOCKWISE
        left_orientation = other.orientation_of(self.left_point)
        right_orientation = other.orientation_of(self.right_point)
        if left_orientation is right_orientation:
            return left_orientation is Orientation.CLOCKWISE
        elif left_orientation is Orientation.COLLINEAR:
            return right_orientation is Orientation.CLOCKWISE
        elif other_right_orientation is Orientation.COLLINEAR:
            return other_left_orientation is Orientation.COUNTERCLOCKWISE
        else:
            return (left_orientation is Orientation.CLOCKWISE
                    if right_orientation is Orientation.COLLINEAR
                    # crossing edges are incomparable
                    else NotImplemented)

    __repr__ = generate_repr(__init__)
