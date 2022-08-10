from typing import Any

from reprit.base import generate_repr

from rene.hints import (Point,
                        Scalar)


class ContourVertex:
    contour_index: int
    index: int
    point: Point

    __slots__ = 'contour_index', 'index', 'point'

    def __new__(
            cls, contour_index: int, index: int, point: Point
    ) -> 'ContourVertex':
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

    def __ge__(self, other: Any) -> Any:
        return (self.point >= other.point
                if isinstance(other, ContourVertex)
                else NotImplemented)

    def __gt__(self, other: Any) -> Any:
        return (self.point > other.point
                if isinstance(other, ContourVertex)
                else NotImplemented)

    def __le__(self, other: Any) -> Any:
        return (self.point <= other.point
                if isinstance(other, ContourVertex)
                else NotImplemented)

    def __lt__(self, other: Any) -> Any:
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
                index: int) -> 'PolygonVertexPosition':
        self = super().__new__(cls)
        self.contour_index, self.index = contour_index, index
        return self

    def __ge__(self, other: Any) -> Any:
        return ((self.contour_index, self.index)
                >= (other.contour_index, other.index)
                if isinstance(other, ContourVertex)
                else NotImplemented)

    def __gt__(self, other: Any) -> Any:
        return ((self.contour_index, self.index)
                > (other.contour_index, other.index)
                if isinstance(other, ContourVertex)
                else NotImplemented)

    def __le__(self, other: Any) -> Any:
        return ((self.contour_index, self.index)
                <= (other.contour_index, other.index)
                if isinstance(other, ContourVertex)
                else NotImplemented)

    def __lt__(self, other: Any) -> Any:
        return ((self.contour_index, self.index)
                < (other.contour_index, other.index)
                if isinstance(other, ContourVertex)
                else NotImplemented)

    __repr__ = generate_repr(__new__)
