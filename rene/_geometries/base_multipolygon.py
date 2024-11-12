from __future__ import annotations

import typing as t
from abc import ABC, abstractmethod

import typing_extensions as te

from rene import Location, Relation, hints
from rene._clipping import (
    intersect_multipolygon_with_multipolygon,
    intersect_multipolygon_with_multisegmental,
    intersect_multipolygon_with_polygon,
    intersect_multipolygon_with_segment,
    subtract_multipolygon_from_multipolygon,
    subtract_polygon_from_multipolygon,
    symmetric_subtract_multipolygon_from_multipolygon,
    symmetric_subtract_polygon_from_multipolygon,
    unite_multipolygon_with_multipolygon,
    unite_multipolygon_with_polygon,
)
from rene._context import Context
from rene._relating import multipolygon

from .base_compound import BaseCompound


class BaseMultipolygon(ABC, BaseCompound[hints.Scalar]):
    @property
    @abstractmethod
    def polygons(self) -> t.Sequence[hints.Polygon[hints.Scalar]]: ...

    @property
    def bounding_box(self, /) -> hints.Box[hints.Scalar]:
        polygons = iter(self.polygons)
        first_polygon_bounding_box = next(polygons).bounding_box
        min_x, max_x, min_y, max_y = (
            first_polygon_bounding_box.min_x,
            first_polygon_bounding_box.max_x,
            first_polygon_bounding_box.min_y,
            first_polygon_bounding_box.max_y,
        )
        for polygon in polygons:
            polygon_bounding_box = polygon.bounding_box
            if polygon_bounding_box.max_x > max_x:
                max_x = polygon_bounding_box.max_x
            if polygon_bounding_box.min_x < min_x:
                min_x = polygon_bounding_box.min_x
            if polygon_bounding_box.max_y > max_y:
                max_y = polygon_bounding_box.max_y
            if polygon_bounding_box.min_y < min_y:
                min_y = polygon_bounding_box.min_y
        return self._context.box_cls(min_x, max_x, min_y, max_y)

    def locate(self, point: hints.Point[hints.Scalar], /) -> Location:
        for polygon in self.polygons:
            location = polygon.locate(point)
            if location is not Location.EXTERIOR:
                return location
        return Location.EXTERIOR

    def relate_to(self, other: hints.Compound[hints.Scalar], /) -> Relation:
        context = self._context
        if isinstance(other, context.contour_cls):
            return multipolygon.relate_to_contour(
                self, other, context.orient, context.intersect_segments
            )
        elif isinstance(other, context.multisegment_cls):
            return multipolygon.relate_to_multisegment(
                self, other, context.orient, context.intersect_segments
            )
        elif isinstance(other, context.segment_cls):
            return multipolygon.relate_to_segment(
                self, other, context.orient, context.intersect_segments
            )
        elif isinstance(other, context.empty_cls):
            return Relation.DISJOINT
        elif isinstance(other, context.multipolygon_cls):
            return multipolygon.relate_to_multipolygon(
                self, other, context.orient, context.intersect_segments
            )
        elif isinstance(other, context.polygon_cls):
            return multipolygon.relate_to_polygon(
                self, other, context.orient, context.intersect_segments
            )
        else:
            raise TypeError(f'Unsupported type: {type(other)!r}.')

    _context: t.ClassVar[Context[t.Any]]

    @t.overload
    def __and__(
        self, other: hints.Empty[hints.Scalar], /
    ) -> hints.Empty[hints.Scalar]: ...

    @t.overload
    def __and__(
        self,
        other: hints.Multipolygon[hints.Scalar] | hints.Polygon[hints.Scalar],
        /,
    ) -> (
        hints.Empty[hints.Scalar]
        | hints.Multipolygon[hints.Scalar]
        | hints.Polygon[hints.Scalar]
    ): ...

    @t.overload
    def __and__(self, other: t.Any, /) -> t.Any: ...

    def __and__(self, other: t.Any, /) -> t.Any:
        context = self._context
        return (
            intersect_multipolygon_with_multipolygon(
                self,
                other,
                context.contour_cls,
                context.empty_cls,
                context.multipolygon_cls,
                context.orient,
                context.polygon_cls,
                context.segment_cls,
                context.intersect_segments,
            )
            if isinstance(other, context.multipolygon_cls)
            else (
                intersect_multipolygon_with_polygon(
                    self,
                    other,
                    context.contour_cls,
                    context.empty_cls,
                    context.multipolygon_cls,
                    context.orient,
                    context.polygon_cls,
                    context.segment_cls,
                    context.intersect_segments,
                )
                if isinstance(other, context.polygon_cls)
                else (
                    intersect_multipolygon_with_multisegmental(
                        self,
                        other,
                        context.empty_cls,
                        context.multisegment_cls,
                        context.orient,
                        context.segment_cls,
                        context.intersect_segments,
                    )
                    if isinstance(
                        other, (context.contour_cls, context.multisegment_cls)
                    )
                    else (
                        intersect_multipolygon_with_segment(
                            self,
                            other,
                            context.empty_cls,
                            context.multisegment_cls,
                            context.orient,
                            context.segment_cls,
                            context.intersect_segments,
                        )
                        if isinstance(other, context.segment_cls)
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

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool: ...

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any: ...

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (
            frozenset(self.polygons) == frozenset(other.polygons)
            if isinstance(other, type(self))
            else NotImplemented
        )

    def __hash__(self, /) -> int:
        return hash(frozenset(self.polygons))

    @t.overload
    def __or__(self, other: hints.Empty[hints.Scalar], /) -> te.Self: ...

    @t.overload
    def __or__(
        self,
        other: hints.Multipolygon[hints.Scalar] | hints.Polygon[hints.Scalar],
        /,
    ) -> hints.Multipolygon[hints.Scalar] | hints.Polygon[hints.Scalar]: ...

    @t.overload
    def __or__(self, other: t.Any, /) -> t.Any: ...

    def __or__(self, other: t.Any, /) -> t.Any:
        context = self._context
        return (
            unite_multipolygon_with_multipolygon(
                self,
                other,
                context.contour_cls,
                context.multipolygon_cls,
                context.orient,
                context.polygon_cls,
                context.segment_cls,
                context.intersect_segments,
            )
            if isinstance(other, context.multipolygon_cls)
            else (
                unite_multipolygon_with_polygon(
                    self,
                    other,
                    context.contour_cls,
                    context.multipolygon_cls,
                    context.orient,
                    context.polygon_cls,
                    context.segment_cls,
                    context.intersect_segments,
                )
                if isinstance(other, context.polygon_cls)
                else (
                    self
                    if isinstance(other, context.empty_cls)
                    else NotImplemented
                )
            )
        )

    def __repr__(self, /) -> str:
        return f'{type(self).__qualname__}([{{}}])'.format(
            ', '.join(map(repr, self.polygons))
        )

    def __str__(self, /) -> str:
        return f'{type(self).__qualname__}([{{}}])'.format(
            ', '.join(map(str, self.polygons))
        )

    @t.overload
    def __sub__(self, other: hints.Empty[hints.Scalar], /) -> te.Self: ...

    @t.overload
    def __sub__(
        self,
        other: hints.Multipolygon[hints.Scalar] | hints.Polygon[hints.Scalar],
        /,
    ) -> (
        hints.Empty[hints.Scalar]
        | hints.Multipolygon[hints.Scalar]
        | hints.Polygon[hints.Scalar]
    ): ...

    @t.overload
    def __sub__(self, other: t.Any, /) -> t.Any: ...

    def __sub__(self, other: t.Any, /) -> t.Any:
        context = self._context
        return (
            subtract_multipolygon_from_multipolygon(
                self,
                other,
                context.contour_cls,
                context.empty_cls,
                context.multipolygon_cls,
                context.orient,
                context.polygon_cls,
                context.segment_cls,
                context.intersect_segments,
            )
            if isinstance(other, context.multipolygon_cls)
            else (
                subtract_polygon_from_multipolygon(
                    self,
                    other,
                    context.contour_cls,
                    context.empty_cls,
                    context.multipolygon_cls,
                    context.orient,
                    context.polygon_cls,
                    context.segment_cls,
                    context.intersect_segments,
                )
                if isinstance(other, context.polygon_cls)
                else (
                    self
                    if isinstance(other, context.empty_cls)
                    else NotImplemented
                )
            )
        )

    @t.overload
    def __xor__(self, other: hints.Empty[hints.Scalar], /) -> te.Self: ...

    @t.overload
    def __xor__(
        self,
        other: hints.Multipolygon[hints.Scalar] | hints.Polygon[hints.Scalar],
        /,
    ) -> (
        hints.Empty[hints.Scalar]
        | hints.Multipolygon[hints.Scalar]
        | hints.Polygon[hints.Scalar]
    ): ...

    @t.overload
    def __xor__(self, other: t.Any, /) -> t.Any: ...

    def __xor__(self, other: t.Any, /) -> t.Any:
        context = self._context
        return (
            symmetric_subtract_multipolygon_from_multipolygon(
                self,
                other,
                context.contour_cls,
                context.empty_cls,
                context.multipolygon_cls,
                context.orient,
                context.polygon_cls,
                context.segment_cls,
                context.intersect_segments,
            )
            if isinstance(other, context.multipolygon_cls)
            else (
                symmetric_subtract_polygon_from_multipolygon(
                    self,
                    other,
                    context.contour_cls,
                    context.empty_cls,
                    context.multipolygon_cls,
                    context.orient,
                    context.polygon_cls,
                    context.segment_cls,
                    context.intersect_segments,
                )
                if isinstance(other, context.polygon_cls)
                else (
                    self
                    if isinstance(other, context.empty_cls)
                    else NotImplemented
                )
            )
        )
