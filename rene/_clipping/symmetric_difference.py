import typing as t
from itertools import (chain,
                       groupby)

from rene import (Orientation,
                  hints)
from rene._utils import (all_equal,
                         do_boxes_have_no_common_continuum,
                         flags_to_false_indices,
                         flags_to_true_indices,
                         orient,
                         polygon_to_correctly_oriented_segments,
                         to_boxes_have_common_continuum,
                         to_segments_intersection_point,
                         to_sorted_pair)
from . import (linear,
               shaped)
from .event import Event


class LinearSymmetricDifference(linear.Operation[hints.Scalar]):
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
            if all_equal(self._is_from_first_operand_event(event)
                         for event in equal_segment_events)
        ]


class ShapedSymmetricDifference(shaped.Operation[hints.Scalar]):
    def _detect_if_left_event_from_result(self, event: Event, /) -> bool:
        return not self._is_overlap_left_event(event)


def symmetric_subtract_polygon_from_polygon(
        minuend: hints.Polygon[hints.Scalar],
        subtrahend: hints.Polygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    first_bounding_box, second_bounding_box = (minuend.bounding_box,
                                               subtrahend.bounding_box)
    if do_boxes_have_no_common_continuum(first_bounding_box,
                                         second_bounding_box):
        return [minuend, subtrahend]
    operation = ShapedSymmetricDifference.from_segments_iterables(
            polygon_to_correctly_oriented_segments(minuend, segment_cls),
            polygon_to_correctly_oriented_segments(subtrahend, segment_cls)
    )
    return operation.reduce_events(list(operation), contour_cls, polygon_cls)


def symmetric_subtract_polygon_from_multipolygon(
        minuend: hints.Multipolygon[hints.Scalar],
        subtrahend: hints.Polygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    minuend_polygons = minuend.polygons
    if do_boxes_have_no_common_continuum(minuend_bounding_box,
                                         subtrahend_bounding_box):
        return [*minuend_polygons, subtrahend]
    minuend_boxes = [polygon.bounding_box for polygon in minuend_polygons]
    minuend_boxes_have_common_continuum = to_boxes_have_common_continuum(
            minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_continuum_polygons_ids = flags_to_true_indices(
            minuend_boxes_have_common_continuum
    )
    if not minuend_common_continuum_polygons_ids:
        return [*minuend_polygons, subtrahend]
    minuend_common_continuum_polygons = [
        minuend_polygons[index]
        for index in minuend_common_continuum_polygons_ids
    ]
    operation = ShapedSymmetricDifference.from_segments_iterables(
            chain.from_iterable(
                    polygon_to_correctly_oriented_segments(polygon,
                                                           segment_cls)
                    for polygon in minuend_common_continuum_polygons
            ),
            polygon_to_correctly_oriented_segments(subtrahend, segment_cls)
    )
    result = operation.reduce_events(list(operation), contour_cls, polygon_cls)
    result.extend(
            minuend_polygons[index]
            for index in flags_to_false_indices(
                    minuend_boxes_have_common_continuum
            )
    )
    return result


def symmetric_subtract_multipolygon_from_polygon(
        minuend: hints.Polygon[hints.Scalar],
        subtrahend: hints.Multipolygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]]
) -> t.List[hints.Polygon[hints.Scalar]]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    subtrahend_polygons = subtrahend.polygons
    if do_boxes_have_no_common_continuum(minuend_bounding_box,
                                         subtrahend_bounding_box):
        return [minuend, *subtrahend_polygons]
    subtrahend_boxes = [polygon.bounding_box
                        for polygon in subtrahend_polygons]
    subtrahend_boxes_have_common_continuum = to_boxes_have_common_continuum(
            subtrahend_boxes, minuend_bounding_box
    )
    subtrahend_common_continuum_polygons_ids = flags_to_true_indices(
            subtrahend_boxes_have_common_continuum
    )
    if not subtrahend_common_continuum_polygons_ids:
        return [minuend, *subtrahend_polygons]
    subtrahend_common_continuum_polygons = [
        subtrahend_polygons[index]
        for index in subtrahend_common_continuum_polygons_ids
    ]
    operation = ShapedSymmetricDifference.from_segments_iterables(
            polygon_to_correctly_oriented_segments(minuend, segment_cls),
            chain.from_iterable(
                    polygon_to_correctly_oriented_segments(polygon,
                                                           segment_cls)
                    for polygon in subtrahend_common_continuum_polygons
            )
    )
    result = operation.reduce_events(list(operation), contour_cls, polygon_cls)
    result.extend(
            subtrahend_polygons[index]
            for index in flags_to_false_indices(
                    subtrahend_boxes_have_common_continuum
            )
    )
    return result


def symmetric_subtract_multipolygon_from_multipolygon(
        minuend: hints.Multipolygon[hints.Scalar],
        subtrahend: hints.Multipolygon[hints.Scalar],
        contour_cls: t.Type[hints.Contour[hints.Scalar]],
        polygon_cls: t.Type[hints.Polygon[hints.Scalar]],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Polygon[hints.Scalar]]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    minuend_polygons, subtrahend_polygons = (minuend.polygons,
                                             subtrahend.polygons)
    if do_boxes_have_no_common_continuum(minuend_bounding_box,
                                         subtrahend_bounding_box):
        return [*minuend_polygons, *subtrahend_polygons]
    minuend_boxes = [polygon.bounding_box for polygon in minuend_polygons]
    minuend_boxes_have_common_continuum = to_boxes_have_common_continuum(
            minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_continuum_polygons_ids = flags_to_true_indices(
            minuend_boxes_have_common_continuum
    )
    if not minuend_common_continuum_polygons_ids:
        return [*minuend_polygons, *subtrahend_polygons]
    subtrahend_boxes = [polygon.bounding_box for polygon in
                        subtrahend_polygons]
    subtrahend_boxes_have_common_continuum = to_boxes_have_common_continuum(
            subtrahend_boxes, minuend_bounding_box
    )
    subtrahend_common_continuum_polygons_ids = flags_to_true_indices(
            subtrahend_boxes_have_common_continuum
    )
    if not subtrahend_common_continuum_polygons_ids:
        return [*minuend_polygons, *subtrahend_polygons]
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
                    polygon_to_correctly_oriented_segments(polygon,
                                                           segment_cls)
                    for polygon in minuend_common_continuum_polygons
            ),
            chain.from_iterable(
                    polygon_to_correctly_oriented_segments(polygon,
                                                           segment_cls)
                    for polygon in subtrahend_common_continuum_polygons
            )
    )
    result = operation.reduce_events(list(operation), contour_cls, polygon_cls)
    result.extend(
            minuend_polygons[index]
            for index in flags_to_false_indices(
                    minuend_boxes_have_common_continuum
            )
    )
    result.extend(
            subtrahend_polygons[index]
            for index in flags_to_false_indices(
                    subtrahend_boxes_have_common_continuum
            )
    )
    return result


