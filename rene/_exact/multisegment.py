from __future__ import annotations

import typing as _t

from reprit.base import generate_repr
from rithm.fraction import Fraction

from rene import (Location,
                  Relation,
                  hints as _hints)
from rene._bentley_ottmann.base import sweep
from rene._context import Context
from rene._rene import MIN_MULTISEGMENT_SEGMENTS_COUNT


class Multisegment:
    @property
    def bounding_box(self) -> _hints.Box[Fraction]:
        segments = iter(self._segments)
        first_segment = next(segments)
        min_x = min(first_segment.start.x, first_segment.end.x)
        max_x = max(first_segment.start.x, first_segment.end.x)
        min_y = min(first_segment.start.y, first_segment.end.y)
        max_y = max(first_segment.start.y, first_segment.end.y)
        for segment in segments:
            segment_max_x = max(segment.start.x, segment.end.x)
            if segment_max_x > max_x:
                max_x = segment_max_x
            segment_min_x = min(segment.start.x, segment.end.x)
            if segment_min_x < min_x:
                min_x = segment_min_x
            segment_max_y = max(segment.start.y, segment.end.y)
            if segment_max_y > max_y:
                max_y = segment_max_y
            segment_min_y = min(segment.start.y, segment.end.y)
            if segment_min_y < min_y:
                min_y = segment_min_y
        return self._context.box_cls(min_x, max_x, min_y, max_y)

    @property
    def segments(self) -> _t.Sequence[_hints.Segment[Fraction]]:
        return self._segments[:]

    @property
    def segments_count(self) -> int:
        return len(self._segments)

    def locate(self, point: _hints.Point[Fraction]) -> Location:
        for segment in self._segments:
            location = segment.locate(point)
            if location is not Location.EXTERIOR:
                return location
        return Location.EXTERIOR

    def is_valid(self) -> bool:
        return all(intersection.relation is Relation.TOUCH
                   for intersection in sweep(self._segments))

    _context: _t.ClassVar[Context[Fraction]]
    _segments: _t.Sequence[_hints.Segment[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_segments',

    def __new__(
            cls, segments: _t.Sequence[_hints.Segment[Fraction]]
    ) -> Multisegment:
        if len(segments) < MIN_MULTISEGMENT_SEGMENTS_COUNT:
            raise ValueError('Multisegment should have at least '
                             f'{MIN_MULTISEGMENT_SEGMENTS_COUNT} segments, '
                             f'but found {len(segments)}.')
        self = super().__new__(cls)
        self._segments = list(segments)
        return self

    def __contains__(self, point: _hints.Point[Fraction]) -> bool:
        return self.locate(point) is not Location.EXTERIOR

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
