from __future__ import annotations

import enum
from collections.abc import Sequence
from typing import Any, ClassVar, NoReturn, TYPE_CHECKING, overload

from rithm.fraction import Fraction
from typing_extensions import Self, final

from rene import hints
from rene._geometries.base_multipolygon import BaseMultipolygon
from rene.constants import MIN_MULTIPOLYGON_POLYGONS_COUNT

if TYPE_CHECKING:
    from rene._context import Context


@final
class Multipolygon(BaseMultipolygon[Fraction]):
    @property
    def polygons(self, /) -> Sequence[hints.Polygon[Fraction]]:
        return _MultipolygonPolygons(self._polygons, _TOKEN)

    _context: ClassVar[Context[Fraction]]
    _polygons: Sequence[hints.Polygon[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = ('_polygons',)

    def __init_subclass__(cls, /) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(cls, polygons: Sequence[hints.Polygon[Fraction]], /) -> Self:
        if len(polygons) < MIN_MULTIPOLYGON_POLYGONS_COUNT:
            raise ValueError(
                'Multipolygon should have at least '
                f'{MIN_MULTIPOLYGON_POLYGONS_COUNT} polygons, '
                f'but found {len(polygons)}.'
            )
        self = object.__new__(cls)
        self._polygons = tuple(polygons)
        return self


class _Token(enum.Enum):
    VALUE = object()


_TOKEN = _Token.VALUE


@final
class _MultipolygonPolygons(Sequence[hints.Polygon[Fraction]]):
    def count(self, value: Any) -> int:
        return self._polygons.count(value)

    def index(
        self, value: Any, start: int = 0, stop: int | None = None
    ) -> int:
        return self._polygons.index(
            value, start, *(() if stop is None else (stop,))
        )

    _polygons: Sequence[hints.Polygon[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = ('_polygons',)

    def __init_subclass__(cls, /) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(
        cls, polygons: Sequence[hints.Polygon[Fraction]], token: _Token, /
    ) -> Self:
        if token is not _TOKEN:
            raise ValueError(
                f'{cls.__qualname__!r} is internal '
                'and its instances should not be instantiated '
                'outside of the library.'
            )
        self = super().__new__(cls)
        self._polygons = polygons
        return self

    @overload
    def __eq__(self, other: Self, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...

    def __eq__(self, other: Any, /) -> Any:
        return (
            self._polygons == other._polygons
            if isinstance(other, _MultipolygonPolygons)
            else NotImplemented
        )

    @overload
    def __getitem__(self, item: int) -> hints.Polygon[Fraction]: ...

    @overload
    def __getitem__(self, item: slice) -> Self: ...

    def __getitem__(self, item: int | slice) -> hints.Polygon[Fraction] | Self:
        return (
            type(self)(self._polygons[item], _TOKEN)
            if type(item) is slice
            else self._polygons[item]
        )

    def __hash__(self, /) -> int:
        return hash(self._polygons)

    def __len__(self, /) -> int:
        return len(self._polygons)
