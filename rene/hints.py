from __future__ import annotations

import typing as _t

import typing_extensions as _te

from rene import (Orientation as _Orientation,
                  Relation as _Relation)

_Self = _t.TypeVar('_Self')


class _Scalar(_te.Protocol):
    def __add__(self: _Self, other: _Self) -> _Self:
        pass

    @_t.overload
    def __eq__(self: _Self, other: _Self) -> bool:
        pass

    @_t.overload
    def __eq__(self: _Self, other: _t.Any) -> _t.Any:
        pass

    @_t.overload
    def __ge__(self: _Self, other: _Self) -> bool:
        pass

    @_t.overload
    def __ge__(self: _Self, other: _t.Any) -> _t.Any:
        pass

    @_t.overload
    def __gt__(self: _Self, other: _Self) -> bool:
        pass

    @_t.overload
    def __gt__(self: _Self, other: _t.Any) -> _t.Any:
        pass

    @_t.overload
    def __le__(self: _Self, other: _Self) -> bool:
        pass

    @_t.overload
    def __le__(self: _Self, other: _t.Any) -> _t.Any:
        pass

    @_t.overload
    def __lt__(self: _Self, other: _Self) -> bool:
        pass

    @_t.overload
    def __lt__(self: _Self, other: _t.Any) -> _t.Any:
        pass

    def __mul__(self: _Self, other: _Self) -> _Self:
        pass

    def __neg__(self: _Self) -> _Self:
        pass

    def __pos__(self: _Self) -> _Self:
        pass

    def __sub__(self: _Self, other: _Self) -> _Self:
        pass

    def __truediv__(self: _Self, other: _Self) -> _Self:
        pass


Scalar = _t.TypeVar('Scalar',
                    bound=_Scalar)


class _SelfComparable(_te.Protocol):
    @_t.overload
    def __eq__(self: _Self, other: _Self) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...


class Point(_SelfComparable, _te.Protocol[Scalar]):
    @property
    def x(self) -> Scalar:
        ...

    @property
    def y(self) -> Scalar:
        ...

    def __new__(cls, x: Scalar, y: Scalar) -> Point[Scalar]:
        ...

    def __ge__(self, other: Point[Scalar]) -> bool:
        ...

    def __gt__(self, other: Point[Scalar]) -> bool:
        ...

    def __hash__(self) -> int:
        ...

    def __le__(self, other: Point[Scalar]) -> bool:
        ...

    def __lt__(self, other: Point[Scalar]) -> bool:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Empty(_SelfComparable, _te.Protocol):
    def __new__(cls) -> Empty:
        ...

    def __and__(
            self,
            other: _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]
    ) -> Empty:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty) -> Empty:
        ...

    @_t.overload
    def __or__(self, other: Multipolygon[Scalar]) -> Multipolygon[Scalar]:
        ...

    @_t.overload
    def __or__(self, other: Polygon[Scalar]) -> Polygon[Scalar]:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...

    def __sub__(
            self,
            other: _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]
    ) -> Empty:
        ...

    @_t.overload
    def __xor__(self, other: Empty) -> Empty:
        ...

    @_t.overload
    def __xor__(self, other: Multipolygon[Scalar]) -> Multipolygon[Scalar]:
        ...

    @_t.overload
    def __xor__(self, other: Polygon[Scalar]) -> Polygon[Scalar]:
        ...


