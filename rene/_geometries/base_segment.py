from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Any, overload

from typing_extensions import Self

from rene import hints
from rene._clipping import (
    intersect_segment_with_multipolygon,
    intersect_segment_with_multisegmental,
    intersect_segment_with_polygon,
    intersect_segment_with_segment,
    subtract_multisegmental_from_segment,
    subtract_segment_from_segment,
    symmetric_subtract_multisegmental_from_segment,
    symmetric_subtract_segment_from_segment,
    unite_segment_with_multisegmental,
    unite_segment_with_segment,
)
from rene._geometries.base_compound import BaseCompound
from rene._geometries.utils import (
    is_contour,
    is_empty,
    is_multipolygon,
    is_multisegment,
    is_multisegmental,
    is_polygon,
    is_segment,
)
from rene._relating import segment
from rene._utils import locate_point_in_segment
from rene.enums import Location, Relation


class BaseSegment(ABC, BaseCompound[hints.ScalarT]):
    @property
    @abstractmethod
    def end(self, /) -> hints.Point[hints.ScalarT]: ...

    @property
    @abstractmethod
    def start(self, /) -> hints.Point[hints.ScalarT]: ...

    @property
    def bounding_box(self, /) -> hints.Box[hints.ScalarT]:
        return self._context.box_cls(
            min(self.end.x, self.start.x),
            max(self.end.x, self.start.x),
            min(self.end.y, self.start.y),
            max(self.end.y, self.start.y),
        )

    def locate(self, point: hints.Point[hints.ScalarT], /) -> Location:
        return locate_point_in_segment(
            self.start, self.end, point, self._context.orient
        )

    def relate_to(self, other: hints.Compound[hints.ScalarT], /) -> Relation:
        context = self._context
        if is_contour(other, context=context):
            return segment.relate_to_contour(self, other, context.orient)
        if is_empty(other, context=context):
            return Relation.DISJOINT
        if is_multipolygon(other, context=context):
            return segment.relate_to_multipolygon(
                self, other, context.orient, context.intersect_segments
            )
        if is_multisegment(other, context=context):
            return segment.relate_to_multisegment(
                self,
                other,
                context.orient,
                context.to_segments_intersection_scale,
            )
        if is_polygon(other, context=context):
            return segment.relate_to_polygon(
                self, other, context.orient, context.intersect_segments
            )
        if is_segment(other, context=context):
            return segment.relate_to_segment(self, other, context.orient)
        raise TypeError(f'Unsupported type: {type(other)!r}.')

    @abstractmethod
    def __new__(
        cls,
        start: hints.Point[hints.ScalarT],
        end: hints.Point[hints.ScalarT],
        /,
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
        ),
        /,
    ) -> (
        hints.Empty[hints.ScalarT]
        | hints.Multisegment[hints.ScalarT]
        | hints.Segment[hints.ScalarT]
    ): ...

    @overload
    def __and__(
        self, other: hints.Segment[hints.ScalarT], /
    ) -> hints.Empty[hints.ScalarT] | hints.Segment[hints.ScalarT]: ...

    @overload
    def __and__(self, other: Any, /) -> Any: ...

    def __and__(self, other: Any, /) -> Any:
        context = self._context
        return (
            intersect_segment_with_multisegmental(
                self,
                other,
                context.empty_cls,
                context.multisegment_cls,
                context.orient,
                context.segment_cls,
            )
            if is_multisegmental(other, context=context)
            else (
                intersect_segment_with_segment(
                    self,
                    other,
                    context.empty_cls,
                    context.orient,
                    context.segment_cls,
                )
                if is_segment(other, context=context)
                else (
                    intersect_segment_with_polygon(
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
                        intersect_segment_with_multipolygon(
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

    def __hash__(self, /) -> int:
        return hash(frozenset((self.start, self.end)))

    @overload
    def __eq__(self, other: Self, /) -> bool:
        pass

    @overload
    def __eq__(self, other: Any, /) -> Any:
        pass

    def __eq__(self, other: Any, /) -> Any:
        return (
            (self.start == other.start and self.end == other.end)
            or (self.end == other.start and self.start == other.end)
            if isinstance(other, type(self))
            else NotImplemented
        )

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
            unite_segment_with_multisegmental(
                self,
                other,
                context.multisegment_cls,
                context.orient,
                context.segment_cls,
                context.intersect_segments,
            )
            if is_multisegmental(other, context=context)
            else (
                unite_segment_with_segment(
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
        return f'{type(self).__qualname__}({self.start!r}, {self.end!r})'

    def __str__(self, /) -> str:
        return f'{type(self).__qualname__}({self.start}, {self.end})'

    @overload
    def __sub__(self, other: hints.Empty[hints.ScalarT], /) -> Self: ...

    @overload
    def __sub__(
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
    def __sub__(self, other: Any, /) -> Any: ...

    def __sub__(self, other: Any, /) -> Any:
        context = self._context
        return (
            subtract_multisegmental_from_segment(
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
                subtract_segment_from_segment(
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
            symmetric_subtract_multisegmental_from_segment(
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
                symmetric_subtract_segment_from_segment(
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
