from __future__ import annotations

import typing as t

from rene import (Location,
                  Orientation,
                  Relation,
                  hints)
from rene._utils import (locate_point_in_segment,
                         orient,
                         point_vertex_line_divides_angle,
                         subtract_segments_overlap,
                         to_segments_intersection_scale,
                         to_sorted_pair)


def relate_to_contour(
        start: hints.Point[hints.Scalar],
        end: hints.Point[hints.Scalar],
        contour: hints.Contour[hints.Scalar],
        /
) -> Relation:
    has_no_cross = has_no_touch = True
    last_touched_edge_index = last_touched_edge_start = None
    contour_segments = contour.segments
    first_contour_segment = contour_segments[0]
    for index, contour_segment in enumerate(contour_segments):
        contour_segment_start, contour_segment_end = (contour_segment.start,
                                                      contour_segment.end)
        relation = relate_to_segment(
                start, end, contour_segment_start, contour_segment_end
        )
        if relation is Relation.COMPONENT or relation is Relation.EQUAL:
            return Relation.COMPONENT
        elif relation is Relation.OVERLAP or relation is Relation.COMPOSITE:
            return Relation.OVERLAP
        elif relation is Relation.TOUCH:
            if has_no_touch:
                has_no_touch = False
            elif has_no_cross:
                assert last_touched_edge_index is not None
                assert last_touched_edge_start is not None
                if (
                        index - last_touched_edge_index == 1
                        and start != contour_segment_start != end
                        and start != contour_segment_end != end
                        and (orient(start, end, contour_segment_start)
                             is Orientation.COLLINEAR)
                        and
                        point_vertex_line_divides_angle(
                                start, contour_segment_start,
                                contour_segment_end, last_touched_edge_start
                        )
                ):
                    has_no_cross = False
            last_touched_edge_index = index
            last_touched_edge_start = contour_segment_start
        elif relation is Relation.CROSS:
            if has_no_cross:
                has_no_cross = False
        else:
            assert relation is Relation.DISJOINT
    if (not has_no_touch
            and has_no_cross
            and last_touched_edge_index == len(contour.segments) - 1
            and start != first_contour_segment.start != end
            and start != first_contour_segment.end != end
            and (orient(start, end, first_contour_segment.start)
                 is Orientation.COLLINEAR)):
        assert last_touched_edge_start is not None
        if point_vertex_line_divides_angle(start,
                                           first_contour_segment.start,
                                           first_contour_segment.end,
                                           last_touched_edge_start):
            has_no_cross = False
    return ((Relation.DISJOINT if has_no_touch else Relation.TOUCH)
            if has_no_cross
            else Relation.CROSS)


def relate_to_multisegment(
        start: hints.Point[hints.Scalar],
        end: hints.Point[hints.Scalar],
        multisegment: hints.Multisegment[hints.Scalar]
) -> Relation:
    is_segment_superset = has_no_touch = has_no_cross = has_no_overlap = True
    clockwise_middle_touch_scales: t.List[hints.Scalar] = []
    counterclockwise_middle_touch_scales: t.List[hints.Scalar] = []
    components: t.List[
        t.Tuple[hints.Point[hints.Scalar], hints.Point[hints.Scalar]]
    ] = []
    if start > end:
        start, end = end, start
    original_start, original_end = start, end
    for multisegment_segment in multisegment.segments:
        multisegment_segment_start, multisegment_segment_end = (
            multisegment_segment.start, multisegment_segment.end
        )
        relation = relate_to_segment(original_start, original_end,
                                     multisegment_segment_start,
                                     multisegment_segment_end)
        if relation is Relation.COMPONENT or relation is Relation.EQUAL:
            return Relation.COMPONENT
        elif relation is Relation.COMPOSITE:
            if has_no_overlap:
                has_no_overlap = False
            if (start == multisegment_segment_start
                    or start == multisegment_segment_end):
                start = max(multisegment_segment_start,
                            multisegment_segment_end)
            elif (end == multisegment_segment_start
                  or end == multisegment_segment_end):
                end = min(multisegment_segment_start, multisegment_segment_end)
            else:
                components.append(to_sorted_pair(multisegment_segment_start,
                                                 multisegment_segment_end))
        elif relation is Relation.OVERLAP:
            if is_segment_superset:
                is_segment_superset = False
            if has_no_overlap:
                has_no_overlap = False
            start, end = subtract_segments_overlap(start, end,
                                                   multisegment_segment_start,
                                                   multisegment_segment_end)
        else:
            if is_segment_superset:
                is_segment_superset = False
            if has_no_overlap:
                if relation is Relation.TOUCH:
                    if has_no_touch:
                        has_no_touch = False
                    if (has_no_cross
                            and
                            ((multisegment_segment_start != original_start
                              != multisegment_segment_end)
                             and (multisegment_segment_start != original_end
                                  != multisegment_segment_end))):
                        intersection_scale = to_segments_intersection_scale(
                                original_start, original_end,
                                multisegment_segment_start,
                                multisegment_segment_end
                        )
                        non_touched_endpoint = (
                            multisegment_segment_start
                            if locate_point_in_segment(
                                    original_start, original_end,
                                    multisegment_segment_end
                            ) is Location.BOUNDARY
                            else multisegment_segment_end
                        )
                        (
                            counterclockwise_middle_touch_scales
                            if (orient(original_start, original_end,
                                       non_touched_endpoint)
                                is Orientation.COUNTERCLOCKWISE)
                            else clockwise_middle_touch_scales
                        ).append(intersection_scale)
                elif has_no_cross and relation is Relation.CROSS:
                    has_no_cross = False
    if has_no_overlap:
        if (has_no_cross
                and clockwise_middle_touch_scales
                and counterclockwise_middle_touch_scales):
            less_scales, more_scales = (
                (clockwise_middle_touch_scales,
                 counterclockwise_middle_touch_scales)
                if (len(clockwise_middle_touch_scales)
                    < len(counterclockwise_middle_touch_scales))
                else (counterclockwise_middle_touch_scales,
                      clockwise_middle_touch_scales)
            )
            more_scales_set = set(more_scales)
            if any(scale in more_scales_set for scale in less_scales):
                has_no_cross = False
        return (Relation.DISJOINT
                if has_no_touch and has_no_cross
                else (Relation.TOUCH
                      if has_no_cross
                      else Relation.CROSS))
    elif components:
        components_iterator = iter(components)
        min_component_start, max_component_end = next(components_iterator)
        components_starts = {min_component_start}
        for component_start, component_end in components_iterator:
            components_starts.add(component_start)
            if min_component_start > component_start:
                min_component_start = component_start
            if max_component_end < component_end:
                max_component_end = component_end
        return ((Relation.EQUAL
                 if is_segment_superset
                 else Relation.COMPONENT)
                if (min_component_start == start
                    and max_component_end == end
                    and all(component_end in components_starts
                            or component_end == max_component_end
                            for _, component_end in components))
                else (Relation.COMPOSITE
                      if is_segment_superset
                      else Relation.OVERLAP))
    else:
        return ((Relation.EQUAL
                 if is_segment_superset
                 else Relation.COMPONENT)
                if start == end
                else (Relation.COMPOSITE
                      if is_segment_superset
                      else Relation.OVERLAP))