def symmetric_subtract_segment_from_segment(
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
            and subtrahend_end_orientation is not Orientation.COLLINEAR):
        if subtrahend_start_orientation == subtrahend_end_orientation:
            return multisegment_cls([minuend, subtrahend])
        else:
            minuend_start_orientation = orient(subtrahend_start,
                                               subtrahend_end, minuend_start)
            minuend_end_orientation = orient(subtrahend_start, subtrahend_end,
                                             minuend_end)
            if (minuend_start_orientation is not Orientation.COLLINEAR
                    and minuend_end_orientation is not Orientation.COLLINEAR):
                if minuend_start_orientation == minuend_end_orientation:
                    return multisegment_cls([minuend, subtrahend])
                else:
                    cross_point = to_segments_intersection_point(
                            minuend_start, minuend_end, subtrahend_start,
                            subtrahend_end
                    )
                    return multisegment_cls(
                            [segment_cls(minuend_start, cross_point),
                             segment_cls(cross_point, minuend_end),
                             segment_cls(subtrahend_start, cross_point),
                             segment_cls(cross_point, subtrahend_end)]
                    )
            elif minuend_start_orientation is not Orientation.COLLINEAR:
                if subtrahend_start < minuend_end < subtrahend_end:
                    return multisegment_cls(
                            [minuend,
                             segment_cls(subtrahend_start, minuend_end),
                             segment_cls(minuend_end, subtrahend_end)]
                    )
                else:
                    return multisegment_cls([minuend, subtrahend])
            elif subtrahend_start < minuend_start < subtrahend_end:
                return multisegment_cls(
                        [minuend,
                         segment_cls(subtrahend_start, minuend_start),
                         segment_cls(minuend_start, subtrahend_end)]
                )
            else:
                return multisegment_cls([minuend, subtrahend])
    elif subtrahend_start_orientation is not Orientation.COLLINEAR:
        if minuend_start < subtrahend_end < minuend_end:
            return multisegment_cls(
                    [minuend,
                     segment_cls(minuend_start, subtrahend_end),
                     segment_cls(subtrahend_end, minuend_end)]
            )
        else:
            return multisegment_cls([minuend, subtrahend])
    elif subtrahend_end_orientation is not Orientation.COLLINEAR:
        if minuend_start < subtrahend_start < minuend_end:
            return multisegment_cls(
                    [minuend,
                     segment_cls(minuend_start, subtrahend_start),
                     segment_cls(subtrahend_start, minuend_end)]
            )
        else:
            return multisegment_cls([minuend, subtrahend])
    elif starts_equal:
        return segment_cls(minuend_end, subtrahend_end)
    elif ends_equal:
        return segment_cls(minuend_start, subtrahend_start)
    elif subtrahend_start == minuend_end or subtrahend_end == minuend_start:
        return multisegment_cls([minuend, subtrahend])
    elif minuend_start < subtrahend_start < minuend_end:
        if subtrahend_end < minuend_end:
            return multisegment_cls(
                    [segment_cls(minuend_start, subtrahend_start),
                     segment_cls(subtrahend_end, minuend_end)]
            )
        else:
            return segment_cls(minuend_start, subtrahend_start)
    elif subtrahend_start < minuend_start < subtrahend_end:
        if minuend_end < subtrahend_end:
            return multisegment_cls(
                    [segment_cls(subtrahend_start, minuend_start),
                     segment_cls(minuend_end, subtrahend_end)]
            )
        else:
            return segment_cls(subtrahend_start, minuend_start)
    else:
        return multisegment_cls([minuend, subtrahend])


