from numbers import Rational as _Rational
from typing import (Any as _Any,
                    List as _List,
                    Sequence as _Sequence,
                    Union as _Union,
                    overload as _overload)

from rithm import Fraction as _Fraction

from rene import Orientation as _Orientation


class Contour:
    @property
    def orientation(self) -> _Orientation:
        ...

    @property
    def segments(self) -> _Sequence[Segment]:
        ...

    @property
    def vertices(self) -> _Sequence[Point]:
        ...

    def is_valid(self) -> bool:
        ...

    def __new__(cls, vertices: _Sequence[Point]) -> 'Contour':
        ...

    @_overload
    def __eq__(self, other: 'Contour') -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Multisegment:
    @property
    def segments(self) -> _Sequence[Segment]:
        ...

    def is_valid(self) -> bool:
        ...

    def __new__(cls, segments: _Sequence[Segment]) -> 'Multisegment':
        ...

    @_overload
    def __eq__(self, other: 'Multisegment') -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Point:
    @property
    def x(self) -> _Fraction:
        ...

    @property
    def y(self) -> _Fraction:
        ...

    def __new__(cls, x: _Union[_Rational, float], y: _Union[_Rational, float]
                ) -> 'Point':
        ...

    @_overload
    def __eq__(self, other: 'Point') -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...

    def __ge__(self, other: 'Point') -> bool:
        ...

    def __gt__(self, other: 'Point') -> bool:
        ...

    def __hash__(self) -> int:
        ...

    def __le__(self, other: 'Point') -> bool:
        ...

    def __lt__(self, other: 'Point') -> bool:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Polygon:
    @property
    def border(self) -> Contour:
        ...

    @property
    def holes(self) -> _Sequence[Contour]:
        ...

    def __new__(cls, border: Contour, holes: _Sequence[Contour]) -> 'Polygon':
        ...

    @_overload
    def __eq__(self, other: 'Polygon') -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Segment:
    @property
    def end(self) -> Point:
        ...

    @property
    def start(self) -> Point:
        ...

    def __new__(cls, start: Point, end: Point) -> 'Segment':
        ...

    @_overload
    def __eq__(self, other: 'Segment') -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Triangulation:
    @classmethod
    def delaunay(cls, points: _Sequence[Point]) -> 'Triangulation':
        ...

    def boundary(self) -> Contour:
        ...

    def triangles(self) -> _List[Contour]:
        ...

    def __bool__(self) -> bool:
        ...
