from __future__ import annotations

import typing as t

import typing_extensions as te

from rene import (Orientation,
                  hints)
from rene._utils import orient


class Edge(t.Generic[hints.Scalar]):
    interior_to_left: bool
    left_point_index: int
    right_point_index: int

    @classmethod
    def from_endpoints(cls,
                       left_point_index: int,
                       right_point_index: int,
                       interior_to_left: bool) -> te.Self:
        return cls(left_point_index, right_point_index, interior_to_left)

    def orientation_of(
            self,
            point: hints.Point[hints.Scalar],
            endpoints: t.Sequence[hints.Point[hints.Scalar]]
    ) -> Orientation:
        return orient(endpoints[self.left_point_index],
                      endpoints[self.right_point_index], point)

    __slots__ = 'interior_to_left', 'left_point_index', 'right_point_index'

    def __new__(cls,
                left_point_index: int,
                right_point_index: int,
                interior_to_left: bool) -> te.Self:
        self = super().__new__(cls)
        (
            self.interior_to_left, self.left_point_index,
            self.right_point_index
        ) = interior_to_left, left_point_index, right_point_index
        return self

    def is_under(self,
                 other: te.Self,
                 endpoints: t.Sequence[hints.Point[hints.Scalar]]) -> bool:
        other_left_orientation = self.orientation_of(
                endpoints[other.left_point_index], endpoints
        )
        other_right_orientation = self.orientation_of(
                endpoints[other.right_point_index], endpoints
        )
        if other_left_orientation is other_right_orientation:
            return other_left_orientation is Orientation.COUNTERCLOCKWISE
        elif other_left_orientation is Orientation.COLLINEAR:
            return other_right_orientation is Orientation.COUNTERCLOCKWISE
        left_orientation = other.orientation_of(
                endpoints[self.left_point_index], endpoints
        )
        right_orientation = other.orientation_of(
                endpoints[self.right_point_index], endpoints
        )
        if left_orientation is right_orientation:
            return left_orientation is Orientation.CLOCKWISE
        elif left_orientation is Orientation.COLLINEAR:
            return right_orientation is Orientation.CLOCKWISE
        elif other_right_orientation is Orientation.COLLINEAR:
            return other_left_orientation is Orientation.COUNTERCLOCKWISE
        elif right_orientation is Orientation.COLLINEAR:
            return left_orientation is Orientation.CLOCKWISE
        else:
            # crossing edges are incomparable
            return False
