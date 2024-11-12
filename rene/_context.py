from __future__ import annotations

from typing import Generic, TYPE_CHECKING

from typing_extensions import Self

from rene import Orientation, hints

if TYPE_CHECKING:
    from rene._hints import (
        Orienteer,
        SegmentsIntersectionScale,
        SegmentsIntersector,
    )


class Context(Generic[hints.Scalar]):
    @property
    def box_cls(self, /) -> type[hints.Box[hints.Scalar]]:
        return self._box_cls

    @property
    def contour_cls(self, /) -> type[hints.Contour[hints.Scalar]]:
        return self._contour_cls

    @property
    def empty_cls(self, /) -> type[hints.Empty[hints.Scalar]]:
        return self._empty_cls

    @property
    def multipolygon_cls(self, /) -> type[hints.Multipolygon[hints.Scalar]]:
        return self._multipolygon_cls

    @property
    def multisegment_cls(self, /) -> type[hints.Multisegment[hints.Scalar]]:
        return self._multisegment_cls

    @property
    def point_cls(self, /) -> type[hints.Point[hints.Scalar]]:
        return self._point_cls

    @property
    def polygon_cls(self, /) -> type[hints.Polygon[hints.Scalar]]:
        return self._polygon_cls

    @property
    def segment_cls(self, /) -> type[hints.Segment[hints.Scalar]]:
        return self._segment_cls

    def intersect_segments(
        self,
        first_start: hints.Point[hints.Scalar],
        first_end: hints.Point[hints.Scalar],
        second_start: hints.Point[hints.Scalar],
        second_end: hints.Point[hints.Scalar],
        /,
    ) -> hints.Point[hints.Scalar]:
        return self._segments_intersector(
            first_start, first_end, second_start, second_end
        )

    def to_segments_intersection_scale(
        self,
        first_start: hints.Point[hints.Scalar],
        first_end: hints.Point[hints.Scalar],
        second_start: hints.Point[hints.Scalar],
        second_end: hints.Point[hints.Scalar],
        /,
    ) -> hints.Scalar:
        return self._segments_intersection_scale(
            first_start, first_end, second_start, second_end
        )

    def orient(
        self,
        vertex: hints.Point[hints.Scalar],
        first_ray_point: hints.Point[hints.Scalar],
        second_ray_point: hints.Point[hints.Scalar],
        /,
    ) -> Orientation:
        return self._orienteer(vertex, first_ray_point, second_ray_point)

    _box_cls: type[hints.Box[hints.Scalar]]
    _contour_cls: type[hints.Contour[hints.Scalar]]
    _empty_cls: type[hints.Empty[hints.Scalar]]
    _multipolygon_cls: type[hints.Multipolygon[hints.Scalar]]
    _multisegment_cls: type[hints.Multisegment[hints.Scalar]]
    _orienteer: Orienteer[hints.Scalar]
    _point_cls: type[hints.Point[hints.Scalar]]
    _polygon_cls: type[hints.Polygon[hints.Scalar]]
    _segment_cls: type[hints.Segment[hints.Scalar]]
    _segments_intersection_scale: SegmentsIntersectionScale[hints.Scalar]
    _segments_intersector: SegmentsIntersector[hints.Scalar]

    __module__ = 'rene.exact'
    __slots__ = (
        '_box_cls',
        '_contour_cls',
        '_empty_cls',
        '_multipolygon_cls',
        '_multisegment_cls',
        '_orienteer',
        '_point_cls',
        '_polygon_cls',
        '_segment_cls',
        '_segments_intersection_scale',
        '_segments_intersector',
    )

    def __new__(
        cls,
        *,
        box_cls: type[hints.Box[hints.Scalar]],
        contour_cls: type[hints.Contour[hints.Scalar]],
        empty_cls: type[hints.Empty[hints.Scalar]],
        multipolygon_cls: type[hints.Multipolygon[hints.Scalar]],
        multisegment_cls: type[hints.Multisegment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        point_cls: type[hints.Point[hints.Scalar]],
        polygon_cls: type[hints.Polygon[hints.Scalar]],
        segment_cls: type[hints.Segment[hints.Scalar]],
        segments_intersection_scale: SegmentsIntersectionScale[hints.Scalar],
        segments_intersector: SegmentsIntersector[hints.Scalar],
    ) -> Self:
        self = super().__new__(cls)
        (
            self._box_cls,
            self._contour_cls,
            self._empty_cls,
            self._multipolygon_cls,
            self._multisegment_cls,
            self._orienteer,
            self._point_cls,
            self._polygon_cls,
            self._segment_cls,
            self._segments_intersection_scale,
            self._segments_intersector,
        ) = (
            box_cls,
            contour_cls,
            empty_cls,
            multipolygon_cls,
            multisegment_cls,
            orienteer,
            point_cls,
            polygon_cls,
            segment_cls,
            segments_intersection_scale,
            segments_intersector,
        )
        return self