class Box(_SelfComparable, _te.Protocol[Scalar]):
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

    def covers(self, other: Box[Scalar]) -> bool:
        ...

    def disjoint_with(self, other: Box[Scalar]) -> bool:
        ...

    def enclosed_by(self, other: Box[Scalar]) -> bool:
        ...

    def encloses(self, other: Box[Scalar]) -> bool:
        ...

    def equals_to(self, other: Box[Scalar]) -> bool:
        ...

    def is_valid(self) -> bool:
        ...

    def overlaps(self, other: Box[Scalar]) -> bool:
        ...

    def relate_to(self, other: Box[Scalar]) -> _Relation:
        ...

    def touches(self, other: Box[Scalar]) -> bool:
        ...

    def within(self, other: Box[Scalar]) -> bool:
        ...

    def __new__(cls,
                min_x: Scalar,
                max_x: Scalar,
                min_y: Scalar,
                max_y: Scalar) -> Box[Scalar]:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Segment(_SelfComparable, _te.Protocol[Scalar]):
    @property
    def end(self) -> Point[Scalar]:
        ...

    @property
    def start(self) -> Point[Scalar]:
        ...

    def __new__(cls,
                start: Point[Scalar],
                end: Point[Scalar]) -> Segment[Scalar]:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Contour(_SelfComparable, _te.Protocol[Scalar]):
    @property
    def bounding_box(self) -> Box[Scalar]:
        ...

    @property
    def orientation(self) -> _Orientation:
        ...

    @property
    def segments(self) -> _t.Sequence[Segment[Scalar]]:
        ...

    @property
    def segments_count(self) -> int:
        ...

    @property
    def vertices(self) -> _t.Sequence[Point[Scalar]]:
        ...

    @property
    def vertices_count(self) -> int:
        ...

    def is_valid(self) -> bool:
        ...

    def __new__(cls, vertices: _t.Sequence[Point[Scalar]]) -> Contour[Scalar]:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


_Segmental = _t.TypeVar(
        '_Segmental',
                      covariant=True)


class Multisegmental(_te.Protocol[_Segmental]):
    @property
    def segments(self) -> _t.Sequence[_Segmental]:
        ...

    @property
    def segments_count(self) -> int:
        ...


class Multisegment(_SelfComparable, Multisegmental[Segment[Scalar]],
                   _te.Protocol[Scalar]):
    def is_valid(self) -> bool:
        ...

    def __new__(
            cls, segments: _t.Sequence[Segment[Scalar]]
    ) -> Multisegment[Scalar]:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Polygon(_SelfComparable, Multisegmental[Segment[Scalar]],
              _te.Protocol[Scalar]):
    @property
    def border(self) -> Contour[Scalar]:
        ...

    @property
    def bounding_box(self) -> Box[Scalar]:
        ...

    @property
    def holes(self) -> _t.Sequence[Contour[Scalar]]:
        ...

    @property
    def holes_count(self) -> int:
        ...

    def __new__(cls,
                border: Contour[Scalar],
                holes: _t.Sequence[Contour[Scalar]]) -> Polygon[Scalar]:
        ...

    def __and__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty) -> Polygon[Scalar]:
        ...

    @_t.overload
    def __or__(
            self, other: Multipolygon[Scalar]
    ) -> _t.Union[Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __or__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...

    @_t.overload
    def __sub__(self, other: Empty) -> Polygon[Scalar]:
        ...

    @_t.overload
    def __sub__(
            self, other: Multipolygon[Scalar]
    ) -> _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __sub__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __xor__(self, other: Empty) -> Polygon[Scalar]:
        ...

    @_t.overload
    def __xor__(
            self, other: Multipolygon[Scalar]
    ) -> _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __xor__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]:
        ...


class Multipolygon(_SelfComparable, Multisegmental[Segment[Scalar]],
                   _te.Protocol[Scalar]):
    @property
    def polygons(self) -> _t.Sequence[Polygon[Scalar]]:
        ...

    @property
    def polygons_count(self) -> int:
        ...

    def __new__(
            cls, vertices: _t.Sequence[Polygon[Scalar]]
    ) -> Multipolygon[Scalar]:
        ...

    @_t.overload
    def __and__(self, other: Empty) -> Empty:
        ...

    @_t.overload
    def __and__(
            self, other: Multipolygon[Scalar]
    ) -> _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __and__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty) -> Polygon[Scalar]:
        ...

    @_t.overload
    def __or__(
            self, other: Multipolygon[Scalar]
    ) -> _t.Union[Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __or__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    def __repr__(self) -> str:
        ...

    @_t.overload
    def __sub__(self, other: Empty) -> Multipolygon[Scalar]:
        ...

    @_t.overload
    def __sub__(
            self, other: Multipolygon[Scalar]
    ) -> _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __sub__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    def __str__(self) -> str:
        ...

    @_t.overload
    def __xor__(self, other: Empty) -> Multipolygon[Scalar]:
        ...

    @_t.overload
    def __xor__(
            self, other: Multipolygon[Scalar]
    ) -> _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __xor__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Empty, Multipolygon[Scalar], Polygon[Scalar]]:
        ...