def relate_to_segment(
        first_start: hints.Point[hints.Scalar],
        first_end: hints.Point[hints.Scalar],
        second_start: hints.Point[hints.Scalar],
        second_end: hints.Point[hints.Scalar],
        /
) -> Relation:
    assert first_start != first_end
    assert first_start != first_end
    first_start, first_end = to_sorted_pair(first_start, first_end)
    second_start, second_end = to_sorted_pair(second_start, second_end)
    starts_equal = second_start == first_start
    ends_equal = second_end == first_end
    if starts_equal and ends_equal:
        return Relation.EQUAL
    second_start_orientation = orient(first_end, first_start, second_start)
    second_end_orientation = orient(first_end, first_start, second_end)
    if (second_start_orientation is not Orientation.COLLINEAR
            and second_end_orientation is not Orientation.COLLINEAR):
        if second_start_orientation is second_end_orientation:
            return Relation.DISJOINT
        else:
            first_start_orientation = orient(second_start, second_end,
                                             first_start)
            first_end_orientation = orient(second_start, second_end, first_end)
            if (first_start_orientation is not Orientation.COLLINEAR
                    and first_end_orientation is not Orientation.COLLINEAR):
                if first_start_orientation is first_end_orientation:
                    return Relation.DISJOINT
                else:
                    return Relation.CROSS
            elif first_start_orientation is not Orientation.COLLINEAR:
                if second_start < first_end < second_end:
                    return Relation.TOUCH
                else:
                    return Relation.DISJOINT
            elif second_start < first_start < second_end:
                return Relation.TOUCH
            else:
                return Relation.DISJOINT
    elif second_start_orientation is not Orientation.COLLINEAR:
        if first_start <= second_end <= first_end:
            return Relation.TOUCH
        else:
            return Relation.DISJOINT
    elif second_end_orientation is not Orientation.COLLINEAR:
        if first_start <= second_start <= first_end:
            return Relation.TOUCH
        else:
            return Relation.DISJOINT
    elif starts_equal:
        if second_end < first_end:
            return Relation.COMPOSITE
        else:
            return Relation.COMPONENT
    elif ends_equal:
        if second_start < first_start:
            return Relation.COMPONENT
        else:
            return Relation.COMPOSITE
    elif second_start == first_end or second_end == first_start:
        return Relation.TOUCH
    elif first_start < second_start < first_end:
        if second_end < first_end:
            return Relation.COMPOSITE
        else:
            return Relation.OVERLAP
    elif second_start < first_start < second_end:
        if first_end < second_end:
            return Relation.COMPONENT
        else:
            return Relation.OVERLAP
    else:
        return Relation.DISJOINT
