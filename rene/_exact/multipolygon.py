from __future__ import annotations

import enum
import typing as t

import typing_extensions as te
from rithm.fraction import Fraction

from rene import MIN_MULTIPOLYGON_POLYGONS_COUNT, hints
from rene._context import Context
from rene._geometries.base_multipolygon import BaseMultipolygon


@te.final
class Multipolygon(BaseMultipolygon[Fraction]):
    @property
    def polygons(self, /) -> t.Sequence[hints.Polygon[Fraction]]:
        return _MultipolygonPolygons(self._polygons, _TOKEN)

    _context: t.ClassVar[Context[Fraction]]
    _polygons: t.Sequence[hints.Polygon[Fraction]]

    __module__ = "rene.exact"
    __slots__ = ("_polygons",)

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f"type {cls.__qualname__!r} " "is not an acceptable base type")

    def __new__(cls, polygons: t.Sequence[hints.Polygon[Fraction]], /) -> te.Self:
        if len(polygons) < MIN_MULTIPOLYGON_POLYGONS_COUNT:
            raise ValueError(
                "Multipolygon should have at least "
                f"{MIN_MULTIPOLYGON_POLYGONS_COUNT} polygons, "
                f"but found {len(polygons)}."
            )
        self = super().__new__(cls)
        self._polygons = tuple(polygons)
        return self


class _Token(enum.Enum):
    VALUE = object()


_TOKEN = _Token.VALUE


@te.final
class _MultipolygonPolygons(t.Sequence[hints.Polygon[Fraction]]):
    def count(self, polygon: hints.Polygon[Fraction], /) -> int:
        return self._polygons.count(polygon)

    def index(
        self,
        polygon: hints.Polygon[Fraction],
        start: int = 0,
        stop: int | None = None,
        /,
    ) -> int:
        return self._polygons.index(polygon, start, *(() if stop is None else (stop,)))

    _polygons: t.Sequence[hints.Polygon[Fraction]]

    __module__ = "rene.exact"
    __slots__ = ("_polygons",)

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f"type {cls.__qualname__!r} " "is not an acceptable base type")

    def __new__(
        cls, polygons: t.Sequence[hints.Polygon[Fraction]], token: _Token, /
    ) -> te.Self:
        if token is not _TOKEN:
            raise ValueError(
                f"{cls.__qualname__!r} is internal "
                "and its instances should not be instantiated "
                "outside of the library."
            )
        self = super().__new__(cls)
        self._polygons = polygons
        return self

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool: ...

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any: ...

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (
            self._polygons == other._polygons
            if isinstance(other, _MultipolygonPolygons)
            else NotImplemented
        )

    @t.overload
    def __getitem__(self, item: int) -> hints.Polygon[Fraction]: ...

    @t.overload
    def __getitem__(self, item: slice) -> te.Self: ...

    def __getitem__(self, item: int | slice) -> hints.Polygon[Fraction] | te.Self:
        return (
            _MultipolygonPolygons(self._polygons[item], _TOKEN)
            if type(item) is slice
            else self._polygons[item]
        )

    def __hash__(self, /) -> int:
        return hash(self._polygons)

    def __len__(self) -> int:
        return len(self._polygons)
