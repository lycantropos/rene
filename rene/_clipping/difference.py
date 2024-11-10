import typing as t
from itertools import chain, groupby

from rene import Orientation, hints
from rene._hints import Orienteer, SegmentsIntersector
from rene._utils import (
    collect_maybe_empty_polygons,
    collect_maybe_empty_segments,
    do_boxes_have_no_common_area,
    do_boxes_have_no_common_continuum,
    flags_to_false_indices,
    flags_to_true_indices,
    polygon_to_correctly_oriented_segments,
    to_boxes_have_common_area,
    to_boxes_have_common_continuum,
    to_boxes_ids_with_common_area,
    to_boxes_ids_with_common_continuum,
    to_sorted_pair,
)

from . import linear, mixed, shaped
from .event import Event, is_event_right


class LinearDifference(linear.Operation[hints.Scalar]):
    def reduce_events(
        self,
        events: t.List[Event],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /,
    ) -> t.List[hints.Segment[hints.Scalar]]:
        return [
            segment_cls(start, end)
            for (start, end), equal_segment_events in groupby(
                events, key=self._to_event_endpoints
            )
            if all(
                self._is_event_from_first_operand(event)
                for event in equal_segment_events
            )
        ]


class LinearShapedDifference(mixed.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return self._is_left_event_from_first_operand(
            event
        ) and self._is_left_event_outside(event)


class ShapedDifference(shaped.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return (
            self._is_left_event_outside(event)
            if self._is_left_event_from_first_operand(event)
            else (
                self._is_left_event_inside(event)
                or self._is_left_event_common_polyline_component(event)
            )
        )


_Multisegmental = t.Union[hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar]]


def subtract_multipolygon_from_multipolygon(
    minuend: hints.Multipolygon[hints.Scalar],
    subtrahend: hints.Multipolygon[hints.Scalar],
    contour_cls: t.Type[hints.Contour[hints.Scalar]],
    empty_cls: t.Type[hints.Empty[hints.Scalar]],
    multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
    segment_cls: t.Type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> t.Union[
    hints.Empty[hints.Scalar],
    hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    if do_boxes_have_no_common_area(minuend_bounding_box, subtrahend_bounding_box):
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
    subtrahend_boxes = [polygon.bounding_box for polygon in subtrahend_polygons]
    subtrahend_common_area_polygons_ids = to_boxes_ids_with_common_area(
        subtrahend_boxes, minuend_bounding_box
    )
    if not subtrahend_common_area_polygons_ids:
        return minuend
    minuend_common_area_polygons = [
        minuend_polygons[polygon_id] for polygon_id in minuend_common_area_polygons_ids
    ]
    subtrahend_common_area_polygons = [
        subtrahend_polygons[polygon_id]
        for polygon_id in subtrahend_common_area_polygons_ids
    ]
    minuend_max_x = max(
        minuend_boxes[polygon_id].max_x
        for polygon_id in minuend_common_area_polygons_ids
    )
    minuend_min_x = min(
        minuend_boxes[polygon_id].min_x
        for polygon_id in minuend_common_area_polygons_ids
    )
    operation = ShapedDifference.from_segments_iterables(
        chain.from_iterable(
            polygon_to_correctly_oriented_segments(polygon, orienteer, segment_cls)
            for polygon in minuend_common_area_polygons
        ),
        (
            segment
            for polygon in subtrahend_common_area_polygons
            for segment in polygon_to_correctly_oriented_segments(
                polygon, orienteer, segment_cls
            )
            if (
                minuend_min_x <= max(segment.start.x, segment.end.x)
                and min(segment.start.x, segment.end.x) <= minuend_max_x
            )
        ),
        orienteer,
        segments_intersector,
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


def subtract_multipolygon_from_multisegmental(
    minuend: _Multisegmental[hints.Scalar],
    subtrahend: hints.Multipolygon[hints.Scalar],
    empty_cls: t.Type[hints.Empty[hints.Scalar]],
    multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: t.Type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> t.Union[
    hints.Contour[hints.Scalar],
    hints.Empty[hints.Scalar],
    hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    if do_boxes_have_no_common_area(minuend_bounding_box, subtrahend_bounding_box):
        return minuend
    minuend_segments = minuend.segments
    minuend_boxes = [polygon.bounding_box for polygon in minuend_segments]
    minuend_boxes_have_common_continuum = to_boxes_have_common_continuum(
        minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_continuum_segments_ids = flags_to_true_indices(
        minuend_boxes_have_common_continuum
    )
    if not minuend_common_continuum_segments_ids:
        return minuend
    subtrahend_polygons = subtrahend.polygons
    subtrahend_boxes = [polygon.bounding_box for polygon in subtrahend_polygons]
    subtrahend_common_continuum_polygons_ids = to_boxes_ids_with_common_continuum(
        subtrahend_boxes, minuend_bounding_box
    )
    if not subtrahend_common_continuum_polygons_ids:
        return minuend
    minuend_common_continuum_segments = [
        minuend_segments[segment_id]
        for segment_id in minuend_common_continuum_segments_ids
    ]
    subtrahend_common_continuum_polygons = [
        subtrahend_polygons[polygon_id]
        for polygon_id in subtrahend_common_continuum_polygons_ids
    ]
    minuend_max_x = max(
        minuend_boxes[segment_id].max_x
        for segment_id in minuend_common_continuum_segments_ids
    )
    minuend_min_x = min(
        minuend_boxes[segment_id].min_x
        for segment_id in minuend_common_continuum_segments_ids
    )
    operation = LinearShapedDifference.from_segments_iterables(
        minuend_common_continuum_segments,
        (
            segment
            for polygon in subtrahend_common_continuum_polygons
            for segment in polygon_to_correctly_oriented_segments(
                polygon, orienteer, segment_cls
            )
            if (
                minuend_min_x <= max(segment.start.x, segment.end.x)
                and min(segment.start.x, segment.end.x) <= minuend_max_x
            )
        ),
        orienteer,
        segments_intersector,
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        events.append(event)
    segments = operation.reduce_events(events, segment_cls)
    segments.extend(
        minuend_segments[index]
        for index in flags_to_false_indices(minuend_boxes_have_common_continuum)
    )
    return collect_maybe_empty_segments(segments, empty_cls, multisegment_cls)


def subtract_multipolygon_from_polygon(
    minuend: hints.Polygon[hints.Scalar],
    subtrahend: hints.Multipolygon[hints.Scalar],
    contour_cls: t.Type[hints.Contour[hints.Scalar]],
    empty_cls: t.Type[hints.Empty[hints.Scalar]],
    multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
    segment_cls: t.Type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> t.Union[
    hints.Empty[hints.Scalar],
    hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    if do_boxes_have_no_common_area(minuend_bounding_box, subtrahend_bounding_box):
        return minuend
    subtrahend_polygons = subtrahend.polygons
    subtrahend_boxes = [polygon.bounding_box for polygon in subtrahend_polygons]
    subtrahend_common_area_polygons_ids = to_boxes_ids_with_common_area(
        subtrahend_boxes, minuend_bounding_box
    )
    if not subtrahend_common_area_polygons_ids:
        return minuend
    subtrahend_common_area_polygons = [
        subtrahend_polygons[polygon_id]
        for polygon_id in subtrahend_common_area_polygons_ids
    ]
    minuend_max_x, minuend_min_x = (
        minuend_bounding_box.max_x,
        minuend_bounding_box.min_x,
    )
    operation = ShapedDifference.from_segments_iterables(
        polygon_to_correctly_oriented_segments(minuend, orienteer, segment_cls),
        (
            segment
            for polygon in subtrahend_common_area_polygons
            for segment in polygon_to_correctly_oriented_segments(
                polygon, orienteer, segment_cls
            )
            if (
                minuend_min_x <= max(segment.start.x, segment.end.x)
                and min(segment.start.x, segment.end.x) <= minuend_max_x
            )
        ),
        orienteer,
        segments_intersector,
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        events.append(event)
    return collect_maybe_empty_polygons(
        operation.reduce_events(events, contour_cls, polygon_cls),
        empty_cls,
        multipolygon_cls,
    )


def subtract_multipolygon_from_segment(
    minuend: hints.Segment[hints.Scalar],
    subtrahend: hints.Multipolygon[hints.Scalar],
    empty_cls: t.Type[hints.Empty[hints.Scalar]],
    multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: t.Type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> t.Union[
    hints.Empty[hints.Scalar],
    hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    if do_boxes_have_no_common_area(minuend_bounding_box, subtrahend_bounding_box):
        return minuend
    subtrahend_polygons = subtrahend.polygons
    subtrahend_boxes = [polygon.bounding_box for polygon in subtrahend_polygons]
    subtrahend_common_continuum_polygons_ids = to_boxes_ids_with_common_continuum(
        subtrahend_boxes, minuend_bounding_box
    )
    if not subtrahend_common_continuum_polygons_ids:
        return minuend
    subtrahend_common_continuum_polygons = [
        subtrahend_polygons[polygon_id]
        for polygon_id in subtrahend_common_continuum_polygons_ids
    ]
    minuend_max_x, minuend_min_x = (
        minuend_bounding_box.max_x,
        minuend_bounding_box.min_x,
    )
    operation = LinearShapedDifference.from_segments_iterables(
        [minuend],
        (
            segment
            for polygon in subtrahend_common_continuum_polygons
            for segment in polygon_to_correctly_oriented_segments(
                polygon, orienteer, segment_cls
            )
            if (
                minuend_min_x <= max(segment.start.x, segment.end.x)
                and min(segment.start.x, segment.end.x) <= minuend_max_x
            )
        ),
        orienteer,
        segments_intersector,
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        events.append(event)
    return collect_maybe_empty_segments(
        operation.reduce_events(events, segment_cls),
        empty_cls,
        multisegment_cls,
    )


def subtract_multisegmental_from_multisegmental(
    minuend: _Multisegmental[hints.Scalar],
    subtrahend: _Multisegmental[hints.Scalar],
    empty_cls: t.Type[hints.Empty[hints.Scalar]],
    multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: t.Type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> t.Union[
    hints.Contour[hints.Scalar],
    hints.Empty[hints.Scalar],
    hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    if do_boxes_have_no_common_continuum(minuend_bounding_box, subtrahend_bounding_box):
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
    subtrahend_boxes = [segment.bounding_box for segment in subtrahend_segments]
    subtrahend_common_continuum_segments_ids = to_boxes_ids_with_common_continuum(
        subtrahend_boxes, minuend_bounding_box
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
        (
            segment
            for segment in subtrahend_common_continuum_segments
            if (
                minuend_min_x <= max(segment.start.x, segment.end.x)
                and min(segment.start.x, segment.end.x) <= minuend_max_x
            )
        ),
        orienteer,
        segments_intersector,
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        if is_event_right(event):
            events.append(operation.to_opposite_event(event))
    segments = operation.reduce_events(events, segment_cls)
    segments.extend(
        minuend_segments[index]
        for index in flags_to_false_indices(minuend_boxes_have_common_continuum)
    )
    return collect_maybe_empty_segments(segments, empty_cls, multisegment_cls)


def subtract_multisegmental_from_segment(
    minuend: hints.Segment[hints.Scalar],
    subtrahend: _Multisegmental[hints.Scalar],
    empty_cls: t.Type[hints.Empty[hints.Scalar]],
    multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: t.Type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> t.Union[
    hints.Empty[hints.Scalar],
    hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    if do_boxes_have_no_common_continuum(minuend_bounding_box, subtrahend_bounding_box):
        return minuend
    subtrahend_segments = subtrahend.segments
    subtrahend_boxes = [segment.bounding_box for segment in subtrahend_segments]
    subtrahend_common_continuum_segments_ids = to_boxes_ids_with_common_continuum(
        subtrahend_boxes, minuend_bounding_box
    )
    if not subtrahend_common_continuum_segments_ids:
        return minuend
    subtrahend_common_continuum_segments = [
        subtrahend_segments[segment_id]
        for segment_id in subtrahend_common_continuum_segments_ids
    ]
    minuend_max_x, minuend_min_x = (
        minuend_bounding_box.max_x,
        minuend_bounding_box.min_x,
    )
    operation = LinearDifference.from_segments_iterables(
        [minuend],
        (
            segment
            for segment in subtrahend_common_continuum_segments
            if (
                minuend_min_x <= max(segment.start.x, segment.end.x)
                and min(segment.start.x, segment.end.x) <= minuend_max_x
            )
        ),
        orienteer,
        segments_intersector,
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        if is_event_right(event):
            events.append(operation.to_opposite_event(event))
    return collect_maybe_empty_segments(
        operation.reduce_events(events, segment_cls),
        empty_cls,
        multisegment_cls,
    )


def subtract_polygon_from_multipolygon(
    minuend: hints.Multipolygon[hints.Scalar],
    subtrahend: hints.Polygon[hints.Scalar],
    contour_cls: t.Type[hints.Contour[hints.Scalar]],
    empty_cls: t.Type[hints.Empty[hints.Scalar]],
    multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
    segment_cls: t.Type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> t.Union[
    hints.Empty[hints.Scalar],
    hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    if do_boxes_have_no_common_area(minuend_bounding_box, subtrahend_bounding_box):
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
        minuend_polygons[polygon_id] for polygon_id in minuend_common_area_polygons_ids
    ]
    minuend_max_x = max(
        minuend_boxes[polygon_id].max_x
        for polygon_id in minuend_common_area_polygons_ids
    )
    minuend_min_x = min(
        minuend_boxes[polygon_id].min_x
        for polygon_id in minuend_common_area_polygons_ids
    )
    operation = ShapedDifference.from_segments_iterables(
        chain.from_iterable(
            polygon_to_correctly_oriented_segments(polygon, orienteer, segment_cls)
            for polygon in minuend_common_area_polygons
        ),
        (
            segment
            for segment in polygon_to_correctly_oriented_segments(
                subtrahend, orienteer, segment_cls
            )
            if (
                minuend_min_x <= max(segment.start.x, segment.end.x)
                and min(segment.start.x, segment.end.x) <= minuend_max_x
            )
        ),
        orienteer,
        segments_intersector,
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


def subtract_polygon_from_multisegmental(
    minuend: _Multisegmental[hints.Scalar],
    subtrahend: hints.Polygon[hints.Scalar],
    empty_cls: t.Type[hints.Empty[hints.Scalar]],
    multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: t.Type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> t.Union[
    hints.Contour[hints.Scalar],
    hints.Empty[hints.Scalar],
    hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    if do_boxes_have_no_common_area(minuend_bounding_box, subtrahend_bounding_box):
        return minuend
    minuend_segments = minuend.segments
    minuend_boxes = [polygon.bounding_box for polygon in minuend_segments]
    minuend_boxes_have_common_continuum = to_boxes_have_common_continuum(
        minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_continuum_segments_ids = flags_to_true_indices(
        minuend_boxes_have_common_continuum
    )
    if not minuend_common_continuum_segments_ids:
        return minuend
    minuend_common_continuum_segments = [
        minuend_segments[segment_id]
        for segment_id in minuend_common_continuum_segments_ids
    ]
    minuend_max_x = max(
        minuend_boxes[segment_id].max_x
        for segment_id in minuend_common_continuum_segments_ids
    )
    minuend_min_x = min(
        minuend_boxes[segment_id].min_x
        for segment_id in minuend_common_continuum_segments_ids
    )
    operation = LinearShapedDifference.from_segments_iterables(
        minuend_common_continuum_segments,
        (
            segment
            for segment in polygon_to_correctly_oriented_segments(
                subtrahend, orienteer, segment_cls
            )
            if (
                minuend_min_x <= max(segment.start.x, segment.end.x)
                and min(segment.start.x, segment.end.x) <= minuend_max_x
            )
        ),
        orienteer,
        segments_intersector,
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        events.append(event)
    segments = operation.reduce_events(events, segment_cls)
    segments.extend(
        minuend_segments[index]
        for index in flags_to_false_indices(minuend_boxes_have_common_continuum)
    )
    return collect_maybe_empty_segments(segments, empty_cls, multisegment_cls)


def subtract_polygon_from_polygon(
    minuend: hints.Polygon[hints.Scalar],
    subtrahend: hints.Polygon[hints.Scalar],
    contour_cls: t.Type[hints.Contour[hints.Scalar]],
    empty_cls: t.Type[hints.Empty[hints.Scalar]],
    multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
    segment_cls: t.Type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> t.Union[
    hints.Empty[hints.Scalar],
    hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    if do_boxes_have_no_common_area(minuend_bounding_box, subtrahend_bounding_box):
        return minuend
    minuend_max_x, minuend_min_x = (
        minuend_bounding_box.max_x,
        minuend_bounding_box.min_x,
    )
    operation = ShapedDifference.from_segments_iterables(
        polygon_to_correctly_oriented_segments(minuend, orienteer, segment_cls),
        (
            segment
            for segment in polygon_to_correctly_oriented_segments(
                subtrahend, orienteer, segment_cls
            )
            if (
                minuend_min_x <= max(segment.start.x, segment.end.x)
                and min(segment.start.x, segment.end.x) <= minuend_max_x
            )
        ),
        orienteer,
        segments_intersector,
    )
    minuend_max_x = minuend_bounding_box.max_x
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        events.append(event)
    return collect_maybe_empty_polygons(
        operation.reduce_events(events, contour_cls, polygon_cls),
        empty_cls,
        multipolygon_cls,
    )


def subtract_polygon_from_segment(
    minuend: hints.Segment[hints.Scalar],
    subtrahend: hints.Polygon[hints.Scalar],
    empty_cls: t.Type[hints.Empty[hints.Scalar]],
    multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: t.Type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> t.Union[
    hints.Empty[hints.Scalar],
    hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    if do_boxes_have_no_common_area(minuend_bounding_box, subtrahend_bounding_box):
        return minuend
    minuend_max_x, minuend_min_x = (
        minuend_bounding_box.max_x,
        minuend_bounding_box.min_x,
    )
    operation = LinearShapedDifference.from_segments_iterables(
        [minuend],
        (
            segment
            for segment in polygon_to_correctly_oriented_segments(
                subtrahend, orienteer, segment_cls
            )
            if (
                minuend_min_x <= max(segment.start.x, segment.end.x)
                and min(segment.start.x, segment.end.x) <= minuend_max_x
            )
        ),
        orienteer,
        segments_intersector,
    )
    events = []
    for event in operation:
        if operation.to_event_start(event).x > minuend_max_x:
            break
        events.append(event)
    return collect_maybe_empty_segments(
        operation.reduce_events(events, segment_cls),
        empty_cls,
        multisegment_cls,
    )


def subtract_segment_from_multisegmental(
    minuend: _Multisegmental[hints.Scalar],
    subtrahend: hints.Segment[hints.Scalar],
    empty_cls: t.Type[hints.Empty[hints.Scalar]],
    multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: t.Type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> t.Union[
    hints.Contour[hints.Scalar],
    hints.Empty[hints.Scalar],
    hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar],
]:
    minuend_bounding_box, subtrahend_bounding_box = (
        minuend.bounding_box,
        subtrahend.bounding_box,
    )
    if do_boxes_have_no_common_continuum(minuend_bounding_box, subtrahend_bounding_box):
        return minuend
    segments = []
    minuend_segments = minuend.segments
    subtrahend_start, subtrahend_end = to_sorted_pair(subtrahend.start, subtrahend.end)
    for index, minuend_segment in enumerate(minuend_segments):
        if minuend_segment.bounding_box.disjoint_with(subtrahend_bounding_box):
            segments.append(minuend_segment)
            continue
        minuend_start, minuend_end = to_sorted_pair(
            minuend_segment.start, minuend_segment.end
        )
        if subtrahend_start == minuend_start and subtrahend_end == minuend_end:
            segments.extend(minuend_segments[index + 1 :])
            break
        minuend_start_orientation = orienteer(
            subtrahend_end, subtrahend_start, minuend_start
        )
        minuend_end_orientation = orienteer(
            subtrahend_end, subtrahend_start, minuend_end
        )
        if (
            minuend_start_orientation is not Orientation.COLLINEAR
            and minuend_end_orientation is not Orientation.COLLINEAR
            and minuend_start_orientation is not minuend_end_orientation
        ):
            subtrahend_start_orientation = orienteer(
                minuend_start, minuend_end, subtrahend_start
            )
            subtrahend_end_orientation = orienteer(
                minuend_start, minuend_end, subtrahend_end
            )
            if (
                subtrahend_start_orientation is not Orientation.COLLINEAR
                and subtrahend_end_orientation is not Orientation.COLLINEAR
                and (subtrahend_start_orientation is not subtrahend_end_orientation)
            ):
                cross_point = segments_intersector(
                    minuend_start,
                    minuend_end,
                    subtrahend_start,
                    subtrahend_end,
                )
                segments.append(segment_cls(minuend_start, cross_point))
                segments.append(segment_cls(cross_point, minuend_end))
                continue
        elif (
            minuend_start_orientation is Orientation.COLLINEAR
            and minuend_end_orientation is Orientation.COLLINEAR
        ):
            if subtrahend_start == minuend_start:
                if subtrahend_end < minuend_end:
                    segments.append(segment_cls(subtrahend_end, minuend_end))
                continue
            elif subtrahend_end == minuend_end:
                if minuend_start < subtrahend_start:
                    segments.append(segment_cls(minuend_start, subtrahend_start))
                continue
            elif subtrahend_start < minuend_start < subtrahend_end:
                if subtrahend_end < minuend_end:
                    segments.append(segment_cls(subtrahend_end, minuend_end))
                continue
            elif minuend_start < subtrahend_start < minuend_end:
                segments.append(segment_cls(minuend_start, subtrahend_start))
                if subtrahend_end < minuend_end:
                    segments.append(segment_cls(subtrahend_end, minuend_end))
                continue
        segments.append(minuend_segment)
    return collect_maybe_empty_segments(segments, empty_cls, multisegment_cls)


def subtract_segment_from_segment(
    minuend: hints.Segment[hints.Scalar],
    subtrahend: hints.Segment[hints.Scalar],
    empty_cls: t.Type[hints.Empty[hints.Scalar]],
    multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segment_cls: t.Type[hints.Segment[hints.Scalar]],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> t.Union[
    hints.Empty[hints.Scalar],
    hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar],
]:
    minuend_start, minuend_end = to_sorted_pair(minuend.start, minuend.end)
    subtrahend_start, subtrahend_end = to_sorted_pair(subtrahend.start, subtrahend.end)
    if minuend_start == subtrahend_start and minuend_end == subtrahend_end:
        return empty_cls()
    subtrahend_start_orientation = orienteer(
        minuend_end, minuend_start, subtrahend_start
    )
    subtrahend_end_orientation = orienteer(minuend_end, minuend_start, subtrahend_end)
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
                ]
            )
    elif (
        subtrahend_start_orientation is Orientation.COLLINEAR
        and subtrahend_end_orientation is Orientation.COLLINEAR
        and subtrahend_start < minuend_end
        and minuend_start < subtrahend_end
    ):
        if minuend_start == subtrahend_start:
            if subtrahend_end < minuend_end:
                return segment_cls(subtrahend_end, minuend_end)
            else:
                return empty_cls()
        elif minuend_end == subtrahend_end:
            if subtrahend_start < minuend_start:
                return empty_cls()
            else:
                return segment_cls(subtrahend_start, minuend_start)
        elif minuend_start < subtrahend_start:
            if subtrahend_end < minuend_end:
                return multisegment_cls(
                    [
                        segment_cls(minuend_start, subtrahend_start),
                        segment_cls(subtrahend_end, minuend_end),
                    ]
                )
            else:
                return segment_cls(minuend_start, subtrahend_start)
        elif subtrahend_start < minuend_start:
            if minuend_end < subtrahend_end:
                return empty_cls()
            else:
                return segment_cls(subtrahend_end, minuend_end)
    return minuend
