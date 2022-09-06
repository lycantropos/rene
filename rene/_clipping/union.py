from typing import List

from rene._utils import (do_boxes_have_no_common_continuum,
                         flags_to_false_indices,
                         flags_to_true_indices,
                         merge_boxes,
                         to_boxes_have_common_continuum)
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
    first_boxes = [polygon.bounding_box for polygon in first_polygons]
    first_bounding_box, second_bounding_box = (
        merge_boxes(first_boxes), second.bounding_box
    )
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [*first.polygons, second]
    first_boxes_have_common_continuum = to_boxes_have_common_continuum(
            first_boxes, second_bounding_box
    )
    first_common_continuum_polygons_ids = flags_to_true_indices(
            first_boxes_have_common_continuum
    )
    if not first_common_continuum_polygons_ids:
        return [*first.polygons, second]
    first_common_continuum_polygons = [
        first_polygons[index]
        for index in first_common_continuum_polygons_ids
    ]
    operation = (
        Union.from_multisegmentals_sequence_multisegmental(
                first_common_continuum_polygons, second
        )
    )
    result = operation.reduce_events(list(operation),
                                     type(first_polygons[0].border),
                                     type(first_polygons[0]))
    result.extend(
            first_polygons[index]
            for index in flags_to_false_indices(
                    first_boxes_have_common_continuum
            )
    )
    return result


def unite_multipolygons(first: Multipolygon,
                        second: Multipolygon) -> List[Polygon]:
    first_polygons, second_polygons = first.polygons, second.polygons
    first_boxes = [polygon.bounding_box for polygon in first_polygons]
    second_boxes = [polygon.bounding_box
                             for polygon in second_polygons]
    first_bounding_box, second_bounding_box = (
        merge_boxes(first_boxes), merge_boxes(second_boxes)
    )
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [*first.polygons, *second.polygons]
    do_first_boxes_have_common_continuum = to_boxes_have_common_continuum(
            first_boxes, second_bounding_box
    )
    first_common_continuum_polygons_ids = flags_to_true_indices(
            do_first_boxes_have_common_continuum
    )
    if not first_common_continuum_polygons_ids:
        return [*first.polygons, *second.polygons]
    do_second_boxes_have_common_continuum = to_boxes_have_common_continuum(
            second_boxes, first_bounding_box
    )
    second_common_continuum_polygons_ids = flags_to_true_indices(
            do_second_boxes_have_common_continuum
    )
    if not second_common_continuum_polygons_ids:
        return [*first.polygons, *second.polygons]
    first_common_continuum_polygons = [
        first_polygons[index]
        for index in first_common_continuum_polygons_ids
    ]
    second_common_continuum_polygons = [
        second_polygons[index]
        for index in second_common_continuum_polygons_ids
    ]
    operation = Union.from_multisegmentals_sequences(
            first_common_continuum_polygons, second_common_continuum_polygons
    )
    result = operation.reduce_events(list(operation),
                                     type(first_polygons[0].border),
                                     type(first_polygons[0]))
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


def unite_polygon_with_multipolygon(first: Polygon,
                                    second: Multipolygon) -> List[Polygon]:
    second_polygons = second.polygons
    second_boxes = [polygon.bounding_box
                             for polygon in second_polygons]
    first_bounding_box, second_bounding_box = (
        first.bounding_box, merge_boxes(second_boxes)
    )
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [first, *second.polygons]
    do_second_boxes_have_common_continuum = to_boxes_have_common_continuum(
            second_boxes, first_bounding_box
    )
    second_common_continuum_polygons_ids = flags_to_true_indices(
            do_second_boxes_have_common_continuum
    )
    if not second_common_continuum_polygons_ids:
        return [first, *second.polygons]
    second_common_continuum_polygons = [
        second_polygons[index]
        for index in second_common_continuum_polygons_ids
    ]
    operation = (
        Union.from_multisegmental_multisegmentals_sequence(
                first, second_common_continuum_polygons
        )
    )
    result = operation.reduce_events(list(operation), type(first.border),
                                     type(first))
    result.extend(
            second_polygons[index]
            for index in flags_to_false_indices(
                    do_second_boxes_have_common_continuum
            )
    )
    return result


def unite_polygons(first: Polygon, second: Polygon) -> List[Polygon]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [first, second]
    operation = Union.from_multisegmentals(first, second)
    return operation.reduce_events(list(operation), type(first.border),
                                   type(first))
