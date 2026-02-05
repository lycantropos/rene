from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Any, ClassVar, TYPE_CHECKING, overload

from typing_extensions import Self

from rene import hints
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
from rene._relating import multipolygon
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

    from rene._context import Context


class BaseMultipolygon(ABC, BaseCompound[hints.ScalarT]):
    @property
    @abstractmethod
    def polygons(self, /) -> Sequence[hints.Polygon[hints.ScalarT]]:
        raise NotImplementedError

    @property
    def bounding_box(self, /) -> hints.Box[hints.ScalarT]:
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

    def locate(self, point: hints.Point[hints.ScalarT], /) -> Location:
        for polygon in self.polygons:
            location = polygon.locate(point)
            if location is not Location.EXTERIOR:
                return location
        return Location.EXTERIOR

    def relate_to(self, other: hints.Compound[hints.ScalarT], /) -> Relation:
        context = self._context
        if is_contour(other, context=context):
            return multipolygon.relate_to_contour(
                self, other, context.orient, context.intersect_segments
            )
        if is_empty(other, context=context):
            return Relation.DISJOINT
        if is_multisegment(other, context=context):
            return multipolygon.relate_to_multisegment(
                self, other, context.orient, context.intersect_segments
            )
        if is_multipolygon(other, context=context):
            return multipolygon.relate_to_multipolygon(
                self, other, context.orient, context.intersect_segments
            )
        if is_polygon(other, context=context):
            return multipolygon.relate_to_polygon(
                self, other, context.orient, context.intersect_segments
            )
        if is_segment(other, context=context):
            return multipolygon.relate_to_segment(
                self, other, context.orient, context.intersect_segments
            )
        raise TypeError(f'Unsupported type: {type(other)!r}.')

    _context: ClassVar[Context[Any]]

    @abstractmethod
    def __new__(
        cls, polygons: Sequence[hints.Polygon[hints.ScalarT]], /
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
            hints.Multipolygon[hints.ScalarT] | hints.Polygon[hints.ScalarT]
        ),
        /,
    ) -> (
        hints.Empty[hints.ScalarT]
        | hints.Multipolygon[hints.ScalarT]
        | hints.Polygon[hints.ScalarT]
    ): ...

    @overload
    def __and__(self, other: Any, /) -> Any: ...

    def __and__(self, other: Any, /) -> Any:
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
            if is_multipolygon(other, context=context)
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
                if is_polygon(other, context=context)
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
                    if is_multisegmental(other, context=context)
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
                        if is_segment(other, context=context)
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
            frozenset(self.polygons) == frozenset(other.polygons)
            if isinstance(other, type(self))
            else NotImplemented
        )

    def __hash__(self, /) -> int:
        return hash(frozenset(self.polygons))

    @overload
    def __or__(self, other: hints.Empty[hints.ScalarT], /) -> Self: ...

    @overload
    def __or__(
        self,
        other: (
            hints.Multipolygon[hints.ScalarT] | hints.Polygon[hints.ScalarT]
        ),
        /,
    ) -> hints.Multipolygon[hints.ScalarT] | hints.Polygon[hints.ScalarT]: ...

    @overload
    def __or__(self, other: Any, /) -> Any: ...

    def __or__(self, other: Any, /) -> Any:
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
            if is_multipolygon(other, context=context)
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
                if is_polygon(other, context=context)
                else (
                    self
                    if is_empty(other, context=context)
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

    @overload
    def __sub__(self, other: hints.Empty[hints.ScalarT], /) -> Self: ...

    @overload
    def __sub__(
        self,
        other: (
            hints.Multipolygon[hints.ScalarT] | hints.Polygon[hints.ScalarT]
        ),
        /,
    ) -> (
        hints.Empty[hints.ScalarT]
        | hints.Multipolygon[hints.ScalarT]
        | hints.Polygon[hints.ScalarT]
    ): ...

    @overload
    def __sub__(self, other: Any, /) -> Any: ...

    def __sub__(self, other: Any, /) -> Any:
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
            if is_multipolygon(other, context=context)
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
                if is_polygon(other, context=context)
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
            hints.Multipolygon[hints.ScalarT] | hints.Polygon[hints.ScalarT]
        ),
        /,
    ) -> (
        hints.Empty[hints.ScalarT]
        | hints.Multipolygon[hints.ScalarT]
        | hints.Polygon[hints.ScalarT]
    ): ...

    @overload
    def __xor__(self, other: Any, /) -> Any: ...

    def __xor__(self, other: Any, /) -> Any:
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
            if is_multipolygon(other, context=context)
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
                if is_polygon(other, context=context)
                else (
                    self
                    if is_empty(other, context=context)
                    else NotImplemented
                )
            )
        )
