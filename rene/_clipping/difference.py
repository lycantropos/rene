import typing as t
from itertools import (chain,
                       groupby)

from rene import (Orientation,
                  Relation,
                  hints)
from rene._utils import (collect_maybe_empty_polygons,
                         collect_maybe_empty_segments,
                         do_boxes_have_no_common_area,
                         do_boxes_have_no_common_continuum,
                         flags_to_false_indices,
                         flags_to_true_indices,
                         orient,
                         polygon_to_correctly_oriented_segments,
                         subtract_segments_overlap,
                         to_boxes_have_common_area,
                         to_boxes_have_common_continuum,
                         to_boxes_ids_with_common_area,
                         to_boxes_ids_with_common_continuum,
                         to_segments_intersection_point,
                         to_sorted_pair)
from . import (linear,
               shaped)
from .event import Event


class LinearDifference(linear.Operation[hints.Scalar]):
    def reduce_events(self, events: t.List[Event],
                      segment_cls: t.Type[hints.Segment[hints.Scalar]],
                      /) -> t.List[hints.Segment[hints.Scalar]]:
        return [
            segment_cls(start, end)
            for (start, end), equal_segment_events in groupby(
                    events,
                    key=self._to_event_endpoints
            )
            if all(self._is_from_first_operand_event(event)
                   for event in equal_segment_events)
        ]


class ShapedDifference(shaped.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return (self._is_outside_left_event(event)
                if self._is_left_event_from_first_operand(event)
                else (self._is_inside_left_event(event)
                      or self._is_common_polyline_component_left_event(event)))


_Multisegmental = t.Union[
    hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar]
]


