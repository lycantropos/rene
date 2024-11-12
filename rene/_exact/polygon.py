from __future__ import annotations

import enum
from collections.abc import Sequence
from typing import Any, ClassVar, NoReturn, TYPE_CHECKING, overload

from rithm.fraction import Fraction
from typing_extensions import Self, final

from rene import hints
from rene._geometries.base_polygon import BasePolygon

if TYPE_CHECKING:
    from rene._context import Context


@final
class Polygon(BasePolygon[Fraction]):
    @property
    def border(self, /) -> hints.Contour[Fraction]:
        return self._border

    @property
    def holes(self, /) -> Sequence[hints.Contour[Fraction]]:
        return _PolygonHoles(self._holes, _TOKEN)

    _context: ClassVar[Context[Fraction]]
    _border: hints.Contour[Fraction]
    _holes: Sequence[hints.Contour[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_border', '_holes'

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(
        cls,
        border: hints.Contour[Fraction],
        holes: Sequence[hints.Contour[Fraction]],
        /,
    ) -> Self:
        self = super().__new__(cls)
        self._border, self._holes = border, tuple(holes)
        return self


class _Token(enum.Enum):
    VALUE = object()


_TOKEN = _Token.VALUE


@final
class _PolygonHoles(Sequence[hints.Contour[Fraction]]):
    def count(self, contour: hints.Contour[Fraction], /) -> int:
        return self._holes.count(contour)

    def index(
        self,
        contour: hints.Contour[Fraction],
        start: int = 0,
        stop: int | None = None,
        /,
    ) -> int:
        return self._holes.index(
            contour, start, *(() if stop is None else (stop,))
        )

    _holes: Sequence[hints.Contour[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = ('_holes',)

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(
        cls, holes: Sequence[hints.Contour[Fraction]], token: _Token, /
    ) -> Self:
        if token is not _TOKEN:
            raise ValueError(
                f'{cls.__qualname__!r} is internal '
                'and its instances should not be instantiated '
                'outside of the library.'
            )
        self = super().__new__(cls)
        self._holes = holes
        return self

    @overload
    def __eq__(self, other: Self, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...

    def __eq__(self, other: Any, /) -> Any:
        return (
            self._holes == other._holes
            if isinstance(other, _PolygonHoles)
            else NotImplemented
        )

    @overload
    def __getitem__(self, item: int) -> hints.Contour[Fraction]: ...

    @overload
    def __getitem__(self, item: slice) -> Self: ...

    def __getitem__(self, item: int | slice) -> hints.Contour[Fraction] | Self:
        return (
            _PolygonHoles(self._holes[item], _TOKEN)
            if type(item) is slice
            else self._holes[item]
        )

    def __hash__(self, /) -> int:
        return hash(self._holes)

    def __len__(self) -> int:
        return len(self._holes)
