from numbers import Rational as _Rational
from typing import (Any as _Any,
                    Sequence as _Sequence,
                    TypeVar as _TypeVar,
                    overload as _overload)

from typing_extensions import Protocol as _Protocol

from rene import (Orientation as _Orientation,
                  Relation as _Relation)

Scalar = _TypeVar('Scalar',
                  bound=_Rational)


class Point(_Protocol[Scalar]):
    @property
    def x(self) -> Scalar:
        ...

    @property
    def y(self) -> Scalar:
        ...

    def __new__(cls, x: Scalar, y: Scalar) -> 'Point[Scalar]':
        ...

    @_overload
    def __eq__(self, other: 'Point[Scalar]') -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...

    def __eq__(self, other):
        ...

    def __ge__(self, other: 'Point[Scalar]') -> bool:
        ...

    def __gt__(self, other: 'Point[Scalar]') -> bool:
        ...

    def __hash__(self) -> int:
        ...

    def __le__(self, other: 'Point[Scalar]') -> bool:
        ...

    def __lt__(self, other: 'Point[Scalar]') -> bool:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Empty(_Protocol):
    def __new__(cls) -> 'Empty':
        ...

    @_overload
    def __eq__(self, other: 'Empty') -> bool:
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


class Segment(_Protocol[Scalar]):
    @property
    def end(self) -> Point[Scalar]:
        ...

    @property
    def start(self) -> Point[Scalar]:
        ...

    def __new__(cls,
                start: Point[Scalar],
                end: Point[Scalar]) -> 'Segment[Scalar]':
        ...

    @_overload
    def __eq__(self, other: 'Segment[Scalar]') -> bool:
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


class Contour(_Protocol[Scalar]):
    @property
    def orientation(self) -> _Orientation:
        ...

    @property
    def segments(self) -> _Sequence[Segment[Scalar]]:
        ...

    @property
    def vertices(self) -> _Sequence[Point[Scalar]]:
        ...

    def is_valid(self) -> bool:
        ...

    def __new__(cls, vertices: _Sequence[Point[Scalar]]) -> 'Contour[Scalar]':
        ...

    @_overload
    def __eq__(self, other: 'Contour[Scalar]') -> bool:
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


class Multisegment(_Protocol[Scalar]):
    @property
    def segments(self) -> _Sequence[Segment[Scalar]]:
        ...

    def is_valid(self) -> bool:
        ...

    def __new__(
            cls, vertices: _Sequence[Segment[Scalar]]
    ) -> 'Multisegment[Scalar]':
        ...

    @_overload
    def __eq__(self, other: 'Multisegment[Scalar]') -> bool:
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


class Box(_Protocol[Scalar]):
    @property
    def max_x(self) -> Scalar:
        ...

    @property
    def max_y(self) -> Scalar:
        ...

    @property
    def min_x(self) -> Scalar:
        ...

    @property
    def min_y(self) -> Scalar:
        ...

    def enclosed_by(self, other: 'Box[Scalar]') -> bool:
        ...

    def encloses(self, other: 'Box[Scalar]') -> bool:
        ...

    def equals_to(self, other: 'Box[Scalar]') -> bool:
        ...

    def is_valid(self) -> bool:
        ...

    def overlaps(self, other: 'Box[Scalar]') -> bool:
        ...

    def relate_to(self, other: 'Box[Scalar]') -> _Relation:
        ...

    def touches(self, other: 'Box[Scalar]') -> bool:
        ...

    def within(self, other: 'Box[Scalar]') -> bool:
        ...

    def __new__(cls,
                min_x: Scalar,
                max_x: Scalar,
                min_y: Scalar,
                max_y: Scalar) -> 'Box[Scalar]':
        ...

    @_overload
    def __eq__(self, other: 'Box[Scalar]') -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...

    def __eq__(self, other):
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Polygon(_Protocol[Scalar]):
    @property
    def border(self) -> Contour[Scalar]:
        ...

    @property
    def holes(self) -> _Sequence[Contour[Scalar]]:
        ...

    def __new__(cls,
                border: Contour[Scalar],
                holes: _Sequence[Contour[Scalar]]) -> 'Polygon[Scalar]':
        ...

    @_overload
    def __eq__(self, other: 'Polygon[Scalar]') -> bool:
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


class Multipolygon(_Protocol[Scalar]):
    @property
    def polygons(self) -> _Sequence[Polygon[Scalar]]:
        ...

    def __new__(
            cls, vertices: _Sequence[Polygon[Scalar]]
    ) -> 'Multipolygon[Scalar]':
        ...

    @_overload
    def __eq__(self, other: 'Multipolygon[Scalar]') -> bool:
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
