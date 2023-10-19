from __future__ import annotations

import typing as t

import typing_extensions as te
from rithm.fraction import Fraction

from rene import hints
from rene._context import Context
from rene._geometries.base_segment import BaseSegment


@te.final
class Segment(BaseSegment[Fraction]):
    @property
    def end(self) -> hints.Point[Fraction]:
        return self._end

    @property
    def start(self) -> hints.Point[Fraction]:
        return self._start

    _context: t.ClassVar[Context[Fraction]]
    _end: hints.Point[Fraction]
    _start: hints.Point[Fraction]

    __module__ = 'rene.exact'
    __slots__ = '_end', '_start'

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(cls,
                start: hints.Point[Fraction],
                end: hints.Point[Fraction],
                /) -> te.Self:
        self = super().__new__(cls)
        self._end, self._start = end, start
        return self
