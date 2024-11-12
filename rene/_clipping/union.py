from itertools import chain, groupby
from typing import Union

from rene import Orientation, hints
from rene._hints import Orienteer, SegmentsIntersector
from rene._utils import (
    collect_non_empty_polygons,
    collect_non_empty_segments,
    do_boxes_have_no_common_continuum,
    flags_to_false_indices,
    flags_to_true_indices,
    polygon_to_correctly_oriented_segments,
    to_boxes_have_common_continuum,
    to_sorted_pair,
)

from . import linear, shaped
from .event import Event, is_event_right


class LinearUnion(linear.Operation[hints.Scalar]):
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
        ]


class ShapedUnion(shaped.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return self._is_left_event_outside(event) or (
            not self._is_left_event_from_first_operand(event)
            and self._is_left_event_common_region_boundary(event)
        )


_Multisegmental = Union[
    hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar]
]


def unite_multipolygon_with_multipolygon(
    first: hints.Multipolygon[hints.Scalar],
    second: hints.Multipolygon[hints.Scalar],
    contour_cls: type[hints.Contour[hints.Scalar]],
    multipolygon_cls: type[hints.Multipolygon[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    polygon_cls: type[hints.Polygon[hints.Scalar]],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[hints.Multipolygon[hints.Scalar], hints.Polygon[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (
        first.bounding_box,
        second.bounding_box,
    )
    first_polygons, second_polygons = first.polygons, second.polygons
    if do_boxes_have_no_common_continuum(
        first_bounding_box, second_bounding_box
    ):
        return multipolygon_cls([*first_polygons, *second_polygons])
    first_boxes = [polygon.bounding_box for polygon in first_polygons]
    second_boxes = [polygon.bounding_box for polygon in second_polygons]
    do_first_boxes_have_common_continuum = to_boxes_have_common_continuum(
        first_boxes, second_bounding_box
    )
    first_common_continuum_polygons_ids = flags_to_true_indices(
        do_first_boxes_have_common_continuum
    )
    if not first_common_continuum_polygons_ids:
        return multipolygon_cls([*first_polygons, *second_polygons])
    do_second_boxes_have_common_continuum = to_boxes_have_common_continuum(
        second_boxes, first_bounding_box
    )
    second_common_continuum_polygons_ids = flags_to_true_indices(
        do_second_boxes_have_common_continuum
    )
    if not second_common_continuum_polygons_ids:
        return multipolygon_cls([*first_polygons, *second_polygons])
    first_common_continuum_polygons = [
        first_polygons[index] for index in first_common_continuum_polygons_ids
    ]
    second_common_continuum_polygons = [
        second_polygons[index]
        for index in second_common_continuum_polygons_ids
    ]
    operation = ShapedUnion.from_segments_iterables(
        chain.from_iterable(
            polygon_to_correctly_oriented_segments(
                polygon, orienteer, segment_cls
            )
            for polygon in first_common_continuum_polygons
        ),
        chain.from_iterable(
            polygon_to_correctly_oriented_segments(
                polygon, orienteer, segment_cls
            )
            for polygon in second_common_continuum_polygons
        ),
        orienteer,
        segments_intersector,
    )
    polygons = operation.reduce_events(
        list(operation), contour_cls, polygon_cls
    )
    polygons.extend(
        first_polygons[index]
        for index in flags_to_false_indices(
            do_first_boxes_have_common_continuum
        )
    )
    polygons.extend(
        second_polygons[index]
        for index in flags_to_false_indices(
            do_second_boxes_have_common_continuum
        )
    )
    return collect_non_empty_polygons(polygons, multipolygon_cls)


def unite_multipolygon_with_polygon(
    first: hints.Multipolygon[hints.Scalar],
    second: hints.Polygon[hints.Scalar],
    contour_cls: type[hints.Contour[hints.Scalar]],
    multipolygon_cls: type[hints.Multipolygon[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    polygon_cls: type[hints.Polygon[hints.Scalar]],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[hints.Multipolygon[hints.Scalar], hints.Polygon[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (
        first.bounding_box,
        second.bounding_box,
    )
    first_polygons = first.polygons
    if do_boxes_have_no_common_continuum(
        first_bounding_box, second_bounding_box
    ):
        return multipolygon_cls([*first_polygons, second])
    first_boxes = [polygon.bounding_box for polygon in first_polygons]
    first_boxes_have_common_continuum = to_boxes_have_common_continuum(
        first_boxes, second_bounding_box
    )
    first_common_continuum_polygons_ids = flags_to_true_indices(
        first_boxes_have_common_continuum
    )
    if not first_common_continuum_polygons_ids:
        return multipolygon_cls([*first_polygons, second])
    first_common_continuum_polygons = [
        first_polygons[index] for index in first_common_continuum_polygons_ids
    ]
    operation = ShapedUnion.from_segments_iterables(
        chain.from_iterable(
            polygon_to_correctly_oriented_segments(
                polygon, orienteer, segment_cls
            )
            for polygon in first_common_continuum_polygons
        ),
        polygon_to_correctly_oriented_segments(second, orienteer, segment_cls),
        orienteer,
        segments_intersector,
    )
    polygons = operation.reduce_events(
        list(operation), contour_cls, polygon_cls
    )
    polygons.extend(
        first_polygons[index]
        for index in flags_to_false_indices(first_boxes_have_common_continuum)
    )
    return collect_non_empty_polygons(polygons, multipolygon_cls)


def unite_multisegmental_with_multisegmental(
    first: _Multisegmental[hints.Scalar],
    second: _Multisegmental[hints.Scalar],
    multisegment_cls: type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[hints.Multisegment[hints.Scalar], hints.Segment[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (
        first.bounding_box,
        second.bounding_box,
    )
    first_segments, second_segments = first.segments, second.segments
    if do_boxes_have_no_common_continuum(
        first_bounding_box, second_bounding_box
    ):
        return multisegment_cls([*first_segments, *second_segments])
    first_boxes = [segment.bounding_box for segment in first_segments]
    second_boxes = [segment.bounding_box for segment in second_segments]
    do_first_boxes_have_common_continuum = to_boxes_have_common_continuum(
        first_boxes, second_bounding_box
    )
    first_common_continuum_segments_ids = flags_to_true_indices(
        do_first_boxes_have_common_continuum
    )
    if not first_common_continuum_segments_ids:
        return multisegment_cls([*first_segments, *second_segments])
    do_second_boxes_have_common_continuum = to_boxes_have_common_continuum(
        second_boxes, first_bounding_box
    )
    second_common_continuum_segments_ids = flags_to_true_indices(
        do_second_boxes_have_common_continuum
    )
    if not second_common_continuum_segments_ids:
        return multisegment_cls([*first_segments, *second_segments])
    first_common_continuum_segments = [
        first_segments[index] for index in first_common_continuum_segments_ids
    ]
    second_common_continuum_segments = [
        second_segments[index]
        for index in second_common_continuum_segments_ids
    ]
    operation = LinearUnion.from_segments_iterables(
        first_common_continuum_segments,
        second_common_continuum_segments,
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
        first_segments[index]
        for index in flags_to_false_indices(
            do_first_boxes_have_common_continuum
        )
    )
    segments.extend(
        second_segments[index]
        for index in flags_to_false_indices(
            do_second_boxes_have_common_continuum
        )
    )
    return collect_non_empty_segments(segments, multisegment_cls)


def unite_multisegmental_with_segment(
    first: _Multisegmental[hints.Scalar],
    second: hints.Segment[hints.Scalar],
    multisegment_cls: type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[hints.Multisegment[hints.Scalar], hints.Segment[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (
        first.bounding_box,
        second.bounding_box,
    )
    first_segments = first.segments
    if do_boxes_have_no_common_continuum(
        first_bounding_box, second_bounding_box
    ):
        return multisegment_cls([*first_segments, second])
    segments = []
    second_break_points = []
    second_start, second_end = to_sorted_pair(second.start, second.end)
    for index, first_segment in enumerate(first_segments):
        if first_segment.bounding_box.disjoint_with(second_bounding_box):
            segments.append(first_segment)
            continue
        first_start, first_end = to_sorted_pair(
            first_segment.start, first_segment.end
        )
        if second_start == first_start and second_end == first_end:
            segments.extend(first_segments[index + 1 :])
            break
        first_start_orientation = orienteer(
            second_end, second_start, first_start
        )
        first_end_orientation = orienteer(second_end, second_start, first_end)
        if first_start_orientation is first_end_orientation:
            if first_start_orientation is Orientation.COLLINEAR:
                if second_start == first_start:
                    if second_end < first_end:
                        segments.append(segment_cls(second_end, first_end))
                    continue
                elif second_end == first_end:
                    if first_start < second_start:
                        segments.append(segment_cls(first_start, second_start))
                    continue
                elif second_start < first_start < second_end:
                    if second_end < first_end:
                        segments.append(segment_cls(second_end, first_end))
                    continue
                elif first_start < second_start < first_end:
                    segments.append(segment_cls(first_start, second_start))
                    if second_end < first_end:
                        segments.append(segment_cls(second_end, first_end))
                    continue
        elif first_start_orientation is Orientation.COLLINEAR:
            if second_start < first_start < second_end:
                second_break_points.append(first_start)
        elif first_end_orientation is Orientation.COLLINEAR:
            if second_start < first_end < second_end:
                second_break_points.append(first_end)
        else:
            second_start_orientation = orienteer(
                first_start, first_end, second_start
            )
            second_end_orientation = orienteer(
                first_start, first_end, second_end
            )
            if second_start_orientation is Orientation.COLLINEAR:
                if first_start < second_start < first_end:
                    segments.append(segment_cls(first_start, second_start))
                    segments.append(segment_cls(second_start, first_end))
                    continue
            elif second_end_orientation is Orientation.COLLINEAR:
                if first_start < second_end < first_end:
                    segments.append(segment_cls(first_start, second_end))
                    segments.append(segment_cls(second_end, first_end))
                    continue
            elif second_start_orientation is not second_end_orientation:
                cross_point = segments_intersector(
                    first_start, first_end, second_start, second_end
                )
                second_break_points.append(cross_point)
                segments.append(segment_cls(first_start, cross_point))
                segments.append(segment_cls(cross_point, first_end))
                continue
        segments.append(first_segment)
    if second_break_points:
        second_break_points.sort()
        start = second_start
        for end, _ in groupby(second_break_points):
            segments.append(segment_cls(start, end))
            start = end
        segments.append(segment_cls(start, second_end))
    else:
        segments.append(second)
    return collect_non_empty_segments(segments, multisegment_cls)


def unite_polygon_with_multipolygon(
    first: hints.Polygon[hints.Scalar],
    second: hints.Multipolygon[hints.Scalar],
    contour_cls: type[hints.Contour[hints.Scalar]],
    multipolygon_cls: type[hints.Multipolygon[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    polygon_cls: type[hints.Polygon[hints.Scalar]],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[hints.Multipolygon[hints.Scalar], hints.Polygon[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (
        first.bounding_box,
        second.bounding_box,
    )
    second_polygons = second.polygons
    if do_boxes_have_no_common_continuum(
        first_bounding_box, second_bounding_box
    ):
        return multipolygon_cls([first, *second_polygons])
    second_boxes = [polygon.bounding_box for polygon in second_polygons]
    do_second_boxes_have_common_continuum = to_boxes_have_common_continuum(
        second_boxes, first_bounding_box
    )
    second_common_continuum_polygons_ids = flags_to_true_indices(
        do_second_boxes_have_common_continuum
    )
    if not second_common_continuum_polygons_ids:
        return multipolygon_cls([first, *second_polygons])
    second_common_continuum_polygons = [
        second_polygons[index]
        for index in second_common_continuum_polygons_ids
    ]
    operation = ShapedUnion.from_segments_iterables(
        polygon_to_correctly_oriented_segments(first, orienteer, segment_cls),
        chain.from_iterable(
            polygon_to_correctly_oriented_segments(
                polygon, orienteer, segment_cls
            )
            for polygon in second_common_continuum_polygons
        ),
        orienteer,
        segments_intersector,
    )
    polygons = operation.reduce_events(
        list(operation), contour_cls, polygon_cls
    )
    polygons.extend(
        second_polygons[index]
        for index in flags_to_false_indices(
            do_second_boxes_have_common_continuum
        )
    )
    return collect_non_empty_polygons(polygons, multipolygon_cls)


def unite_polygon_with_polygon(
    first: hints.Polygon[hints.Scalar],
    second: hints.Polygon[hints.Scalar],
    contour_cls: type[hints.Contour[hints.Scalar]],
    multipolygon_cls: type[hints.Multipolygon[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    polygon_cls: type[hints.Polygon[hints.Scalar]],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[hints.Multipolygon[hints.Scalar], hints.Polygon[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (
        first.bounding_box,
        second.bounding_box,
    )
    if do_boxes_have_no_common_continuum(
        first_bounding_box, second_bounding_box
    ):
        return multipolygon_cls([first, second])
    operation = ShapedUnion.from_segments_iterables(
        polygon_to_correctly_oriented_segments(first, orienteer, segment_cls),
        polygon_to_correctly_oriented_segments(second, orienteer, segment_cls),
        orienteer,
        segments_intersector,
    )
    return collect_non_empty_polygons(
        operation.reduce_events(list(operation), contour_cls, polygon_cls),
        multipolygon_cls,
    )


def unite_segment_with_multisegmental(
    first: hints.Segment[hints.Scalar],
    second: _Multisegmental[hints.Scalar],
    multisegment_cls: type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[hints.Multisegment[hints.Scalar], hints.Segment[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (
        first.bounding_box,
        second.bounding_box,
    )
    second_segments = second.segments
    if do_boxes_have_no_common_continuum(
        first_bounding_box, second_bounding_box
    ):
        return multisegment_cls([first, *second_segments])
    segments = []
    first_break_points = []
    first_start, first_end = to_sorted_pair(first.start, first.end)
    for index, second_segment in enumerate(second_segments):
        if second_segment.bounding_box.disjoint_with(first_bounding_box):
            segments.append(second_segment)
            continue
        second_start, second_end = to_sorted_pair(
            second_segment.start, second_segment.end
        )
        if first_start == second_start and first_end == second_end:
            segments.extend(second_segments[index + 1 :])
            break
        second_start_orientation = orienteer(
            first_end, first_start, second_start
        )
        second_end_orientation = orienteer(first_end, first_start, second_end)
        if second_start_orientation is second_end_orientation:
            if second_start_orientation is Orientation.COLLINEAR:
                if first_start == second_start:
                    if first_end < second_end:
                        segments.append(segment_cls(first_end, second_end))
                    continue
                elif first_end == second_end:
                    if second_start < first_start:
                        segments.append(segment_cls(second_start, first_start))
                    continue
                elif first_start < second_start < first_end:
                    if first_end < second_end:
                        segments.append(segment_cls(first_end, second_end))
                    continue
                elif second_start < first_start < second_end:
                    segments.append(segment_cls(second_start, first_start))
                    if first_end < second_end:
                        segments.append(segment_cls(first_end, second_end))
                    continue
        elif second_start_orientation is Orientation.COLLINEAR:
            if first_start < second_start < first_end:
                first_break_points.append(second_start)
        elif second_end_orientation is Orientation.COLLINEAR:
            if first_start < second_end < first_end:
                first_break_points.append(second_end)
        else:
            first_start_orientation = orienteer(
                second_start, second_end, first_start
            )
            first_end_orientation = orienteer(
                second_start, second_end, first_end
            )
            if first_start_orientation is Orientation.COLLINEAR:
                if second_start < first_start < second_end:
                    segments.append(segment_cls(second_start, first_start))
                    segments.append(segment_cls(first_start, second_end))
                    continue
            elif first_end_orientation is Orientation.COLLINEAR:
                if second_start < first_end < second_end:
                    segments.append(segment_cls(second_start, first_end))
                    segments.append(segment_cls(first_end, second_end))
                    continue
            elif first_start_orientation is not first_end_orientation:
                cross_point = segments_intersector(
                    second_start, second_end, first_start, first_end
                )
                first_break_points.append(cross_point)
                segments.append(segment_cls(second_start, cross_point))
                segments.append(segment_cls(cross_point, second_end))
                continue
        segments.append(second_segment)
    if first_break_points:
        first_break_points.sort()
        start = first_start
        for end, _ in groupby(first_break_points):
            segments.append(segment_cls(start, end))
            start = end
        segments.append(segment_cls(start, first_end))
    else:
        segments.append(first)
    return collect_non_empty_segments(segments, multisegment_cls)


def unite_segment_with_segment(
    first: hints.Segment[hints.Scalar],
    second: hints.Segment[hints.Scalar],
    multisegment_cls: type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Union[hints.Multisegment[hints.Scalar], hints.Segment[hints.Scalar]]:
    first_start, first_end = to_sorted_pair(first.start, first.end)
    second_start, second_end = to_sorted_pair(second.start, second.end)
    if first_start == second_start and first_end == second_end:
        return first
    second_start_orientation = orienteer(first_end, first_start, second_start)
    second_end_orientation = orienteer(first_end, first_start, second_end)
    if (
        second_start_orientation is not Orientation.COLLINEAR
        and second_end_orientation is not Orientation.COLLINEAR
        and second_start_orientation is not second_end_orientation
    ):
        first_start_orientation = orienteer(
            second_start, second_end, first_start
        )
        first_end_orientation = orienteer(second_start, second_end, first_end)
        if (
            first_start_orientation is not Orientation.COLLINEAR
            and first_end_orientation is not Orientation.COLLINEAR
            and first_start_orientation is not first_end_orientation
        ):
            cross_point = segments_intersector(
                first_start, first_end, second_start, second_end
            )
            return multisegment_cls(
                [
                    segment_cls(first_start, cross_point),
                    segment_cls(cross_point, first_end),
                    segment_cls(second_start, cross_point),
                    segment_cls(cross_point, second_end),
                ]
            )
    elif (
        second_start_orientation is Orientation.COLLINEAR
        and second_end_orientation is Orientation.COLLINEAR
        and second_start <= first_end
        and first_start <= second_end
    ):
        return segment_cls(
            min(first_start, second_start), max(first_end, second_end)
        )
    return multisegment_cls([first, second])
