from __future__ import annotations

import enum
import typing as t

import typing_extensions as te
from rithm.fraction import Fraction

from rene import hints
from rene._context import Context
from rene._geometries.base_polygon import BasePolygon


@te.final
class Polygon(BasePolygon[Fraction]):
    @property
    def border(self) -> hints.Contour[Fraction]:
        return self._border

    @property
    def holes(self) -> t.Sequence[hints.Contour[Fraction]]:
        return _PolygonHoles(self._holes, _TOKEN)

    _context: t.ClassVar[Context[Fraction]]
    _border: hints.Contour[Fraction]
    _holes: t.Sequence[hints.Contour[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_border', '_holes'

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(cls,
                border: hints.Contour[Fraction],
                holes: t.Sequence[hints.Contour[Fraction]],
                /) -> te.Self:
        self = super().__new__(cls)
        self._border, self._holes = border, tuple(holes)
        return self


class _Token(enum.Enum):
    VALUE = object()


_TOKEN = _Token.VALUE


@te.final
class _PolygonHoles(t.Sequence[hints.Contour[Fraction]]):
    def count(self, contour: hints.Contour[Fraction], /) -> int:
        return self._holes.count(contour)

    def index(self,
              contour: hints.Contour[Fraction],
              start: int = 0,
              stop: t.Optional[int] = None,
              /) -> int:
        return self._holes.index(contour, start,
                                 *(() if stop is None else (stop,)))

    _holes: t.Sequence[hints.Contour[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_holes',

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(cls,
                holes: t.Sequence[hints.Contour[Fraction]],
                token: _Token,
                /) -> te.Self:
        if token is not _TOKEN:
            raise ValueError(f'{cls.__qualname__!r} is internal '
                             'and its instances should not be instantiated '
                             'outside of the library.')
        self = super().__new__(cls)
        self._holes = holes
        return self

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        ...

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (self._holes == other._holes
                if isinstance(other, _PolygonHoles)
                else NotImplemented)

    @t.overload
    def __getitem__(self, item: int) -> hints.Contour[Fraction]:
        ...

    @t.overload
    def __getitem__(self, item: slice) -> te.Self:
        ...

    def __getitem__(
            self, item: t.Union[int, slice]
    ) -> t.Union[hints.Contour[Fraction], te.Self]:
        return (_PolygonHoles(self._holes[item], _TOKEN)
                if type(item) is slice
                else self._holes[item])

    def __hash__(self) -> int:
        return hash(self._holes)

    def __len__(self) -> int:
        return len(self._holes)
