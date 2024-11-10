from __future__ import annotations

import typing as t
from abc import ABC, abstractmethod

import typing_extensions as te

from rene import Location, Relation, hints
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
from rene._relating import segment
from rene._utils import locate_point_in_segment


class BaseSegment(ABC, BaseCompound[hints.Scalar]):
    @property
    @abstractmethod
    def end(self) -> hints.Point[hints.Scalar]: ...

    @property
    @abstractmethod
    def start(self) -> hints.Point[hints.Scalar]: ...

    @property
    def bounding_box(self, /) -> hints.Box[hints.Scalar]:
        return self._context.box_cls(
            min(self.end.x, self.start.x),
            max(self.end.x, self.start.x),
            min(self.end.y, self.start.y),
            max(self.end.y, self.start.y),
        )

    def locate(self, point: hints.Point[hints.Scalar], /) -> Location:
        return locate_point_in_segment(
            self.start, self.end, point, self._context.orient
        )

    def relate_to(self, other: hints.Compound[hints.Scalar], /) -> Relation:
        context = self._context
        if isinstance(other, context.contour_cls):
            return segment.relate_to_contour(self, other, context.orient)
        elif isinstance(other, context.multisegment_cls):
            return segment.relate_to_multisegment(
                self,
                other,
                context.orient,
                context.to_segments_intersection_scale,
            )
        elif isinstance(other, context.segment_cls):
            return segment.relate_to_segment(self, other, context.orient)
        elif isinstance(other, context.polygon_cls):
            return segment.relate_to_polygon(
                self, other, context.orient, context.intersect_segments
            )
        elif isinstance(other, context.multipolygon_cls):
            return segment.relate_to_multipolygon(
                self, other, context.orient, context.intersect_segments
            )
        elif isinstance(other, context.empty_cls):
            return Relation.DISJOINT
        else:
            raise TypeError(f"Unsupported type: {type(other)!r}.")

    @t.overload
    def __and__(
        self, other: hints.Empty[hints.Scalar], /
    ) -> hints.Empty[hints.Scalar]: ...

    @t.overload
    def __and__(
        self,
        other: (
            hints.Contour[hints.Scalar]
            | hints.Multipolygon[hints.Scalar]
            | hints.Multisegment[hints.Scalar]
            | hints.Polygon[hints.Scalar]
        ),
        /,
    ) -> (
        hints.Empty[hints.Scalar]
        | hints.Multisegment[hints.Scalar]
        | hints.Segment[hints.Scalar]
    ): ...

    @t.overload
    def __and__(
        self, other: hints.Segment[hints.Scalar], /
    ) -> hints.Empty[hints.Scalar] | hints.Segment[hints.Scalar]: ...

    @t.overload
    def __and__(self, other: t.Any, /) -> t.Any: ...

    def __and__(self, other: t.Any, /) -> t.Any:
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
            if isinstance(other, (context.contour_cls, context.multisegment_cls))
            else (
                intersect_segment_with_segment(
                    self,
                    other,
                    context.empty_cls,
                    context.orient,
                    context.segment_cls,
                )
                if isinstance(other, context.segment_cls)
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
                    if isinstance(other, context.polygon_cls)
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
                        if isinstance(other, context.multipolygon_cls)
                        else (
                            other
                            if isinstance(other, context.empty_cls)
                            else NotImplemented
                        )
                    )
                )
            )
        )

    def __contains__(self, point: hints.Point[hints.Scalar], /) -> bool:
        return self.locate(point) is not Location.EXTERIOR

    def __hash__(self, /) -> int:
        return hash(frozenset((self.start, self.end)))

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool:
        pass

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        pass

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (
            self.start == other.start
            and self.end == other.end
            or self.end == other.start
            and self.start == other.end
            if isinstance(other, type(self))
            else NotImplemented
        )

    @t.overload
    def __or__(self, other: hints.Empty[hints.Scalar], /) -> te.Self: ...

    @t.overload
    def __or__(
        self,
        other: (
            hints.Contour[hints.Scalar]
            | hints.Multisegment[hints.Scalar]
            | hints.Segment[hints.Scalar]
        ),
        /,
    ) -> hints.Multisegment[hints.Scalar] | hints.Segment[hints.Scalar]: ...

    @t.overload
    def __or__(self, other: t.Any, /) -> t.Any: ...

    def __or__(self, other: t.Any, /) -> t.Any:
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
            if isinstance(other, (context.contour_cls, context.multisegment_cls))
            else (
                unite_segment_with_segment(
                    self,
                    other,
                    context.multisegment_cls,
                    context.orient,
                    context.segment_cls,
                    context.intersect_segments,
                )
                if isinstance(other, context.segment_cls)
                else (self if isinstance(other, context.empty_cls) else NotImplemented)
            )
        )

    def __repr__(self, /) -> str:
        return f"{type(self).__qualname__}({self.start!r}, {self.end!r})"

    def __str__(self, /) -> str:
        return f"{type(self).__qualname__}({self.start}, {self.end})"

    @t.overload
    def __sub__(self, other: hints.Empty[hints.Scalar], /) -> te.Self: ...

    @t.overload
    def __sub__(
        self,
        other: (
            hints.Contour[hints.Scalar]
            | hints.Multisegment[hints.Scalar]
            | hints.Segment[hints.Scalar]
        ),
        /,
    ) -> (
        hints.Empty[hints.Scalar]
        | hints.Multisegment[hints.Scalar]
        | hints.Segment[hints.Scalar]
    ): ...

    @t.overload
    def __sub__(self, other: t.Any, /) -> t.Any: ...

    def __sub__(self, other: t.Any, /) -> t.Any:
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
            if isinstance(other, (context.contour_cls, context.multisegment_cls))
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
                if isinstance(other, context.segment_cls)
                else (self if isinstance(other, context.empty_cls) else NotImplemented)
            )
        )

    @t.overload
    def __xor__(self, other: hints.Empty[hints.Scalar], /) -> te.Self: ...

    @t.overload
    def __xor__(
        self,
        other: (
            hints.Contour[hints.Scalar]
            | hints.Multisegment[hints.Scalar]
            | hints.Segment[hints.Scalar]
        ),
        /,
    ) -> (
        hints.Empty[hints.Scalar]
        | hints.Multisegment[hints.Scalar]
        | hints.Segment[hints.Scalar]
    ): ...

    @t.overload
    def __xor__(self, other: t.Any, /) -> t.Any: ...

    def __xor__(self, other: t.Any, /) -> t.Any:
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
            if isinstance(other, (context.contour_cls, context.multisegment_cls))
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
                if isinstance(other, context.segment_cls)
                else (self if isinstance(other, context.empty_cls) else NotImplemented)
            )
        )
