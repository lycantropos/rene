from __future__ import annotations

import typing as _t

from reprit.base import generate_repr
from rithm.fraction import Fraction

from rene import hints as _hints
from rene._bentley_ottmann.base import sweep
from rene._context import Context
from rene._rene import (MIN_MULTISEGMENT_SEGMENTS_COUNT,
                        Relation)


class Multisegment:
    @property
    def segments(self) -> _t.Sequence[_hints.Segment[Fraction]]:
        return self._segments[:]

    @property
    def segments_count(self) -> int:
        return len(self._segments)

    def is_valid(self) -> bool:
        return all(intersection.relation is Relation.TOUCH
                   for intersection in sweep(self._segments))

    _context: _t.ClassVar[Context[Fraction]]
    _segments: _t.Sequence[_hints.Segment[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_segments',

    def __new__(cls, segments: _t.Sequence[
        _hints.Segment[Fraction]]) -> Multisegment:
        if len(segments) < MIN_MULTISEGMENT_SEGMENTS_COUNT:
            raise ValueError('Multisegment should have at least '
                             f'{MIN_MULTISEGMENT_SEGMENTS_COUNT} segments, '
                             f'but found {len(segments)}.')
        self = super().__new__(cls)
        self._segments = list(segments)
        return self

    @_t.overload
    def __eq__(self, other: Multisegment) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __eq__(self, other: _t.Any) -> _t.Any:
        return (frozenset(self.segments) == frozenset(other.segments)
                if isinstance(other, Multisegment)
                else NotImplemented)

    def __hash__(self) -> int:
        return hash(frozenset(self.segments))

    __repr__ = generate_repr(__new__)

    def __str__(self) -> str:
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.segments))))
