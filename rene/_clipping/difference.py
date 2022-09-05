from typing import List

from rene._utils import (are_boxes_uncoupled,
                         flags_to_false_indices,
                         flags_to_true_indices,
                         merge_boxes,
                         to_are_boxes_coupled_with_box,
                         to_boxes_ids_coupled_with_box)
from rene.hints import (Multipolygon,
                        Polygon)
from .event import Event
from .operation import Operation


class Difference(Operation):
    def _detect_if_left_event_from_result(self, event: Event) -> bool:
        return (self._is_outside_left_event(event)
                if self._is_left_event_from_first_operand(event)
                else (self._is_inside_left_event(event)
                      or self._is_common_polyline_component_left_event(event)))


def subtract_multipolygons(first: Multipolygon,
                           second: Multipolygon) -> List[Polygon]:
    first_polygons, second_polygons = first.polygons, second.polygons
    first_bounding_boxes = [polygon.bounding_box for polygon in first_polygons]
    second_bounding_boxes = [polygon.bounding_box
                             for polygon in second_polygons]
    first_bounding_box, second_bounding_box = (
        merge_boxes(first_bounding_boxes), merge_boxes(second_bounding_boxes)
    )
    if are_boxes_uncoupled(first_bounding_box, second_bounding_box):
        return list(first.polygons)
    are_first_bounding_boxes_coupled = to_are_boxes_coupled_with_box(
            first_bounding_boxes, second_bounding_box
    )
    first_coupled_polygons_ids = flags_to_true_indices(
            are_first_bounding_boxes_coupled
    )
    if not first_coupled_polygons_ids:
        return list(first.polygons)
    second_coupled_polygons_ids = to_boxes_ids_coupled_with_box(
            second_bounding_boxes, first_bounding_box
    )
    if not second_coupled_polygons_ids:
        return list(first.polygons)
    first_coupled_polygons = [first_polygons[polygon_id]
                              for polygon_id in first_coupled_polygons_ids]
    second_coupled_polygons = [second_polygons[polygon_id]
                               for polygon_id in second_coupled_polygons_ids]
    max_x = max(first_bounding_boxes[polygon_id].max_x
                for polygon_id in first_coupled_polygons_ids)
    operation = Difference.from_multisegmentals_sequences(
            first_coupled_polygons, second_coupled_polygons
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > max_x:
            break
        events.append(event)
    result = operation.reduce_events(events, type(first_polygons[0].border),
                                     type(first_polygons[0]))
    result.extend(
            first_polygons[index]
            for index in flags_to_false_indices(
                    are_first_bounding_boxes_coupled
            )
    )
    return result


def subtract_polygon_from_multipolygon(first: Multipolygon,
                                       second: Polygon) -> List[Polygon]:
    first_polygons = first.polygons
    first_bounding_boxes = [polygon.bounding_box for polygon in first_polygons]
    first_bounding_box, second_bounding_box = (
        merge_boxes(first_bounding_boxes), second.bounding_box
    )
    if are_boxes_uncoupled(first_bounding_box, second_bounding_box):
        return list(first.polygons)
    are_first_bounding_boxes_coupled = to_are_boxes_coupled_with_box(
            first_bounding_boxes, second_bounding_box
    )
    first_coupled_polygons_ids = flags_to_true_indices(
            are_first_bounding_boxes_coupled
    )
    if not first_coupled_polygons_ids:
        return list(first.polygons)
    first_coupled_polygons = [first_polygons[polygon_id]
                              for polygon_id in first_coupled_polygons_ids]
    max_x = max(first_bounding_boxes[polygon_id].max_x
                for polygon_id in first_coupled_polygons_ids)
    operation = Difference.from_multisegmentals_sequence_multisegmental(
            first_coupled_polygons, second
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > max_x:
            break
        events.append(event)
    result = operation.reduce_events(events, type(first_polygons[0].border),
                                     type(first_polygons[0]))
    result.extend(
            first_polygons[index]
            for index in flags_to_false_indices(
                    are_first_bounding_boxes_coupled
            )
    )
    return result


def subtract_multipolygon_from_polygon(first: Polygon,
                                       second: Multipolygon) -> List[Polygon]:
    second_polygons = second.polygons
    second_bounding_boxes = [polygon.bounding_box
                             for polygon in second_polygons]
    first_bounding_box, second_bounding_box = (
        first.bounding_box, merge_boxes(second_bounding_boxes)
    )
    if are_boxes_uncoupled(first_bounding_box, second_bounding_box):
        return [first]
    second_coupled_polygons_ids = to_boxes_ids_coupled_with_box(
            second_bounding_boxes, first_bounding_box
    )
    if not second_coupled_polygons_ids:
        return [first]
    second_coupled_polygons = [second_polygons[polygon_id]
                               for polygon_id in second_coupled_polygons_ids]
    max_x = first_bounding_box.max_x
    operation = Difference.from_multisegmental_multisegmentals_sequence(
            first, second_coupled_polygons
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > max_x:
            break
        events.append(event)
    return operation.reduce_events(events, type(first.border), type(first))


def subtract_polygons(first: Polygon, second: Polygon) -> List[Polygon]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if (first_bounding_box.touches(second_bounding_box)
            or first_bounding_box.touches(second_bounding_box)):
        return [first]
    operation = Difference.from_multisegmentals(first, second)
    max_x = first_bounding_box.max_x
    events = []
    for event in operation:
        if operation.to_event_start(event).x > max_x:
            break
        events.append(event)
    return operation.reduce_events(list(operation), type(first.border),
                                   type(first))
