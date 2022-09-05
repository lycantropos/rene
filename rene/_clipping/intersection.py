from typing import List

from rene._utils import (are_boxes_uncoupled,
                         merge_boxes,
                         to_boxes_ids_coupled_with_box)
from rene.hints import (Multipolygon,
                        Polygon)
from .event import Event
from .operation import Operation


class Intersection(Operation):
    def _detect_if_left_event_from_result(self, event: Event) -> bool:
        return (self._is_inside_left_event(event)
                or not self._is_left_event_from_first_operand(event)
                and self._is_common_region_boundary_left_event(event))


def intersect_multipolygons(first: Multipolygon,
                            second: Multipolygon) -> List[Polygon]:
    first_polygons, second_polygons = first.polygons, second.polygons
    first_bounding_boxes = [polygon.bounding_box for polygon in first_polygons]
    second_bounding_boxes = [polygon.bounding_box
                             for polygon in second_polygons]
    first_bounding_box, second_bounding_box = (
        merge_boxes(first_bounding_boxes), merge_boxes(second_bounding_boxes)
    )
    if are_boxes_uncoupled(first_bounding_box, second_bounding_box):
        return []
    first_coupled_polygons_ids = to_boxes_ids_coupled_with_box(
            first_bounding_boxes, second_bounding_box
    )
    if not first_coupled_polygons_ids:
        return []
    second_coupled_polygons_ids = to_boxes_ids_coupled_with_box(
            second_bounding_boxes, first_bounding_box
    )
    if not second_coupled_polygons_ids:
        return []
    first_coupled_polygons = [first_polygons[polygon_id]
                              for polygon_id in first_coupled_polygons_ids]
    second_coupled_polygons = [second_polygons[polygon_id]
                               for polygon_id in second_coupled_polygons_ids]
    min_max_x = min(max(first_bounding_boxes[polygon_id].max_x
                        for polygon_id in first_coupled_polygons_ids),
                    max(second_bounding_boxes[polygon_id].max_x
                        for polygon_id in second_coupled_polygons_ids))
    operation = Intersection.from_multisegmentals_sequences(
            first_coupled_polygons, second_coupled_polygons
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return operation.reduce_events(events, type(first_polygons[0].border),
                                   type(first_polygons[0]))


def intersect_multipolygon_with_polygon(first: Multipolygon,
                                        second: Polygon) -> List[Polygon]:
    first_polygons = first.polygons
    first_bounding_boxes = [polygon.bounding_box for polygon in first_polygons]
    first_bounding_box, second_bounding_box = (
        merge_boxes(first_bounding_boxes), second.bounding_box
    )
    if are_boxes_uncoupled(first_bounding_box, second_bounding_box):
        return []
    first_coupled_polygons_ids = to_boxes_ids_coupled_with_box(
            first_bounding_boxes, second_bounding_box
    )
    if not first_coupled_polygons_ids:
        return []
    first_coupled_polygons = [first_polygons[polygon_id]
                              for polygon_id in first_coupled_polygons_ids]
    min_max_x = min(max(first_bounding_boxes[polygon_id].max_x
                        for polygon_id in first_coupled_polygons_ids),
                    second_bounding_box.max_x)
    operation = Intersection.from_multisegmentals_sequence_multisegmental(
            first_coupled_polygons, second
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return operation.reduce_events(events, type(first_polygons[0].border),
                                   type(first_polygons[0]))


def intersect_polygon_with_multipolygon(first: Polygon,
                                        second: Multipolygon) -> List[Polygon]:
    second_polygons = second.polygons
    second_bounding_boxes = [polygon.bounding_box
                             for polygon in second_polygons]
    first_bounding_box, second_bounding_box = (
        first.bounding_box, merge_boxes(second_bounding_boxes)
    )
    if are_boxes_uncoupled(first_bounding_box, second_bounding_box):
        return []
    second_coupled_polygons_ids = to_boxes_ids_coupled_with_box(
            second_bounding_boxes, first_bounding_box
    )
    if not second_coupled_polygons_ids:
        return []
    second_coupled_polygons = [second_polygons[polygon_id]
                               for polygon_id in second_coupled_polygons_ids]
    min_max_x = min(first_bounding_box.max_x,
                    max(second_bounding_boxes[polygon_id].max_x
                        for polygon_id in second_coupled_polygons_ids))
    operation = Intersection.from_multisegmental_multisegmentals_sequence(
            first, second_coupled_polygons
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return operation.reduce_events(events, type(first.border), type(first))


def intersect_polygons(first: Polygon, second: Polygon) -> List[Polygon]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if are_boxes_uncoupled(first_bounding_box, second_bounding_box):
        return []
    min_max_x = min(first_bounding_box.max_x, second_bounding_box.max_x)
    operation = Intersection.from_multisegmentals(first, second)
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return operation.reduce_events(events, type(first.border), type(first))
