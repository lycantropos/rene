from __future__ import annotations

import typing as t

import typing_extensions as te

from rene.hints import Point, Scalar


class ContourVertex(t.Generic[Scalar]):
    contour_index: int
    index: int
    point: Point[Scalar]

    __slots__ = 'contour_index', 'index', 'point'

    def __new__(
        cls, contour_index: int, index: int, point: Point[Scalar], /
    ) -> te.Self:
        self = super().__new__(cls)
        self.contour_index, self.index, self.point = (
            contour_index,
            index,
            point,
        )
        return self

    @property
    def x(self, /) -> Scalar:
        return self.point.x

    @property
    def y(self, /) -> Scalar:
        return self.point.y

    def __ge__(self, other: t.Any, /) -> t.Any:
        return (
            self.point >= other.point
            if isinstance(other, ContourVertex)
            else NotImplemented
        )

    def __gt__(self, other: t.Any, /) -> t.Any:
        return (
            self.point > other.point
            if isinstance(other, ContourVertex)
            else NotImplemented
        )

    def __le__(self, other: t.Any, /) -> t.Any:
        return (
            self.point <= other.point
            if isinstance(other, ContourVertex)
            else NotImplemented
        )

    def __lt__(self, other: t.Any, /) -> t.Any:
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

    def __ge__(self, other: t.Any, /) -> t.Any:
        return (
            (self.contour_index, self.index)
            >= (other.contour_index, other.index)
            if isinstance(other, ContourVertex)
            else NotImplemented
        )

    def __gt__(self, other: t.Any, /) -> t.Any:
        return (
            (self.contour_index, self.index)
            > (other.contour_index, other.index)
            if isinstance(other, ContourVertex)
            else NotImplemented
        )

    def __le__(self, other: t.Any, /) -> t.Any:
        return (
            (self.contour_index, self.index)
            <= (other.contour_index, other.index)
            if isinstance(other, ContourVertex)
            else NotImplemented
        )

    def __lt__(self, other: t.Any, /) -> t.Any:
        return (
            (self.contour_index, self.index)
            < (other.contour_index, other.index)
            if isinstance(other, ContourVertex)
            else NotImplemented
        )
