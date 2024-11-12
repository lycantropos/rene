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

if TYPE_CHECKING:
    from collections.abc import Sequence


class BasePolygon(ABC, BaseCompound[hints.Scalar]):
    @property
    @abstractmethod
    def border(self) -> hints.Contour[hints.Scalar]: ...

    @property
    @abstractmethod
    def holes(self) -> Sequence[hints.Contour[hints.Scalar]]: ...

    @property
    def bounding_box(self, /) -> hints.Box[hints.Scalar]:
        return self.border.bounding_box

    def locate(self, point: hints.Point[hints.Scalar], /) -> Location:
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
                elif location_in_hole is Location.BOUNDARY:
                    return Location.BOUNDARY
        return location_without_holes

    def relate_to(self, other: hints.Compound[hints.Scalar], /) -> Relation:
        context = self._context
        if isinstance(other, context.contour_cls):
            return polygon.relate_to_contour(
                self, other, context.orient, context.intersect_segments
            )
        elif isinstance(other, context.multisegment_cls):
            return polygon.relate_to_multisegment(
                self, other, context.orient, context.intersect_segments
            )
        elif isinstance(other, context.segment_cls):
            return polygon.relate_to_segment(
                self, other, context.orient, context.intersect_segments
            )
        elif isinstance(other, context.empty_cls):
            return Relation.DISJOINT
        elif isinstance(other, context.multipolygon_cls):
            return polygon.relate_to_multipolygon(
                self, other, context.orient, context.intersect_segments
            )
        elif isinstance(other, context.polygon_cls):
            return polygon.relate_to_polygon(
                self, other, context.orient, context.intersect_segments
            )
        else:
            raise TypeError(f'Unsupported type: {type(other)!r}.')

    @overload
    def __and__(
        self, other: hints.Empty[hints.Scalar], /
    ) -> hints.Empty[hints.Scalar]: ...

    @overload
    def __and__(
        self,
        other: hints.Multipolygon[hints.Scalar] | hints.Polygon[hints.Scalar],
        /,
    ) -> (
        hints.Empty[hints.Scalar]
        | hints.Multipolygon[hints.Scalar]
        | hints.Polygon[hints.Scalar]
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
            if isinstance(other, context.multipolygon_cls)
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
                if isinstance(other, context.polygon_cls)
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
                    if isinstance(
                        other, (context.contour_cls, context.multisegment_cls)
                    )
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
    def __or__(self, other: hints.Empty[hints.Scalar], /) -> Self: ...

    @overload
    def __or__(
        self,
        other: hints.Multipolygon[hints.Scalar] | hints.Polygon[hints.Scalar],
        /,
    ) -> hints.Multipolygon[hints.Scalar] | hints.Polygon[hints.Scalar]: ...

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
            if isinstance(other, context.multipolygon_cls)
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
                if isinstance(other, context.polygon_cls)
                else (
                    self
                    if isinstance(other, context.empty_cls)
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
    def __sub__(self, other: hints.Empty[hints.Scalar], /) -> Self: ...

    @overload
    def __sub__(
        self,
        other: hints.Multipolygon[hints.Scalar] | hints.Polygon[hints.Scalar],
        /,
    ) -> (
        hints.Empty[hints.Scalar]
        | hints.Multipolygon[hints.Scalar]
        | hints.Polygon[hints.Scalar]
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
            if isinstance(other, context.multipolygon_cls)
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
                if isinstance(other, context.polygon_cls)
                else (
                    self
                    if isinstance(other, context.empty_cls)
                    else NotImplemented
                )
            )
        )

    @overload
    def __xor__(self, other: hints.Empty[hints.Scalar], /) -> Self: ...

    @overload
    def __xor__(
        self,
        other: hints.Multipolygon[hints.Scalar] | hints.Polygon[hints.Scalar],
        /,
    ) -> (
        hints.Empty[hints.Scalar]
        | hints.Multipolygon[hints.Scalar]
        | hints.Polygon[hints.Scalar]
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
            if isinstance(other, context.multipolygon_cls)
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
                if isinstance(other, context.polygon_cls)
                else (
                    self
                    if isinstance(other, context.empty_cls)
                    else NotImplemented
                )
            )
        )
