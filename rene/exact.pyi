from __future__ import annotations

import typing as _t
from numbers import Rational as _Rational

import typing_extensions as _te
from rithm.fraction import Fraction as _Fraction

from rene import (Location as _Location,
                  Orientation as _Orientation,
                  Relation as _Relation)
from rene.hints import Seeder as _Seeder

_ScalarT = _t.Union[_Fraction, _Rational, float, int]


class Box:
    @property
    def max_x(self) -> _Fraction:
        ...

    @property
    def max_y(self) -> _Fraction:
        ...

    @property
    def min_x(self) -> _Fraction:
        ...

    @property
    def min_y(self) -> _Fraction:
        ...

    def covers(self, other: _te.Self, /) -> bool:
        ...

    def disjoint_with(self, other: _te.Self, /) -> bool:
        ...

    def enclosed_by(self, other: _te.Self, /) -> bool:
        ...

    def encloses(self, other: _te.Self, /) -> bool:
        ...

    def equals_to(self, other: _te.Self, /) -> bool:
        ...

    def is_valid(self) -> bool:
        ...

    def overlaps(self, other: _te.Self, /) -> bool:
        ...

    def relate_to(self, other: _te.Self, /) -> _Relation:
        ...

    def touches(self, other: _te.Self, /) -> bool:
        ...

    def within(self, other: _te.Self, /) -> bool:
        ...

    def __new__(cls,
                min_x: _ScalarT,
                max_x: _ScalarT,
                min_y: _ScalarT,
                max_y: _ScalarT,
                /) -> _te.Self:
        ...

    @_t.overload
    def __eq__(self, other: _te.Self, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any, /) -> _t.Any:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Contour:
    @property
    def bounding_box(self) -> Box:
        ...

    @property
    def orientation(self) -> _Orientation:
        ...

    @property
    def segments(self) -> _t.Sequence[Segment]:
        ...

    @property
    def vertices(self) -> _t.Sequence[Point]:
        ...

    def is_valid(self) -> bool:
        ...

    def locate(self, point: Point, /) -> _Location:
        ...

    def relate_to(self, other: _Compound, /) -> _Relation:
        ...

    def __new__(cls, vertices: _t.Sequence[Point], /) -> _te.Self:
        ...

    @_t.overload
    def __and__(self, other: Empty, /) -> Empty:
        ...

    @_t.overload
    def __and__(
            self,
            other: _t.Union[
                Multipolygon, Multisegment, Polygon, Segment, _te.Self
            ],
            /
    ) -> _t.Union[Empty, Multisegment, Segment]:
        ...

    def __contains__(self, point: Point, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _te.Self, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any, /) -> _t.Any:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __or__(
            self, other: _t.Union[Multisegment, Segment, _te.Self], /
    ) -> _t.Union[Multisegment, Segment]:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...

    @_t.overload
    def __sub__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __sub__(
            self, other: _t.Union[Multisegment, Segment, _te.Self], /
    ) -> _t.Union[Empty, Multisegment, Segment]:
        ...

    @_t.overload
    def __xor__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __xor__(
            self, other: _t.Union[Multisegment, Segment, _te.Self], /
    ) -> _t.Union[Empty, Multisegment, Segment]:
        ...


class Empty:
    def locate(self, point: Point, /) -> _Location:
        ...

    def relate_to(self, other: _Compound, /) -> _Relation:
        ...

    def __new__(cls) -> _te.Self:
        ...

    def __and__(
            self,
            other: _t.Union[
                Contour, Multipolygon, Multisegment, Polygon, Segment, _te.Self
            ],
            /
    ) -> _te.Self:
        ...

    def __contains__(self, point: Point, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _te.Self, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any, /) -> _t.Any:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: _te.Self, /) -> _te.Self:
        ...

    @_t.overload
    def __or__(self, other: Contour, /) -> Contour:
        ...

    @_t.overload
    def __or__(self, other: Multipolygon, /) -> Multipolygon:
        ...

    @_t.overload
    def __or__(self, other: Multisegment, /) -> Multisegment:
        ...

    @_t.overload
    def __or__(self, other: Polygon, /) -> Polygon:
        ...

    @_t.overload
    def __or__(self, other: Segment, /) -> Segment:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...

    def __sub__(
            self,
            other: _t.Union[
                Contour, Multipolygon, Multisegment, Polygon, Segment, _te.Self
            ],
            /
    ) -> _te.Self:
        ...

    @_t.overload
    def __xor__(self, other: _te.Self, /) -> _te.Self:
        ...

    @_t.overload
    def __xor__(self, other: Contour, /) -> Contour:
        ...

    @_t.overload
    def __xor__(self, other: Multipolygon, /) -> Multipolygon:
        ...

    @_t.overload
    def __xor__(self, other: Multisegment, /) -> Multisegment:
        ...

    @_t.overload
    def __xor__(self, other: Polygon, /) -> Polygon:
        ...

    @_t.overload
    def __xor__(self, other: Segment, /) -> Segment:
        ...


