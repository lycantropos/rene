from __future__ import annotations

from collections.abc import Callable
from typing import Any, TYPE_CHECKING, TypeAlias, TypeVar, overload

from typing_extensions import Protocol, Self

if TYPE_CHECKING:
    from collections.abc import Sequence

    from rene.enums import (
        Location as _Location,
        Orientation as _Orientation,
        Relation as _Relation,
    )


class Scalar(Protocol):
    @overload
    def __add__(self, other: int, /) -> Self: ...

    @overload
    def __add__(self, other: Self, /) -> Self: ...

    @overload
    def __eq__(self, other: Self, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...

    @overload
    def __ge__(self, other: Self, /) -> bool: ...

    @overload
    def __ge__(self, other: Any, /) -> Any: ...

    @overload
    def __gt__(self, other: Self, /) -> bool: ...

    @overload
    def __gt__(self, other: Any, /) -> Any: ...

    @overload
    def __le__(self, other: Self, /) -> bool: ...

    @overload
    def __le__(self, other: Any, /) -> Any: ...

    @overload
    def __lt__(self, other: Self, /) -> bool: ...

    @overload
    def __lt__(self, other: Any, /) -> Any: ...

    def __mul__(self, other: Self, /) -> Self: ...

    def __neg__(self, /) -> Self: ...

    def __pos__(self, /) -> Self: ...

    @overload
    def __sub__(self, other: int, /) -> Self: ...

    @overload
    def __sub__(self, other: Self, /) -> Self: ...

    def __truediv__(self, other: Self, /) -> Self: ...


ScalarT = TypeVar('ScalarT', bound=Scalar)
ScalarT_co = TypeVar('ScalarT_co', bound=Scalar, covariant=True)


class _SelfComparable(Protocol):
    @overload
    def __eq__(self, other: Self, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...


class Point(_SelfComparable, Protocol[ScalarT_co]):
    @property
    def x(self, /) -> ScalarT_co: ...

    @property
    def y(self, /) -> ScalarT_co: ...

    def __new__(cls, x: ScalarT_co, y: ScalarT_co, /) -> Self: ...

    def __ge__(self, other: Self, /) -> bool: ...

    def __gt__(self, other: Self, /) -> bool: ...

    def __hash__(self, /) -> int: ...

    def __le__(self, other: Self, /) -> bool: ...

    def __lt__(self, other: Self, /) -> bool: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...


class Empty(_SelfComparable, Protocol[ScalarT]):
    def locate(self, point: Point[ScalarT], /) -> _Location: ...

    def relate_to(self, other: Compound[ScalarT], /) -> _Relation: ...

    def __new__(cls, /) -> Self: ...

    def __and__(
        self,
        other: (
            Contour[ScalarT]
            | Multipolygon[ScalarT]
            | Multisegment[ScalarT]
            | Polygon[ScalarT]
            | Segment[ScalarT]
            | Self
        ),
        /,
    ) -> Self: ...

    def __contains__(self, point: Point[ScalarT], /) -> bool: ...

    def __hash__(self, /) -> int: ...

    @overload
    def __or__(self, other: Self, /) -> Self: ...

    @overload
    def __or__(self, other: Contour[ScalarT], /) -> Contour[ScalarT]: ...

    @overload
    def __or__(
        self, other: Multipolygon[ScalarT], /
    ) -> Multipolygon[ScalarT]: ...

    @overload
    def __or__(
        self, other: Multisegment[ScalarT], /
    ) -> Multisegment[ScalarT]: ...

    @overload
    def __or__(self, other: Polygon[ScalarT], /) -> Polygon[ScalarT]: ...

    @overload
    def __or__(self, other: Segment[ScalarT], /) -> Segment[ScalarT]: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...

    def __sub__(
        self,
        other: (
            Contour[ScalarT]
            | Multipolygon[ScalarT]
            | Multisegment[ScalarT]
            | Polygon[ScalarT]
            | Segment[ScalarT]
            | Self
        ),
        /,
    ) -> Self: ...

    @overload
    def __xor__(self, other: Self, /) -> Self: ...

    @overload
    def __xor__(self, other: Contour[ScalarT], /) -> Contour[ScalarT]: ...

    @overload
    def __xor__(
        self, other: Multipolygon[ScalarT], /
    ) -> Multipolygon[ScalarT]: ...

    @overload
    def __xor__(
        self, other: Multisegment[ScalarT], /
    ) -> Multisegment[ScalarT]: ...

    @overload
    def __xor__(self, other: Polygon[ScalarT], /) -> Polygon[ScalarT]: ...

    @overload
    def __xor__(self, other: Segment[ScalarT], /) -> Segment[ScalarT]: ...


class Box(_SelfComparable, Protocol[ScalarT_co]):
    @property
    def max_x(self, /) -> ScalarT_co: ...

    @property
    def max_y(self, /) -> ScalarT_co: ...

    @property
    def min_x(self, /) -> ScalarT_co: ...

    @property
    def min_y(self, /) -> ScalarT_co: ...

    def covers(self, other: Self, /) -> bool: ...

    def disjoint_with(self, other: Self, /) -> bool: ...

    def enclosed_by(self, other: Self, /) -> bool: ...

    def encloses(self, other: Self, /) -> bool: ...

    def equals_to(self, other: Self, /) -> bool: ...

    def is_valid(self, /) -> bool: ...

    def overlaps(self, other: Self, /) -> bool: ...

    def relate_to(self, other: Self, /) -> _Relation: ...

    def touches(self, other: Self, /) -> bool: ...

    def within(self, other: Self, /) -> bool: ...

    def __new__(
        cls,
        min_x: ScalarT_co,
        max_x: ScalarT_co,
        min_y: ScalarT_co,
        max_y: ScalarT_co,
        /,
    ) -> Self: ...

    def __hash__(self, /) -> int: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...


class Segment(_SelfComparable, Protocol[ScalarT]):
    @property
    def bounding_box(self, /) -> Box[ScalarT]: ...

    @property
    def end(self, /) -> Point[ScalarT]: ...

    @property
    def start(self, /) -> Point[ScalarT]: ...

    def locate(self, point: Point[ScalarT], /) -> _Location: ...

    def relate_to(self, other: Compound[ScalarT], /) -> _Relation: ...

    def __new__(
        cls, start: Point[ScalarT], end: Point[ScalarT], /
    ) -> Self: ...

    @overload
    def __and__(self, other: Empty[ScalarT], /) -> Empty[ScalarT]: ...

    @overload
    def __and__(
        self,
        other: (
            Contour[ScalarT]
            | Multipolygon[ScalarT]
            | Multisegment[ScalarT]
            | Polygon[ScalarT]
        ),
        /,
    ) -> Empty[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT]: ...

    @overload
    def __and__(
        self, other: Segment[ScalarT], /
    ) -> Empty[ScalarT] | Segment[ScalarT]: ...

    def __contains__(self, point: Point[ScalarT], /) -> bool: ...

    def __hash__(self, /) -> int: ...

    @overload
    def __or__(self, other: Empty[ScalarT], /) -> Self: ...

    @overload
    def __or__(
        self,
        other: Contour[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT],
        /,
    ) -> Multisegment[ScalarT] | Segment[ScalarT]: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...

    @overload
    def __sub__(self, other: Empty[ScalarT], /) -> Self: ...

    @overload
    def __sub__(
        self,
        other: Contour[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT],
        /,
    ) -> Empty[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT]: ...

    @overload
    def __xor__(self, other: Empty[ScalarT], /) -> Self: ...

    @overload
    def __xor__(
        self,
        other: Contour[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT],
        /,
    ) -> Empty[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT]: ...


_Segmental_co = TypeVar('_Segmental_co', bound=Segment[Any], covariant=True)


class Multisegmental(Protocol[_Segmental_co]):
    @property
    def segments(self, /) -> Sequence[_Segmental_co]: ...


class Contour(
    _SelfComparable, Multisegmental[Segment[ScalarT]], Protocol[ScalarT]
):
    @property
    def bounding_box(self, /) -> Box[ScalarT]: ...

    @property
    def orientation(self, /) -> _Orientation: ...

    @property
    def vertices(self, /) -> Sequence[Point[ScalarT]]: ...

    def is_valid(self, /) -> bool: ...

    def locate(self, point: Point[ScalarT], /) -> _Location: ...

    def relate_to(self, other: Compound[ScalarT], /) -> _Relation: ...

    def __new__(cls, vertices: Sequence[Point[ScalarT]], /) -> Self: ...

    @overload
    def __and__(self, other: Empty[ScalarT], /) -> Empty[ScalarT]: ...

    @overload
    def __and__(
        self,
        other: (
            Contour[ScalarT]
            | Multipolygon[ScalarT]
            | Multisegment[ScalarT]
            | Polygon[ScalarT]
            | Segment[ScalarT]
        ),
        /,
    ) -> Empty[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT]: ...

    def __contains__(self, point: Point[ScalarT], /) -> bool: ...

    def __hash__(self, /) -> int: ...

    @overload
    def __or__(self, other: Empty[ScalarT], /) -> Self: ...

    @overload
    def __or__(
        self,
        other: Contour[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT],
        /,
    ) -> Multisegment[ScalarT] | Segment[ScalarT]: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...

    @overload
    def __sub__(self, other: Empty[ScalarT], /) -> Self: ...

    @overload
    def __sub__(
        self,
        other: Contour[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT],
        /,
    ) -> Empty[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT]: ...

    @overload
    def __xor__(self, other: Empty[ScalarT], /) -> Self: ...

    @overload
    def __xor__(
        self,
        other: Contour[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT],
        /,
    ) -> Empty[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT]: ...


class Multisegment(
    _SelfComparable, Multisegmental[Segment[ScalarT]], Protocol[ScalarT]
):
    @property
    def bounding_box(self, /) -> Box[ScalarT]: ...

    def is_valid(self, /) -> bool: ...

    def locate(self, point: Point[ScalarT], /) -> _Location: ...

    def relate_to(self, other: Compound[ScalarT], /) -> _Relation: ...

    def __new__(cls, segments: Sequence[Segment[ScalarT]], /) -> Self: ...

    @overload
    def __and__(self, other: Empty[ScalarT], /) -> Empty[ScalarT]: ...

    @overload
    def __and__(
        self,
        other: Contour[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT],
        /,
    ) -> Empty[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT]: ...

    def __contains__(self, point: Point[ScalarT], /) -> bool: ...

    def __hash__(self, /) -> int: ...

    @overload
    def __or__(self, other: Empty[ScalarT], /) -> Self: ...

    @overload
    def __or__(
        self,
        other: Contour[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT],
        /,
    ) -> Multisegment[ScalarT] | Segment[ScalarT]: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...

    @overload
    def __sub__(self, other: Empty[ScalarT], /) -> Self: ...

    @overload
    def __sub__(
        self,
        other: Contour[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT],
        /,
    ) -> Empty[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT]: ...

    @overload
    def __xor__(self, other: Empty[ScalarT], /) -> Self: ...

    @overload
    def __xor__(
        self,
        other: Contour[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT],
        /,
    ) -> Empty[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT]: ...


class Polygon(_SelfComparable, Protocol[ScalarT]):
    @property
    def bounding_box(self, /) -> Box[ScalarT]: ...

    @property
    def border(self, /) -> Contour[ScalarT]: ...

    @property
    def holes(self, /) -> Sequence[Contour[ScalarT]]: ...

    def locate(self, point: Point[ScalarT], /) -> _Location: ...

    def relate_to(self, other: Compound[ScalarT], /) -> _Relation: ...

    def __new__(
        cls, border: Contour[ScalarT], holes: Sequence[Contour[ScalarT]], /
    ) -> Self: ...

    @overload
    def __and__(self, other: Empty[ScalarT], /) -> Empty[ScalarT]: ...

    @overload
    def __and__(
        self, other: Multipolygon[ScalarT] | Polygon[ScalarT], /
    ) -> Empty[ScalarT] | Multipolygon[ScalarT] | Polygon[ScalarT]: ...

    @overload
    def __and__(
        self,
        other: Contour[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT],
        /,
    ) -> Empty[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT]: ...

    def __contains__(self, point: Point[ScalarT], /) -> bool: ...

    def __hash__(self, /) -> int: ...

    @overload
    def __or__(self, other: Empty[ScalarT], /) -> Polygon[ScalarT]: ...

    @overload
    def __or__(
        self, other: Multipolygon[ScalarT] | Polygon[ScalarT], /
    ) -> Multipolygon[ScalarT] | Polygon[ScalarT]: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...

    @overload
    def __sub__(self, other: Empty[ScalarT], /) -> Polygon[ScalarT]: ...

    @overload
    def __sub__(
        self, other: Multipolygon[ScalarT] | Polygon[ScalarT], /
    ) -> Empty[ScalarT] | Multipolygon[ScalarT] | Polygon[ScalarT]: ...

    @overload
    def __xor__(self, other: Empty[ScalarT], /) -> Polygon[ScalarT]: ...

    @overload
    def __xor__(
        self, other: Multipolygon[ScalarT] | Polygon[ScalarT], /
    ) -> Empty[ScalarT] | Multipolygon[ScalarT] | Polygon[ScalarT]: ...


class Multipolygon(_SelfComparable, Protocol[ScalarT]):
    @property
    def bounding_box(self, /) -> Box[ScalarT]: ...

    @property
    def polygons(self, /) -> Sequence[Polygon[ScalarT]]: ...

    def locate(self, point: Point[ScalarT], /) -> _Location: ...

    def relate_to(self, other: Compound[ScalarT], /) -> _Relation: ...

    def __new__(cls, polygons: Sequence[Polygon[ScalarT]], /) -> Self: ...

    @overload
    def __and__(self, other: Empty[ScalarT], /) -> Empty[ScalarT]: ...

    @overload
    def __and__(
        self, other: Multipolygon[ScalarT] | Polygon[ScalarT], /
    ) -> Empty[ScalarT] | Multipolygon[ScalarT] | Polygon[ScalarT]: ...

    @overload
    def __and__(
        self,
        other: Contour[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT],
        /,
    ) -> Empty[ScalarT] | Multisegment[ScalarT] | Segment[ScalarT]: ...

    def __contains__(self, point: Point[ScalarT], /) -> bool: ...

    def __hash__(self, /) -> int: ...

    @overload
    def __or__(self, other: Empty[ScalarT], /) -> Multipolygon[ScalarT]: ...

    @overload
    def __or__(
        self, other: Multipolygon[ScalarT] | Polygon[ScalarT], /
    ) -> Multipolygon[ScalarT] | Polygon[ScalarT]: ...

    def __repr__(self, /) -> str: ...

    @overload
    def __sub__(self, other: Empty[ScalarT], /) -> Multipolygon[ScalarT]: ...

    @overload
    def __sub__(
        self, other: Multipolygon[ScalarT] | Polygon[ScalarT], /
    ) -> Empty[ScalarT] | Multipolygon[ScalarT] | Polygon[ScalarT]: ...

    def __str__(self, /) -> str: ...

    @overload
    def __xor__(self, other: Empty[ScalarT], /) -> Multipolygon[ScalarT]: ...

    @overload
    def __xor__(
        self, other: Multipolygon[ScalarT] | Polygon[ScalarT], /
    ) -> Empty[ScalarT] | Multipolygon[ScalarT] | Polygon[ScalarT]: ...


Seeder = Callable[[], int]

Compound: TypeAlias = (
    Contour[ScalarT]
    | Empty[ScalarT]
    | Multisegment[ScalarT]
    | Multipolygon[ScalarT]
    | Polygon[ScalarT]
    | Segment[ScalarT]
)
