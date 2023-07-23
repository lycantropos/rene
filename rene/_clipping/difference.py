import typing as t
from itertools import chain

from rene import hints
from rene._utils import (do_boxes_have_no_common_area,
                         flags_to_false_indices,
                         flags_to_true_indices,
                         merge_boxes,
                         polygon_to_correctly_oriented_segments,
                         to_boxes_have_common_area,
                         to_boxes_ids_with_common_area)
from . import shaped
from .event import Event


class ShapedDifference(shaped.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return (self._is_outside_left_event(event)
                if self._is_left_event_from_first_operand(event)
                else (self._is_inside_left_event(event)
                      or self._is_common_polyline_component_left_event(event)))


def subtract_polygon_from_polygon(
        minuend: hints.Polygon[hints.Scalar],
        subtrahend: hints.Polygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    if do_boxes_have_no_common_area(minuend_bounding_box,
                                    subtrahend_bounding_box):
        return [minuend]
    minuend_max_x, minuend_min_x = (minuend_bounding_box.max_x,
                                    minuend_bounding_box.min_x)
    operation = ShapedDifference.from_segments_iterables(
            polygon_to_correctly_oriented_segments(minuend, segment_cls),
            (segment
             for segment in polygon_to_correctly_oriented_segments(subtrahend,
                                                                   segment_cls)
             if (minuend_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= minuend_max_x))
    )
    minuend_max_x = minuend_bounding_box.max_x
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        events.append(event)
    return operation.reduce_events(events, contour_cls, polygon_cls)


def subtract_polygon_from_polygons(
        minuend: t.Sequence[hints.Polygon[hints.Scalar]],
        subtrahend: hints.Polygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    minuend_boxes = [polygon.bounding_box for polygon in minuend]
    minuend_bounding_box, subtrahend_bounding_box = (
        merge_boxes(minuend_boxes), subtrahend.bounding_box
    )
    if do_boxes_have_no_common_area(minuend_bounding_box,
                                    subtrahend_bounding_box):
        return [*minuend]
    minuend_boxes_have_common_area = to_boxes_have_common_area(
            minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_area_polygons_ids = flags_to_true_indices(
            minuend_boxes_have_common_area
    )
    if not minuend_common_area_polygons_ids:
        return [*minuend]
    minuend_common_area_polygons = [
        minuend[polygon_id]
        for polygon_id in minuend_common_area_polygons_ids
    ]
    minuend_max_x = max(minuend_boxes[polygon_id].max_x
                        for polygon_id in minuend_common_area_polygons_ids)
    minuend_min_x = min(minuend_boxes[polygon_id].min_x
                        for polygon_id in minuend_common_area_polygons_ids)
    operation = ShapedDifference.from_segments_iterables(
            chain.from_iterable(
                    polygon_to_correctly_oriented_segments(polygon,
                                                           segment_cls)
                    for polygon in minuend_common_area_polygons
            ),
            (segment
             for segment in polygon_to_correctly_oriented_segments(subtrahend,
                                                                   segment_cls)
             if (minuend_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= minuend_max_x))
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        events.append(event)
    result = operation.reduce_events(events, contour_cls, polygon_cls)
    result.extend(
            minuend[index]
            for index in flags_to_false_indices(minuend_boxes_have_common_area)
    )
    return result


def subtract_polygons_from_polygon(
        minuend: hints.Polygon[hints.Scalar],
        subtrahend: t.Sequence[hints.Polygon[hints.Scalar]],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    subtrahend_boxes = [polygon.bounding_box for polygon in subtrahend]
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box, merge_boxes(subtrahend_boxes)
    )
    if do_boxes_have_no_common_area(minuend_bounding_box,
                                    subtrahend_bounding_box):
        return [minuend]
    subtrahend_common_area_polygons_ids = to_boxes_ids_with_common_area(
            subtrahend_boxes, minuend_bounding_box
    )
    if not subtrahend_common_area_polygons_ids:
        return [minuend]
    subtrahend_common_area_polygons = [
        subtrahend[polygon_id]
        for polygon_id in subtrahend_common_area_polygons_ids
    ]
    minuend_max_x = minuend_bounding_box.max_x
    minuend_min_x = minuend_bounding_box.min_x
    operation = ShapedDifference.from_segments_iterables(
            polygon_to_correctly_oriented_segments(minuend, segment_cls),
            (segment
             for polygon in subtrahend_common_area_polygons
             for segment in polygon_to_correctly_oriented_segments(polygon,
                                                                   segment_cls)
             if (minuend_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= minuend_max_x))
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        events.append(event)
    return operation.reduce_events(events, contour_cls, polygon_cls)


def subtract_polygons_from_polygons(
        minuend: t.Sequence[hints.Polygon[hints.Scalar]],
        subtrahend: t.Sequence[hints.Polygon[hints.Scalar]],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    minuend_boxes = [polygon.bounding_box for polygon in minuend]
    subtrahend_boxes = [polygon.bounding_box for polygon in subtrahend]
    minuend_bounding_box, subtrahend_bounding_box = (
        merge_boxes(minuend_boxes), merge_boxes(subtrahend_boxes)
    )
    if do_boxes_have_no_common_area(minuend_bounding_box,
                                    subtrahend_bounding_box):
        return [*minuend]
    minuend_boxes_have_common_area = to_boxes_have_common_area(
            minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_area_polygons_ids = flags_to_true_indices(
            minuend_boxes_have_common_area
    )
    if not minuend_common_area_polygons_ids:
        return [*minuend]
    subtrahend_common_area_polygons_ids = to_boxes_ids_with_common_area(
            subtrahend_boxes, minuend_bounding_box
    )
    if not subtrahend_common_area_polygons_ids:
        return [*minuend]
    minuend_common_area_polygons = [
        minuend[polygon_id]
        for polygon_id in minuend_common_area_polygons_ids
    ]
    subtrahend_common_area_polygons = [
        subtrahend[polygon_id]
        for polygon_id in subtrahend_common_area_polygons_ids
    ]
    minuend_max_x = max(minuend_boxes[polygon_id].max_x
                        for polygon_id in minuend_common_area_polygons_ids)
    minuend_min_x = min(minuend_boxes[polygon_id].min_x
                        for polygon_id in minuend_common_area_polygons_ids)
    operation = ShapedDifference.from_segments_iterables(
            chain.from_iterable(
                    polygon_to_correctly_oriented_segments(polygon,
                                                           segment_cls)
                    for polygon in minuend_common_area_polygons
            ),
            (segment
             for polygon in subtrahend_common_area_polygons
             for segment in polygon_to_correctly_oriented_segments(polygon,
                                                                   segment_cls)
             if (minuend_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= minuend_max_x))
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        events.append(event)
    result = operation.reduce_events(events, contour_cls, polygon_cls)
    result.extend(
            minuend[index]
            for index in flags_to_false_indices(minuend_boxes_have_common_area)
    )
    return result