class Multipolygon:
    @property
    def bounding_box(self) -> Box:
        ...

    @property
    def polygons(self) -> _t.Sequence[Polygon]:
        ...

    def locate(self, point: Point, /) -> _Location:
        ...

    def relate_to(self, other: _Compound, /) -> _Relation:
        ...

    def __new__(cls, polygons: _t.Sequence[Polygon], /) -> _te.Self:
        ...

    @_t.overload
    def __and__(self, other: Empty, /) -> Empty:
        ...

    @_t.overload
    def __and__(
            self, other: _t.Union[Polygon, _te.Self], /
    ) -> _t.Union[Empty, Polygon, _te.Self]:
        ...

    @_t.overload
    def __and__(
            self, other: _t.Union[Contour, Multisegment, Segment], /
    ) -> _t.Union[Empty, Multisegment, Segment]:
        ...

    def __contains__(self, point: Point, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _te.Self, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any, /) -> _t.Any:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __or__(
            self, other: _t.Union[Polygon, _te.Self], /
    ) -> _t.Union[Polygon, _te.Self]:
        ...

    def __repr__(self) -> str:
        ...

    @_t.overload
    def __sub__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __sub__(
            self, other: _t.Union[Polygon, _te.Self], /
    ) -> _t.Union[Empty, Polygon, _te.Self]:
        ...

    def __str__(self) -> str:
        ...

    @_t.overload
    def __xor__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __xor__(
            self, other: _t.Union[Polygon, _te.Self], /
    ) -> _t.Union[Empty, Polygon, _te.Self]:
        ...


class Multisegment:
    @property
    def bounding_box(self) -> Box:
        ...

    @property
    def segments(self) -> _t.Sequence[Segment]:
        ...

    def is_valid(self) -> bool:
        ...

    def locate(self, point: Point, /) -> _Location:
        ...

    def relate_to(self, other: _Compound, /) -> _Relation:
        ...

    def __new__(cls, segments: _t.Sequence[Segment], /) -> _te.Self:
        ...

    @_t.overload
    def __and__(self, other: Empty, /) -> Empty:
        ...

    @_t.overload
    def __and__(
            self,
            other: _t.Union[Contour, Multipolygon, Polygon, Segment, _te.Self],
            /
    ) -> _t.Union[Empty, Segment, _te.Self]:
        ...

    def __contains__(self, point: Point, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _te.Self, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any, /) -> _t.Any:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __or__(
            self, other: _t.Union[Contour, Segment, _te.Self], /
    ) -> _t.Union[Segment, _te.Self]:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...

    @_t.overload
    def __sub__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __sub__(
            self, other: _t.Union[Contour, Segment, _te.Self], /
    ) -> _t.Union[Empty, Segment, _te.Self]:
        ...

    @_t.overload
    def __xor__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __xor__(
            self, other: _t.Union[Contour, Segment, _te.Self], /
    ) -> _t.Union[Empty, Segment, _te.Self]:
        ...


