from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Any, TYPE_CHECKING, overload

from typing_extensions import Self

from rene import hints
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
    unite_multisegmental_with_segment,
)
from rene._relating import multisegment
from rene.enums import Location, Relation

from .base_compound import BaseCompound
from .utils import (
    is_contour,
    is_empty,
    is_multipolygon,
    is_multisegment,
    is_multisegmental,
    is_polygon,
    is_segment,
)

if TYPE_CHECKING:
    from collections.abc import Sequence


class BaseMultisegment(ABC, BaseCompound[hints.ScalarT]):
    @property
    @abstractmethod
    def segments(self, /) -> Sequence[hints.Segment[hints.ScalarT]]: ...

    @property
    def bounding_box(self, /) -> hints.Box[hints.ScalarT]:
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

    def is_valid(self, /) -> bool:
        return all(
            intersection.relation is Relation.TOUCH
            for intersection in sweep(
                self.segments,
                self._context.orient,
                self._context.intersect_segments,
            )
        )

    def locate(self, point: hints.Point[hints.ScalarT], /) -> Location:
        for segment in self.segments:
            location = segment.locate(point)
            if location is not Location.EXTERIOR:
                return location
        return Location.EXTERIOR

    def relate_to(self, other: hints.Compound[hints.ScalarT], /) -> Relation:
        context = self._context
        if is_contour(other, context=context):
            return multisegment.relate_to_contour(
                self,
                other,
                context.orient,
                context.to_segments_intersection_scale,
                context.intersect_segments,
            )
        if is_empty(other, context=context):
            return Relation.DISJOINT
        if is_multisegment(other, context=context):
            return multisegment.relate_to_multisegment(
                self,
                other,
                context.orient,
                context.to_segments_intersection_scale,
                context.intersect_segments,
            )
        if is_multipolygon(other, context=context):
            return multisegment.relate_to_multipolygon(
                self, other, context.orient, context.intersect_segments
            )
        if is_polygon(other, context=context):
            return multisegment.relate_to_polygon(
                self, other, context.orient, context.intersect_segments
            )
        if is_segment(other, context=context):
            return multisegment.relate_to_segment(
                self,
                other,
                context.orient,
                context.to_segments_intersection_scale,
            )
        raise TypeError(f'Unsupported type: {type(other)!r}.')

    @abstractmethod
    def __new__(
        cls, segments: Sequence[hints.Segment[hints.ScalarT]], /
    ) -> Self:
        raise NotImplementedError

    @overload
    def __and__(
        self, other: hints.Empty[hints.ScalarT], /
    ) -> hints.Empty[hints.ScalarT]: ...

    @overload
    def __and__(
        self,
        other: (
            hints.Contour[hints.ScalarT]
            | hints.Multipolygon[hints.ScalarT]
            | hints.Multisegment[hints.ScalarT]
            | hints.Polygon[hints.ScalarT]
            | hints.Segment[hints.ScalarT]
        ),
        /,
    ) -> (
        hints.Empty[hints.ScalarT]
        | hints.Multisegment[hints.ScalarT]
        | hints.Segment[hints.ScalarT]
    ): ...

    @overload
    def __and__(self, other: Any, /) -> Any: ...

    def __and__(self, other: Any, /) -> Any:
        context = self._context
        return (
            intersect_multisegmental_with_multisegmental(
                self,
                other,
                context.empty_cls,
                context.multisegment_cls,
                context.orient,
                context.segment_cls,
                context.intersect_segments,
            )
            if is_multisegmental(other, context=context)
            else (
                intersect_multisegmental_with_segment(
                    self,
                    other,
                    context.empty_cls,
                    context.multisegment_cls,
                    context.orient,
                    context.segment_cls,
                )
                if is_segment(other, context=context)
                else (
                    intersect_multisegmental_with_polygon(
                        self,
                        other,
                        context.empty_cls,
                        context.multisegment_cls,
                        context.orient,
                        context.segment_cls,
                        context.intersect_segments,
                    )
                    if is_polygon(other, context=context)
                    else (
                        intersect_multisegmental_with_multipolygon(
                            self,
                            other,
                            context.empty_cls,
                            context.multisegment_cls,
                            context.orient,
                            context.segment_cls,
                            context.intersect_segments,
                        )
                        if is_multipolygon(other, context=context)
                        else (
                            other
                            if is_empty(other, context=context)
                            else NotImplemented
                        )
                    )
                )
            )
        )

    def __contains__(self, point: hints.Point[hints.ScalarT], /) -> bool:
        return self.locate(point) is not Location.EXTERIOR

    @overload
    def __eq__(self, other: Self, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...

    def __eq__(self, other: Any, /) -> Any:
        return (
            frozenset(self.segments) == frozenset(other.segments)
            if isinstance(other, type(self))
            else NotImplemented
        )

    def __hash__(self, /) -> int:
        return hash(frozenset(self.segments))

    @overload
    def __or__(self, other: hints.Empty[hints.ScalarT], /) -> Self: ...

    @overload
    def __or__(
        self,
        other: (
            hints.Contour[hints.ScalarT]
            | hints.Multisegment[hints.ScalarT]
            | hints.Segment[hints.ScalarT]
        ),
        /,
    ) -> hints.Multisegment[hints.ScalarT] | hints.Segment[hints.ScalarT]: ...

    @overload
    def __or__(self, other: Any, /) -> Any: ...

    def __or__(self, other: Any, /) -> Any:
        context = self._context
        return (
            unite_multisegmental_with_multisegmental(
                self,
                other,
                context.multisegment_cls,
                context.orient,
                context.segment_cls,
                context.intersect_segments,
            )
            if is_multisegmental(other, context=context)
            else (
                unite_multisegmental_with_segment(
                    self,
                    other,
                    context.multisegment_cls,
                    context.orient,
                    context.segment_cls,
                    context.intersect_segments,
                )
                if is_segment(other, context=context)
                else (
                    self
                    if is_empty(other, context=context)
                    else NotImplemented
                )
            )
        )

    def __repr__(self, /) -> str:
        return f'{type(self).__qualname__}([{{}}])'.format(
            ', '.join(map(repr, self.segments))
        )

    def __str__(self, /) -> str:
        return f'{type(self).__qualname__}([{{}}])'.format(
            ', '.join(map(str, self.segments))
        )

    @overload
    def __sub__(self, other: hints.Empty[hints.ScalarT], /) -> Self: ...

    @overload
    def __sub__(
        self,
        other: (
            hints.Contour[hints.ScalarT]
            | hints.Multipolygon[hints.ScalarT]
            | hints.Multisegment[hints.ScalarT]
            | hints.Segment[hints.ScalarT]
        ),
        /,
    ) -> (
        hints.Empty[hints.ScalarT]
        | hints.Multisegment[hints.ScalarT]
        | hints.Segment[hints.ScalarT]
    ): ...

    @overload
    def __sub__(self, other: Any, /) -> Any: ...

    def __sub__(self, other: Any, /) -> Any:
        context = self._context
        return (
            subtract_multisegmental_from_multisegmental(
                self,
                other,
                context.empty_cls,
                context.multisegment_cls,
                context.orient,
                context.segment_cls,
                context.intersect_segments,
            )
            if is_multisegmental(other, context=context)
            else (
                subtract_segment_from_multisegmental(
                    self,
                    other,
                    context.empty_cls,
                    context.multisegment_cls,
                    context.orient,
                    context.segment_cls,
                    context.intersect_segments,
                )
                if is_segment(other, context=context)
                else (
                    subtract_multipolygon_from_multisegmental(
                        self,
                        other,
                        context.empty_cls,
                        context.multisegment_cls,
                        context.orient,
                        context.segment_cls,
                        context.intersect_segments,
                    )
                    if is_multipolygon(other, context=context)
                    else (
                        subtract_polygon_from_multisegmental(
                            self,
                            other,
                            context.empty_cls,
                            context.multisegment_cls,
                            context.orient,
                            context.segment_cls,
                            context.intersect_segments,
                        )
                        if is_polygon(other, context=context)
                        else (
                            self
                            if is_empty(other, context=context)
                            else NotImplemented
                        )
                    )
                )
            )
        )

    @overload
    def __xor__(self, other: hints.Empty[hints.ScalarT], /) -> Self: ...

    @overload
    def __xor__(
        self,
        other: (
            hints.Contour[hints.ScalarT]
            | hints.Multisegment[hints.ScalarT]
            | hints.Segment[hints.ScalarT]
        ),
        /,
    ) -> (
        hints.Empty[hints.ScalarT]
        | hints.Multisegment[hints.ScalarT]
        | hints.Segment[hints.ScalarT]
    ): ...

    @overload
    def __xor__(self, other: Any, /) -> Any: ...

    def __xor__(self, other: Any, /) -> Any:
        context = self._context
        return (
            symmetric_subtract_multisegmental_from_multisegmental(
                self,
                other,
                context.empty_cls,
                context.multisegment_cls,
                context.orient,
                context.segment_cls,
                context.intersect_segments,
            )
            if is_multisegmental(other, context=context)
            else (
                symmetric_subtract_segment_from_multisegmental(
                    self,
                    other,
                    context.empty_cls,
                    context.multisegment_cls,
                    context.orient,
                    context.segment_cls,
                    context.intersect_segments,
                )
                if is_segment(other, context=context)
                else (
                    self
                    if is_empty(other, context=context)
                    else NotImplemented
                )
            )
        )
