"""Computational geometry."""

from __future__ import annotations

from typing import TYPE_CHECKING

__version__ = "0.1.0"

if TYPE_CHECKING:
    from typing import Any, ClassVar, overload

    from typing_extensions import Self, final

    MIN_CONTOUR_VERTICES_COUNT: int
    MIN_MULTIPOLYGON_POLYGONS_COUNT: int
    MIN_MULTISEGMENT_SEGMENTS_COUNT: int

    @final
    class Location:
        BOUNDARY: ClassVar[Self]
        EXTERIOR: ClassVar[Self]
        INTERIOR: ClassVar[Self]

        @overload
        def __eq__(self, other: Self, /) -> bool: ...

        @overload
        def __eq__(self, other: Any, /) -> Any: ...

        def __eq__(self, other: Any, /) -> Any: ...

        def __repr__(self, /) -> str: ...

        def __str__(self, /) -> str: ...

    @final
    class Orientation:
        CLOCKWISE: Self
        COLLINEAR: Self
        COUNTERCLOCKWISE: Self

        @overload
        def __eq__(self, other: Self, /) -> bool: ...

        @overload
        def __eq__(self, other: Any, /) -> Any: ...

        def __eq__(self, other: Any, /) -> Any: ...

        def __repr__(self, /) -> str: ...

        def __str__(self, /) -> str: ...

    @final
    class Relation:
        COMPONENT: ClassVar[Self]
        COMPOSITE: ClassVar[Self]
        COVER: ClassVar[Self]
        CROSS: ClassVar[Self]
        DISJOINT: ClassVar[Self]
        ENCLOSED: ClassVar[Self]
        ENCLOSES: ClassVar[Self]
        EQUAL: ClassVar[Self]
        OVERLAP: ClassVar[Self]
        TOUCH: ClassVar[Self]
        WITHIN: ClassVar[Self]

        @property
        def complement(self, /) -> Self: ...

        @overload
        def __eq__(self, other: Self, /) -> bool: ...

        @overload
        def __eq__(self, other: Any, /) -> Any: ...

        def __eq__(self, other: Any, /) -> Any: ...

        def __repr__(self, /) -> str: ...

        def __str__(self, /) -> str: ...

else:
    try:
        from . import _crene
    except ImportError:
        from ._rene import (
            Location,
            MIN_CONTOUR_VERTICES_COUNT,
            MIN_MULTIPOLYGON_POLYGONS_COUNT,
            MIN_MULTISEGMENT_SEGMENTS_COUNT,
            Orientation,
            Relation,
        )
    else:
        MIN_CONTOUR_VERTICES_COUNT = _crene.MIN_CONTOUR_VERTICES_COUNT
        MIN_MULTIPOLYGON_POLYGONS_COUNT = _crene.MIN_MULTIPOLYGON_POLYGONS_COUNT
        MIN_MULTISEGMENT_SEGMENTS_COUNT = _crene.MIN_MULTISEGMENT_SEGMENTS_COUNT
        Location = _crene.Location
        Orientation = _crene.Orientation
        Relation = _crene.Relation
