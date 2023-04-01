from __future__ import annotations

import typing as _t

import typing_extensions as _te

from rene import (Location as _Location,
                  Orientation as _Orientation,
                  Relation as _Relation)


class _Scalar(_te.Protocol):
    def __add__(self, other: _te.Self) -> _te.Self:
        pass

    @_t.overload
    def __eq__(self, other: _te.Self) -> bool:
        pass

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        pass

    @_t.overload
    def __ge__(self, other: _te.Self) -> bool:
        pass

    @_t.overload
    def __ge__(self: _te.Self, other: _t.Any) -> _t.Any:
        pass

    @_t.overload
    def __gt__(self: _te.Self, other: _te.Self) -> bool:
        pass

    @_t.overload
    def __gt__(self: _te.Self, other: _t.Any) -> _t.Any:
        pass

    @_t.overload
    def __le__(self: _te.Self, other: _te.Self) -> bool:
        pass

    @_t.overload
    def __le__(self: _te.Self, other: _t.Any) -> _t.Any:
        pass

    @_t.overload
    def __lt__(self: _te.Self, other: _te.Self) -> bool:
        pass

    @_t.overload
    def __lt__(self: _te.Self, other: _t.Any) -> _t.Any:
        pass

    def __mul__(self: _te.Self, other: _te.Self) -> _te.Self:
        pass

    def __neg__(self: _te.Self) -> _te.Self:
        pass

    def __pos__(self: _te.Self) -> _te.Self:
        pass

    def __sub__(self: _te.Self, other: _te.Self) -> _te.Self:
        pass

    def __truediv__(self: _te.Self, other: _te.Self) -> _te.Self:
        pass


Scalar = _t.TypeVar('Scalar',
                    bound=_Scalar)
Scalar_co = _t.TypeVar('Scalar_co',
                       bound=_Scalar,
                       covariant=True)


class _SelfComparable(_te.Protocol):
    @_t.overload
    def __eq__(self, other: _te.Self) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...


class Point(_SelfComparable, _te.Protocol[Scalar_co]):
    @property
    def x(self) -> Scalar_co:
        ...

    @property
    def y(self) -> Scalar_co:
        ...

    def __new__(cls, x: Scalar_co, y: Scalar_co) -> _te.Self:
        ...

    def __ge__(self, other: _te.Self) -> bool:
        ...

    def __gt__(self, other: _te.Self) -> bool:
        ...

    def __hash__(self) -> int:
        ...

    def __le__(self, other: _te.Self) -> bool:
        ...

    def __lt__(self, other: _te.Self) -> bool:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Empty(_SelfComparable, _te.Protocol[Scalar]):
    def __new__(cls) -> _te.Self:
        ...

    def __and__(
            self,
            other: _t.Union[_te.Self, Multipolygon[Scalar], Polygon[Scalar]]
    ) -> _te.Self:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: _te.Self) -> _te.Self:
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
            other: _t.Union[_te.Self, Multipolygon[Scalar], Polygon[Scalar]]
    ) -> _te.Self:
        ...

    @_t.overload
    def __xor__(self, other: _te.Self) -> _te.Self:
        ...

    @_t.overload
    def __xor__(self, other: Multipolygon[Scalar]) -> Multipolygon[Scalar]:
        ...

    @_t.overload
    def __xor__(self, other: Polygon[Scalar]) -> Polygon[Scalar]:
        ...


