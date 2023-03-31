from __future__ import annotations

import typing as _t

import typing_extensions as _te
from reprit.base import generate_repr
from rithm.fraction import Fraction

from rene import (Location,
                  Relation)
from rene._context import Context
from rene._utils import (locate_point_in_segment,
                         relate_segments)
from rene.hints import Point


class Segment:
    _context: Context[Fraction]
    _end: Point[Fraction]
    _start: Point[Fraction]

    @property
    def end(self) -> Point[Fraction]:
        return self._end

    @property
    def start(self) -> Point[Fraction]:
        return self._start

    def locate(self, other: Point[Fraction]) -> Location:
        if isinstance(other, self._context.point_cls):
            return locate_point_in_segment(self.start, self.end, other)
        raise TypeError(f'Unsupported type: {type(other)!r}.')

    def relate_to(self, other: _te.Self) -> Relation:
        if isinstance(other, self._context.segment_cls):
            return relate_segments(self.start, self.end, other.start,
                                   other.end)
        raise TypeError(f'Unsupported type: {type(other)!r}.')

    __module__ = 'rene.exact'
    __slots__ = '_end', '_start'

    def __new__(cls, start: Point[Fraction], end: Point[Fraction]) -> _te.Self:
        self = super().__new__(cls)
        self._end, self._start = end, start
        return self

    def __hash__(self) -> int:
        return hash(frozenset((self.start, self.end)))

    @_t.overload
    def __eq__(self, other: Segment) -> bool:
        pass

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        pass

    def __eq__(self, other: _t.Any) -> _t.Any:
        return (self.start == other.start and self.end == other.end
                or self.end == other.start and self.start == other.end
                if isinstance(other, Segment)
                else NotImplemented)

    __repr__ = generate_repr(__new__)

    def __str__(self) -> str:
        return f'{type(self).__qualname__}({self.start}, {self.end})'
