from typing import List

from rene import hints
from rene._utils import (do_boxes_have_no_common_area,
                         merge_boxes,
                         to_boxes_ids_with_common_area)
from . import shaped
from .event import Event


class ShapedIntersection(shaped.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return (self._is_inside_left_event(event)
                or not self._is_left_event_from_first_operand(event)
                and self._is_common_region_boundary_left_event(event))


def intersect_multipolygons(first: hints.Multipolygon[hints.Scalar],
                            second: hints.Multipolygon[hints.Scalar],
                            /) -> List[hints.Polygon[hints.Scalar]]:
    first_polygons, second_polygons = first.polygons, second.polygons
    first_boxes = [polygon.bounding_box for polygon in first_polygons]
    second_boxes = [polygon.bounding_box for polygon in second_polygons]
    first_bounding_box, second_bounding_box = (merge_boxes(first_boxes),
                                               merge_boxes(second_boxes))
    if do_boxes_have_no_common_area(first_bounding_box, second_bounding_box):
        return []
    first_common_area_polygons_ids = to_boxes_ids_with_common_area(
            first_boxes, second_bounding_box
    )
    if not first_common_area_polygons_ids:
        return []
    second_common_area_polygons_ids = to_boxes_ids_with_common_area(
            second_boxes, first_bounding_box
    )
    if not second_common_area_polygons_ids:
        return []
    first_common_area_polygons = [
        first_polygons[polygon_id]
        for polygon_id in first_common_area_polygons_ids
    ]
    second_common_area_polygons = [
        second_polygons[polygon_id]
        for polygon_id in second_common_area_polygons_ids
    ]
    max_min_x = max(min(first_boxes[polygon_id].min_x
                        for polygon_id in first_common_area_polygons_ids),
                    min(second_boxes[polygon_id].min_x
                        for polygon_id in second_common_area_polygons_ids))
    min_max_x = min(max(first_boxes[polygon_id].max_x
                        for polygon_id in first_common_area_polygons_ids),
                    max(second_boxes[polygon_id].max_x
                        for polygon_id in second_common_area_polygons_ids))
    operation = ShapedIntersection.from_segments_iterables(
            (segment
             for polygon in first_common_area_polygons
             for segment in polygon.segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for polygon in second_common_area_polygons
             for segment in polygon.segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return operation.reduce_events(events, type(first_polygons[0].border),
                                   type(first_polygons[0]))


def intersect_multipolygon_with_polygon(
        first: hints.Multipolygon[hints.Scalar],
        second: hints.Polygon[hints.Scalar],
        /
) -> List[hints.Polygon[hints.Scalar]]:
    first_polygons = first.polygons
    first_boxes = [polygon.bounding_box for polygon in first_polygons]
    first_bounding_box, second_bounding_box = (merge_boxes(first_boxes),
                                               second.bounding_box)
    if do_boxes_have_no_common_area(first_bounding_box, second_bounding_box):
        return []
    first_common_area_polygons_ids = to_boxes_ids_with_common_area(
            first_boxes, second_bounding_box
    )
    if not first_common_area_polygons_ids:
        return []
    first_common_area_polygons = [
        first_polygons[polygon_id]
        for polygon_id in first_common_area_polygons_ids
    ]
    max_min_x = max(min(first_boxes[polygon_id].min_x
                        for polygon_id in first_common_area_polygons_ids),
                    second_bounding_box.min_x)
    min_max_x = min(max(first_boxes[polygon_id].max_x
                        for polygon_id in first_common_area_polygons_ids),
                    second_bounding_box.max_x)
    operation = ShapedIntersection.from_segments_iterables(
            (segment
             for polygon in first_common_area_polygons
             for segment in polygon.segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for segment in second.segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x))
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return operation.reduce_events(events, type(first_polygons[0].border),
                                   type(first_polygons[0]))


def intersect_polygon_with_multipolygon(
        first: hints.Polygon[hints.Scalar],
        second: hints.Multipolygon[hints.Scalar],
        /
) -> List[hints.Polygon[hints.Scalar]]:
    second_polygons = second.polygons
    second_boxes = [polygon.bounding_box for polygon in second_polygons]
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               merge_boxes(second_boxes))
    if do_boxes_have_no_common_area(first_bounding_box, second_bounding_box):
        return []
    second_common_area_polygons_ids = to_boxes_ids_with_common_area(
            second_boxes, first_bounding_box
    )
    if not second_common_area_polygons_ids:
        return []
    second_common_area_polygons = [
        second_polygons[polygon_id]
        for polygon_id in second_common_area_polygons_ids
    ]
    max_min_x = max(first_bounding_box.min_x,
                    min(second_boxes[polygon_id].min_x
                        for polygon_id in second_common_area_polygons_ids))
    min_max_x = min(first_bounding_box.max_x,
                    max(second_boxes[polygon_id].max_x
                        for polygon_id in second_common_area_polygons_ids))
    operation = ShapedIntersection.from_segments_iterables(
            (segment
             for segment in first.segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for polygon in second_common_area_polygons
             for segment in polygon.segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x))
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return operation.reduce_events(events, type(first.border), type(first))


def intersect_polygons(first: hints.Polygon[hints.Scalar],
                       second: hints.Polygon[hints.Scalar],
                       /) -> List[hints.Polygon[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_area(first_bounding_box, second_bounding_box):
        return []
    max_min_x = max(first_bounding_box.min_x, second_bounding_box.min_x)
    min_max_x = min(first_bounding_box.max_x, second_bounding_box.max_x)
    operation = ShapedIntersection.from_segments_iterables(
            (segment
             for segment in first.segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for segment in second.segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x))
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return operation.reduce_events(events, type(first.border), type(first))
