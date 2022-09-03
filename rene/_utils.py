from itertools import groupby
from typing import (List,
                    Sequence,
                    Tuple,
                    TypeVar)

from rithm import Fraction
from typing_extensions import Protocol

from rene._rene import (MIN_CONTOUR_VERTICES_COUNT,
                        Location,
                        Orientation,
                        Relation)
from rene.hints import Point

_Self = TypeVar('_Self',
                contravariant=True)


class Ordered(Protocol[_Self]):
    def __lt__(self: _Self, other: _Self) -> bool:
        ...


_Ordered = TypeVar('_Ordered',
                   bound=Ordered)

_T = TypeVar('_T')


def ceil_log2(number: int) -> int:
    return number.bit_length() - (not (number & (number - 1)))


def cross_multiply(first_start: Point,
                   first_end: Point,
                   second_start: Point,
                   second_end: Point) -> Fraction:
    return ((first_end.x - first_start.x) * (second_end.y - second_start.y)
            - (first_end.y - first_start.y) * (second_end.x - second_start.x))


def deduplicate(values: List[_T]) -> List[_T]:
    return [value for value, _ in groupby(values)]


def intersect_crossing_segments(first_start: Point,
                                first_end: Point,
                                second_start: Point,
                                second_end: Point) -> Point:
    scale = (cross_multiply(first_start, second_start, second_start,
                            second_end)
             / cross_multiply(first_start, first_end, second_start,
                              second_end))
    return type(first_start)(
            first_start.x + (first_end.x - first_start.x) * scale,
            first_start.y + (first_end.y - first_start.y) * scale
    )


def is_even(value: int) -> bool:
    return value & 1 == 0


def locate_point_in_point_point_point_circle(
        point: Point, first: Point, second: Point, third: Point
) -> Location:
    first_dx, first_dy = first.x - point.x, first.y - point.y
    second_dx, second_dy = second.x - point.x, second.y - point.y
    third_dx, third_dy = third.x - point.x, third.y - point.y
    return Location(
            to_sign(((first_dx * first_dx + first_dy * first_dy)
                     * (second_dx * third_dy - second_dy * third_dx))
                    - ((second_dx * second_dx + second_dy * second_dy)
                       * (first_dx * third_dy - first_dy * third_dx))
                    + ((third_dx * third_dx + third_dy * third_dy)
                       * (first_dx * second_dy - first_dy * second_dx)))
    )


def orient(vertex: Point,
           first_ray_point: Point,
           second_ray_point: Point) -> Orientation:
    return Orientation(to_sign(cross_multiply(vertex, first_ray_point, vertex,
                                              second_ray_point)))


def relate_segments(
        goal_start: Point, goal_end: Point, test_start: Point, test_end: Point
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
                if test_start < goal_end and goal_end < test_end:
                    return Relation.TOUCH
                else:
                    return Relation.DISJOINT
            elif test_start < goal_start and goal_start < test_end:
                return Relation.TOUCH
            else:
                return Relation.DISJOINT
    elif test_start_orientation is not Orientation.COLLINEAR:
        if goal_start <= test_end and test_end <= goal_end:
            return Relation.TOUCH
        else:
            return Relation.DISJOINT
    elif test_end_orientation is not Orientation.COLLINEAR:
        if goal_start <= test_start and test_start <= goal_end:
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
    elif goal_start < test_start and test_start < goal_end:
        if test_end < goal_end:
            return Relation.COMPOSITE
        else:
            return Relation.OVERLAP
    elif test_start < goal_start and goal_start < test_end:
        if goal_end < test_end:
            return Relation.COMPONENT
        else:
            return Relation.OVERLAP
    else:
        return Relation.DISJOINT


def shrink_collinear_vertices(vertices: Sequence[Point]) -> List[Point]:
    assert len(vertices) >= MIN_CONTOUR_VERTICES_COUNT
    result = [vertices[0]]
    for index in range(1, len(vertices) - 1):
        if (orient(result[-1], vertices[index], vertices[index + 1])
                is not Orientation.COLLINEAR):
            result.append(vertices[index])
    if (orient(result[-1], vertices[-1], result[0])
            is not Orientation.COLLINEAR):
        result.append(vertices[-1])
    elif len(result) > 2:
        result[0] = result.pop()
    return result


def to_sign(value: Fraction) -> int:
    return 1 if value > 0 else (-1 if value else 0)


def to_sorted_pair(start: _Ordered, end: _Ordered
                   ) -> Tuple[_Ordered, _Ordered]:
    return (start, end) if start < end else (end, start)
