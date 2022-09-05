from numbers import Rational as _Rational
from typing import (Any as _Any,
                    Sequence as _Sequence,
                    TypeVar as _TypeVar,
                    Union as _Union,
                    overload as _overload)

from typing_extensions import Protocol as _Protocol

from rene import (Orientation as _Orientation,
                  Relation as _Relation)

Scalar = _TypeVar('Scalar',
                  bound=_Rational)

_Self = _TypeVar('_Self')


class _SelfComparable(_Protocol):
    @_overload
    def __eq__(self: _Self, other: _Self) -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...


class Point(_SelfComparable, _Protocol[Scalar]):
    @property
    def x(self) -> Scalar:
        ...

    @property
    def y(self) -> Scalar:
        ...

    def __new__(cls, x: Scalar, y: Scalar) -> 'Point[Scalar]':
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


class Empty(_SelfComparable, _Protocol):
    def __new__(cls) -> 'Empty':
        ...

    def __hash__(self) -> int:
        ...

    @_overload
    def __or__(self, other: 'Empty') -> 'Empty':
        ...

    @_overload
    def __or__(self, other: 'Multipolygon[Scalar]') -> 'Multipolygon[Scalar]':
        ...

    @_overload
    def __or__(self, other: 'Polygon[Scalar]') -> 'Polygon[Scalar]':
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Box(_SelfComparable, _Protocol[Scalar]):
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

    def covers(self, other: 'Box[Scalar]') -> bool:
        ...

    def disjoint_with(self, other: 'Box[Scalar]') -> bool:
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

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Segment(_SelfComparable, _Protocol[Scalar]):
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

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Contour(_SelfComparable, _Protocol[Scalar]):
    @property
    def bounding_box(self) -> Box[Scalar]:
        ...

    @property
    def orientation(self) -> _Orientation:
        ...

    @property
    def segments(self) -> _Sequence[Segment[Scalar]]:
        ...

    @property
    def segments_count(self) -> int:
        ...

    @property
    def vertices(self) -> _Sequence[Point[Scalar]]:
        ...

    @property
    def vertices_count(self) -> int:
        ...

    def is_valid(self) -> bool:
        ...

    def __new__(cls, vertices: _Sequence[Point[Scalar]]) -> 'Contour[Scalar]':
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


_Segmental = _TypeVar('_Segmental',
                      covariant=True)


class Multisegmental(_Protocol[_Segmental]):
    @property
    def segments(self) -> _Sequence[_Segmental]:
        ...

    @property
    def segments_count(self) -> int:
        ...


class Multisegment(_SelfComparable, Multisegmental[Segment[Scalar]],
                   _Protocol[Scalar]):
    def is_valid(self) -> bool:
        ...

    def __new__(
            cls, segments: _Sequence[Segment[Scalar]]
    ) -> 'Multisegment[Scalar]':
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Polygon(_SelfComparable, Multisegmental[Segment[Scalar]],
              _Protocol[Scalar]):
    @property
    def border(self) -> Contour[Scalar]:
        ...

    @property
    def bounding_box(self) -> Box[Scalar]:
        ...

    @property
    def holes(self) -> _Sequence[Contour[Scalar]]:
        ...

    def __new__(cls,
                border: Contour[Scalar],
                holes: _Sequence[Contour[Scalar]]) -> 'Polygon[Scalar]':
        ...

    def __and__(
            self, other: 'Polygon[Scalar]'
    ) -> _Union[Empty, 'Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    def __hash__(self) -> int:
        ...

    @_overload
    def __or__(self, other: 'Empty') -> 'Polygon[Scalar]':
        ...

    @_overload
    def __or__(
            self, other: 'Multipolygon[Scalar]'
    ) -> _Union['Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    @_overload
    def __or__(
            self, other: 'Polygon[Scalar]'
    ) -> _Union['Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...

    @_overload
    def __sub__(self, other: 'Empty') -> 'Polygon[Scalar]':
        ...

    @_overload
    def __sub__(
            self, other: 'Multipolygon[Scalar]'
    ) -> _Union['Empty', 'Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    @_overload
    def __sub__(
            self, other: 'Polygon[Scalar]'
    ) -> _Union[Empty, 'Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    @_overload
    def __xor__(self, other: 'Empty') -> 'Polygon[Scalar]':
        ...

    @_overload
    def __xor__(
            self, other: 'Multipolygon[Scalar]'
    ) -> _Union['Empty', 'Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    @_overload
    def __xor__(
            self, other: 'Polygon[Scalar]'
    ) -> _Union[Empty, 'Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...


class Multipolygon(_SelfComparable, Multisegmental[Segment[Scalar]],
                   _Protocol[Scalar]):
    @property
    def polygons(self) -> _Sequence[Polygon[Scalar]]:
        ...

    def __new__(
            cls, vertices: _Sequence[Polygon[Scalar]]
    ) -> 'Multipolygon[Scalar]':
        ...

    @_overload
    def __and__(self, other: 'Empty') -> 'Empty':
        ...

    @_overload
    def __and__(
            self, other: 'Multipolygon[Scalar]'
    ) -> _Union['Empty', 'Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    @_overload
    def __and__(
            self, other: 'Polygon[Scalar]'
    ) -> _Union['Empty', 'Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    def __hash__(self) -> int:
        ...

    @_overload
    def __or__(self, other: 'Empty') -> 'Polygon[Scalar]':
        ...

    @_overload
    def __or__(
            self, other: 'Multipolygon[Scalar]'
    ) -> _Union['Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    @_overload
    def __or__(
            self, other: 'Polygon[Scalar]'
    ) -> _Union['Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    def __repr__(self) -> str:
        ...

    @_overload
    def __sub__(self, other: 'Empty') -> 'Multipolygon[Scalar]':
        ...

    @_overload
    def __sub__(
            self, other: 'Multipolygon[Scalar]'
    ) -> _Union['Empty', 'Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    @_overload
    def __sub__(
            self, other: 'Polygon[Scalar]'
    ) -> _Union['Empty', 'Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    def __str__(self) -> str:
        ...

    @_overload
    def __xor__(self, other: 'Empty') -> 'Multipolygon[Scalar]':
        ...

    @_overload
    def __xor__(
            self, other: 'Multipolygon[Scalar]'
    ) -> _Union['Empty', 'Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...

    @_overload
    def __xor__(
            self, other: 'Polygon[Scalar]'
    ) -> _Union['Empty', 'Multipolygon[Scalar]', 'Polygon[Scalar]']:
        ...
