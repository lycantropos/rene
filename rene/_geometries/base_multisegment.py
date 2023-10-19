from __future__ import annotations

import typing as t
from abc import ABC, abstractmethod

import typing_extensions as te

from rene import (Location,
                  Relation,
                  hints)
from rene._bentley_ottmann.base import sweep
from rene._clipping import (
    intersect_multisegmental_with_multipolygon,
    intersect_multisegmental_with_multisegmental,
    intersect_multisegmental_with_polygon,
    intersect_multisegmental_with_segment,
    subtract_multipolygon_from_multisegmental,
    subtract_multisegmental_from_multisegmental,
    subtract_polygon_from_multisegmental,
    subtract_segment_from_multisegmental,
    symmetric_subtract_multisegmental_from_multisegmental,
    symmetric_subtract_segment_from_multisegmental,
    unite_multisegmental_with_multisegmental,
    unite_multisegmental_with_segment
)
from rene._relating import multisegment
from .base_compound import BaseCompound


class BaseMultisegment(ABC, BaseCompound[hints.Scalar]):
    @property
    @abstractmethod
    def segments(self) -> t.Sequence[hints.Segment[hints.Scalar]]:
        ...

    @property
    def bounding_box(self) -> hints.Box[hints.Scalar]:
        segments = iter(self.segments)
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

    def is_valid(self) -> bool:
        return all(intersection.relation is Relation.TOUCH
                   for intersection in sweep(self.segments))

    def locate(self, point: hints.Point[hints.Scalar], /) -> Location:
        for segment in self.segments:
            location = segment.locate(point)
            if location is not Location.EXTERIOR:
                return location
        return Location.EXTERIOR

    def relate_to(self, other: hints.Compound[hints.Scalar], /) -> Relation:
        if isinstance(other, self._context.contour_cls):
            return multisegment.relate_to_contour(self, other)
        elif isinstance(other, self._context.multisegment_cls):
            return multisegment.relate_to_multisegment(self, other)
        elif isinstance(other, self._context.segment_cls):
            return multisegment.relate_to_segment(self, other)
        elif isinstance(other, self._context.polygon_cls):
            return multisegment.relate_to_polygon(self, other)
        elif isinstance(other, self._context.multipolygon_cls):
            return multisegment.relate_to_multipolygon(self, other)
        elif isinstance(other, self._context.empty_cls):
            return Relation.DISJOINT
        else:
            raise TypeError(f'Unsupported type: {type(other)!r}.')

    @t.overload
    def __and__(
            self, other: hints.Empty[hints.Scalar], /
    ) -> hints.Empty[hints.Scalar]:
        ...

    @t.overload
    def __and__(
            self,
            other: t.Union[
                hints.Contour[hints.Scalar], hints.Multipolygon[hints.Scalar],
                hints.Multisegment[hints.Scalar], hints.Polygon[hints.Scalar],
                hints.Segment[hints.Scalar]
            ],
            /
    ) -> t.Union[
        hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
        hints.Segment[hints.Scalar]
    ]:
        ...

    @t.overload
    def __and__(self, other: t.Any, /) -> t.Any:
        ...

    def __and__(self, other: t.Any, /) -> t.Any:
        return (
            intersect_multisegmental_with_multisegmental(
                    self, other, self._context.empty_cls,
                    self._context.multisegment_cls, self._context.segment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                intersect_multisegmental_with_segment(
                        self, other, self._context.empty_cls,
                        self._context.multisegment_cls,
                        self._context.segment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else (
                    intersect_multisegmental_with_polygon(
                            self, other, self._context.empty_cls,
                            self._context.multisegment_cls,
                            self._context.segment_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else (
                        intersect_multisegmental_with_multipolygon(
                                self, other, self._context.empty_cls,
                                self._context.multisegment_cls,
                                self._context.segment_cls
                        )
                        if isinstance(other,
                                      self._context.multipolygon_cls)
                        else (other
                              if isinstance(other, self._context.empty_cls)
                              else NotImplemented)
                    )
                )
            )
        )

    def __contains__(self, point: hints.Point[hints.Scalar], /) -> bool:
        return self.locate(point) is not Location.EXTERIOR

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        ...

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (frozenset(self.segments) == frozenset(other.segments)
                if isinstance(other, type(self))
                else NotImplemented)

    def __hash__(self) -> int:
        return hash(frozenset(self.segments))

    @t.overload
    def __or__(self, other: hints.Empty[hints.Scalar], /) -> te.Self:
        ...

    @t.overload
    def __or__(
            self,
            other: t.Union[
                hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar],
                hints.Segment[hints.Scalar]
            ],
            /
    ) -> t.Union[
        hints.Multisegment[hints.Scalar], hints.Segment[hints.Scalar]]:
        ...

    @t.overload
    def __or__(self, other: t.Any, /) -> t.Any:
        ...

    def __or__(self, other: t.Any, /) -> t.Any:
        return (
            unite_multisegmental_with_multisegmental(
                    self, other, self._context.multisegment_cls,
                    self._context.segment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                unite_multisegmental_with_segment(
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
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(repr, self.segments))))

    def __str__(self) -> str:
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.segments))))

    @t.overload
    def __sub__(self, other: hints.Empty[hints.Scalar], /) -> te.Self:
        ...

    @t.overload
    def __sub__(
            self,
            other: t.Union[
                hints.Contour[hints.Scalar], hints.Multipolygon[hints.Scalar],
                hints.Multisegment[hints.Scalar], hints.Segment[hints.Scalar]
            ],
            /
    ) -> t.Union[
        hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
        hints.Segment[hints.Scalar]
    ]:
        ...

    @t.overload
    def __sub__(self, other: t.Any, /) -> t.Any:
        ...

    def __sub__(self, other: t.Any, /) -> t.Any:
        return (
            subtract_multisegmental_from_multisegmental(
                    self, other, self._context.empty_cls,
                    self._context.multisegment_cls, self._context.segment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                subtract_segment_from_multisegmental(
                        self, other, self._context.empty_cls,
                        self._context.multisegment_cls,
                        self._context.segment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else (
                    subtract_multipolygon_from_multisegmental(
                            self, other, self._context.empty_cls,
                            self._context.multisegment_cls,
                            self._context.segment_cls
                    )
                    if isinstance(other, self._context.multipolygon_cls)
                    else (
                        subtract_polygon_from_multisegmental(
                                self, other, self._context.empty_cls,
                                self._context.multisegment_cls,
                                self._context.segment_cls
                        )
                        if isinstance(other, self._context.polygon_cls)
                        else (self
                              if isinstance(other, self._context.empty_cls)
                              else NotImplemented)
                    )
                )
            )
        )

    @t.overload
    def __xor__(self, other: hints.Empty[hints.Scalar], /) -> te.Self:
        ...

    @t.overload
    def __xor__(
            self,
            other: t.Union[
                hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar],
                hints.Segment[hints.Scalar]
            ],
            /
    ) -> t.Union[
        hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
        hints.Segment[hints.Scalar]
    ]:
        ...

    @t.overload
    def __xor__(self, other: t.Any, /) -> t.Any:
        ...

    def __xor__(self, other: t.Any, /) -> t.Any:
        return (
            symmetric_subtract_multisegmental_from_multisegmental(
                    self, other, self._context.empty_cls,
                    self._context.multisegment_cls, self._context.segment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                symmetric_subtract_segment_from_multisegmental(
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
