from __future__ import annotations

from typing import Generic, TYPE_CHECKING

from typing_extensions import Self

from rene import hints
from rene.enums import Orientation

if TYPE_CHECKING:
    from collections.abc import Sequence

    from rene._hints import Orienteer


class Edge(Generic[hints.Scalar]):
    interior_to_left: bool
    left_point_index: int
    right_point_index: int

    @classmethod
    def from_endpoints(
        cls,
        left_point_index: int,
        right_point_index: int,
        interior_to_left: bool,
        orienteer: Orienteer[hints.Scalar],
    ) -> Self:
        return cls(
            left_point_index, right_point_index, interior_to_left, orienteer
        )

    def orientation_of(
        self,
        point: hints.Point[hints.Scalar],
        endpoints: Sequence[hints.Point[hints.Scalar]],
    ) -> Orientation:
        return self._orienteer(
            endpoints[self.left_point_index],
            endpoints[self.right_point_index],
            point,
        )

    _orienteer: Orienteer[hints.Scalar]

    __slots__ = (
        'interior_to_left',
        'left_point_index',
        'right_point_index',
        '_orienteer',
    )

    def __new__(
        cls,
        left_point_index: int,
        right_point_index: int,
        interior_to_left: bool,
        orienteer: Orienteer[hints.Scalar],
        /,
    ) -> Self:
        self = super().__new__(cls)
        (
            self.interior_to_left,
            self.left_point_index,
            self.right_point_index,
            self._orienteer,
        ) = (interior_to_left, left_point_index, right_point_index, orienteer)
        return self

    def is_under(
        self, other: Self, endpoints: Sequence[hints.Point[hints.Scalar]]
    ) -> bool:
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
