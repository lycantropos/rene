from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Sequence
    from numbers import Rational as _Rational
    from typing import Any, Union, overload

    from rithm.fraction import Fraction as _Fraction
    from typing_extensions import Self, final

    from rene import (
        Location as _Location,
        Orientation as _Orientation,
        Relation as _Relation,
    )
    from rene.hints import Seeder as _Seeder

    _ScalarT = Union[_Fraction, _Rational, float, int]

    class Box:
        @property
        def max_x(self, /) -> _Fraction: ...

        @property
        def max_y(self, /) -> _Fraction: ...

        @property
        def min_x(self, /) -> _Fraction: ...

        @property
        def min_y(self, /) -> _Fraction: ...

        def covers(self, other: Self, /) -> bool: ...

        def disjoint_with(self, other: Self, /) -> bool: ...

        def enclosed_by(self, other: Self, /) -> bool: ...

        def encloses(self, other: Self, /) -> bool: ...

        def equals_to(self, other: Self, /) -> bool: ...

        def is_valid(self) -> bool: ...

        def overlaps(self, other: Self, /) -> bool: ...

        def relate_to(self, other: Self, /) -> _Relation: ...

        def touches(self, other: Self, /) -> bool: ...

        def within(self, other: Self, /) -> bool: ...

        def __new__(
            cls,
            min_x: _ScalarT,
            max_x: _ScalarT,
            min_y: _ScalarT,
            max_y: _ScalarT,
            /,
        ) -> Self: ...

        @overload
        def __eq__(self, other: Self, /) -> bool: ...

        @overload
        def __eq__(self, other: Any, /) -> Any: ...

        def __eq__(self, other: Any, /) -> Any: ...

        def __hash__(self, /) -> int: ...

        def __repr__(self, /) -> str: ...

        def __str__(self, /) -> str: ...

    class Contour:
        @property
        def bounding_box(self, /) -> Box: ...

        @property
        def orientation(self, /) -> _Orientation: ...

        @property
        def segments(self, /) -> Sequence[Segment]: ...

        @property
        def vertices(self, /) -> Sequence[Point]: ...

        def is_valid(self) -> bool: ...

        def locate(self, point: Point, /) -> _Location: ...

        def relate_to(self, other: _Compound, /) -> _Relation: ...

        def __new__(cls, vertices: Sequence[Point], /) -> Self: ...

        @overload
        def __and__(self, other: Empty, /) -> Empty: ...

        @overload
        def __and__(
            self,
            other: Multipolygon | Multisegment | Polygon | Segment | Self,
            /,
        ) -> Empty | Multisegment | Segment: ...

        def __and__(
            self,
            other: (
                Empty | Multipolygon | Multisegment | Polygon | Segment | Self
            ),
            /,
        ) -> Empty | Multisegment | Segment: ...

        def __contains__(self, point: Point, /) -> bool: ...

        @overload
        def __eq__(self, other: Self, /) -> bool: ...

        @overload
        def __eq__(self, other: Any, /) -> Any: ...

        def __eq__(self, other: Any, /) -> Any: ...

        def __hash__(self, /) -> int: ...

        @overload
        def __or__(self, other: Empty, /) -> Self: ...

        @overload
        def __or__(
            self, other: Multisegment | Segment | Self, /
        ) -> Multisegment | Segment: ...

        def __or__(
            self, other: Empty | Multisegment | Segment | Self, /
        ) -> Multisegment | Segment | Self: ...

        def __repr__(self, /) -> str: ...

        def __str__(self, /) -> str: ...

        @overload
        def __sub__(self, other: Empty, /) -> Self: ...

        @overload
        def __sub__(
            self, other: Multisegment | Segment | Self, /
        ) -> Empty | Multisegment | Segment: ...

        def __sub__(
            self, other: Empty | Multisegment | Segment | Self, /
        ) -> Empty | Multisegment | Segment | Self: ...

        @overload
        def __xor__(self, other: Empty, /) -> Self: ...

        @overload
        def __xor__(
            self, other: Multisegment | Segment | Self, /
        ) -> Empty | Multisegment | Segment: ...

        def __xor__(
            self, other: Empty | Multisegment | Segment | Self, /
        ) -> Empty | Multisegment | Segment | Self: ...

    class Empty:
        def locate(self, point: Point, /) -> _Location: ...

        def relate_to(self, other: _Compound, /) -> _Relation: ...

        def __new__(cls) -> Self: ...

        def __and__(
            self,
            other: (
                Contour
                | Multipolygon
                | Multisegment
                | Polygon
                | Segment
                | Self
            ),
            /,
        ) -> Self: ...

        def __contains__(self, point: Point, /) -> bool: ...

        @overload
        def __eq__(self, other: Self, /) -> bool: ...

        @overload
        def __eq__(self, other: Any, /) -> Any: ...

        def __eq__(self, other: Any, /) -> Any: ...

        def __hash__(self, /) -> int: ...

        @overload
        def __or__(self, other: Self, /) -> Self: ...

        @overload
        def __or__(self, other: Contour, /) -> Contour: ...

        @overload
        def __or__(self, other: Multipolygon, /) -> Multipolygon: ...

        @overload
        def __or__(self, other: Multisegment, /) -> Multisegment: ...

        @overload
        def __or__(self, other: Polygon, /) -> Polygon: ...

        @overload
        def __or__(self, other: Segment, /) -> Segment: ...

        def __or__(
            self,
            other: (
                Contour
                | Multipolygon
                | Multisegment
                | Polygon
                | Segment
                | Self
            ),
            /,
        ) -> (
            Contour | Multipolygon | Multisegment | Polygon | Segment | Self
        ): ...

        def __repr__(self, /) -> str: ...

        def __str__(self, /) -> str: ...

        def __sub__(
            self,
            other: (
                Contour
                | Multipolygon
                | Multisegment
                | Polygon
                | Segment
                | Self
            ),
            /,
        ) -> Self: ...

        @overload
        def __xor__(self, other: Self, /) -> Self: ...

        @overload
        def __xor__(self, other: Contour, /) -> Contour: ...

        @overload
        def __xor__(self, other: Multipolygon, /) -> Multipolygon: ...

        @overload
        def __xor__(self, other: Multisegment, /) -> Multisegment: ...

        @overload
        def __xor__(self, other: Polygon, /) -> Polygon: ...

        @overload
        def __xor__(self, other: Segment, /) -> Segment: ...

        def __xor__(
            self,
            other: (
                Contour
                | Multipolygon
                | Multisegment
                | Polygon
                | Segment
                | Self
            ),
            /,
        ) -> (
            Contour | Multipolygon | Multisegment | Polygon | Segment | Self
        ): ...

    class Multipolygon:
        @property
        def bounding_box(self, /) -> Box: ...

        @property
        def polygons(self, /) -> Sequence[Polygon]: ...

        def locate(self, point: Point, /) -> _Location: ...

        def relate_to(self, other: _Compound, /) -> _Relation: ...

        def __new__(cls, polygons: Sequence[Polygon], /) -> Self: ...

        @overload
        def __and__(self, other: Empty, /) -> Empty: ...

        @overload
        def __and__(
            self, other: Polygon | Self, /
        ) -> Empty | Polygon | Self: ...

        @overload
        def __and__(
            self, other: Contour | Multisegment | Segment, /
        ) -> Empty | Multisegment | Segment: ...

        def __and__(
            self,
            other: Contour | Empty | Multisegment | Polygon | Segment | Self,
            /,
        ) -> Empty | Multisegment | Polygon | Segment | Self: ...

        def __contains__(self, point: Point, /) -> bool: ...

        @overload
        def __eq__(self, other: Self, /) -> bool: ...

        @overload
        def __eq__(self, other: Any, /) -> Any: ...

        def __eq__(self, other: Any, /) -> Any: ...

        def __hash__(self, /) -> int: ...

        @overload
        def __or__(self, other: Empty, /) -> Self: ...

        @overload
        def __or__(self, other: Polygon | Self, /) -> Polygon | Self: ...

        def __or__(
            self, other: Empty | Polygon | Self, /
        ) -> Polygon | Self: ...

        def __repr__(self, /) -> str: ...

        @overload
        def __sub__(self, other: Empty, /) -> Self: ...

        @overload
        def __sub__(
            self, other: Polygon | Self, /
        ) -> Empty | Polygon | Self: ...

        def __sub__(
            self, other: Empty | Polygon | Self, /
        ) -> Empty | Polygon | Self: ...

        def __str__(self, /) -> str: ...

        @overload
        def __xor__(self, other: Empty, /) -> Self: ...

        @overload
        def __xor__(
            self, other: Polygon | Self, /
        ) -> Empty | Polygon | Self: ...

        def __xor__(
            self, other: Empty | Polygon | Self, /
        ) -> Empty | Polygon | Self: ...

    class Multisegment:
        @property
        def bounding_box(self, /) -> Box: ...

        @property
        def segments(self, /) -> Sequence[Segment]: ...

        def is_valid(self) -> bool: ...

        def locate(self, point: Point, /) -> _Location: ...

        def relate_to(self, other: _Compound, /) -> _Relation: ...

        def __new__(cls, segments: Sequence[Segment], /) -> Self: ...

        @overload
        def __and__(self, other: Empty, /) -> Empty: ...

        @overload
        def __and__(
            self, other: Contour | Multipolygon | Polygon | Segment | Self, /
        ) -> Empty | Segment | Self: ...

        def __and__(
            self,
            other: Contour | Empty | Multipolygon | Polygon | Segment | Self,
            /,
        ) -> Empty | Segment | Self: ...

        def __contains__(self, point: Point, /) -> bool: ...

        @overload
        def __eq__(self, other: Self, /) -> bool: ...

        @overload
        def __eq__(self, other: Any, /) -> Any: ...

        def __eq__(self, other: Any, /) -> Any: ...

        def __hash__(self, /) -> int: ...

        @overload
        def __or__(self, other: Empty, /) -> Self: ...

        @overload
        def __or__(
            self, other: Contour | Segment | Self, /
        ) -> Segment | Self: ...

        def __or__(
            self, other: Contour | Empty | Segment | Self, /
        ) -> Segment | Self: ...

        def __repr__(self, /) -> str: ...

        def __str__(self, /) -> str: ...

        @overload
        def __sub__(self, other: Empty, /) -> Self: ...

        @overload
        def __sub__(
            self, other: Contour | Segment | Self, /
        ) -> Empty | Segment | Self: ...

        def __sub__(
            self, other: Contour | Empty | Segment | Self, /
        ) -> Empty | Segment | Self: ...

        @overload
        def __xor__(self, other: Empty, /) -> Self: ...

        @overload
        def __xor__(
            self, other: Contour | Segment | Self, /
        ) -> Empty | Segment | Self: ...

        def __xor__(
            self, other: Contour | Empty | Segment | Self, /
        ) -> Empty | Segment | Self: ...

    class Point:
        @property
        def x(self, /) -> _Fraction: ...

        @property
        def y(self, /) -> _Fraction: ...

        def __new__(cls, x: _ScalarT, y: _ScalarT, /) -> Self: ...

        @overload
        def __eq__(self, other: Self, /) -> bool: ...

        @overload
        def __eq__(self, other: Any, /) -> Any: ...

        def __eq__(self, other: Any, /) -> Any: ...

        def __ge__(self, other: Self, /) -> bool: ...

        def __gt__(self, other: Self, /) -> bool: ...

        def __hash__(self, /) -> int: ...

        def __le__(self, other: Self, /) -> bool: ...

        def __lt__(self, other: Self, /) -> bool: ...

        def __repr__(self, /) -> str: ...

        def __str__(self, /) -> str: ...

    class Polygon:
        @property
        def border(self, /) -> Contour: ...

        @property
        def bounding_box(self, /) -> Box: ...

        @property
        def holes(self, /) -> Sequence[Contour]: ...

        def locate(self, point: Point, /) -> _Location: ...

        def relate_to(self, other: _Compound, /) -> _Relation: ...

        def __new__(
            cls, border: Contour, holes: Sequence[Contour], /
        ) -> Self: ...

        @overload
        def __and__(self, other: Empty, /) -> Empty: ...

        @overload
        def __and__(
            self, other: Multipolygon | Self, /
        ) -> Empty | Multipolygon | Self: ...

        @overload
        def __and__(
            self, other: Contour | Multisegment | Segment, /
        ) -> Empty | Multisegment | Segment: ...

        def __and__(
            self,
            other: (
                Contour | Empty | Multipolygon | Multisegment | Segment | Self
            ),
            /,
        ) -> Empty | Multipolygon | Multisegment | Segment | Self: ...

        def __contains__(self, point: Point, /) -> bool: ...

        @overload
        def __eq__(self, other: Self, /) -> bool: ...

        @overload
        def __eq__(self, other: Any, /) -> Any: ...

        def __eq__(self, other: Any, /) -> Any: ...

        def __hash__(self, /) -> int: ...

        @overload
        def __or__(self, other: Empty, /) -> Self: ...

        @overload
        def __or__(
            self, other: Multipolygon | Self, /
        ) -> Multipolygon | Self: ...

        def __or__(
            self, other: Empty | Multipolygon | Self, /
        ) -> Multipolygon | Self: ...

        def __repr__(self, /) -> str: ...

        def __str__(self, /) -> str: ...

        @overload
        def __sub__(self, other: Empty, /) -> Self: ...

        @overload
        def __sub__(
            self, other: Multipolygon | Self, /
        ) -> Empty | Multipolygon | Self: ...

        def __sub__(
            self, other: Empty | Multipolygon | Self, /
        ) -> Empty | Multipolygon | Self: ...

        @overload
        def __xor__(self, other: Empty, /) -> Self: ...

        @overload
        def __xor__(
            self, other: Multipolygon | Self, /
        ) -> Empty | Multipolygon | Self: ...

        def __xor__(
            self, other: Empty | Multipolygon | Self, /
        ) -> Empty | Multipolygon | Self: ...

    class Segment:
        @property
        def bounding_box(self, /) -> Box: ...

        @property
        def end(self, /) -> Point: ...

        @property
        def start(self, /) -> Point: ...

        def locate(self, point: Point, /) -> _Location: ...

        def relate_to(self, other: _Compound, /) -> _Relation: ...

        def __new__(cls, start: Point, end: Point, /) -> Self: ...

        @overload
        def __and__(self, other: Empty, /) -> Empty: ...

        @overload
        def __and__(
            self, other: Contour | Multipolygon | Multisegment | Polygon, /
        ) -> Empty | Multisegment | Self: ...

        @overload
        def __and__(self, other: Self, /) -> Empty | Self: ...

        def __and__(
            self,
            other: (
                Contour | Empty | Multipolygon | Multisegment | Polygon | Self
            ),
            /,
        ) -> Empty | Multisegment | Self: ...

        def __contains__(self, point: Point, /) -> bool: ...

        @overload
        def __eq__(self, other: Self, /) -> bool: ...

        @overload
        def __eq__(self, other: Any, /) -> Any: ...

        def __eq__(self, other: Any, /) -> Any: ...

        def __hash__(self, /) -> int: ...

        @overload
        def __or__(self, other: Empty, /) -> Self: ...

        @overload
        def __or__(
            self, other: Contour | Multisegment | Self, /
        ) -> Multisegment | Self: ...

        def __or__(
            self, other: Contour | Empty | Multisegment | Self, /
        ) -> Multisegment | Self: ...

        def __repr__(self, /) -> str: ...

        def __str__(self, /) -> str: ...

        @overload
        def __sub__(self, other: Empty, /) -> Self: ...

        @overload
        def __sub__(
            self, other: Contour | Multisegment | Self, /
        ) -> Empty | Multisegment | Self: ...

        def __sub__(
            self, other: Contour | Empty | Multisegment | Self, /
        ) -> Empty | Multisegment | Self: ...

        @overload
        def __xor__(self, other: Empty, /) -> Self: ...

        @overload
        def __xor__(
            self, other: Contour | Multisegment | Self, /
        ) -> Empty | Multisegment | Self: ...

        def __xor__(
            self, other: Contour | Empty | Multisegment | Self, /
        ) -> Empty | Multisegment | Self: ...

    @final
    class ConstrainedDelaunayTriangulation:
        @classmethod
        def from_polygon(cls, polygon: Polygon, /) -> Self: ...

        @property
        def border(self, /) -> Contour: ...

        @property
        def triangles(self, /) -> Sequence[Contour]: ...

        def __bool__(self) -> bool: ...

    @final
    class DelaunayTriangulation:
        @classmethod
        def from_points(cls, points: Sequence[Point], /) -> Self: ...

        @property
        def border(self, /) -> Contour: ...

        @property
        def triangles(self, /) -> Sequence[Contour]: ...

        def __bool__(self) -> bool: ...

    @final
    class Trapezoidation:
        @classmethod
        def from_multisegment(
            cls, multisegment: Multisegment, /, *, seeder: _Seeder = ...
        ) -> Self: ...

        @classmethod
        def from_polygon(
            cls, polygon: Polygon, /, *, seeder: _Seeder | None = None
        ) -> Self: ...

        @property
        def height(self, /) -> int: ...

        def locate(self, point: Point, /) -> _Location: ...

        def __contains__(self, point: Point, /) -> bool: ...

    _Compound = Union[
        Contour, Empty, Multisegment, Multipolygon, Polygon, Segment
    ]
