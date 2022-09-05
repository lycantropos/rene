import sys
from numbers import Rational as _Rational
from typing import (Any as _Any,
                    Sequence as _Sequence,
                    Union as _Union,
                    overload as _overload)

if sys.version_info < (3, 8):
    from typing_extensions import final as _final
else:
    from typing import final as _final

from rithm import Fraction as _Fraction

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

    def covers(self, other: 'Box') -> bool:
        ...

    def disjoint_with(self, other: 'Box') -> bool:
        ...

    def enclosed_by(self, other: 'Box') -> bool:
        ...

    def encloses(self, other: 'Box') -> bool:
        ...

    def equals_to(self, other: 'Box') -> bool:
        ...

    def is_valid(self) -> bool:
        ...

    def overlaps(self, other: 'Box') -> bool:
        ...

    def relate_to(self, other: 'Box') -> _Relation:
        ...

    def touches(self, other: 'Box') -> bool:
        ...

    def within(self, other: 'Box') -> bool:
        ...

    def __new__(cls,
                min_x: _Union[_Rational, float],
                max_x: _Union[_Rational, float],
                min_y: _Union[_Rational, float],
                max_y: _Union[_Rational, float]) -> 'Box':
        ...

    @_overload
    def __eq__(self, other: 'Box') -> bool:
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


class Contour:
    @property
    def bounding_box(self) -> Box:
        ...

    @property
    def orientation(self) -> _Orientation:
        ...

    @property
    def segments(self) -> _Sequence[Segment]:
        ...

    @property
    def segments_count(self) -> int:
        ...

    @property
    def vertices(self) -> _Sequence[Point]:
        ...

    @property
    def vertices_count(self) -> int:
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


class Empty:
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


class Multipolygon:
    @property
    def polygons(self) -> _Sequence[Polygon]:
        ...

    @property
    def segments_count(self) -> int:
        ...

    def __new__(cls, polygons: _Sequence[Polygon]) -> 'Multipolygon':
        ...

    @_overload
    def __eq__(self, other: 'Multipolygon') -> bool:
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

    @property
    def segments_count(self) -> int:
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

    def __new__(cls,
                x: _Union[_Rational, float],
                y: _Union[_Rational, float]) -> 'Point':
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
    def bounding_box(self) -> Box:
        ...

    @property
    def holes(self) -> _Sequence[Contour]:
        ...

    @property
    def segments(self) -> _Sequence[Segment]:
        ...

    @property
    def segments_count(self) -> int:
        ...

    def __new__(cls, border: Contour, holes: _Sequence[Contour]) -> 'Polygon':
        ...

    def __and__(self,
                other: 'Polygon') -> _Union[Empty, Multipolygon, 'Polygon']:
        ...

    @_overload
    def __eq__(self, other: 'Polygon') -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...

    def __hash__(self) -> int:
        ...

    def __or__(self, other: 'Polygon') -> _Union[Multipolygon, 'Polygon']:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...

    def __sub__(self,
                other: 'Polygon') -> _Union[Empty, Multipolygon, 'Polygon']:
        ...

    def __xor__(self,
                other: 'Polygon') -> _Union[Empty, Multipolygon, 'Polygon']:
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


@_final
class ConstrainedDelaunayTriangulation:
    @classmethod
    def from_polygon(cls,
                     polygon: Polygon) -> 'ConstrainedDelaunayTriangulation':
        ...

    @property
    def border(self) -> Contour:
        ...

    @property
    def triangles(self) -> _Sequence[Contour]:
        ...

    def __bool__(self) -> bool:
        ...


@_final
class DelaunayTriangulation:
    @classmethod
    def from_points(cls, points: _Sequence[Point]) -> 'DelaunayTriangulation':
        ...

    @property
    def border(self) -> Contour:
        ...

    @property
    def triangles(self) -> _Sequence[Contour]:
        ...

    def __bool__(self) -> bool:
        ...
