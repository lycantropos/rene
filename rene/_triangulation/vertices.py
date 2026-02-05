from __future__ import annotations

from typing import Any, Generic

from typing_extensions import Self

from rene.hints import Point, ScalarT


class ContourVertex(Generic[ScalarT]):
    contour_index: int
    index: int
    point: Point[ScalarT]

    __slots__ = 'contour_index', 'index', 'point'

    def __new__(
        cls, contour_index: int, index: int, point: Point[ScalarT], /
    ) -> Self:
        self = super().__new__(cls)
        self.contour_index, self.index, self.point = (
            contour_index,
            index,
            point,
        )
        return self

    @property
    def x(self, /) -> ScalarT:
        return self.point.x

    @property
    def y(self, /) -> ScalarT:
        return self.point.y

    def __ge__(self, other: Any, /) -> Any:
        return (
            self.point >= other.point
            if isinstance(other, ContourVertex)
            else NotImplemented
        )

    def __gt__(self, other: Any, /) -> Any:
        return (
            self.point > other.point
            if isinstance(other, ContourVertex)
            else NotImplemented
        )

    def __le__(self, other: Any, /) -> Any:
        return (
            self.point <= other.point
            if isinstance(other, ContourVertex)
            else NotImplemented
        )

    def __lt__(self, other: Any, /) -> Any:
        return (
            self.point < other.point
            if isinstance(other, ContourVertex)
            else NotImplemented
        )


class PolygonVertexPosition:
    contour_index: int
    index: int

    __slots__ = 'contour_index', 'index'

    def __new__(
        cls, contour_index: int, index: int, /
    ) -> PolygonVertexPosition:
        self = super().__new__(cls)
        self.contour_index, self.index = contour_index, index
        return self

    def __ge__(self, other: Any, /) -> Any:
        return (
            (self.contour_index, self.index)
            >= (other.contour_index, other.index)
            if isinstance(other, ContourVertex)
            else NotImplemented
        )

    def __gt__(self, other: Any, /) -> Any:
        return (
            (self.contour_index, self.index)
            > (other.contour_index, other.index)
            if isinstance(other, ContourVertex)
            else NotImplemented
        )

    def __le__(self, other: Any, /) -> Any:
        return (
            (self.contour_index, self.index)
            <= (other.contour_index, other.index)
            if isinstance(other, ContourVertex)
            else NotImplemented
        )

    def __lt__(self, other: Any, /) -> Any:
        return (
            (self.contour_index, self.index)
            < (other.contour_index, other.index)
            if isinstance(other, ContourVertex)
            else NotImplemented
        )
