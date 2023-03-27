from __future__ import annotations

import typing as _t

import typing_extensions as _te
from reprit.base import generate_repr

from rene.hints import (Point,
                        Scalar)


class ContourVertex(_t.Generic[Scalar]):
    contour_index: int
    index: int
    point: Point[Scalar]

    __slots__ = 'contour_index', 'index', 'point'

    def __new__(
            cls, contour_index: int, index: int, point: Point[Scalar]
    ) -> _te.Self:
        self = super().__new__(cls)
        self.contour_index, self.index, self.point = (
            contour_index, index, point
        )
        return self

    @property
    def x(self) -> Scalar:
        return self.point.x

    @property
    def y(self) -> Scalar:
        return self.point.y

    def __ge__(self, other: _t.Any) -> _t.Any:
        return (self.point >= other.point
                if isinstance(other, ContourVertex)
                else NotImplemented)

    def __gt__(self, other: _t.Any) -> _t.Any:
        return (self.point > other.point
                if isinstance(other, ContourVertex)
                else NotImplemented)

    def __le__(self, other: _t.Any) -> _t.Any:
        return (self.point <= other.point
                if isinstance(other, ContourVertex)
                else NotImplemented)

    def __lt__(self, other: _t.Any) -> _t.Any:
        return (self.point < other.point
                if isinstance(other, ContourVertex)
                else NotImplemented)

    __repr__ = generate_repr(__new__)


class PolygonVertexPosition:
    contour_index: int
    index: int

    __slots__ = 'contour_index', 'index'

    def __new__(cls,
                contour_index: int,
                index: int) -> PolygonVertexPosition:
        self = super().__new__(cls)
        self.contour_index, self.index = contour_index, index
        return self

    def __ge__(self, other: _t.Any) -> _t.Any:
        return ((self.contour_index, self.index)
                >= (other.contour_index, other.index)
                if isinstance(other, ContourVertex)
                else NotImplemented)

    def __gt__(self, other: _t.Any) -> _t.Any:
        return ((self.contour_index, self.index)
                > (other.contour_index, other.index)
                if isinstance(other, ContourVertex)
                else NotImplemented)

    def __le__(self, other: _t.Any) -> _t.Any:
        return ((self.contour_index, self.index)
                <= (other.contour_index, other.index)
                if isinstance(other, ContourVertex)
                else NotImplemented)

    def __lt__(self, other: _t.Any) -> _t.Any:
        return ((self.contour_index, self.index)
                < (other.contour_index, other.index)
                if isinstance(other, ContourVertex)
                else NotImplemented)

    __repr__ = generate_repr(__new__)
