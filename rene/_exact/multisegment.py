from __future__ import annotations

import typing as t

import typing_extensions as te
from reprit.base import generate_repr
from rithm.fraction import Fraction

from rene import (MIN_MULTISEGMENT_SEGMENTS_COUNT,
                  Location,
                  Relation,
                  hints)
from rene._bentley_ottmann.base import sweep
from rene._clipping.intersection import (intersect_multisegments,
                                         intersect_segment_with_segments)
from rene._context import Context
from rene._utils import collect_maybe_empty_segments


class Multisegment:
    @property
    def bounding_box(self) -> hints.Box[Fraction]:
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
    def segments(self) -> t.Sequence[hints.Segment[Fraction]]:
        return self._segments[:]

    @property
    def segments_count(self) -> int:
        return len(self._segments)

    def locate(self, point: hints.Point[Fraction], /) -> Location:
        for segment in self._segments:
            location = segment.locate(point)
            if location is not Location.EXTERIOR:
                return location
        return Location.EXTERIOR

    def is_valid(self) -> bool:
        return all(intersection.relation is Relation.TOUCH
                   for intersection in sweep(self._segments))

    _context: t.ClassVar[Context[Fraction]]
    _segments: t.Sequence[hints.Segment[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_segments',

    def __new__(
            cls, segments: t.Sequence[hints.Segment[Fraction]], /
    ) -> te.Self:
        if len(segments) < MIN_MULTISEGMENT_SEGMENTS_COUNT:
            raise ValueError('Multisegment should have at least '
                             f'{MIN_MULTISEGMENT_SEGMENTS_COUNT} segments, '
                             f'but found {len(segments)}.')
        self = super().__new__(cls)
        self._segments = list(segments)
        return self

    @t.overload
    def __and__(
            self, other: hints.Empty[Fraction], /
    ) -> hints.Empty[Fraction]:
        ...

    @t.overload
    def __and__(
            self, other: hints.Multisegment[Fraction], /
    ) -> t.Union[
        hints.Empty[Fraction], hints.Multisegment[Fraction],
        hints.Segment[Fraction]
    ]:
        ...

    @t.overload
    def __and__(
            self, other: hints.Segment[Fraction], /
    ) -> t.Union[
        hints.Empty[Fraction], hints.Multisegment[Fraction],
        hints.Segment[Fraction]
    ]:
        ...

    @t.overload
    def __and__(self, other: t.Any, /) -> t.Any:
        ...

    def __and__(self, other: t.Any, /) -> t.Any:
        return (
            collect_maybe_empty_segments(
                    intersect_multisegments(self, other),
                    self._context.empty_cls, self._context.multisegment_cls
            )
            if isinstance(other, self._context.multisegment_cls)
            else (
                collect_maybe_empty_segments(
                        intersect_segment_with_segments(other, self.segments),
                        self._context.empty_cls, self._context.multisegment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else NotImplemented)
        )

    def __contains__(self, point: hints.Point[Fraction], /) -> bool:
        return self.locate(point) is not Location.EXTERIOR

    @t.overload
    def __eq__(self, other: Multisegment, /) -> bool:
        ...

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        ...

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (frozenset(self.segments) == frozenset(other.segments)
                if isinstance(other, Multisegment)
                else NotImplemented)

    def __hash__(self) -> int:
        return hash(frozenset(self.segments))

    __repr__ = generate_repr(__new__)

    def __str__(self) -> str:
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.segments))))
