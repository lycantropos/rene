from __future__ import annotations

from typing import Any, Callable, TYPE_CHECKING, TypeVar, Union, overload

from typing_extensions import Protocol, Self

if TYPE_CHECKING:
    from collections.abc import Sequence

    from rene.enums import (
        Location as _Location,
        Orientation as _Orientation,
        Relation as _Relation,
    )


class _Scalar(Protocol):
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

    def __neg__(self) -> Self: ...

    def __pos__(self) -> Self: ...

    @overload
    def __sub__(self, other: int, /) -> Self: ...

    @overload
    def __sub__(self, other: Self, /) -> Self: ...

    def __truediv__(self, other: Self, /) -> Self: ...


Scalar = TypeVar('Scalar', bound=_Scalar)
Scalar_co = TypeVar('Scalar_co', bound=_Scalar, covariant=True)


class _SelfComparable(Protocol):
    @overload
    def __eq__(self, other: Self, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...


class Point(_SelfComparable, Protocol[Scalar_co]):
    @property
    def x(self, /) -> Scalar_co: ...

    @property
    def y(self, /) -> Scalar_co: ...

    def __new__(cls, x: Scalar_co, y: Scalar_co, /) -> Self: ...

    def __ge__(self, other: Self, /) -> bool: ...

    def __gt__(self, other: Self, /) -> bool: ...

    def __hash__(self, /) -> int: ...

    def __le__(self, other: Self, /) -> bool: ...

    def __lt__(self, other: Self, /) -> bool: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...


class Empty(_SelfComparable, Protocol[Scalar]):
    def locate(self, point: Point[Scalar], /) -> _Location: ...

    def relate_to(self, other: Compound[Scalar], /) -> _Relation: ...

    def __new__(cls) -> Self: ...

    def __and__(
        self,
        other: (
            Contour[Scalar]
            | Multipolygon[Scalar]
            | Multisegment[Scalar]
            | Polygon[Scalar]
            | Segment[Scalar]
            | Self
        ),
        /,
    ) -> Self: ...

    def __contains__(self, point: Point[Scalar], /) -> bool: ...

    def __hash__(self, /) -> int: ...

    @overload
    def __or__(self, other: Self, /) -> Self: ...

    @overload
    def __or__(self, other: Contour[Scalar], /) -> Contour[Scalar]: ...

    @overload
    def __or__(
        self, other: Multipolygon[Scalar], /
    ) -> Multipolygon[Scalar]: ...

    @overload
    def __or__(
        self, other: Multisegment[Scalar], /
    ) -> Multisegment[Scalar]: ...

    @overload
    def __or__(self, other: Polygon[Scalar], /) -> Polygon[Scalar]: ...

    @overload
    def __or__(self, other: Segment[Scalar], /) -> Segment[Scalar]: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...

    def __sub__(
        self,
        other: (
            Contour[Scalar]
            | Multipolygon[Scalar]
            | Multisegment[Scalar]
            | Polygon[Scalar]
            | Segment[Scalar]
            | Self
        ),
        /,
    ) -> Self: ...

    @overload
    def __xor__(self, other: Self, /) -> Self: ...

    @overload
    def __xor__(self, other: Contour[Scalar], /) -> Contour[Scalar]: ...

    @overload
    def __xor__(
        self, other: Multipolygon[Scalar], /
    ) -> Multipolygon[Scalar]: ...

    @overload
    def __xor__(
        self, other: Multisegment[Scalar], /
    ) -> Multisegment[Scalar]: ...

    @overload
    def __xor__(self, other: Polygon[Scalar], /) -> Polygon[Scalar]: ...

    @overload
    def __xor__(self, other: Segment[Scalar], /) -> Segment[Scalar]: ...


class Box(_SelfComparable, Protocol[Scalar_co]):
    @property
    def max_x(self, /) -> Scalar_co: ...

    @property
    def max_y(self, /) -> Scalar_co: ...

    @property
    def min_x(self, /) -> Scalar_co: ...

    @property
    def min_y(self, /) -> Scalar_co: ...

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
        min_x: Scalar_co,
        max_x: Scalar_co,
        min_y: Scalar_co,
        max_y: Scalar_co,
        /,
    ) -> Self: ...

    def __hash__(self, /) -> int: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...


