import typing as t
from itertools import (chain,
                       groupby)

from rene import (Orientation,
                  hints)
from rene._utils import (do_boxes_have_no_common_continuum,
                         flags_to_false_indices,
                         flags_to_true_indices,
                         orient,
                         polygon_to_correctly_oriented_segments,
                         to_boxes_have_common_continuum,
                         to_segments_intersection_point,
                         to_sorted_pair)
from . import (linear,
               shaped)
from .difference import subtract_segment_from_multisegmental
from .event import Event


class LinearUnion(linear.Operation[hints.Scalar]):
    def reduce_events(self,
                      events: t.List[Event],
                      segment_cls: t.Type[hints.Segment[hints.Scalar]],
                      /) -> t.List[hints.Segment[hints.Scalar]]:
        return [
            segment_cls(start, end)
            for (start, end), equal_segment_events in groupby(
                    events,
                    key=self._to_event_endpoints
            )
        ]


class ShapedUnion(shaped.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return (self._is_outside_left_event(event)
                or (not self._is_left_event_from_first_operand(event)
                    and self._is_common_region_boundary_left_event(event)))


_Multisegmental = t.Union[
    hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar]
]


def unite_multipolygon_with_multipolygon(
        first: hints.Multipolygon[hints.Scalar],
        second: hints.Multipolygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    first_polygons, second_polygons = first.polygons, second.polygons
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [*first_polygons, *second_polygons]
    first_boxes = [polygon.bounding_box for polygon in first_polygons]
    second_boxes = [polygon.bounding_box for polygon in second_polygons]
    do_first_boxes_have_common_continuum = to_boxes_have_common_continuum(
            first_boxes, second_bounding_box
    )
    first_common_continuum_polygons_ids = flags_to_true_indices(
            do_first_boxes_have_common_continuum
    )
    if not first_common_continuum_polygons_ids:
        return [*first_polygons, *second_polygons]
    do_second_boxes_have_common_continuum = to_boxes_have_common_continuum(
            second_boxes, first_bounding_box
    )
    second_common_continuum_polygons_ids = flags_to_true_indices(
            do_second_boxes_have_common_continuum
    )
    if not second_common_continuum_polygons_ids:
        return [*first_polygons, *second_polygons]
    first_common_continuum_polygons = [
        first_polygons[index]
        for index in first_common_continuum_polygons_ids
    ]
    second_common_continuum_polygons = [
        second_polygons[index]
        for index in second_common_continuum_polygons_ids
    ]
    operation = ShapedUnion.from_segments_iterables(
            chain.from_iterable(
                    polygon_to_correctly_oriented_segments(polygon,
                                                           segment_cls)
                    for polygon in first_common_continuum_polygons
            ),
            chain.from_iterable(
                    polygon_to_correctly_oriented_segments(polygon,
                                                           segment_cls)
                    for polygon in second_common_continuum_polygons
            )
    )
    result = operation.reduce_events(list(operation), contour_cls, polygon_cls)
    result.extend(
            first_polygons[index]
            for index in flags_to_false_indices(
                    do_first_boxes_have_common_continuum
            )
    )
    result.extend(
            second_polygons[index]
            for index in flags_to_false_indices(
                    do_second_boxes_have_common_continuum
            )
    )
    return result


def unite_multipolygon_with_polygon(
        first: hints.Multipolygon[hints.Scalar],
        second: hints.Polygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    first_polygons = first.polygons
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [*first_polygons, second]
    first_boxes = [polygon.bounding_box for polygon in first_polygons]
    first_boxes_have_common_continuum = to_boxes_have_common_continuum(
            first_boxes, second_bounding_box
    )
    first_common_continuum_polygons_ids = flags_to_true_indices(
            first_boxes_have_common_continuum
    )
    if not first_common_continuum_polygons_ids:
        return [*first_polygons, second]
    first_common_continuum_polygons = [
        first_polygons[index]
        for index in first_common_continuum_polygons_ids
    ]
    operation = ShapedUnion.from_segments_iterables(
            chain.from_iterable(
                    polygon_to_correctly_oriented_segments(polygon,
                                                           segment_cls)
                    for polygon in first_common_continuum_polygons
            ),
            polygon_to_correctly_oriented_segments(second, segment_cls)
    )
    result = operation.reduce_events(list(operation), contour_cls, polygon_cls)
    result.extend(
            first_polygons[index]
            for index in flags_to_false_indices(
                    first_boxes_have_common_continuum
            )
    )
    return result


def unite_multisegmental_with_multisegmental(
        first: _Multisegmental[hints.Scalar],
        second: _Multisegmental[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Segment[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    first_segments, second_segments = first.segments, second.segments
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [*first_segments, *second_segments]
    first_boxes = [segment.bounding_box for segment in first_segments]
    second_boxes = [segment.bounding_box for segment in second_segments]
    do_first_boxes_have_common_continuum = to_boxes_have_common_continuum(
            first_boxes, second_bounding_box
    )
    first_common_continuum_segments_ids = flags_to_true_indices(
            do_first_boxes_have_common_continuum
    )
    if not first_common_continuum_segments_ids:
        return [*first_segments, *second_segments]
    do_second_boxes_have_common_continuum = to_boxes_have_common_continuum(
            second_boxes, first_bounding_box
    )
    second_common_continuum_segments_ids = flags_to_true_indices(
            do_second_boxes_have_common_continuum
    )
    if not second_common_continuum_segments_ids:
        return [*first_segments, *second_segments]
    first_common_continuum_segments = [
        first_segments[index]
        for index in first_common_continuum_segments_ids
    ]
    second_common_continuum_segments = [
        second_segments[index]
        for index in second_common_continuum_segments_ids
    ]
    operation = LinearUnion.from_segments_iterables(
            first_common_continuum_segments, second_common_continuum_segments
    )
    result = operation.reduce_events(list(operation), segment_cls)
    result.extend(
            first_segments[index]
            for index in flags_to_false_indices(
                    do_first_boxes_have_common_continuum
            )
    )
    result.extend(
            second_segments[index]
            for index in flags_to_false_indices(
                    do_second_boxes_have_common_continuum
            )
    )
    return result


def unite_multisegmental_with_segment(
        first: _Multisegmental[hints.Scalar],
        second: hints.Segment[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Segment[hints.Scalar]]:
    result = subtract_segment_from_multisegmental(first, second, segment_cls)
    result.append(second)
    return result


def unite_polygon_with_multipolygon(
        first: hints.Polygon[hints.Scalar],
        second: hints.Multipolygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    second_polygons = second.polygons
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [first, *second_polygons]
    second_boxes = [polygon.bounding_box for polygon in second_polygons]
    do_second_boxes_have_common_continuum = to_boxes_have_common_continuum(
            second_boxes, first_bounding_box
    )
    second_common_continuum_polygons_ids = flags_to_true_indices(
            do_second_boxes_have_common_continuum
    )
    if not second_common_continuum_polygons_ids:
        return [first, *second_polygons]
    second_common_continuum_polygons = [
        second_polygons[index]
        for index in second_common_continuum_polygons_ids
    ]
    operation = ShapedUnion.from_segments_iterables(
            polygon_to_correctly_oriented_segments(first, segment_cls),
            chain.from_iterable(
                    polygon_to_correctly_oriented_segments(polygon,
                                                           segment_cls)
                    for polygon in second_common_continuum_polygons
            )
    )
    result = operation.reduce_events(list(operation), contour_cls, polygon_cls)
    result.extend(
            second_polygons[index]
            for index in flags_to_false_indices(
                    do_second_boxes_have_common_continuum
            )
    )
    return result


def unite_polygon_with_polygon(
        first: hints.Polygon[hints.Scalar],
        second: hints.Polygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [first, second]
    operation = ShapedUnion.from_segments_iterables(
            polygon_to_correctly_oriented_segments(first, segment_cls),
            polygon_to_correctly_oriented_segments(second, segment_cls)
    )
    return operation.reduce_events(list(operation), contour_cls, polygon_cls)


def unite_segment_with_multisegmental(
        first: hints.Segment[hints.Scalar],
        second: _Multisegmental[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Segment[hints.Scalar]]:
    result = subtract_segment_from_multisegmental(second, first, segment_cls)
    result.append(first)
    return result


def unite_segment_with_segment(
        first: hints.Segment[hints.Scalar],
        second: hints.Segment[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Segment[hints.Scalar]]:
    first_start, first_end = to_sorted_pair(first.start, first.end)
    second_start, second_end = to_sorted_pair(second.start, second.end)
    if first_start == second_start and first_end == second_end:
        return [first]
    second_start_orientation = orient(first_end, first_start, second_start)
    second_end_orientation = orient(first_end, first_start, second_end)
    if (second_start_orientation is not Orientation.COLLINEAR
            and second_end_orientation is not Orientation.COLLINEAR
            and second_start_orientation is not second_end_orientation):
        first_start_orientation = orient(second_start, second_end, first_start)
        first_end_orientation = orient(second_start, second_end, first_end)
        if (first_start_orientation is not Orientation.COLLINEAR
                and first_end_orientation is not Orientation.COLLINEAR
                and first_start_orientation is not first_end_orientation):
            cross_point = to_segments_intersection_point(
                    first_start, first_end, second_start, second_end
            )
            return [segment_cls(first_start, cross_point),
                    segment_cls(cross_point, first_end),
                    segment_cls(second_start, cross_point),
                    segment_cls(cross_point, second_end)]
    elif (second_start_orientation is Orientation.COLLINEAR
          and second_end_orientation is Orientation.COLLINEAR
          and second_start <= first_end and first_start <= second_end):
        return [segment_cls(min(first_start, second_start),
                            max(first_end, second_end))]
    return [first, second]