class Box(_SelfComparable, _te.Protocol[Scalar_co]):
    @property
    def max_x(self) -> Scalar_co:
        ...

    @property
    def max_y(self) -> Scalar_co:
        ...

    @property
    def min_x(self) -> Scalar_co:
        ...

    @property
    def min_y(self) -> Scalar_co:
        ...

    def covers(self, other: _te.Self) -> bool:
        ...

    def disjoint_with(self, other: _te.Self) -> bool:
        ...

    def enclosed_by(self, other: _te.Self) -> bool:
        ...

    def encloses(self, other: _te.Self) -> bool:
        ...

    def equals_to(self, other: _te.Self) -> bool:
        ...

    def is_valid(self) -> bool:
        ...

    def overlaps(self, other: _te.Self) -> bool:
        ...

    def relate_to(self, other: _te.Self) -> _Relation:
        ...

    def touches(self, other: _te.Self) -> bool:
        ...

    def within(self, other: _te.Self) -> bool:
        ...

    def __new__(cls,
                min_x: Scalar_co,
                max_x: Scalar_co,
                min_y: Scalar_co,
                max_y: Scalar_co) -> _te.Self:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Segment(_SelfComparable, _te.Protocol[Scalar_co]):
    @property
    def end(self) -> Point[Scalar_co]:
        ...

    @property
    def start(self) -> Point[Scalar_co]:
        ...

    def locate(self, point: Point[Scalar_co]) -> _Location:
        ...

    def relate_to(self, other: _te.Self) -> _Relation:
        ...

    def __new__(cls,
                start: Point[Scalar_co],
                end: Point[Scalar_co]) -> _te.Self:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


_Segmental = _t.TypeVar('_Segmental',
                        bound=Segment[_t.Any],
                        covariant=True)


class Multisegmental(_te.Protocol[_Segmental]):
    @property
    def segments(self) -> _t.Sequence[_Segmental]:
        ...

    @property
    def segments_count(self) -> int:
        ...


class Contour(_SelfComparable, Multisegmental[Segment[Scalar_co]],
              _te.Protocol[Scalar_co]):
    @property
    def bounding_box(self) -> Box[Scalar_co]:
        ...

    @property
    def orientation(self) -> _Orientation:
        ...

    @property
    def vertices(self) -> _t.Sequence[Point[Scalar_co]]:
        ...

    @property
    def vertices_count(self) -> int:
        ...

    def is_valid(self) -> bool:
        ...

    def __new__(cls, vertices: _t.Sequence[Point[Scalar_co]]) -> _te.Self:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Multisegment(_SelfComparable, Multisegmental[Segment[Scalar_co]],
                   _te.Protocol[Scalar_co]):
    def is_valid(self) -> bool:
        ...

    def __new__(cls, segments: _t.Sequence[Segment[Scalar_co]]) -> _te.Self:
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
    ) -> _t.Union[Empty[Scalar], Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty[Scalar]) -> Polygon[Scalar]:
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
    def __sub__(self, other: Empty[Scalar]) -> Polygon[Scalar]:
        ...

    @_t.overload
    def __sub__(
            self, other: Multipolygon[Scalar]
    ) -> _t.Union[Empty[Scalar], Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __sub__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Empty[Scalar], Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __xor__(self, other: Empty[Scalar]) -> Polygon[Scalar]:
        ...

    @_t.overload
    def __xor__(
            self, other: Multipolygon[Scalar]
    ) -> _t.Union[Empty[Scalar], Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __xor__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Empty[Scalar], Multipolygon[Scalar], Polygon[Scalar]]:
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
    def __and__(self, other: Empty[Scalar]) -> Empty[Scalar]:
        ...

    @_t.overload
    def __and__(
            self, other: Multipolygon[Scalar]
    ) -> _t.Union[Empty[Scalar], Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __and__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Empty[Scalar], Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty[Scalar]) -> Multipolygon[Scalar]:
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
    def __sub__(self, other: Empty[Scalar]) -> Multipolygon[Scalar]:
        ...

    @_t.overload
    def __sub__(
            self, other: Multipolygon[Scalar]
    ) -> _t.Union[Empty[Scalar], Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __sub__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Empty[Scalar], Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    def __str__(self) -> str:
        ...

    @_t.overload
    def __xor__(self, other: Empty[Scalar]) -> Multipolygon[Scalar]:
        ...

    @_t.overload
    def __xor__(
            self, other: Multipolygon[Scalar]
    ) -> _t.Union[Empty[Scalar], Multipolygon[Scalar], Polygon[Scalar]]:
        ...

    @_t.overload
    def __xor__(
            self, other: Polygon[Scalar]
    ) -> _t.Union[Empty[Scalar], Multipolygon[Scalar], Polygon[Scalar]]:
        ...
