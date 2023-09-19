from __future__ import annotations

import typing as t

import typing_extensions as te
from rithm.fraction import Fraction

from rene import (Location,
                  Relation,
                  hints)
from rene._clipping import (intersect_segment_with_multipolygon,
                            intersect_segment_with_multisegmental,
                            intersect_segment_with_polygon,
                            intersect_segment_with_segment,
                            subtract_multisegmental_from_segment,
                            subtract_segment_from_segment,
                            symmetric_subtract_multisegmental_from_segment,
                            symmetric_subtract_segment_from_segment,
                            unite_segment_with_multisegmental,
                            unite_segment_with_segment)
from rene._context import Context
from rene._relating import segment
from rene._utils import locate_point_in_segment


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
        return locate_point_in_segment(self._start, self._end, point)

    def relate_to(self, other: hints.Compound[Fraction], /) -> Relation:
        if isinstance(other, self._context.contour_cls):
            return segment.relate_to_contour(self, other)
        elif isinstance(other, self._context.multisegment_cls):
            return segment.relate_to_multisegment(self, other)
        elif isinstance(other, self._context.segment_cls):
            return segment.relate_to_segment(self, other)
        elif isinstance(other, self._context.polygon_cls):
            return segment.relate_to_polygon(self, other)
        elif isinstance(other, self._context.multipolygon_cls):
            return segment.relate_to_multipolygon(self, other)
        elif isinstance(other, self._context.empty_cls):
            return Relation.DISJOINT
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
            self,
            other: t.Union[
                hints.Contour[Fraction], hints.Multipolygon[Fraction],
                hints.Multisegment[Fraction], hints.Polygon[Fraction]
            ],
            /
    ) -> t.Union[
        hints.Empty[Fraction], hints.Multisegment[Fraction],
        hints.Segment[Fraction]
    ]:
        ...

    @t.overload
    def __and__(
            self, other: hints.Segment[Fraction], /
    ) -> t.Union[hints.Empty[Fraction], hints.Segment[Fraction]]:
        ...

    @t.overload
    def __and__(self, other: t.Any, /) -> t.Any:
        ...

    def __and__(self, other: t.Any, /) -> t.Any:
        return (
            intersect_segment_with_multisegmental(
                    self, other, self._context.empty_cls,
                    self._context.multisegment_cls, self._context.segment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                intersect_segment_with_segment(
                        self, other, self._context.empty_cls,
                        self._context.segment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else (
                    intersect_segment_with_polygon(
                            self, other, self._context.empty_cls,
                            self._context.multisegment_cls,
                            self._context.segment_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else (
                        intersect_segment_with_multipolygon(
                                self, other, self._context.empty_cls,
                                self._context.multisegment_cls,
                                self._context.segment_cls
                        )
                        if isinstance(other, self._context.multipolygon_cls)
                        else (other
                              if isinstance(other, self._context.empty_cls)
                              else NotImplemented)
                    )
                )
            )
        )

    def __contains__(self, point: hints.Point[Fraction], /) -> bool:
        return self.locate(point) is not Location.EXTERIOR

    def __hash__(self) -> int:
        return hash(frozenset((self._start, self._end)))

    @t.overload
    def __eq__(self, other: Segment, /) -> bool:
        pass

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        pass

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (self._start == other.start and self._end == other.end
                or self._end == other.start and self._start == other.end
                if isinstance(other, Segment)
                else NotImplemented)

    @t.overload
    def __or__(self, other: hints.Empty[Fraction], /) -> te.Self:
        ...

    @t.overload
    def __or__(
            self,
            other: t.Union[
                hints.Contour[Fraction], hints.Multisegment[Fraction],
                hints.Segment[Fraction]
            ],
            /
    ) -> t.Union[hints.Multisegment[Fraction], hints.Segment[Fraction]]:
        ...

    @t.overload
    def __or__(self, other: t.Any, /) -> t.Any:
        ...

    def __or__(self, other: t.Any, /) -> t.Any:
        return (
            unite_segment_with_multisegmental(
                    self, other, self._context.multisegment_cls,
                    self._context.segment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                unite_segment_with_segment(
                        self, other, self._context.multisegment_cls,
                        self._context.segment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else (self
                      if isinstance(other, self._context.empty_cls)
                      else NotImplemented)
            )
        )

    def __repr__(self) -> str:
        return f'{type(self).__qualname__}({self._start!r}, {self._end!r})'

    def __str__(self) -> str:
        return f'{type(self).__qualname__}({self._start}, {self._end})'

    @t.overload
    def __sub__(self, other: hints.Empty[Fraction], /) -> te.Self:
        ...

    @t.overload
    def __sub__(
            self,
            other: t.Union[
                hints.Contour[Fraction], hints.Multisegment[Fraction],
                hints.Segment[Fraction]
            ],
            /
    ) -> t.Union[
        hints.Empty[Fraction], hints.Multisegment[Fraction],
        hints.Segment[Fraction]
    ]:
        ...

    @t.overload
    def __sub__(self, other: t.Any, /) -> t.Any:
        ...

    def __sub__(self, other: t.Any, /) -> t.Any:
        return (
            subtract_multisegmental_from_segment(
                    self, other, self._context.empty_cls,
                    self._context.multisegment_cls, self._context.segment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                subtract_segment_from_segment(
                        self, other, self._context.empty_cls,
                        self._context.multisegment_cls,
                        self._context.segment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else (self
                      if isinstance(other, self._context.empty_cls)
                      else NotImplemented)
            )
        )

    @t.overload
    def __xor__(self, other: hints.Empty[Fraction], /) -> te.Self:
        ...

    @t.overload
    def __xor__(
            self,
            other: t.Union[
                hints.Contour[Fraction], hints.Multisegment[Fraction],
                hints.Segment[Fraction]
            ],
            /
    ) -> t.Union[
        hints.Empty[Fraction], hints.Multisegment[Fraction],
        hints.Segment[Fraction]
    ]:
        ...

    @t.overload
    def __xor__(self, other: t.Any, /) -> t.Any:
        ...

    def __xor__(self, other: t.Any, /) -> t.Any:
        return (
            symmetric_subtract_multisegmental_from_segment(
                    self, other, self._context.empty_cls,
                    self._context.multisegment_cls, self._context.segment_cls
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
                else (self
                      if isinstance(other, self._context.empty_cls)
                      else NotImplemented)
            )
        )
