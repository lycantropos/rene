from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from typing import Any, ClassVar, overload

    from typing_extensions import Self, final

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
        from ._enums import Location, Orientation, Relation
    else:
        Location = _crene.Location
        Orientation = _crene.Orientation
        Relation = _crene.Relation
