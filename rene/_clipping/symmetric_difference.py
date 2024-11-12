from itertools import chain, groupby
from typing import Union

from rene import hints
from rene._hints import Orienteer, SegmentsIntersector
from rene._utils import (
    collect_maybe_empty_polygons,
    collect_maybe_empty_segments,
    do_boxes_have_no_common_continuum,
    flags_to_false_indices,
    flags_to_true_indices,
    polygon_to_correctly_oriented_segments,
    to_boxes_have_common_continuum,
    to_sorted_pair,
)
from rene.enums import Orientation

from . import linear, shaped
from .event import Event, is_event_right
from .utils import has_two_or_more_elements


class LinearSymmetricDifference(linear.Operation[hints.Scalar]):
    def reduce_events(
        self,
        events: list[Event],
        segment_cls: type[hints.Segment[hints.Scalar]],
        /,
    ) -> list[hints.Segment[hints.Scalar]]:
        return [
            segment_cls(start, end)
            for (start, end), equal_segment_events in groupby(
                events, key=self._to_event_endpoints
            )
            if not has_two_or_more_elements(equal_segment_events)
        ]


class ShapedSymmetricDifference(shaped.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return not self._is_left_event_overlapping(event)


_Multisegmental = Union[
    hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar]
]


def symmetric_subtract_polygon_from_polygon(
    minuend: hints.Polygon[hints.Scalar],
    subtrahend: hints.Polygon[hints.Scalar],
    contour_cls: type[hints.Contour[hints.Scalar]],
    empty_cls: type[hints.Empty[hints.Scalar]],
    multipolygon_cls: type[hints.Multipolygon[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    polygon_cls: type[hints.Polygon[hints.Scalar]],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[
    hints.Empty[hints.Scalar],
    hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar],
]:
    first_bounding_box, second_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    if do_boxes_have_no_common_continuum(
        first_bounding_box, second_bounding_box
    ):
        return multipolygon_cls([minuend, subtrahend])
    operation = ShapedSymmetricDifference.from_segments_iterables(
        polygon_to_correctly_oriented_segments(
            minuend, orienteer, segment_cls
        ),
        polygon_to_correctly_oriented_segments(
            subtrahend, orienteer, segment_cls
        ),
        orienteer,
        segments_intersector,
    )
    return collect_maybe_empty_polygons(
        operation.reduce_events(list(operation), contour_cls, polygon_cls),
        empty_cls,
        multipolygon_cls,
    )


def symmetric_subtract_polygon_from_multipolygon(
    minuend: hints.Multipolygon[hints.Scalar],
    subtrahend: hints.Polygon[hints.Scalar],
    contour_cls: type[hints.Contour[hints.Scalar]],
    empty_cls: type[hints.Empty[hints.Scalar]],
    multipolygon_cls: type[hints.Multipolygon[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    polygon_cls: type[hints.Polygon[hints.Scalar]],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[
    hints.Empty[hints.Scalar],
    hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    minuend_polygons = minuend.polygons
    if do_boxes_have_no_common_continuum(
        minuend_bounding_box, subtrahend_bounding_box
    ):
        return multipolygon_cls([*minuend_polygons, subtrahend])
    minuend_boxes = [polygon.bounding_box for polygon in minuend_polygons]
    minuend_boxes_have_common_continuum = to_boxes_have_common_continuum(
        minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_continuum_polygons_ids = flags_to_true_indices(
        minuend_boxes_have_common_continuum
    )
    if not minuend_common_continuum_polygons_ids:
        return multipolygon_cls([*minuend_polygons, subtrahend])
    minuend_common_continuum_polygons = [
        minuend_polygons[index]
        for index in minuend_common_continuum_polygons_ids
    ]
    operation = ShapedSymmetricDifference.from_segments_iterables(
        chain.from_iterable(
            polygon_to_correctly_oriented_segments(
                polygon, orienteer, segment_cls
            )
            for polygon in minuend_common_continuum_polygons
        ),
        polygon_to_correctly_oriented_segments(
            subtrahend, orienteer, segment_cls
        ),
        orienteer,
        segments_intersector,
    )
    polygons = operation.reduce_events(
        list(operation), contour_cls, polygon_cls
    )
    polygons.extend(
        minuend_polygons[index]
        for index in flags_to_false_indices(
            minuend_boxes_have_common_continuum
        )
    )
    return collect_maybe_empty_polygons(polygons, empty_cls, multipolygon_cls)


def symmetric_subtract_multipolygon_from_polygon(
    minuend: hints.Polygon[hints.Scalar],
    subtrahend: hints.Multipolygon[hints.Scalar],
    contour_cls: type[hints.Contour[hints.Scalar]],
    empty_cls: type[hints.Empty[hints.Scalar]],
    multipolygon_cls: type[hints.Multipolygon[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    polygon_cls: type[hints.Polygon[hints.Scalar]],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[
    hints.Empty[hints.Scalar],
    hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    subtrahend_polygons = subtrahend.polygons
    if do_boxes_have_no_common_continuum(
        minuend_bounding_box, subtrahend_bounding_box
    ):
        return multipolygon_cls([minuend, *subtrahend_polygons])
    subtrahend_boxes = [
        polygon.bounding_box for polygon in subtrahend_polygons
    ]
    subtrahend_boxes_have_common_continuum = to_boxes_have_common_continuum(
        subtrahend_boxes, minuend_bounding_box
    )
    subtrahend_common_continuum_polygons_ids = flags_to_true_indices(
        subtrahend_boxes_have_common_continuum
    )
    if not subtrahend_common_continuum_polygons_ids:
        return multipolygon_cls([minuend, *subtrahend_polygons])
    subtrahend_common_continuum_polygons = [
        subtrahend_polygons[index]
        for index in subtrahend_common_continuum_polygons_ids
    ]
    operation = ShapedSymmetricDifference.from_segments_iterables(
        polygon_to_correctly_oriented_segments(
            minuend, orienteer, segment_cls
        ),
        chain.from_iterable(
            polygon_to_correctly_oriented_segments(
                polygon, orienteer, segment_cls
            )
            for polygon in subtrahend_common_continuum_polygons
        ),
        orienteer,
        segments_intersector,
    )
    polygons = operation.reduce_events(
        list(operation), contour_cls, polygon_cls
    )
    polygons.extend(
        subtrahend_polygons[index]
        for index in flags_to_false_indices(
            subtrahend_boxes_have_common_continuum
        )
    )
    return collect_maybe_empty_polygons(polygons, empty_cls, multipolygon_cls)


def symmetric_subtract_multipolygon_from_multipolygon(
    minuend: hints.Multipolygon[hints.Scalar],
    subtrahend: hints.Multipolygon[hints.Scalar],
    contour_cls: type[hints.Contour[hints.Scalar]],
    empty_cls: type[hints.Empty[hints.Scalar]],
    multipolygon_cls: type[hints.Multipolygon[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    polygon_cls: type[hints.Polygon[hints.Scalar]],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[
    hints.Empty[hints.Scalar],
    hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    minuend_polygons, subtrahend_polygons = (
        minuend.polygons,
        subtrahend.polygons,
    )
    if do_boxes_have_no_common_continuum(
        minuend_bounding_box, subtrahend_bounding_box
    ):
        return multipolygon_cls([*minuend_polygons, *subtrahend_polygons])
    minuend_boxes = [polygon.bounding_box for polygon in minuend_polygons]
    minuend_boxes_have_common_continuum = to_boxes_have_common_continuum(
        minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_continuum_polygons_ids = flags_to_true_indices(
        minuend_boxes_have_common_continuum
    )
    if not minuend_common_continuum_polygons_ids:
        return multipolygon_cls([*minuend_polygons, *subtrahend_polygons])
    subtrahend_boxes = [
        polygon.bounding_box for polygon in subtrahend_polygons
    ]
    subtrahend_boxes_have_common_continuum = to_boxes_have_common_continuum(
        subtrahend_boxes, minuend_bounding_box
    )
    subtrahend_common_continuum_polygons_ids = flags_to_true_indices(
        subtrahend_boxes_have_common_continuum
    )
    if not subtrahend_common_continuum_polygons_ids:
        return multipolygon_cls([*minuend_polygons, *subtrahend_polygons])
    minuend_common_continuum_polygons = [
        minuend_polygons[index]
        for index in minuend_common_continuum_polygons_ids
    ]
    subtrahend_common_continuum_polygons = [
        subtrahend_polygons[index]
        for index in subtrahend_common_continuum_polygons_ids
    ]
    operation = ShapedSymmetricDifference.from_segments_iterables(
        chain.from_iterable(
            polygon_to_correctly_oriented_segments(
                polygon, orienteer, segment_cls
            )
            for polygon in minuend_common_continuum_polygons
        ),
        chain.from_iterable(
            polygon_to_correctly_oriented_segments(
                polygon, orienteer, segment_cls
            )
            for polygon in subtrahend_common_continuum_polygons
        ),
        orienteer,
        segments_intersector,
    )
    polygons = operation.reduce_events(
        list(operation), contour_cls, polygon_cls
    )
    polygons.extend(
        minuend_polygons[index]
        for index in flags_to_false_indices(
            minuend_boxes_have_common_continuum
        )
    )
    polygons.extend(
        subtrahend_polygons[index]
        for index in flags_to_false_indices(
            subtrahend_boxes_have_common_continuum
        )
    )
    return collect_maybe_empty_polygons(polygons, empty_cls, multipolygon_cls)


def symmetric_subtract_segment_from_segment(
    minuend: hints.Segment[hints.Scalar],
    subtrahend: hints.Segment[hints.Scalar],
    empty_cls: type[hints.Empty[hints.Scalar]],
    multisegment_cls: type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[
    hints.Empty[hints.Scalar],
    hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar],
]:
    minuend_start, minuend_end = to_sorted_pair(minuend.start, minuend.end)
    subtrahend_start, subtrahend_end = to_sorted_pair(
        subtrahend.start, subtrahend.end
    )
    if subtrahend_start == minuend_start and subtrahend_end == minuend_end:
        return empty_cls()
    subtrahend_start_orientation = orienteer(
        minuend_end, minuend_start, subtrahend_start
    )
    subtrahend_end_orientation = orienteer(
        minuend_end, minuend_start, subtrahend_end
    )
    if (
        subtrahend_start_orientation is not Orientation.COLLINEAR
        and subtrahend_end_orientation is not Orientation.COLLINEAR
        and (subtrahend_start_orientation is not subtrahend_end_orientation)
    ):
        minuend_start_orientation = orienteer(
            subtrahend_start, subtrahend_end, minuend_start
        )
        minuend_end_orientation = orienteer(
            subtrahend_start, subtrahend_end, minuend_end
        )
        if (
            minuend_start_orientation is not Orientation.COLLINEAR
            and minuend_end_orientation is not Orientation.COLLINEAR
            and minuend_start_orientation is not minuend_end_orientation
        ):
            cross_point = segments_intersector(
                minuend_start, minuend_end, subtrahend_start, subtrahend_end
            )
            return multisegment_cls(
                [
                    segment_cls(minuend_start, cross_point),
                    segment_cls(cross_point, minuend_end),
                    segment_cls(subtrahend_start, cross_point),
                    segment_cls(cross_point, subtrahend_end),
                ]
            )
    elif (
        subtrahend_start_orientation is Orientation.COLLINEAR
        and subtrahend_end_orientation is Orientation.COLLINEAR
    ):
        if minuend_start == subtrahend_start:
            return segment_cls(minuend_end, subtrahend_end)
        elif minuend_end == subtrahend_end:
            return segment_cls(minuend_start, subtrahend_start)
        elif minuend_start == subtrahend_end:
            return segment_cls(subtrahend_start, minuend_end)
        elif minuend_end == subtrahend_start:
            return segment_cls(minuend_start, subtrahend_end)
        elif subtrahend_start < minuend_end and minuend_start < subtrahend_end:
            return multisegment_cls(
                [
                    segment_cls(minuend_start, subtrahend_start),
                    segment_cls(subtrahend_end, minuend_end),
                ]
            )
    return multisegment_cls([minuend, subtrahend])


def symmetric_subtract_multisegmental_from_segment(
    minuend: hints.Segment[hints.Scalar],
    subtrahend: _Multisegmental[hints.Scalar],
    empty_cls: type[hints.Empty[hints.Scalar]],
    multisegment_cls: type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[
    hints.Empty[hints.Scalar],
    hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    subtrahend_segments = subtrahend.segments
    if do_boxes_have_no_common_continuum(
        minuend_bounding_box, subtrahend_bounding_box
    ):
        return multisegment_cls([minuend, *subtrahend_segments])
    subtrahend_boxes = [
        segment.bounding_box for segment in subtrahend_segments
    ]
    subtrahend_boxes_have_common_continuum = to_boxes_have_common_continuum(
        subtrahend_boxes, minuend_bounding_box
    )
    subtrahend_common_continuum_segments_ids = flags_to_true_indices(
        subtrahend_boxes_have_common_continuum
    )
    if not subtrahend_common_continuum_segments_ids:
        return multisegment_cls([minuend, *subtrahend_segments])
    subtrahend_common_continuum_segments = [
        subtrahend_segments[index]
        for index in subtrahend_common_continuum_segments_ids
    ]
    operation = LinearSymmetricDifference.from_segments_iterables(
        [minuend],
        subtrahend_common_continuum_segments,
        orienteer,
        segments_intersector,
    )
    segments = operation.reduce_events(
        [
            operation.to_opposite_event(event)
            for event in operation
            if is_event_right(event)
        ],
        segment_cls,
    )
    segments.extend(
        subtrahend_segments[index]
        for index in flags_to_false_indices(
            subtrahend_boxes_have_common_continuum
        )
    )
    return collect_maybe_empty_segments(segments, empty_cls, multisegment_cls)


def symmetric_subtract_segment_from_multisegmental(
    minuend: _Multisegmental[hints.Scalar],
    subtrahend: hints.Segment[hints.Scalar],
    empty_cls: type[hints.Empty[hints.Scalar]],
    multisegment_cls: type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[
    hints.Empty[hints.Scalar],
    hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    minuend_segments = minuend.segments
    if do_boxes_have_no_common_continuum(
        minuend_bounding_box, subtrahend_bounding_box
    ):
        return multisegment_cls([*minuend_segments, subtrahend])
    minuend_boxes = [segment.bounding_box for segment in minuend_segments]
    minuend_boxes_have_common_continuum = to_boxes_have_common_continuum(
        minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_continuum_segments_ids = flags_to_true_indices(
        minuend_boxes_have_common_continuum
    )
    if not minuend_common_continuum_segments_ids:
        return multisegment_cls([*minuend_segments, subtrahend])
    minuend_common_continuum_segments = [
        minuend_segments[index]
        for index in minuend_common_continuum_segments_ids
    ]
    operation = LinearSymmetricDifference.from_segments_iterables(
        minuend_common_continuum_segments,
        [subtrahend],
        orienteer,
        segments_intersector,
    )
    segments = operation.reduce_events(
        [
            operation.to_opposite_event(event)
            for event in operation
            if is_event_right(event)
        ],
        segment_cls,
    )
    segments.extend(
        minuend_segments[index]
        for index in flags_to_false_indices(
            minuend_boxes_have_common_continuum
        )
    )
    return collect_maybe_empty_segments(segments, empty_cls, multisegment_cls)


def symmetric_subtract_multisegmental_from_multisegmental(
    minuend: _Multisegmental[hints.Scalar],
    subtrahend: _Multisegmental[hints.Scalar],
    empty_cls: type[hints.Empty[hints.Scalar]],
    multisegment_cls: type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[
    hints.Empty[hints.Scalar],
    hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    minuend_segments, subtrahend_segments = (
        minuend.segments,
        subtrahend.segments,
    )
    if do_boxes_have_no_common_continuum(
        minuend_bounding_box, subtrahend_bounding_box
    ):
        return multisegment_cls([*minuend_segments, *subtrahend_segments])
    minuend_boxes = [segment.bounding_box for segment in minuend_segments]
    minuend_boxes_have_common_continuum = to_boxes_have_common_continuum(
        minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_continuum_segments_ids = flags_to_true_indices(
        minuend_boxes_have_common_continuum
    )
    if not minuend_common_continuum_segments_ids:
        return multisegment_cls([*minuend_segments, *subtrahend_segments])
    subtrahend_boxes = [
        segment.bounding_box for segment in subtrahend_segments
    ]
    subtrahend_boxes_have_common_continuum = to_boxes_have_common_continuum(
        subtrahend_boxes, minuend_bounding_box
    )
    subtrahend_common_continuum_segments_ids = flags_to_true_indices(
        subtrahend_boxes_have_common_continuum
    )
    if not subtrahend_common_continuum_segments_ids:
        return multisegment_cls([*minuend_segments, *subtrahend_segments])
    minuend_common_continuum_segments = [
        minuend_segments[index]
        for index in minuend_common_continuum_segments_ids
    ]
    subtrahend_common_continuum_segments = [
        subtrahend_segments[index]
        for index in subtrahend_common_continuum_segments_ids
    ]
    operation = LinearSymmetricDifference.from_segments_iterables(
        minuend_common_continuum_segments,
        subtrahend_common_continuum_segments,
        orienteer,
        segments_intersector,
    )
    segments = operation.reduce_events(
        [
            operation.to_opposite_event(event)
            for event in operation
            if is_event_right(event)
        ],
        segment_cls,
    )
    segments.extend(
        minuend_segments[index]
        for index in flags_to_false_indices(
            minuend_boxes_have_common_continuum
        )
    )
    segments.extend(
        subtrahend_segments[index]
        for index in flags_to_false_indices(
            subtrahend_boxes_have_common_continuum
        )
    )
    return collect_maybe_empty_segments(segments, empty_cls, multisegment_cls)