_Multisegmental = t.Union[
    hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar]
]


def symmetric_subtract_multisegmental_from_segment(
        minuend: hints.Segment[hints.Scalar],
        subtrahend: _Multisegmental[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Segment[hints.Scalar]]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    subtrahend_segments = subtrahend.segments
    if do_boxes_have_no_common_continuum(minuend_bounding_box,
                                         subtrahend_bounding_box):
        return [minuend, *subtrahend_segments]
    subtrahend_boxes = [segment.bounding_box
                        for segment in subtrahend_segments]
    subtrahend_boxes_have_common_continuum = to_boxes_have_common_continuum(
            subtrahend_boxes, minuend_bounding_box
    )
    subtrahend_common_continuum_segments_ids = flags_to_true_indices(
            subtrahend_boxes_have_common_continuum
    )
    if not subtrahend_common_continuum_segments_ids:
        return [minuend, *subtrahend_segments]
    subtrahend_common_continuum_segments = [
        subtrahend_segments[index]
        for index in subtrahend_common_continuum_segments_ids
    ]
    operation = LinearSymmetricDifference.from_segments_iterables(
            [minuend], subtrahend_common_continuum_segments
    )
    result = operation.reduce_events(list(operation), segment_cls)
    result.extend(
            subtrahend_segments[index]
            for index in flags_to_false_indices(
                    subtrahend_boxes_have_common_continuum
            )
    )
    return result


def symmetric_subtract_segment_from_multisegmental(
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
        return [*minuend_segments, subtrahend]
    minuend_boxes = [segment.bounding_box for segment in minuend_segments]
    minuend_boxes_have_common_continuum = to_boxes_have_common_continuum(
            minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_continuum_segments_ids = flags_to_true_indices(
            minuend_boxes_have_common_continuum
    )
    if not minuend_common_continuum_segments_ids:
        return [*minuend_segments, subtrahend]
    minuend_common_continuum_segments = [
        minuend_segments[index]
        for index in minuend_common_continuum_segments_ids
    ]
    operation = LinearSymmetricDifference.from_segments_iterables(
            minuend_common_continuum_segments, [subtrahend]
    )
    result = operation.reduce_events(list(operation), segment_cls)
    result.extend(
            minuend_segments[index]
            for index in flags_to_false_indices(
                    minuend_boxes_have_common_continuum
            )
    )
    return result


def symmetric_subtract_multisegmental_from_multisegmental(
        minuend: _Multisegmental[hints.Scalar],
        subtrahend: _Multisegmental[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]],
        /
) -> t.List[hints.Segment[hints.Scalar]]:
    minuend_bounding_box, subtrahend_bounding_box = (minuend.bounding_box,
                                                     subtrahend.bounding_box)
    minuend_segments, subtrahend_segments = (minuend.segments,
                                             subtrahend.segments)
    if do_boxes_have_no_common_continuum(minuend_bounding_box,
                                         subtrahend_bounding_box):
        return [*minuend_segments, *subtrahend_segments]
    minuend_boxes = [segment.bounding_box for segment in minuend_segments]
    minuend_boxes_have_common_continuum = to_boxes_have_common_continuum(
            minuend_boxes, subtrahend_bounding_box
    )
    minuend_common_continuum_segments_ids = flags_to_true_indices(
            minuend_boxes_have_common_continuum
    )
    if not minuend_common_continuum_segments_ids:
        return [*minuend_segments, *subtrahend_segments]
    subtrahend_boxes = [segment.bounding_box
                        for segment in subtrahend_segments]
    subtrahend_boxes_have_common_continuum = to_boxes_have_common_continuum(
            subtrahend_boxes, minuend_bounding_box
    )
    subtrahend_common_continuum_segments_ids = flags_to_true_indices(
            subtrahend_boxes_have_common_continuum
    )
    if not subtrahend_common_continuum_segments_ids:
        return [*minuend_segments, *subtrahend_segments]
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
            subtrahend_common_continuum_segments
    )
    result = operation.reduce_events(list(operation), segment_cls)
    result.extend(
            minuend_segments[index]
            for index in flags_to_false_indices(
                    minuend_boxes_have_common_continuum
            )
    )
    result.extend(
            subtrahend_segments[index]
            for index in flags_to_false_indices(
                    subtrahend_boxes_have_common_continuum
            )
    )
    return result