class Segment(_SelfComparable, Protocol[Scalar]):
    @property
    def bounding_box(self, /) -> Box[Scalar]: ...

    @property
    def end(self, /) -> Point[Scalar]: ...

    @property
    def start(self, /) -> Point[Scalar]: ...

    def locate(self, point: Point[Scalar], /) -> _Location: ...

    def relate_to(self, other: Compound[Scalar], /) -> _Relation: ...

    def __new__(cls, start: Point[Scalar], end: Point[Scalar], /) -> Self: ...

    @overload
    def __and__(self, other: Empty[Scalar], /) -> Empty[Scalar]: ...

    @overload
    def __and__(
        self,
        other: (
            Contour[Scalar]
            | Multipolygon[Scalar]
            | Multisegment[Scalar]
            | Polygon[Scalar]
        ),
        /,
    ) -> Empty[Scalar] | Multisegment[Scalar] | Segment[Scalar]: ...

    @overload
    def __and__(
        self, other: Segment[Scalar], /
    ) -> Empty[Scalar] | Segment[Scalar]: ...

    def __contains__(self, point: Point[Scalar], /) -> bool: ...

    def __hash__(self, /) -> int: ...

    @overload
    def __or__(self, other: Empty[Scalar], /) -> Self: ...

    @overload
    def __or__(
        self,
        other: Contour[Scalar] | Multisegment[Scalar] | Segment[Scalar],
        /,
    ) -> Multisegment[Scalar] | Segment[Scalar]: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...

    @overload
    def __sub__(self, other: Empty[Scalar], /) -> Self: ...

    @overload
    def __sub__(
        self,
        other: Contour[Scalar] | Multisegment[Scalar] | Segment[Scalar],
        /,
    ) -> Empty[Scalar] | Multisegment[Scalar] | Segment[Scalar]: ...

    @overload
    def __xor__(self, other: Empty[Scalar], /) -> Self: ...

    @overload
    def __xor__(
        self,
        other: Contour[Scalar] | Multisegment[Scalar] | Segment[Scalar],
        /,
    ) -> Empty[Scalar] | Multisegment[Scalar] | Segment[Scalar]: ...


_Segmental_co = TypeVar('_Segmental_co', bound=Segment[Any], covariant=True)


class Multisegmental(Protocol[_Segmental_co]):
    @property
    def segments(self, /) -> Sequence[_Segmental_co]: ...


class Contour(
    _SelfComparable, Multisegmental[Segment[Scalar]], Protocol[Scalar]
):
    @property
    def bounding_box(self, /) -> Box[Scalar]: ...

    @property
    def orientation(self, /) -> _Orientation: ...

    @property
    def vertices(self, /) -> Sequence[Point[Scalar]]: ...

    def is_valid(self) -> bool: ...

    def locate(self, point: Point[Scalar], /) -> _Location: ...

    def relate_to(self, other: Compound[Scalar], /) -> _Relation: ...

    def __new__(cls, vertices: Sequence[Point[Scalar]], /) -> Self: ...

    @overload
    def __and__(self, other: Empty[Scalar], /) -> Empty[Scalar]: ...

    @overload
    def __and__(
        self,
        other: (
            Contour[Scalar]
            | Multipolygon[Scalar]
            | Multisegment[Scalar]
            | Polygon[Scalar]
            | Segment[Scalar]
        ),
        /,
    ) -> Empty[Scalar] | Multisegment[Scalar] | Segment[Scalar]: ...

    def __contains__(self, point: Point[Scalar], /) -> bool: ...

    def __hash__(self, /) -> int: ...

    @overload
    def __or__(self, other: Empty[Scalar], /) -> Self: ...

    @overload
    def __or__(
        self,
        other: Contour[Scalar] | Multisegment[Scalar] | Segment[Scalar],
        /,
    ) -> Multisegment[Scalar] | Segment[Scalar]: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...

    @overload
    def __sub__(self, other: Empty[Scalar], /) -> Self: ...

    @overload
    def __sub__(
        self,
        other: Contour[Scalar] | Multisegment[Scalar] | Segment[Scalar],
        /,
    ) -> Empty[Scalar] | Multisegment[Scalar] | Segment[Scalar]: ...

    @overload
    def __xor__(self, other: Empty[Scalar], /) -> Self: ...

    @overload
    def __xor__(
        self,
        other: Contour[Scalar] | Multisegment[Scalar] | Segment[Scalar],
        /,
    ) -> Empty[Scalar] | Multisegment[Scalar] | Segment[Scalar]: ...


class Multisegment(
    _SelfComparable, Multisegmental[Segment[Scalar]], Protocol[Scalar]
):
    @property
    def bounding_box(self, /) -> Box[Scalar]: ...

    def is_valid(self) -> bool: ...

    def locate(self, point: Point[Scalar], /) -> _Location: ...

    def relate_to(self, other: Compound[Scalar], /) -> _Relation: ...

    def __new__(cls, segments: Sequence[Segment[Scalar]], /) -> Self: ...

    @overload
    def __and__(self, other: Empty[Scalar], /) -> Empty[Scalar]: ...

    @overload
    def __and__(
        self,
        other: Contour[Scalar] | Multisegment[Scalar] | Segment[Scalar],
        /,
    ) -> Empty[Scalar] | Multisegment[Scalar] | Segment[Scalar]: ...

    def __contains__(self, point: Point[Scalar], /) -> bool: ...

    def __hash__(self, /) -> int: ...

    @overload
    def __or__(self, other: Empty[Scalar], /) -> Self: ...

    @overload
    def __or__(
        self,
        other: Contour[Scalar] | Multisegment[Scalar] | Segment[Scalar],
        /,
    ) -> Multisegment[Scalar] | Segment[Scalar]: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...

    @overload
    def __sub__(self, other: Empty[Scalar], /) -> Self: ...

    @overload
    def __sub__(
        self,
        other: Contour[Scalar] | Multisegment[Scalar] | Segment[Scalar],
        /,
    ) -> Empty[Scalar] | Multisegment[Scalar] | Segment[Scalar]: ...

    @overload
    def __xor__(self, other: Empty[Scalar], /) -> Self: ...

    @overload
    def __xor__(
        self,
        other: Contour[Scalar] | Multisegment[Scalar] | Segment[Scalar],
        /,
    ) -> Empty[Scalar] | Multisegment[Scalar] | Segment[Scalar]: ...