else:
    try:
        from . import _cexact
    except ImportError:
        from ._exact import (
            Box,
            ConstrainedDelaunayTriangulation,
            Contour,
            DelaunayTriangulation,
            Empty,
            Multipolygon,
            Multisegment,
            Point,
            Polygon,
            Segment,
            Trapezoidation,
        )
    else:
        import random as _random
        from typing import Any, NoReturn

        from typing_extensions import Self, final

        from . import Location as _Location
        from ._utils import (
            polygon_to_segments_count as _polygon_to_segments_count,
            validate_seed as _validate_seed,
        )

        Box = _cexact.Box
        ConstrainedDelaunayTriangulation = (
            _cexact.ConstrainedDelaunayTriangulation
        )
        Contour = _cexact.Contour
        DelaunayTriangulation = _cexact.DelaunayTriangulation
        Empty = _cexact.Empty
        Multipolygon = _cexact.Multipolygon
        Multisegment = _cexact.Multisegment
        Point = _cexact.Point
        Polygon = _cexact.Polygon
        Segment = _cexact.Segment
        _RawTrapezoidation = _cexact.Trapezoidation

        @final
        class Trapezoidation:
            @classmethod
            def from_multisegment(
                cls,
                multisegment: Multisegment,
                /,
                *,
                seeder: _Seeder | None = None,
            ) -> Self:
                seed = (
                    _random.randint(0, len(multisegment.segments))
                    if seeder is None
                    else seeder()
                )
                _validate_seed(seed)
                return cls(
                    _RawTrapezoidation.from_multisegment(multisegment, seed)
                )

            @classmethod
            def from_polygon(
                cls, polygon: Polygon, /, *, seeder: _Seeder | None = None
            ) -> Self:
                seed = (
                    _random.randint(0, _polygon_to_segments_count(polygon))
                    if seeder is None
                    else seeder()
                )
                _validate_seed(seed)
                return cls(_RawTrapezoidation.from_polygon(polygon, seed))

            @property
            def height(self, /) -> int:
                return self._raw.height

            def locate(self, point: Point, /) -> _Location:
                return self._raw.locate(point)

            _raw: _RawTrapezoidation

            __slots__ = ('_raw',)

            def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
                raise TypeError(
                    f'type {cls.__qualname__!r} '
                    'is not an acceptable base type'
                )

            def __new__(cls, raw: _RawTrapezoidation, /) -> Self:
                self = super().__new__(cls)
                self._raw = raw
                return self

            def __contains__(self, point: Point, /) -> bool:
                return self._raw.__contains__(point)
