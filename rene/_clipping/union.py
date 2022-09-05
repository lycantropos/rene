from typing import List

from rene._utils import (are_boxes_uncoupled,
                         flags_to_false_indices,
                         flags_to_true_indices,
                         merge_boxes,
                         to_are_boxes_coupled_with_box)
from rene.hints import (Multipolygon,
                        Polygon)
from .event import Event
from .operation import Operation


class Union(Operation):
    def _detect_if_left_event_from_result(self, event: Event) -> bool:
        return (self._is_outside_left_event(event)
                or (not self._is_left_event_from_first_operand(event)
                    and self._is_common_region_boundary_left_event(event)))


def unite_multipolygon_with_polygon(first: Multipolygon,
                                    second: Polygon) -> List[Polygon]:
    first_polygons = first.polygons
    first_bounding_boxes = [polygon.bounding_box for polygon in first_polygons]
    first_bounding_box, second_bounding_box = (
        merge_boxes(first_bounding_boxes), second.bounding_box
    )
    if are_boxes_uncoupled(first_bounding_box, second_bounding_box):
        return [*first.polygons, second]
    are_first_boxes_coupled = to_are_boxes_coupled_with_box(
            first_bounding_boxes, second_bounding_box
    )
    first_coupled_polygons_ids = flags_to_true_indices(are_first_boxes_coupled)
    if not first_coupled_polygons_ids:
        return [*first.polygons, second]
    first_coupled_polygons = [first_polygons[index]
                              for index in first_coupled_polygons_ids]
    operation = Union.from_multisegmentals_sequence_multisegmental(
            first_coupled_polygons, second
    )
    result = operation.reduce_events(list(operation),
                                     type(first_polygons[0].border),
                                     type(first_polygons[0]))
    result.extend(first_polygons[index]
                  for index in flags_to_false_indices(are_first_boxes_coupled))
    return result


def unite_multipolygons(first: Multipolygon,
                        second: Multipolygon) -> List[Polygon]:
    first_polygons, second_polygons = first.polygons, second.polygons
    first_bounding_boxes = [polygon.bounding_box for polygon in first_polygons]
    second_bounding_boxes = [polygon.bounding_box
                             for polygon in second_polygons]
    first_bounding_box, second_bounding_box = (
        merge_boxes(first_bounding_boxes), merge_boxes(second_bounding_boxes)
    )
    if are_boxes_uncoupled(first_bounding_box, second_bounding_box):
        return [*first.polygons, *second.polygons]
    are_first_boxes_coupled = to_are_boxes_coupled_with_box(
            first_bounding_boxes, second_bounding_box
    )
    first_coupled_polygons_ids = flags_to_true_indices(are_first_boxes_coupled)
    if not first_coupled_polygons_ids:
        return [*first.polygons, *second.polygons]
    are_second_boxes_coupled = to_are_boxes_coupled_with_box(
            second_bounding_boxes, first_bounding_box
    )
    second_coupled_polygons_ids = flags_to_true_indices(
            are_second_boxes_coupled
    )
    if not second_coupled_polygons_ids:
        return [*first.polygons, *second.polygons]
    first_coupled_polygons = [first_polygons[index]
                              for index in first_coupled_polygons_ids]
    second_coupled_polygons = [second_polygons[index]
                               for index in second_coupled_polygons_ids]
    operation = Union.from_multisegmentals_sequences(first_coupled_polygons,
                                                     second_coupled_polygons)
    result = operation.reduce_events(list(operation),
                                     type(first_polygons[0].border),
                                     type(first_polygons[0]))
    result.extend(first_polygons[index]
                  for index in flags_to_false_indices(are_first_boxes_coupled))
    result.extend(
            second_polygons[index]
            for index in flags_to_false_indices(are_second_boxes_coupled)
    )
    return result


def unite_polygon_with_multipolygon(first: Polygon,
                                    second: Multipolygon) -> List[Polygon]:
    second_polygons = second.polygons
    second_bounding_boxes = [polygon.bounding_box for polygon in
                             second_polygons]
    first_bounding_box, second_bounding_box = (
        first.bounding_box, merge_boxes(second_bounding_boxes)
    )
    if are_boxes_uncoupled(first_bounding_box, second_bounding_box):
        return [first, *second.polygons]
    are_second_boxes_coupled = to_are_boxes_coupled_with_box(
            second_bounding_boxes, first_bounding_box
    )
    second_coupled_polygons_ids = flags_to_true_indices(
            are_second_boxes_coupled
    )
    if not second_coupled_polygons_ids:
        return [first, *second.polygons]
    second_coupled_polygons = [second_polygons[index]
                               for index in second_coupled_polygons_ids]
    operation = Union.from_multisegmental_multisegmentals_sequence(
            second, second_coupled_polygons
    )
    result = operation.reduce_events(list(operation), type(first.border),
                                     type(first))
    result.extend(
            second_polygons[index]
            for index in flags_to_false_indices(are_second_boxes_coupled)
    )
    return result


def unite_polygons(first: Polygon, second: Polygon) -> List[Polygon]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if are_boxes_uncoupled(first_bounding_box, second_bounding_box):
        return [first, second]
    operation = Union.from_multisegmentals(first, second)
    return operation.reduce_events(list(operation), type(first.border),
                                   type(first))
