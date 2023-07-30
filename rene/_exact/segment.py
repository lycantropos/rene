from __future__ import annotations

import typing as t

import typing_extensions as te
from reprit.base import generate_repr
from rithm.fraction import Fraction

from rene import (Location,
                  Relation,
                  hints)
from rene._clipping import (intersect_segment_with_segment,
                            intersect_segment_with_segments,
                            symmetric_subtract_segment_from_segment,
                            symmetric_subtract_segments_from_segment)
from rene._context import Context
from rene._relating import segment
from rene._utils import (collect_maybe_empty_segments,
                         locate_point_in_segment,
                         unwrap_or_else)


@te.final
class Segment:
    @property
    def bounding_box(self) -> hints.Box[Fraction]:
        return self._context.box_cls(min(self._end.x, self._start.x),
                                     max(self._end.x, self._start.x),
                                     min(self._end.y, self._start.y),
                                     max(self._end.y, self._start.y))

    @property
    def end(self) -> hints.Point[Fraction]:
        return self._end

    @property
    def start(self) -> hints.Point[Fraction]:
        return self._start

    def locate(self, point: hints.Point[Fraction], /) -> Location:
        return locate_point_in_segment(self.start, self.end, point)

    def relate_to(self, other: hints.Compound[Fraction], /) -> Relation:
        if isinstance(other, self._context.contour_cls):
            return segment.relate_to_contour(self.start, self.end, other)
        elif isinstance(other, self._context.multisegment_cls):
            return segment.relate_to_multisegment(self.start, self.end, other)
        elif isinstance(other, self._context.segment_cls):
            return segment.relate_to_segment(self.start, self.end,
                                             other.start, other.end)
        else:
            raise TypeError(f'Unsupported type: {type(other)!r}.')

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
                    intersect_segment_with_segments(self, other.segments,
                                                    self._context.segment_cls),
                    self._context.empty_cls, self._context.multisegment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                unwrap_or_else(
                        intersect_segment_with_segment(
                                self, other, self._context.segment_cls
                        ),
                        self._context.empty_cls
                )
                if isinstance(other, self._context.segment_cls)
                else NotImplemented
            )
        )

    def __contains__(self, point: hints.Point[Fraction], /) -> bool:
        return self.locate(point) is not Location.EXTERIOR

    def __hash__(self) -> int:
        return hash(frozenset((self.start, self.end)))

    @t.overload
    def __eq__(self, other: Segment, /) -> bool:
        pass

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        pass

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (self.start == other.start and self.end == other.end
                or self.end == other.start and self.start == other.end
                if isinstance(other, Segment)
                else NotImplemented)

    __repr__ = generate_repr(__new__)

    def __str__(self) -> str:
        return f'{type(self).__qualname__}({self.start}, {self.end})'

    def __xor__(self, other: t.Any, /) -> t.Any:
        return (
            collect_maybe_empty_segments(
                    symmetric_subtract_segments_from_segment(
                            self, other.segments, self._context.segment_cls
                    ),
                    self._context.empty_cls, self._context.multisegment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                symmetric_subtract_segment_from_segment(
                        self, other, self._context.empty_cls,
                        self._context.multisegment_cls,
                        self._context.segment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else NotImplemented
            )
        )