class Polygon(_SelfComparable, Protocol[Scalar]):
    @property
    def bounding_box(self, /) -> Box[Scalar]: ...

    @property
    def border(self, /) -> Contour[Scalar]: ...

    @property
    def holes(self, /) -> Sequence[Contour[Scalar]]: ...

    def locate(self, point: Point[Scalar], /) -> _Location: ...

    def relate_to(self, other: Compound[Scalar], /) -> _Relation: ...

    def __new__(
        cls, border: Contour[Scalar], holes: Sequence[Contour[Scalar]], /
    ) -> Self: ...

    @overload
    def __and__(self, other: Empty[Scalar], /) -> Empty[Scalar]: ...

    @overload
    def __and__(
        self, other: Multipolygon[Scalar] | Polygon[Scalar], /
    ) -> Empty[Scalar] | Multipolygon[Scalar] | Polygon[Scalar]: ...

    @overload
    def __and__(
        self,
        other: Contour[Scalar] | Multisegment[Scalar] | Segment[Scalar],
        /,
    ) -> Empty[Scalar] | Multisegment[Scalar] | Segment[Scalar]: ...

    def __contains__(self, point: Point[Scalar], /) -> bool: ...

    def __hash__(self, /) -> int: ...

    @overload
    def __or__(self, other: Empty[Scalar], /) -> Polygon[Scalar]: ...

    @overload
    def __or__(
        self, other: Multipolygon[Scalar] | Polygon[Scalar], /
    ) -> Multipolygon[Scalar] | Polygon[Scalar]: ...

    def __repr__(self, /) -> str: ...

    def __str__(self, /) -> str: ...

    @overload
    def __sub__(self, other: Empty[Scalar], /) -> Polygon[Scalar]: ...

    @overload
    def __sub__(
        self, other: Multipolygon[Scalar] | Polygon[Scalar], /
    ) -> Empty[Scalar] | Multipolygon[Scalar] | Polygon[Scalar]: ...

    @overload
    def __xor__(self, other: Empty[Scalar], /) -> Polygon[Scalar]: ...

    @overload
    def __xor__(
        self, other: Multipolygon[Scalar] | Polygon[Scalar], /
    ) -> Empty[Scalar] | Multipolygon[Scalar] | Polygon[Scalar]: ...


class Multipolygon(_SelfComparable, Protocol[Scalar]):
    @property
    def bounding_box(self, /) -> Box[Scalar]: ...

    @property
    def polygons(self, /) -> Sequence[Polygon[Scalar]]: ...

    def locate(self, point: Point[Scalar], /) -> _Location: ...

    def relate_to(self, other: Compound[Scalar], /) -> _Relation: ...

    def __new__(cls, vertices: Sequence[Polygon[Scalar]], /) -> Self: ...

    @overload
    def __and__(self, other: Empty[Scalar], /) -> Empty[Scalar]: ...

    @overload
    def __and__(
        self, other: Multipolygon[Scalar] | Polygon[Scalar], /
    ) -> Empty[Scalar] | Multipolygon[Scalar] | Polygon[Scalar]: ...

    @overload
    def __and__(
        self,
        other: Contour[Scalar] | Multisegment[Scalar] | Segment[Scalar],
        /,
    ) -> Empty[Scalar] | Multisegment[Scalar] | Segment[Scalar]: ...

    def __contains__(self, point: Point[Scalar], /) -> bool: ...

    def __hash__(self, /) -> int: ...

    @overload
    def __or__(self, other: Empty[Scalar], /) -> Multipolygon[Scalar]: ...

    @overload
    def __or__(
        self, other: Multipolygon[Scalar] | Polygon[Scalar], /
    ) -> Multipolygon[Scalar] | Polygon[Scalar]: ...

    def __repr__(self, /) -> str: ...

    @overload
    def __sub__(self, other: Empty[Scalar], /) -> Multipolygon[Scalar]: ...

    @overload
    def __sub__(
        self, other: Multipolygon[Scalar] | Polygon[Scalar], /
    ) -> Empty[Scalar] | Multipolygon[Scalar] | Polygon[Scalar]: ...

    def __str__(self, /) -> str: ...

    @overload
    def __xor__(self, other: Empty[Scalar], /) -> Multipolygon[Scalar]: ...

    @overload
    def __xor__(
        self, other: Multipolygon[Scalar] | Polygon[Scalar], /
    ) -> Empty[Scalar] | Multipolygon[Scalar] | Polygon[Scalar]: ...


Seeder = Callable[[], int]

Compound = Union[
    Contour[Scalar],
    Empty[Scalar],
    Multisegment[Scalar],
    Multipolygon[Scalar],
    Polygon[Scalar],
    Segment[Scalar],
]
