import typing as t
from itertools import groupby

from rene import (Orientation,
                  hints)
from rene._hints import (Orienteer,
                         SegmentsIntersector)
from rene._utils import (collect_maybe_empty_polygons,
                         collect_maybe_empty_segments,
                         do_boxes_have_common_continuum,
                         do_boxes_have_no_common_area,
                         do_boxes_have_no_common_continuum,
                         polygon_to_correctly_oriented_segments,
                         to_boxes_ids_with_common_area,
                         to_boxes_ids_with_common_continuum,
                         to_sorted_pair)
from . import (linear,
               mixed,
               shaped)
from .event import (Event,
                    is_event_left,
                    is_event_right)
from .utils import has_two_or_more_elements


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
            if has_two_or_more_elements(equal_segment_events)
        ]


class LinearShapedIntersection(mixed.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return (self._is_left_event_from_first_operand(event)
                and not self._is_left_event_outside(event))


class ShapedLinearIntersection(mixed.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return (not self._is_left_event_from_first_operand(event)
                and not self._is_left_event_outside(event))


class ShapedIntersection(shaped.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return (self._is_left_event_inside(event)
                or not self._is_left_event_from_first_operand(event)
                and self._is_left_event_common_region_boundary(event))


_Multisegmental = t.Union[
    hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar]
]


def intersect_multipolygon_with_multipolygon(
        first: hints.Multipolygon[hints.Scalar],
        second: hints.Multipolygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_area(first_bounding_box, second_bounding_box):
        return empty_cls()
    first_polygons = first.polygons
    first_boxes = [polygon.bounding_box for polygon in first_polygons]
    first_common_area_polygons_ids = to_boxes_ids_with_common_area(
            first_boxes, second_bounding_box
    )
    if not first_common_area_polygons_ids:
        return empty_cls()
    second_polygons = second.polygons
    second_boxes = [polygon.bounding_box for polygon in second_polygons]
    second_common_area_polygons_ids = to_boxes_ids_with_common_area(
            second_boxes, first_bounding_box
    )
    if not second_common_area_polygons_ids:
        return empty_cls()
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
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for polygon in second_common_area_polygons
             for segment in polygon_to_correctly_oriented_segments(polygon,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return collect_maybe_empty_polygons(
            operation.reduce_events(events, contour_cls, polygon_cls),
            empty_cls, multipolygon_cls
    )


def intersect_multipolygon_with_multisegmental(
        first: hints.Multipolygon[hints.Scalar],
        second: _Multisegmental[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return empty_cls()
    first_polygons = first.polygons
    first_boxes = [polygon.bounding_box for polygon in first_polygons]
    first_common_continuum_polygons_ids = to_boxes_ids_with_common_continuum(
            first_boxes, second_bounding_box
    )
    if not first_common_continuum_polygons_ids:
        return empty_cls()
    second_segments = second.segments
    second_boxes = [segment.bounding_box for segment in second_segments]
    second_common_continuum_segments_ids = to_boxes_ids_with_common_continuum(
            second_boxes, first_bounding_box
    )
    if not second_common_continuum_segments_ids:
        return empty_cls()
    first_common_continuum_polygons = [
        first_polygons[polygon_id]
        for polygon_id in first_common_continuum_polygons_ids
    ]
    second_common_continuum_segments = [
        second_segments[segment_id]
        for segment_id in second_common_continuum_segments_ids
    ]
    max_min_x = max(
            min(first_boxes[polygon_id].min_x
                for polygon_id in first_common_continuum_polygons_ids),
            min(second_boxes[segment_id].min_x
                for segment_id in second_common_continuum_segments_ids)
    )
    min_max_x = min(
            max(first_boxes[polygon_id].max_x
                for polygon_id in first_common_continuum_polygons_ids),
            max(second_boxes[segment_id].max_x
                for segment_id in second_common_continuum_segments_ids)
    )
    operation = ShapedLinearIntersection.from_segments_iterables(
            (segment
             for polygon in first_common_continuum_polygons
             for segment in polygon_to_correctly_oriented_segments(polygon,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for segment in second_common_continuum_segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        if is_event_left(event):
            events.append(event)
    return collect_maybe_empty_segments(
            operation.reduce_events(events, segment_cls), empty_cls,
            multisegment_cls
    )


def intersect_multipolygon_with_polygon(
        first: hints.Multipolygon[hints.Scalar],
        second: hints.Polygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_area(first_bounding_box, second_bounding_box):
        return empty_cls()
    first_polygons = first.polygons
    first_boxes = [polygon.bounding_box for polygon in first_polygons]
    first_common_area_polygons_ids = to_boxes_ids_with_common_area(
            first_boxes, second_bounding_box
    )
    if not first_common_area_polygons_ids:
        return empty_cls()
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
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for segment in polygon_to_correctly_oriented_segments(second,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return collect_maybe_empty_polygons(
            operation.reduce_events(events, contour_cls, polygon_cls),
            empty_cls, multipolygon_cls
    )


def intersect_multipolygon_with_segment(
        first: hints.Multipolygon[hints.Scalar],
        second: hints.Segment[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return empty_cls()
    first_polygons = first.polygons
    first_boxes = [segment.bounding_box for segment in first_polygons]
    first_common_continuum_polygons_ids = to_boxes_ids_with_common_continuum(
            first_boxes, second_bounding_box
    )
    if not first_common_continuum_polygons_ids:
        return empty_cls()
    first_common_continuum_polygons = [
        first_polygons[segment_id]
        for segment_id in first_common_continuum_polygons_ids
    ]
    max_min_x = max(min(first_boxes[polygon_id].min_x
                        for polygon_id in first_common_continuum_polygons_ids),
                    second_bounding_box.min_x)
    min_max_x = min(max(first_boxes[polygon_id].max_x
                        for polygon_id in first_common_continuum_polygons_ids),
                    second_bounding_box.max_x)
    operation = ShapedLinearIntersection.from_segments_iterables(
            (segment
             for polygon in first_common_continuum_polygons
             for segment in polygon_to_correctly_oriented_segments(polygon,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            [second],
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        if is_event_left(event):
            events.append(event)
    return collect_maybe_empty_segments(
            operation.reduce_events(events, segment_cls), empty_cls,
            multisegment_cls
    )


def intersect_multisegmental_with_multipolygon(
        first: _Multisegmental[hints.Scalar],
        second: hints.Multipolygon[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return empty_cls()
    first_segments = first.segments
    first_boxes = [segment.bounding_box for segment in first_segments]
    first_common_continuum_segments_ids = to_boxes_ids_with_common_continuum(
            first_boxes, second_bounding_box
    )
    if not first_common_continuum_segments_ids:
        return empty_cls()
    second_polygons = second.polygons
    second_boxes = [polygon.bounding_box for polygon in second_polygons]
    second_common_continuum_polygons_ids = to_boxes_ids_with_common_continuum(
            second_boxes, first_bounding_box
    )
    if not second_common_continuum_polygons_ids:
        return empty_cls()
    first_common_continuum_segments = [
        first_segments[segment_id]
        for segment_id in first_common_continuum_segments_ids
    ]
    second_common_continuum_polygons = [
        second_polygons[polygon_id]
        for polygon_id in second_common_continuum_polygons_ids
    ]
    max_min_x = max(
            min(first_boxes[segment_id].min_x
                for segment_id in first_common_continuum_segments_ids),
            min(second_boxes[polygon_id].min_x
                for polygon_id in second_common_continuum_polygons_ids)
    )
    min_max_x = min(
            max(first_boxes[segment_id].max_x
                for segment_id in first_common_continuum_segments_ids),
            max(second_boxes[polygon_id].max_x
                for polygon_id in second_common_continuum_polygons_ids)
    )
    operation = LinearShapedIntersection.from_segments_iterables(
            (segment
             for segment in first_common_continuum_segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for polygon in second_common_continuum_polygons
             for segment in polygon_to_correctly_oriented_segments(polygon,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        if is_event_left(event):
            events.append(event)
    return collect_maybe_empty_segments(
            operation.reduce_events(events, segment_cls), empty_cls,
            multisegment_cls
    )


def intersect_multisegmental_with_multisegmental(
        first: _Multisegmental[hints.Scalar],
        second: _Multisegmental[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return empty_cls()
    first_segments = first.segments
    first_boxes = [segment.bounding_box for segment in first_segments]
    first_common_continuum_segments_ids = to_boxes_ids_with_common_continuum(
            first_boxes, second_bounding_box
    )
    if not first_common_continuum_segments_ids:
        return empty_cls()
    second_segments = second.segments
    second_boxes = [segment.bounding_box for segment in second_segments]
    second_common_continuum_segments_ids = to_boxes_ids_with_common_continuum(
            second_boxes, first_bounding_box
    )
    if not second_common_continuum_segments_ids:
        return empty_cls()
    first_common_continuum_segments = [
        first_segments[segment_id]
        for segment_id in first_common_continuum_segments_ids
    ]
    second_common_continuum_segments = [
        second_segments[segment_id]
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
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        if is_event_right(event):
            events.append(operation.to_opposite_event(event))
    return collect_maybe_empty_segments(
            operation.reduce_events(events, segment_cls), empty_cls,
            multisegment_cls
    )


def intersect_multisegmental_with_polygon(
        first: _Multisegmental[hints.Scalar],
        second: hints.Polygon[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return empty_cls()
    first_segments = first.segments
    first_boxes = [segment.bounding_box for segment in first_segments]
    first_common_continuum_segments_ids = to_boxes_ids_with_common_continuum(
            first_boxes, second_bounding_box
    )
    if not first_common_continuum_segments_ids:
        return empty_cls()
    first_common_continuum_segments = [
        first_segments[segment_id]
        for segment_id in first_common_continuum_segments_ids
    ]
    max_min_x = max(min(first_boxes[segment_id].min_x
                        for segment_id in first_common_continuum_segments_ids),
                    second_bounding_box.min_x)
    min_max_x = min(max(first_boxes[segment_id].max_x
                        for segment_id in first_common_continuum_segments_ids),
                    second_bounding_box.max_x)
    operation = LinearShapedIntersection.from_segments_iterables(
            (segment
             for segment in first_common_continuum_segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for segment in polygon_to_correctly_oriented_segments(second,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        if is_event_left(event):
            events.append(event)
    return collect_maybe_empty_segments(
            operation.reduce_events(events, segment_cls), empty_cls,
            multisegment_cls
    )


def intersect_multisegmental_with_segment(
        first: _Multisegmental[hints.Scalar],
        second: hints.Segment[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    second_bounding_box = second.bounding_box
    second_start, second_end = second.start, second.end
    return collect_maybe_empty_segments(
            [maybe_segment
             for maybe_segment in [
                 intersect_segments_with_common_continuum_bounding_boxes(
                         first_segment.start, first_segment.end, second_start,
                         second_end, orienteer, segment_cls
                 )
                 for first_segment in first.segments
                 if do_boxes_have_common_continuum(first_segment.bounding_box,
                                                   second_bounding_box)
             ]
             if maybe_segment is not None],
            empty_cls, multisegment_cls
    )


def intersect_polygon_with_multipolygon(
        first: hints.Polygon[hints.Scalar],
        second: hints.Multipolygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_area(first_bounding_box, second_bounding_box):
        return empty_cls()
    second_polygons = second.polygons
    second_boxes = [polygon.bounding_box for polygon in second_polygons]
    second_common_area_polygons_ids = to_boxes_ids_with_common_area(
            second_boxes, first_bounding_box
    )
    if not second_common_area_polygons_ids:
        return empty_cls()
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
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for polygon in second_common_area_polygons
             for segment in polygon_to_correctly_oriented_segments(polygon,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return collect_maybe_empty_polygons(
            operation.reduce_events(events, contour_cls, polygon_cls),
            empty_cls, multipolygon_cls
    )


def intersect_polygon_with_multisegmental(
        first: hints.Polygon[hints.Scalar],
        second: _Multisegmental[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return empty_cls()
    second_segments = second.segments
    second_boxes = [segment.bounding_box for segment in second_segments]
    second_common_continuum_segments_ids = to_boxes_ids_with_common_continuum(
            second_boxes, first_bounding_box
    )
    if not second_common_continuum_segments_ids:
        return empty_cls()
    second_common_area_segments = [
        second_segments[segment_id]
        for segment_id in second_common_continuum_segments_ids
    ]
    max_min_x = max(
            first_bounding_box.min_x,
            min(second_boxes[segment_id].min_x
                for segment_id in second_common_continuum_segments_ids)
    )
    min_max_x = min(
            first_bounding_box.max_x,
            max(second_boxes[segment_id].max_x
                for segment_id in second_common_continuum_segments_ids)
    )
    operation = ShapedLinearIntersection.from_segments_iterables(
            (segment
             for segment in polygon_to_correctly_oriented_segments(first,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for segment in second_common_area_segments
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        if is_event_left(event):
            events.append(event)
    return collect_maybe_empty_segments(
            operation.reduce_events(events, segment_cls), empty_cls,
            multisegment_cls
    )


def intersect_polygon_with_polygon(
        first: hints.Polygon[hints.Scalar],
        second: hints.Polygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_area(first_bounding_box, second_bounding_box):
        return empty_cls()
    max_min_x = max(first_bounding_box.min_x, second_bounding_box.min_x)
    min_max_x = min(first_bounding_box.max_x, second_bounding_box.max_x)
    operation = ShapedIntersection.from_segments_iterables(
            (segment
             for segment in polygon_to_correctly_oriented_segments(first,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            (segment
             for segment in polygon_to_correctly_oriented_segments(second,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        events.append(event)
    return collect_maybe_empty_polygons(
            operation.reduce_events(events, contour_cls, polygon_cls),
            empty_cls, multipolygon_cls
    )


def intersect_polygon_with_segment(
        first: hints.Polygon[hints.Scalar],
        second: hints.Segment[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return empty_cls()
    max_min_x = max(first_bounding_box.min_x, second_bounding_box.min_x)
    min_max_x = min(first_bounding_box.max_x, second_bounding_box.max_x)
    operation = ShapedLinearIntersection.from_segments_iterables(
            (segment
             for segment in polygon_to_correctly_oriented_segments(first,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            [second],
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        if is_event_left(event):
            events.append(event)
    return collect_maybe_empty_segments(
            operation.reduce_events(events, segment_cls), empty_cls,
            multisegment_cls
    )


def intersect_segments_with_common_continuum_bounding_boxes(
        start: hints.Point[hints.Scalar],
        end: hints.Point[hints.Scalar],
        other_start: hints.Point[hints.Scalar],
        other_end: hints.Point[hints.Scalar],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Optional[hints.Segment[hints.Scalar]]:
    start, end = to_sorted_pair(start, end)
    other_start, other_end = to_sorted_pair(other_start, other_end)
    return (segment_cls(max(start, other_start), min(end, other_end))
            if ((start == other_start
                 or (orienteer(end, start, other_start)
                     is Orientation.COLLINEAR))
                and (end == other_end
                     or (orienteer(end, start, other_end)
                         is Orientation.COLLINEAR)))
            else None)


def intersect_segment_with_multipolygon(
        first: hints.Segment[hints.Scalar],
        second: hints.Multipolygon[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return empty_cls()
    second_polygons = second.polygons
    second_boxes = [segment.bounding_box for segment in second_polygons]
    second_common_continuum_polygons_ids = to_boxes_ids_with_common_continuum(
            second_boxes, first_bounding_box
    )
    if not second_common_continuum_polygons_ids:
        return empty_cls()
    second_common_continuum_polygons = [
        second_polygons[segment_id]
        for segment_id in second_common_continuum_polygons_ids
    ]
    max_min_x = max(
            first_bounding_box.min_x,
            min(second_boxes[polygon_id].min_x
                for polygon_id in second_common_continuum_polygons_ids)
    )
    min_max_x = min(
            first_bounding_box.max_x,
            max(second_boxes[polygon_id].max_x
                for polygon_id in second_common_continuum_polygons_ids)
    )
    operation = LinearShapedIntersection.from_segments_iterables(
            [first],
            (segment
             for polygon in second_common_continuum_polygons
             for segment in polygon_to_correctly_oriented_segments(polygon,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        if is_event_left(event):
            events.append(event)
    return collect_maybe_empty_segments(
            operation.reduce_events(events, segment_cls), empty_cls,
            multisegment_cls
    )


def intersect_segment_with_multisegmental(
        first: hints.Segment[hints.Scalar],
        second: _Multisegmental[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    first_bounding_box = first.bounding_box
    first_start, first_end = first.start, first.end
    return collect_maybe_empty_segments(
            [maybe_segment
             for maybe_segment in [
                 intersect_segments_with_common_continuum_bounding_boxes(
                         first_start, first_end, second_segment.start,
                         second_segment.end, orienteer, segment_cls
                 )
                 for second_segment in second.segments
                 if do_boxes_have_common_continuum(second_segment.bounding_box,
                                                   first_bounding_box)
             ]
             if maybe_segment is not None],
            empty_cls, multisegment_cls
    )


def intersect_segment_with_polygon(
        first: hints.Segment[hints.Scalar],
        second: hints.Polygon[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        segments_intersector: SegmentsIntersector[hints.Scalar],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return empty_cls()
    max_min_x = max(first_bounding_box.min_x, second_bounding_box.min_x)
    min_max_x = min(first_bounding_box.max_x, second_bounding_box.max_x)
    operation = LinearShapedIntersection.from_segments_iterables(
            [first],
            (segment
             for segment in polygon_to_correctly_oriented_segments(second,
                                                                   orienteer,
                                                                   segment_cls)
             if (max_min_x <= max(segment.start.x, segment.end.x)
                 and min(segment.start.x, segment.end.x) <= min_max_x)),
            orienteer, segments_intersector
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > min_max_x:
            break
        if is_event_left(event):
            events.append(event)
    return collect_maybe_empty_segments(
            operation.reduce_events(events, segment_cls), empty_cls,
            multisegment_cls
    )


def intersect_segment_with_segment(
        first: hints.Segment[hints.Scalar],
        second: hints.Segment[hints.Scalar],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        orienteer: Orienteer[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.Union[hints.Empty[hints.Scalar], hints.Segment[hints.Scalar]]:
    if do_boxes_have_no_common_continuum(first.bounding_box,
                                         second.bounding_box):
        return empty_cls()
    maybe_result = intersect_segments_with_common_continuum_bounding_boxes(
            first.start, first.end, second.start, second.end,
            orienteer, segment_cls
    )
    return empty_cls() if maybe_result is None else maybe_result
