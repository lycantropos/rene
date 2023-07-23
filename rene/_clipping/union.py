import typing as t
from itertools import chain

from rene import hints
from rene._utils import (do_boxes_have_no_common_continuum,
                         flags_to_false_indices,
                         flags_to_true_indices,
                         merge_boxes,
                         polygon_to_correctly_oriented_segments,
                         to_boxes_have_common_continuum)
from . import shaped
from .event import Event


class ShapedUnion(shaped.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return (self._is_outside_left_event(event)
                or (not self._is_left_event_from_first_operand(event)
                    and self._is_common_region_boundary_left_event(event)))


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


def unite_polygon_with_polygons(
        first: hints.Polygon[hints.Scalar],
        second: t.Sequence[hints.Polygon[hints.Scalar]],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    second_boxes = [polygon.bounding_box for polygon in second]
    first_bounding_box, second_bounding_box = (
        first.bounding_box, merge_boxes(second_boxes)
    )
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [first, *second]
    do_second_boxes_have_common_continuum = to_boxes_have_common_continuum(
            second_boxes, first_bounding_box
    )
    second_common_continuum_polygons_ids = flags_to_true_indices(
            do_second_boxes_have_common_continuum
    )
    if not second_common_continuum_polygons_ids:
        return [first, *second]
    second_common_continuum_polygons = [
        second[index]
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
            second[index]
            for index in flags_to_false_indices(
                    do_second_boxes_have_common_continuum
            )
    )
    return result


def unite_polygons_with_polygon(
        first: t.Sequence[hints.Polygon[hints.Scalar]],
        second: hints.Polygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    first_boxes = [polygon.bounding_box for polygon in first]
    first_bounding_box, second_bounding_box = (merge_boxes(first_boxes),
                                               second.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [*first, second]
    first_boxes_have_common_continuum = to_boxes_have_common_continuum(
            first_boxes, second_bounding_box
    )
    first_common_continuum_polygons_ids = flags_to_true_indices(
            first_boxes_have_common_continuum
    )
    if not first_common_continuum_polygons_ids:
        return [*first, second]
    first_common_continuum_polygons = [
        first[index]
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
            first[index]
            for index in flags_to_false_indices(
                    first_boxes_have_common_continuum
            )
    )
    return result


def unite_polygons_with_polygons(
        first: t.Sequence[hints.Polygon[hints.Scalar]],
        second: t.Sequence[hints.Polygon[hints.Scalar]],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    first_boxes = [polygon.bounding_box for polygon in first]
    second_boxes = [polygon.bounding_box for polygon in second]
    first_bounding_box, second_bounding_box = (merge_boxes(first_boxes),
                                               merge_boxes(second_boxes))
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [*first, *second]
    do_first_boxes_have_common_continuum = to_boxes_have_common_continuum(
            first_boxes, second_bounding_box
    )
    first_common_continuum_polygons_ids = flags_to_true_indices(
            do_first_boxes_have_common_continuum
    )
    if not first_common_continuum_polygons_ids:
        return [*first, *second]
    do_second_boxes_have_common_continuum = to_boxes_have_common_continuum(
            second_boxes, first_bounding_box
    )
    second_common_continuum_polygons_ids = flags_to_true_indices(
            do_second_boxes_have_common_continuum
    )
    if not second_common_continuum_polygons_ids:
        return [*first, *second]
    first_common_continuum_polygons = [
        first[index]
        for index in first_common_continuum_polygons_ids
    ]
    second_common_continuum_polygons = [
        second[index]
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
            first[index]
            for index in flags_to_false_indices(
                    do_first_boxes_have_common_continuum
            )
    )
    result.extend(
            second[index]
            for index in flags_to_false_indices(
                    do_second_boxes_have_common_continuum
            )
    )
    return result