class Point:
    @property
    def x(self) -> _Fraction:
        ...

    @property
    def y(self) -> _Fraction:
        ...

    def __new__(cls, x: _ScalarT, y: _ScalarT, /) -> _te.Self:
        ...

    @_t.overload
    def __eq__(self, other: _te.Self, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any, /) -> _t.Any:
        ...

    def __ge__(self, other: _te.Self, /) -> bool:
        ...

    def __gt__(self, other: _te.Self, /) -> bool:
        ...

    def __hash__(self) -> int:
        ...

    def __le__(self, other: _te.Self, /) -> bool:
        ...

    def __lt__(self, other: _te.Self, /) -> bool:
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
    def bounding_box(self) -> Box:
        ...

    @property
    def holes(self) -> _t.Sequence[Contour]:
        ...

    def locate(self, point: Point, /) -> _Location:
        ...

    def relate_to(self, other: _Compound, /) -> _Relation:
        ...

    def __new__(
            cls, border: Contour, holes: _t.Sequence[Contour], /
    ) -> _te.Self:
        ...

    @_t.overload
    def __and__(self, other: Empty, /) -> Empty:
        ...

    @_t.overload
    def __and__(
            self, other: _t.Union[Multipolygon, _te.Self], /
    ) -> _t.Union[Empty, Multipolygon, _te.Self]:
        ...

    @_t.overload
    def __and__(
            self, other: _t.Union[Contour, Multisegment, Segment], /
    ) -> _t.Union[Empty, Multisegment, Segment]:
        ...

    def __contains__(self, point: Point, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _te.Self, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any, /) -> _t.Any:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __or__(
            self, other: _t.Union[Multipolygon, _te.Self], /
    ) -> _t.Union[Multipolygon, _te.Self]:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...

    @_t.overload
    def __sub__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __sub__(
            self, other: _t.Union[Multipolygon, _te.Self], /
    ) -> _t.Union[Empty, Multipolygon, _te.Self]:
        ...

    @_t.overload
    def __xor__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __xor__(
            self, other: _t.Union[Multipolygon, _te.Self], /
    ) -> _t.Union[Empty, Multipolygon, _te.Self]:
        ...


class Segment:
    @property
    def bounding_box(self) -> Box:
        ...

    @property
    def end(self) -> Point:
        ...

    @property
    def start(self) -> Point:
        ...

    def locate(self, point: Point, /) -> _Location:
        ...

    def relate_to(self, other: _Compound, /) -> _Relation:
        ...

    def __new__(cls, start: Point, end: Point, /) -> _te.Self:
        ...

    @_t.overload
    def __and__(self, other: Empty, /) -> Empty:
        ...

    @_t.overload
    def __and__(
            self,
            other: _t.Union[Contour, Multipolygon, Multisegment, Polygon],
            /
    ) -> _t.Union[Empty, Multisegment, _te.Self]:
        ...

    @_t.overload
    def __and__(self, other: _te.Self, /) -> _t.Union[Empty, _te.Self]:
        ...

    def __contains__(self, point: Point, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _te.Self, /) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any, /) -> _t.Any:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __or__(
            self, other: _t.Union[Contour, Multisegment, _te.Self], /
    ) -> _t.Union[Multisegment, _te.Self]:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...

    @_t.overload
    def __sub__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __sub__(
            self, other: _t.Union[Contour, Multisegment, _te.Self], /
    ) -> _t.Union[Empty, Multisegment, _te.Self]:
        ...

    @_t.overload
    def __xor__(self, other: Empty, /) -> _te.Self:
        ...

    @_t.overload
    def __xor__(
            self, other: _t.Union[Contour, Multisegment, _te.Self], /
    ) -> _t.Union[Empty, Multisegment, _te.Self]:
        ...


@_te.final
class ConstrainedDelaunayTriangulation:
    @classmethod
    def from_polygon(cls, polygon: Polygon, /) -> _te.Self:
        ...

    @property
    def border(self) -> Contour:
        ...

    @property
    def triangles(self) -> _t.Sequence[Contour]:
        ...

    def __bool__(self) -> bool:
        ...


@_te.final
class DelaunayTriangulation:
    @classmethod
    def from_points(cls, points: _t.Sequence[Point], /) -> _te.Self:
        ...

    @property
    def border(self) -> Contour:
        ...

    @property
    def triangles(self) -> _t.Sequence[Contour]:
        ...

    def __bool__(self) -> bool:
        ...


@_te.final
class Trapezoidation:
    @classmethod
    def from_multisegment(cls,
                          multisegment: Multisegment,
                          /,
                          *,
                          seeder: _Seeder = ...) -> _te.Self:
        ...

    @classmethod
    def from_polygon(cls,
                     polygon: Polygon,
                     /,
                     *,
                     seeder: _t.Optional[_Seeder] = None) -> _te.Self:
        ...

    @property
    def height(self) -> int:
        ...

    def locate(self, point: Point, /) -> _Location:
        ...

    def __contains__(self, point: Point, /) -> bool:
        ...


_Compound = _t.Union[
    Contour, Empty, Multisegment, Multipolygon, Polygon, Segment
]
