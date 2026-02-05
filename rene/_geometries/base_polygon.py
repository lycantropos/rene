from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Any, TYPE_CHECKING, overload

from typing_extensions import Self

from rene import hints
from rene._clipping import (
    intersect_polygon_with_multipolygon,
    intersect_polygon_with_multisegmental,
    intersect_polygon_with_polygon,
    intersect_polygon_with_segment,
    subtract_multipolygon_from_polygon,
    subtract_polygon_from_polygon,
    symmetric_subtract_multipolygon_from_polygon,
    symmetric_subtract_polygon_from_polygon,
    unite_polygon_with_multipolygon,
    unite_polygon_with_polygon,
)
from rene._relating import polygon
from rene._utils import locate_point_in_region
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


class BasePolygon(ABC, BaseCompound[hints.ScalarT]):
    @property
    @abstractmethod
    def border(self, /) -> hints.Contour[hints.ScalarT]: ...

    @property
    @abstractmethod
    def holes(self, /) -> Sequence[hints.Contour[hints.ScalarT]]: ...

    @property
    def bounding_box(self, /) -> hints.Box[hints.ScalarT]:
        return self.border.bounding_box

    def locate(self, point: hints.Point[hints.ScalarT], /) -> Location:
        location_without_holes = locate_point_in_region(
            self.border, point, self._context.orient
        )
        if location_without_holes is Location.INTERIOR:
            for hole in self.holes:
                location_in_hole = locate_point_in_region(
                    hole, point, self._context.orient
                )
                if location_in_hole is Location.INTERIOR:
                    return Location.EXTERIOR
                if location_in_hole is Location.BOUNDARY:
                    return Location.BOUNDARY
        return location_without_holes

    def relate_to(self, other: hints.Compound[hints.ScalarT], /) -> Relation:
        context = self._context
        if is_contour(other, context=context):
            return polygon.relate_to_contour(
                self, other, context.orient, context.intersect_segments
            )
        if is_empty(other, context=context):
            return Relation.DISJOINT
        if is_multipolygon(other, context=context):
            return polygon.relate_to_multipolygon(
                self, other, context.orient, context.intersect_segments
            )
        if is_multisegment(other, context=context):
            return polygon.relate_to_multisegment(
                self, other, context.orient, context.intersect_segments
            )
        if is_polygon(other, context=context):
            return polygon.relate_to_polygon(
                self, other, context.orient, context.intersect_segments
            )
        if is_segment(other, context=context):
            return polygon.relate_to_segment(
                self, other, context.orient, context.intersect_segments
            )
        raise TypeError(f'Unsupported type: {type(other)!r}.')

    @abstractmethod
    def __new__(
        cls,
        border: hints.Contour[hints.ScalarT],
        holes: Sequence[hints.Contour[hints.ScalarT]],
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
            intersect_polygon_with_multipolygon(
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
                intersect_polygon_with_polygon(
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
                    intersect_polygon_with_multisegmental(
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
                        intersect_polygon_with_segment(
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
            (
                self.border == other.border
                and len(self.holes) == len(other.holes)
                and frozenset(self.holes) == frozenset(other.holes)
            )
            if isinstance(other, type(self))
            else NotImplemented
        )

    def __hash__(self, /) -> int:
        return hash((self.border, frozenset(self.holes)))

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
            unite_polygon_with_multipolygon(
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
                unite_polygon_with_polygon(
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
        return f'{type(self).__qualname__}({self.border!r}, [{{}}])'.format(
            ', '.join(map(repr, self.holes))
        )

    def __str__(self, /) -> str:
        return f'{type(self).__qualname__}({self.border}, [{{}}])'.format(
            ', '.join(map(str, self.holes))
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
            subtract_multipolygon_from_polygon(
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
                subtract_polygon_from_polygon(
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
            symmetric_subtract_multipolygon_from_polygon(
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
                symmetric_subtract_polygon_from_polygon(
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
