from __future__ import annotations

import typing as t

from rene import Location, Orientation, Relation, hints
from rene._hints import Orienteer, SegmentsIntersectionScale
from rene._utils import (
    locate_point_in_segment,
    point_vertex_line_divides_angle,
    subtract_segments_overlap,
    to_sorted_pair,
)


def relate_to_contour(
    start: hints.Point[hints.Scalar],
    end: hints.Point[hints.Scalar],
    contour: hints.Contour[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    /,
) -> Relation:
    return relate_to_contour_segments(start, end, contour.segments, orienteer)


def relate_to_contour_segments(
    start: hints.Point[hints.Scalar],
    end: hints.Point[hints.Scalar],
    contour_segments: t.Sequence[hints.Segment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    /,
) -> Relation:
    has_no_cross = has_no_touch = True
    last_touched_edge_index = last_touched_edge_start = None
    first_contour_segment = contour_segments[0]
    for index, contour_segment in enumerate(contour_segments):
        contour_segment_start, contour_segment_end = (
            contour_segment.start,
            contour_segment.end,
        )
        relation = relate_to_segment_endpoints(
            start, end, contour_segment_start, contour_segment_end, orienteer
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
                    and (
                        orienteer(start, end, contour_segment_start)
                        is Orientation.COLLINEAR
                    )
                    and point_vertex_line_divides_angle(
                        start,
                        contour_segment_start,
                        last_touched_edge_start,
                        contour_segment_end,
                        orienteer,
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
    if (
        not has_no_touch
        and has_no_cross
        and last_touched_edge_index == len(contour_segments) - 1
        and start != first_contour_segment.start != end
        and start != first_contour_segment.end != end
        and (
            orienteer(start, end, first_contour_segment.start)
            is Orientation.COLLINEAR
        )
    ):
        assert last_touched_edge_start is not None
        if point_vertex_line_divides_angle(
            start,
            first_contour_segment.start,
            last_touched_edge_start,
            first_contour_segment.end,
            orienteer,
        ):
            has_no_cross = False
    return (
        (Relation.DISJOINT if has_no_touch else Relation.TOUCH)
        if has_no_cross
        else Relation.CROSS
    )


def relate_to_multisegment(
    start: hints.Point[hints.Scalar],
    end: hints.Point[hints.Scalar],
    multisegment: hints.Multisegment[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersection_scale: SegmentsIntersectionScale[hints.Scalar],
    /,
) -> Relation:
    return relate_to_multisegment_segments(
        start,
        end,
        multisegment.segments,
        orienteer,
        segments_intersection_scale,
    )


def relate_to_multisegment_segments(
    start: hints.Point[hints.Scalar],
    end: hints.Point[hints.Scalar],
    multisegment_segments: t.Sequence[hints.Segment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segments_intersection_scale: SegmentsIntersectionScale[hints.Scalar],
    /,
) -> Relation:
    is_segment_superset = has_no_touch = has_no_cross = has_no_overlap = True
    clockwise_middle_touch_scales: list[hints.Scalar] = []
    counterclockwise_middle_touch_scales: list[hints.Scalar] = []
    components: list[
        tuple[hints.Point[hints.Scalar], hints.Point[hints.Scalar]]
    ] = []
    if start > end:
        start, end = end, start
    original_start, original_end = start, end
    for multisegment_segment in multisegment_segments:
        multisegment_segment_start, multisegment_segment_end = (
            multisegment_segment.start,
            multisegment_segment.end,
        )
        relation = relate_to_segment_endpoints(
            original_start,
            original_end,
            multisegment_segment_start,
            multisegment_segment_end,
            orienteer,
        )
        if relation is Relation.COMPONENT or relation is Relation.EQUAL:
            return Relation.COMPONENT
        elif relation is Relation.COMPOSITE:
            if has_no_overlap:
                has_no_overlap = False
            if start in (multisegment_segment_start, multisegment_segment_end):
                start = max(
                    multisegment_segment_start, multisegment_segment_end
                )
            elif end in (multisegment_segment_start, multisegment_segment_end):
                end = min(multisegment_segment_start, multisegment_segment_end)
            else:
                components.append(
                    to_sorted_pair(
                        multisegment_segment_start, multisegment_segment_end
                    )
                )
        elif relation is Relation.OVERLAP:
            if is_segment_superset:
                is_segment_superset = False
            if has_no_overlap:
                has_no_overlap = False
            start, end = subtract_segments_overlap(
                start,
                end,
                multisegment_segment_start,
                multisegment_segment_end,
            )
        else:
            if is_segment_superset:
                is_segment_superset = False
            if has_no_overlap:
                if relation is Relation.TOUCH:
                    if has_no_touch:
                        has_no_touch = False
                    if has_no_cross and (
                        (
                            multisegment_segment_start
                            != original_start
                            != multisegment_segment_end
                        )
                        and (
                            multisegment_segment_start
                            != original_end
                            != multisegment_segment_end
                        )
                    ):
                        intersection_scale = segments_intersection_scale(
                            original_start,
                            original_end,
                            multisegment_segment_start,
                            multisegment_segment_end,
                        )
                        non_touched_endpoint = (
                            multisegment_segment_start
                            if locate_point_in_segment(
                                original_start,
                                original_end,
                                multisegment_segment_end,
                                orienteer,
                            )
                            is Location.BOUNDARY
                            else multisegment_segment_end
                        )
                        (
                            counterclockwise_middle_touch_scales
                            if (
                                orienteer(
                                    original_start,
                                    original_end,
                                    non_touched_endpoint,
                                )
                                is Orientation.COUNTERCLOCKWISE
                            )
                            else clockwise_middle_touch_scales
                        ).append(intersection_scale)
                elif has_no_cross and relation is Relation.CROSS:
                    has_no_cross = False
    if has_no_overlap:
        if (
            has_no_cross
            and clockwise_middle_touch_scales
            and counterclockwise_middle_touch_scales
        ):
            less_scales, more_scales = (
                (
                    clockwise_middle_touch_scales,
                    counterclockwise_middle_touch_scales,
                )
                if (
                    len(clockwise_middle_touch_scales)
                    < len(counterclockwise_middle_touch_scales)
                )
                else (
                    counterclockwise_middle_touch_scales,
                    clockwise_middle_touch_scales,
                )
            )
            more_scales_set = set(more_scales)
            if any(scale in more_scales_set for scale in less_scales):
                has_no_cross = False
        return (
            Relation.DISJOINT
            if has_no_touch and has_no_cross
            else (Relation.TOUCH if has_no_cross else Relation.CROSS)
        )
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
        return (
            (Relation.EQUAL if is_segment_superset else Relation.COMPONENT)
            if (
                min_component_start == start
                and max_component_end == end
                and all(
                    component_end in components_starts
                    or component_end == max_component_end
                    for _, component_end in components
                )
            )
            else (
                Relation.COMPOSITE if is_segment_superset else Relation.OVERLAP
            )
        )
    else:
        return (
            (Relation.EQUAL if is_segment_superset else Relation.COMPONENT)
            if start == end
            else (
                Relation.COMPOSITE if is_segment_superset else Relation.OVERLAP
            )
        )


def relate_to_segment_endpoints(
    first_start: hints.Point[hints.Scalar],
    first_end: hints.Point[hints.Scalar],
    second_start: hints.Point[hints.Scalar],
    second_end: hints.Point[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    /,
) -> Relation:
    assert first_start != first_end
    assert first_start != first_end
    first_start, first_end = to_sorted_pair(first_start, first_end)
    second_start, second_end = to_sorted_pair(second_start, second_end)
    if first_start == second_start and first_end == second_end:
        return Relation.EQUAL
    second_start_orientation = orienteer(first_end, first_start, second_start)
    second_end_orientation = orienteer(first_end, first_start, second_end)
    if second_start_orientation is second_end_orientation:
        if second_start_orientation is not Orientation.COLLINEAR:
            return Relation.DISJOINT
        elif first_start == second_start:
            if second_end < first_end:
                return Relation.COMPOSITE
            else:
                return Relation.COMPONENT
        elif first_end == second_end:
            if second_start < first_start:
                return Relation.COMPONENT
            else:
                return Relation.COMPOSITE
        elif first_start == second_end or first_end == second_start:
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
    elif second_start_orientation is Orientation.COLLINEAR:
        if first_start <= second_start <= first_end:
            return Relation.TOUCH
        else:
            return Relation.DISJOINT
    elif second_end_orientation is Orientation.COLLINEAR:
        if first_start <= second_end <= first_end:
            return Relation.TOUCH
        else:
            return Relation.DISJOINT
    else:
        first_start_orientation = orienteer(
            second_start, second_end, first_start
        )
        first_end_orientation = orienteer(second_start, second_end, first_end)
        if first_start_orientation is first_end_orientation:
            assert first_start_orientation is not Orientation.COLLINEAR
            return Relation.DISJOINT
        elif first_start_orientation is Orientation.COLLINEAR:
            if second_start < first_start < second_end:
                return Relation.TOUCH
            else:
                return Relation.DISJOINT
        elif first_end_orientation is Orientation.COLLINEAR:
            if second_start < first_end < second_end:
                return Relation.TOUCH
            else:
                return Relation.DISJOINT
        else:
            return Relation.CROSS


def relate_to_region(
    start: hints.Point[hints.Scalar],
    end: hints.Point[hints.Scalar],
    border: hints.Contour[hints.Scalar],
    reverse_orientation: bool,
    orienteer: Orienteer[hints.Scalar],
    /,
) -> Relation:
    edges = border.segments
    relation_with_border = _relate_to_region_border(
        start, end, edges, orienteer
    )
    if (
        relation_with_border is Relation.CROSS
        or relation_with_border is Relation.COMPONENT
    ):
        return relation_with_border
    start_index, start_location = _locate_point_in_region(
        edges, start, orienteer
    )
    if relation_with_border is Relation.DISJOINT:
        return (
            Relation.DISJOINT
            if start_location is Location.EXTERIOR
            else Relation.WITHIN
        )
    elif start_location is Location.EXTERIOR:
        return Relation.TOUCH
    elif start_location is Location.INTERIOR:
        return Relation.ENCLOSED
    else:
        assert start_index is not None
        end_index, end_location = _locate_point_in_region(
            edges, end, orienteer
        )
        if end_location is Location.EXTERIOR:
            return Relation.TOUCH
        elif end_location is Location.INTERIOR:
            return Relation.ENCLOSED
        else:
            assert end_index is not None
            assert end_location is Location.BOUNDARY, end_location
            edge = edges[start_index]
            edge_start, edge_end = edge.start, edge.end
            border_orientation = (
                Orientation.CLOCKWISE
                if reverse_orientation
                else Orientation.COUNTERCLOCKWISE
            )
            if start == edge_start:
                prev_start = (
                    edges[(start_index + 1) % len(edges)].end
                    if reverse_orientation
                    else edges[start_index - 1].start
                )
                if (
                    orienteer(prev_start, edge_start, edge_end)
                    is border_orientation
                ):
                    if (
                        orienteer(edge_start, prev_start, end)
                        is border_orientation
                    ) or (
                        orienteer(edge_end, edge_start, end)
                        is border_orientation
                    ):
                        return Relation.TOUCH
                elif (
                    orienteer(edge_start, prev_start, end)
                    is orienteer(edge_end, edge_start, end)
                    is border_orientation
                ):
                    return Relation.TOUCH
            elif start == edge_end:
                next_end = (
                    edges[start_index - 1].start
                    if reverse_orientation
                    else edges[(start_index + 1) % len(edges)].end
                )
                if (
                    orienteer(edge_start, edge_end, next_end)
                    is border_orientation
                ) and (
                    (
                        orienteer(edge_end, edge_start, end)
                        is border_orientation
                    )
                    or (
                        orienteer(next_end, edge_end, end)
                        is border_orientation
                    )
                    or (
                        orienteer(edge_end, edge_start, end)
                        is orienteer(next_end, edge_end, end)
                        is border_orientation
                    )
                ):
                    return Relation.TOUCH
            elif orienteer(edge_end, edge_start, end) is border_orientation:
                return Relation.TOUCH
            edge = edges[end_index]
            edge_start, edge_end = edge.start, edge.end
            if end == edge_start:
                prev_start = (
                    edges[(end_index + 1) % len(edges)].end
                    if reverse_orientation
                    else edges[end_index - 1].start
                )
                if (
                    orienteer(prev_start, edge_start, edge_end)
                    is border_orientation
                ):
                    if (
                        orienteer(edge_start, prev_start, start)
                        is border_orientation
                    ) or (
                        orienteer(edge_end, edge_start, start)
                        is border_orientation
                    ):
                        return Relation.TOUCH
                elif (
                    orienteer(edge_start, prev_start, start)
                    is orienteer(edge_end, edge_start, start)
                    is border_orientation
                ):
                    return Relation.TOUCH
            elif end == edge_end:
                next_end = (
                    edges[end_index - 1].start
                    if reverse_orientation
                    else edges[(end_index + 1) % len(edges)].end
                )
                if (
                    orienteer(edge_start, edge_end, next_end)
                    is border_orientation
                ):
                    if (
                        orienteer(edge_end, edge_start, start)
                        is border_orientation
                    ) or (
                        orienteer(next_end, edge_end, start)
                        is border_orientation
                    ):
                        return Relation.TOUCH
                elif (
                    orienteer(edge_end, edge_start, start)
                    is orienteer(next_end, edge_end, start)
                    is border_orientation
                ):
                    return Relation.TOUCH
            elif orienteer(edge_end, edge_start, start) is border_orientation:
                return Relation.TOUCH
            return Relation.ENCLOSED


def _relate_to_region_border(
    start: hints.Point[hints.Scalar],
    end: hints.Point[hints.Scalar],
    edges: t.Sequence[hints.Segment[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    /,
) -> Relation:
    # similar to segment-in-contour check
    # but cross has higher priority over overlap
    # because cross with border will be considered as cross with region
    # whereas overlap with border can't be an overlap with region
    # and should be classified by further analysis
    has_touch = has_overlap = False
    last_touched_edge_index = last_touched_edge_start = None
    for index, edge in enumerate(edges):
        edge_start, edge_end = edge.start, edge.end
        relation_with_edge = relate_to_segment_endpoints(
            edge_start, edge_end, start, end, orienteer
        )
        if (
            relation_with_edge is Relation.COMPONENT
            or relation_with_edge is Relation.OVERLAP
        ):
            if not has_overlap:
                has_overlap = True
        elif (
            relation_with_edge is Relation.COMPOSITE
            or relation_with_edge is Relation.EQUAL
        ):
            return Relation.COMPONENT
        elif relation_with_edge is Relation.CROSS:
            return Relation.CROSS
        elif relation_with_edge is Relation.TOUCH:
            if not has_touch:
                has_touch = True
            else:
                assert last_touched_edge_index is not None
                if (
                    index - last_touched_edge_index == 1
                    and start != edge_start
                    and start != edge_end
                    and end != edge_start
                    and end != edge_end
                    and (
                        orienteer(start, end, edge_start)
                        is Orientation.COLLINEAR
                    )
                    and point_vertex_line_divides_angle(
                        start,
                        edge_start,
                        last_touched_edge_start,
                        edge_end,
                        orienteer,
                    )
                ):
                    return Relation.CROSS
            last_touched_edge_index = index
            last_touched_edge_start = edge_start
    if has_touch and last_touched_edge_index == len(edges) - 1:
        first_edge = edges[0]
        first_edge_start, first_edge_end = first_edge.start, first_edge.end
        if (
            (
                relate_to_segment_endpoints(
                    first_edge_start, first_edge_end, start, end, orienteer
                )
                is Relation.TOUCH
            )
            and start != first_edge_start
            and start != first_edge_end
            and end != first_edge_start
            and end != first_edge_end
            and (
                orienteer(start, end, first_edge_start)
                is Orientation.COLLINEAR
            )
            and point_vertex_line_divides_angle(
                start,
                first_edge_start,
                edges[-1].start,
                first_edge_end,
                orienteer,
            )
        ):
            return Relation.CROSS
    return (
        Relation.OVERLAP
        if has_overlap
        else (Relation.TOUCH if has_touch else Relation.DISJOINT)
    )


def _locate_point_in_region(
    edges: t.Sequence[hints.Segment[hints.Scalar]],
    point: hints.Point[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    /,
) -> tuple[int | None, Location]:
    result = False
    point_y = point.y
    for index, edge in enumerate(edges):
        start, end = edge.start, edge.end
        if (
            locate_point_in_segment(start, end, point, orienteer)
            is Location.BOUNDARY
        ):
            return index, Location.BOUNDARY
        if (start.y > point_y) is not (end.y > point_y) and (
            (end.y > start.y)
            is (orienteer(start, end, point) is Orientation.COUNTERCLOCKWISE)
        ):
            result = not result
    return None, (Location.INTERIOR if result else Location.EXTERIOR)
