import typing as t
from itertools import groupby

from rene import (Orientation,
                  hints)
from rene._utils import (do_boxes_have_common_continuum,
                         do_boxes_have_no_common_area,
                         do_boxes_have_no_common_continuum,
                         merge_boxes,
                         orient,
                         polygon_to_correctly_oriented_segments,
                         to_boxes_ids_with_common_area,
                         to_boxes_ids_with_common_continuum,
                         to_sorted_pair)
from . import (linear,
               shaped)
from .event import (Event,
                    is_right_event)


class LinearIntersection(linear.Operation[hints.Scalar]):
    def reduce_events(self,
                      events: t.List[Event],
                      segment_cls: t.Type[hints.Segment[hints.Scalar]],
                      /) -> t.List[hints.Segment[hints.Scalar]]:
        return [
            segment_cls(start, end)
            for (start, end), equal_segment_events in groupby(
                    events,
                    key=self._to_event_endpoints
            )
            if _has_two_or_more_elements(equal_segment_events)
        ]


class ShapedIntersection(shaped.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return (self._is_inside_left_event(event)
                or not self._is_left_event_from_first_operand(event)
                and self._is_common_region_boundary_left_event(event))


def intersect_multipolygons(first: hints.Multipolygon[hints.Scalar],
                            second: hints.Multipolygon[hints.Scalar],
                            segment_cls: t.Type[hints.Segment[hints.Scalar]],
                            /) -> t.List[hints.Polygon[hints.Scalar]]:
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
             for segment in polygon_to_correctly_oriented_segments(polygon,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for polygon in second_common_area_polygons
             for segment in polygon_to_correctly_oriented_segments(polygon,
                                                                   segment_cls)
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
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
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
             for segment in polygon_to_correctly_oriented_segments(polygon,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for segment in polygon_to_correctly_oriented_segments(second,
                                                                   segment_cls)
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
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
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
             for segment in polygon_to_correctly_oriented_segments(first,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for polygon in second_common_area_polygons
             for segment in polygon_to_correctly_oriented_segments(polygon,
                                                                   segment_cls)
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
                       segment_cls: t.Type[hints.Segment[hints.Scalar]],
                       /) -> t.List[hints.Polygon[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_area(first_bounding_box, second_bounding_box):
        return []
    max_min_x = max(first_bounding_box.min_x, second_bounding_box.min_x)
    min_max_x = min(first_bounding_box.max_x, second_bounding_box.max_x)
    operation = ShapedIntersection.from_segments_iterables(
            (segment
             for segment in polygon_to_correctly_oriented_segments(first,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for segment in polygon_to_correctly_oriented_segments(second,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x))
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return operation.reduce_events(events, type(first.border), type(first))


def intersect_segments(
        first: hints.Segment[hints.Scalar],
        second: hints.Segment[hints.Scalar],
        /
) -> t.Optional[hints.Segment[hints.Scalar]]:
    return (
        None
        if do_boxes_have_no_common_continuum(first.bounding_box,
                                             second.bounding_box)
        else intersect_segments_with_common_continuum_bounding_boxes(
                first.start, first.end, second.start, second.end, type(first)
        )
    )


def intersect_segments_with_common_continuum_bounding_boxes(
        start: hints.Point[hints.Scalar],
        end: hints.Point[hints.Scalar],
        other_start: hints.Point[hints.Scalar],
        other_end: hints.Point[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Optional[hints.Segment[hints.Scalar]]:
    start, end = to_sorted_pair(start, end)
    other_start, other_end = to_sorted_pair(other_start, other_end)
    return (segment_cls(max(start, other_start), min(end, other_end))
            if ((start == other_start
                 or orient(end, start, other_start) is Orientation.COLLINEAR)
                and
                (end == other_end
                 or orient(end, start, other_end) is Orientation.COLLINEAR))
            else None)


def intersect_segment_with_segments(
        segment: hints.Segment[hints.Scalar],
        segments: t.Iterable[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Segment[hints.Scalar]]:
    bounding_box = segment.bounding_box
    start, end = segment.start, segment.end
    segment_cls = type(segment)
    return [
        maybe_segment
        for maybe_segment in [
            intersect_segments_with_common_continuum_bounding_boxes(
                    segment.start, segment.end, start, end, segment_cls
            )
            for segment in segments
            if do_boxes_have_common_continuum(segment.bounding_box,
                                              bounding_box)
        ]
        if maybe_segment is not None
    ]


def intersect_segments_sequences(
        first: t.Sequence[hints.Segment[hints.Scalar]],
        second: t.Sequence[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Segment[hints.Scalar]]:
    first_boxes = [segment.bounding_box for segment in first]
    second_boxes = [segment.bounding_box for segment in second]
    first_bounding_box, second_bounding_box = (merge_boxes(first_boxes),
                                               merge_boxes(second_boxes))
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return []
    first_common_continuum_segments_ids = to_boxes_ids_with_common_continuum(
            first_boxes, second_bounding_box
    )
    if not first_common_continuum_segments_ids:
        return []
    second_common_continuum_segments_ids = to_boxes_ids_with_common_continuum(
            second_boxes, first_bounding_box
    )
    if not second_common_continuum_segments_ids:
        return []
    first_common_continuum_segments = [
        first[segment_id] for segment_id in first_common_continuum_segments_ids
    ]
    second_common_continuum_segments = [
        second[segment_id]
        for segment_id in second_common_continuum_segments_ids
    ]
    max_min_x = max(
            min(first_boxes[segment_id].min_x
                for segment_id in first_common_continuum_segments_ids),
            min(second_boxes[segment_id].min_x
                for segment_id in second_common_continuum_segments_ids)
    )
    min_max_x = min(
            max(first_boxes[segment_id].max_x
                for segment_id in first_common_continuum_segments_ids),
            max(second_boxes[segment_id].max_x
                for segment_id in second_common_continuum_segments_ids)
    )
    operation = LinearIntersection.from_segments_iterables(
            (segment
             for segment in first_common_continuum_segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for segment in second_common_continuum_segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
    )
    events: t.List[Event] = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        if is_right_event(event):
            events.append(operation.to_opposite_event(event))
    return operation.reduce_events(events, type(first[0]))


def _has_two_or_more_elements(iterator: t.Iterator[t.Any],
                              /,
                              _sentinel: object = object()) -> bool:
    return (next(iterator, _sentinel) is not _sentinel
            and next(iterator, _sentinel) is not _sentinel)