def subtract_multipolygon_from_multipolygon(
        minuend: hints.Multipolygon[hints.Scalar],
        subtrahend: hints.Multipolygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar]
]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    if do_boxes_have_no_common_area(minuend_bounding_box,
                                    subtrahend_bounding_box):
        return minuend
    minuend_polygons = minuend.polygons
    minuend_boxes = [polygon.bounding_box for polygon in minuend_polygons]
    minuend_boxes_have_common_area = to_boxes_have_common_area(
            minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_area_polygons_ids = flags_to_true_indices(
            minuend_boxes_have_common_area
    )
    if not minuend_common_area_polygons_ids:
        return minuend
    subtrahend_polygons = subtrahend.polygons
    subtrahend_boxes = [polygon.bounding_box
                        for polygon in subtrahend_polygons]
    subtrahend_common_area_polygons_ids = to_boxes_ids_with_common_area(
            subtrahend_boxes, minuend_bounding_box
    )
    if not subtrahend_common_area_polygons_ids:
        return minuend
    minuend_common_area_polygons = [
        minuend_polygons[polygon_id]
        for polygon_id in minuend_common_area_polygons_ids
    ]
    subtrahend_common_area_polygons = [
        subtrahend_polygons[polygon_id]
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
    polygons = operation.reduce_events(events, contour_cls, polygon_cls)
    polygons.extend(
            minuend_polygons[index]
            for index in flags_to_false_indices(minuend_boxes_have_common_area)
    )
    return collect_maybe_empty_polygons(polygons, empty_cls, multipolygon_cls)


def subtract_multipolygon_from_polygon(
        minuend: hints.Polygon[hints.Scalar],
        subtrahend: hints.Multipolygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar]
]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    if do_boxes_have_no_common_area(minuend_bounding_box,
                                    subtrahend_bounding_box):
        return minuend
    subtrahend_polygons = subtrahend.polygons
    subtrahend_boxes = [polygon.bounding_box
                        for polygon in subtrahend_polygons]
    subtrahend_common_area_polygons_ids = to_boxes_ids_with_common_area(
            subtrahend_boxes, minuend_bounding_box
    )
    if not subtrahend_common_area_polygons_ids:
        return minuend
    subtrahend_common_area_polygons = [
        subtrahend_polygons[polygon_id]
        for polygon_id in subtrahend_common_area_polygons_ids
    ]
    minuend_max_x, minuend_min_x = (minuend_bounding_box.max_x,
                                    minuend_bounding_box.min_x)
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
    return collect_maybe_empty_polygons(
            operation.reduce_events(events, contour_cls, polygon_cls),
            empty_cls, multipolygon_cls
    )


def subtract_multisegmental_from_multisegmental(
        minuend: _Multisegmental[hints.Scalar],
        subtrahend: _Multisegmental[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    if do_boxes_have_no_common_continuum(minuend_bounding_box,
                                         subtrahend_bounding_box):
        return minuend
    minuend_segments = minuend.segments
    minuend_boxes = [segment.bounding_box for segment in minuend_segments]
    minuend_boxes_have_common_continuum = to_boxes_have_common_continuum(
            minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_continuum_segments_ids = flags_to_true_indices(
            minuend_boxes_have_common_continuum
    )
    if not minuend_common_continuum_segments_ids:
        return minuend
    subtrahend_segments = subtrahend.segments
    subtrahend_boxes = [segment.bounding_box
                        for segment in subtrahend_segments]
    subtrahend_common_continuum_segments_ids = (
        to_boxes_ids_with_common_continuum(subtrahend_boxes,
                                           minuend_bounding_box)
    )
    if not subtrahend_common_continuum_segments_ids:
        return minuend
    minuend_common_continuum_segments = [
        minuend_segments[segment_id]
        for segment_id in minuend_common_continuum_segments_ids
    ]
    subtrahend_common_continuum_segments = [
        subtrahend_segments[segment_id]
        for segment_id in subtrahend_common_continuum_segments_ids
    ]
    minuend_max_x = max(
            minuend_boxes[segment_id].max_x
            for segment_id in minuend_common_continuum_segments_ids
    )
    minuend_min_x = min(
            minuend_boxes[segment_id].min_x
            for segment_id in minuend_common_continuum_segments_ids
    )
    operation = LinearDifference.from_segments_iterables(
            minuend_common_continuum_segments,
            (segment
             for segment in subtrahend_common_continuum_segments
             if (minuend_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= minuend_max_x))
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        events.append(event)
    segments = operation.reduce_events(events, segment_cls)
    segments.extend(
            minuend_segments[index]
            for index in flags_to_false_indices(
                    minuend_boxes_have_common_continuum
            )
    )
    return collect_maybe_empty_segments(segments, empty_cls, multisegment_cls)


def subtract_multisegmental_from_segment(
        minuend: hints.Segment[hints.Scalar],
        subtrahend: _Multisegmental[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    if do_boxes_have_no_common_continuum(minuend_bounding_box,
                                         subtrahend_bounding_box):
        return minuend
    subtrahend_segments = subtrahend.segments
    subtrahend_boxes = [segment.bounding_box
                        for segment in subtrahend_segments]
    subtrahend_common_continuum_segments_ids = (
        to_boxes_ids_with_common_continuum(subtrahend_boxes,
                                           minuend_bounding_box)
    )
    if not subtrahend_common_continuum_segments_ids:
        return minuend
    subtrahend_common_continuum_segments = [
        subtrahend_segments[segment_id]
        for segment_id in subtrahend_common_continuum_segments_ids
    ]
    minuend_max_x, minuend_min_x = (minuend_bounding_box.max_x,
                                    minuend_bounding_box.min_x)
    operation = LinearDifference.from_segments_iterables(
            [minuend],
            (segment
             for segment in subtrahend_common_continuum_segments
             if (minuend_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= minuend_max_x))
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        events.append(event)
    return collect_maybe_empty_segments(
            operation.reduce_events(events, segment_cls), empty_cls,
            multisegment_cls
    )


def subtract_polygon_from_multipolygon(
        minuend: hints.Multipolygon[hints.Scalar],
        subtrahend: hints.Polygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar]
]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    if do_boxes_have_no_common_area(minuend_bounding_box,
                                    subtrahend_bounding_box):
        return minuend
    minuend_polygons = minuend.polygons
    minuend_boxes = [polygon.bounding_box for polygon in minuend_polygons]
    minuend_boxes_have_common_area = to_boxes_have_common_area(
            minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_area_polygons_ids = flags_to_true_indices(
            minuend_boxes_have_common_area
    )
    if not minuend_common_area_polygons_ids:
        return minuend
    minuend_common_area_polygons = [
        minuend_polygons[polygon_id]
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
    polygons = operation.reduce_events(events, contour_cls, polygon_cls)
    polygons.extend(
            minuend_polygons[index]
            for index in flags_to_false_indices(minuend_boxes_have_common_area)
    )
    return collect_maybe_empty_polygons(polygons, empty_cls, multipolygon_cls)


def subtract_polygon_from_polygon(
        minuend: hints.Polygon[hints.Scalar],
        subtrahend: hints.Polygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar]
]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    if do_boxes_have_no_common_area(minuend_bounding_box,
                                    subtrahend_bounding_box):
        return minuend
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
    return collect_maybe_empty_polygons(
            operation.reduce_events(events, contour_cls, polygon_cls),
            empty_cls, multipolygon_cls
    )


def subtract_segment_from_multisegmental(
        minuend: _Multisegmental[hints.Scalar],
        subtrahend: hints.Segment[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    return collect_maybe_empty_segments(
            raw_subtract_segment_from_multisegmental(minuend, subtrahend,
                                                     segment_cls),
            empty_cls, multisegment_cls
    )


def raw_subtract_segment_from_multisegmental(
        minuend: _Multisegmental[hints.Scalar],
        subtrahend: hints.Segment[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Segment[hints.Scalar]]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    minuend_segments = minuend.segments
    if do_boxes_have_no_common_continuum(minuend_bounding_box,
                                         subtrahend_bounding_box):
        return [*minuend_segments]
    result = []
    for index, minuend_segment in enumerate(minuend_segments):
        if minuend_segment.bounding_box.disjoint_with(subtrahend_bounding_box):
            result.append(minuend_segment)
            continue
        relation = minuend_segment.relate_to(subtrahend)
        if relation is Relation.EQUAL:
            result.extend(minuend_segments[index + 1:])
            break
        elif relation is Relation.COMPOSITE:
            left_start, left_end, right_start, right_end = sorted(
                    [minuend_segment.start, minuend_segment.end,
                     subtrahend.start, subtrahend.end]
            )
            if left_start == left_end:
                result.append(segment_cls(right_start, right_end))
            elif right_start == right_end:
                result.append(segment_cls(left_start, left_end))
            else:
                result.append(segment_cls(left_start, left_end))
                result.append(segment_cls(right_start, right_end))
        elif relation is Relation.CROSS:
            cross_point = to_segments_intersection_point(
                    minuend_segment.start, minuend_segment.end,
                    subtrahend.start, subtrahend.end
            )
            result.append(segment_cls(minuend_segment.start, cross_point))
            result.append(segment_cls(cross_point, minuend_segment.end))
        elif relation is Relation.OVERLAP:
            start, end = subtract_segments_overlap(
                    minuend_segment.start, minuend_segment.end,
                    subtrahend.start, subtrahend.end
            )
            result.append(segment_cls(start, end))
        elif relation is not Relation.COMPONENT:
            result.append(minuend_segment)
    return result


def subtract_segment_from_segment(
        minuend: hints.Segment[hints.Scalar],
        subtrahend: hints.Segment[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    minuend_start, minuend_end = to_sorted_pair(minuend.start, minuend.end)
    subtrahend_start, subtrahend_end = to_sorted_pair(subtrahend.start,
                                                      subtrahend.end)
    starts_equal = subtrahend_start == minuend_start
    ends_equal = subtrahend_end == minuend_end
    if starts_equal and ends_equal:
        return empty_cls()
    subtrahend_start_orientation = orient(minuend_end, minuend_start,
                                          subtrahend_start)
    subtrahend_end_orientation = orient(minuend_end, minuend_start,
                                        subtrahend_end)
    if (subtrahend_start_orientation is not Orientation.COLLINEAR
            and subtrahend_end_orientation is not Orientation.COLLINEAR
            and (subtrahend_start_orientation
                 is not subtrahend_end_orientation)):
        minuend_start_orientation = orient(subtrahend_start, subtrahend_end,
                                           minuend_start)
        minuend_end_orientation = orient(subtrahend_start, subtrahend_end,
                                         minuend_end)
        if (minuend_start_orientation is not Orientation.COLLINEAR
                and minuend_end_orientation is not Orientation.COLLINEAR
                and minuend_start_orientation is not minuend_end_orientation):
            cross_point = to_segments_intersection_point(
                    minuend_start, minuend_end, subtrahend_start,
                    subtrahend_end
            )
            return multisegment_cls([segment_cls(minuend_start, cross_point),
                                     segment_cls(cross_point, minuend_end)])
    elif (subtrahend_start_orientation is Orientation.COLLINEAR
          and subtrahend_end_orientation is Orientation.COLLINEAR
          and subtrahend_start < minuend_end
          and minuend_start < subtrahend_end):
        if starts_equal:
            if subtrahend_end < minuend_end:
                return segment_cls(subtrahend_end, minuend_end)
            else:
                return empty_cls()
        elif ends_equal:
            if subtrahend_start < minuend_start:
                return empty_cls()
            else:
                return segment_cls(subtrahend_start, minuend_start)
        elif minuend_start < subtrahend_start:
            if subtrahend_end < minuend_end:
                return multisegment_cls(
                        [segment_cls(minuend_start, subtrahend_start),
                         segment_cls(subtrahend_end, minuend_end)]
                )
            else:
                return segment_cls(minuend_start, subtrahend_start)
        elif subtrahend_start < minuend_start:
            if minuend_end < subtrahend_end:
                return empty_cls()
            else:
                return segment_cls(subtrahend_end, minuend_end)
    return minuend
