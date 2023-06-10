from __future__ import annotations

from rene import (Orientation,
                  Relation,
                  hints)
from rene._utils import (orient,
                         point_vertex_line_divides_angle,
                         to_sorted_pair)


def relate_to_contour(
        start: hints.Point[hints.Scalar],
        end: hints.Point[hints.Scalar],
        contour: hints.Contour[hints.Scalar],
        /
) -> Relation:
    has_no_cross = has_no_touch = True
    last_touched_edge_index = last_touched_edge_start = None
    for index, contour_segment in enumerate(contour.segments):
        contour_segment_end, contour_segment_start = (contour_segment.end,
                                                      contour_segment.start)
        relation = relate_to_segment(
                contour_segment_start, contour_segment_end, start, end
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
        elif has_no_cross and relation is Relation.CROSS:
            has_no_cross = False
    if (has_no_cross
            and not has_no_touch
            and last_touched_edge_index == contour.vertices_count - 1):
        vertices = contour.vertices
        if ((relate_to_segment(vertices[-1], vertices[0], start, end)
             is Relation.TOUCH)
                and start != vertices[-1] != end
                and start != vertices[0] != end
                and (orient(start, end, vertices[-1])
                     is Orientation.COLLINEAR)
                and point_vertex_line_divides_angle(start, vertices[-1],
                                                    vertices[-2],
                                                    vertices[0])):
            has_no_cross = False
    return ((Relation.DISJOINT if has_no_touch else Relation.TOUCH)
            if has_no_cross
            else Relation.CROSS)


def relate_to_segment(
        goal_start: hints.Point[hints.Scalar],
        goal_end: hints.Point[hints.Scalar],
        test_start: hints.Point[hints.Scalar],
        test_end: hints.Point[hints.Scalar],
        /
) -> Relation:
    assert goal_start != goal_end
    assert goal_start != goal_end
    goal_start, goal_end = to_sorted_pair(goal_start, goal_end)
    test_start, test_end = to_sorted_pair(test_start, test_end)
    starts_equal = test_start == goal_start
    ends_equal = test_end == goal_end
    if starts_equal and ends_equal:
        return Relation.EQUAL
    test_start_orientation = orient(goal_end, goal_start, test_start)
    test_end_orientation = orient(goal_end, goal_start, test_end)
    if (test_start_orientation is not Orientation.COLLINEAR
            and test_end_orientation is not Orientation.COLLINEAR):
        if test_start_orientation == test_end_orientation:
            return Relation.DISJOINT
        else:
            goal_start_orientation = orient(test_start, test_end, goal_start)
            goal_end_orientation = orient(test_start, test_end, goal_end)
            if (goal_start_orientation is not Orientation.COLLINEAR
                    and goal_end_orientation is not Orientation.COLLINEAR):
                if goal_start_orientation == goal_end_orientation:
                    return Relation.DISJOINT
                else:
                    return Relation.CROSS
            elif goal_start_orientation is not Orientation.COLLINEAR:
                if test_start < goal_end < test_end:
                    return Relation.TOUCH
                else:
                    return Relation.DISJOINT
            elif test_start < goal_start < test_end:
                return Relation.TOUCH
            else:
                return Relation.DISJOINT
    elif test_start_orientation is not Orientation.COLLINEAR:
        if goal_start <= test_end <= goal_end:
            return Relation.TOUCH
        else:
            return Relation.DISJOINT
    elif test_end_orientation is not Orientation.COLLINEAR:
        if goal_start <= test_start <= goal_end:
            return Relation.TOUCH
        else:
            return Relation.DISJOINT
    elif starts_equal:
        if test_end < goal_end:
            return Relation.COMPOSITE
        else:
            return Relation.COMPONENT
    elif ends_equal:
        if test_start < goal_start:
            return Relation.COMPONENT
        else:
            return Relation.COMPOSITE
    elif test_start == goal_end or test_end == goal_start:
        return Relation.TOUCH
    elif goal_start < test_start < goal_end:
        if test_end < goal_end:
            return Relation.COMPOSITE
        else:
            return Relation.OVERLAP
    elif test_start < goal_start < test_end:
        if goal_end < test_end:
            return Relation.COMPONENT
        else:
            return Relation.OVERLAP
    else:
        return Relation.DISJOINT