from __future__ import annotations

import typing as _t
from numbers import Rational as _Rational

import typing_extensions as _te
from rithm.fraction import Fraction as _Fraction

from rene import (Orientation as _Orientation,
                  Relation as _Relation)


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

    def covers(self, other: Box) -> bool:
        ...

    def disjoint_with(self, other: Box) -> bool:
        ...

    def enclosed_by(self, other: Box) -> bool:
        ...

    def encloses(self, other: Box) -> bool:
        ...

    def equals_to(self, other: Box) -> bool:
        ...

    def is_valid(self) -> bool:
        ...

    def overlaps(self, other: Box) -> bool:
        ...

    def relate_to(self, other: Box) -> _Relation:
        ...

    def touches(self, other: Box) -> bool:
        ...

    def within(self, other: Box) -> bool:
        ...

    def __new__(cls,
                min_x: _t.Union[_Rational, float],
                max_x: _t.Union[_Rational, float],
                min_y: _t.Union[_Rational, float],
                max_y: _t.Union[_Rational, float]) -> Box:
        ...

    @_t.overload
    def __eq__(self, other: Box) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
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
    def segments_count(self) -> int:
        ...

    @property
    def vertices(self) -> _t.Sequence[Point]:
        ...

    @property
    def vertices_count(self) -> int:
        ...

    def is_valid(self) -> bool:
        ...

    def __new__(cls, vertices: _t.Sequence[Point]) -> Contour:
        ...

    @_t.overload
    def __eq__(self, other: Contour) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


class Empty:
    def __new__(cls) -> Empty:
        ...

    def __and__(
            self, other: _t.Union[Empty, Multipolygon, Polygon]
    ) -> Empty:
        ...

    @_t.overload
    def __eq__(self, other: Empty) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty) -> Empty:
        ...

    @_t.overload
    def __or__(self, other: Multipolygon) -> Multipolygon:
        ...

    @_t.overload
    def __or__(self, other: Polygon) -> Polygon:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...

    def __sub__(
            self, other: _t.Union[Empty, Multipolygon, Polygon]
    ) -> Empty:
        ...

    @_t.overload
    def __xor__(self, other: Empty) -> Empty:
        ...

    @_t.overload
    def __xor__(self, other: Multipolygon) -> Multipolygon:
        ...

    @_t.overload
    def __xor__(self, other: Polygon) -> Polygon:
        ...


class Multipolygon:
    @property
    def polygons(self) -> _t.Sequence[Polygon]:
        ...

    @property
    def polygons_count(self) -> int:
        ...

    @property
    def segments(self) -> _t.Sequence[Segment]:
        ...

    @property
    def segments_count(self) -> int:
        ...

    def __new__(cls, polygons: _t.Sequence[Polygon]) -> Multipolygon:
        ...

    @_t.overload
    def __and__(self, other: Empty) -> Empty:
        ...

    @_t.overload
    def __and__(
            self, other: Multipolygon
    ) -> _t.Union[Empty, Multipolygon, Polygon]:
        ...

    @_t.overload
    def __and__(
            self, other: Polygon
    ) -> _t.Union[Empty, Multipolygon, Polygon]:
        ...

    @_t.overload
    def __eq__(self, other: Multipolygon) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty) -> Multipolygon:
        ...

    @_t.overload
    def __or__(
            self, other: Multipolygon
    ) -> _t.Union[Multipolygon, Polygon]:
        ...

    @_t.overload
    def __or__(self, other: Polygon) -> _t.Union[Multipolygon, Polygon]:
        ...

    def __repr__(self) -> str:
        ...

    @_t.overload
    def __sub__(self, other: Empty) -> Multipolygon:
        ...

    @_t.overload
    def __sub__(
            self, other: Multipolygon
    ) -> _t.Union[Empty, Multipolygon, Polygon]:
        ...

    @_t.overload
    def __sub__(self,
                other: Polygon) -> _t.Union[Empty, Multipolygon, Polygon]:
        ...

    def __str__(self) -> str:
        ...

    @_t.overload
    def __xor__(self, other: Empty) -> Multipolygon:
        ...

    @_t.overload
    def __xor__(
            self, other: Multipolygon
    ) -> _t.Union[Empty, Multipolygon, Polygon]:
        ...

    @_t.overload
    def __xor__(self,
                other: Polygon) -> _t.Union[Empty, Multipolygon, Polygon]:
        ...


class Multisegment:
    @property
    def segments(self) -> _t.Sequence[Segment]:
        ...

    @property
    def segments_count(self) -> int:
        ...

    def is_valid(self) -> bool:
        ...

    def __new__(cls, segments: _t.Sequence[Segment]) -> Multisegment:
        ...

    @_t.overload
    def __eq__(self, other: Multisegment) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
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

    def __new__(cls,
                x: _t.Union[_Rational, float],
                y: _t.Union[_Rational, float]) -> Point:
        ...

    @_t.overload
    def __eq__(self, other: Point) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __ge__(self, other: Point) -> bool:
        ...

    def __gt__(self, other: Point) -> bool:
        ...

    def __hash__(self) -> int:
        ...

    def __le__(self, other: Point) -> bool:
        ...

    def __lt__(self, other: Point) -> bool:
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

    @property
    def holes_count(self) -> int:
        ...

    @property
    def segments(self) -> _t.Sequence[Segment]:
        ...

    @property
    def segments_count(self) -> int:
        ...

    def __new__(cls,
                border: Contour,
                holes: _t.Sequence[Contour]) -> Polygon:
        ...

    def __and__(self,
                other: Polygon) -> _t.Union[Empty, Multipolygon, Polygon]:
        ...

    @_t.overload
    def __eq__(self, other: Polygon) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __hash__(self) -> int:
        ...

    @_t.overload
    def __or__(self, other: Empty) -> Polygon:
        ...

    @_t.overload
    def __or__(self,
               other: Multipolygon) -> _t.Union[Multipolygon, Polygon]:
        ...

    @_t.overload
    def __or__(self, other: Polygon) -> _t.Union[Multipolygon, Polygon]:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...

    @_t.overload
    def __sub__(self, other: Empty) -> Polygon:
        ...

    @_t.overload
    def __sub__(
            self, other: Multipolygon
    ) -> _t.Union[Empty, Multipolygon, Polygon]:
        ...

    @_t.overload
    def __sub__(self,
                other: Polygon) -> _t.Union[Empty, Multipolygon, Polygon]:
        ...

    @_t.overload
    def __xor__(self, other: Empty) -> Polygon:
        ...

    @_t.overload
    def __xor__(
            self, other: Multipolygon
    ) -> _t.Union[Empty, Multipolygon, Polygon]:
        ...

    @_t.overload
    def __xor__(self,
                other: Polygon) -> _t.Union[Empty, Multipolygon, Polygon]:
        ...


class Segment:
    @property
    def end(self) -> Point:
        ...

    @property
    def start(self) -> Point:
        ...

    def relate_to(self, other: Segment) -> _Relation:
        ...

    def __new__(cls, start: Point, end: Point) -> Segment:
        ...

    @_t.overload
    def __eq__(self, other: Segment) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __hash__(self) -> int:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...


@_te.final
class ConstrainedDelaunayTriangulation:
    @classmethod
    def from_polygon(cls,
                     polygon: Polygon) -> ConstrainedDelaunayTriangulation:
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
    def from_points(cls,
                    points: _t.Sequence[Point]) -> DelaunayTriangulation:
        ...

    @property
    def border(self) -> Contour:
        ...

    @property
    def triangles(self) -> _t.Sequence[Contour]:
        ...

    def __bool__(self) -> bool:
        ...
