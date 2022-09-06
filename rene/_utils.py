from itertools import groupby
from typing import (Iterable,
                    List,
                    Sequence,
                    Tuple,
                    Type,
                    TypeVar,
                    Union)

from rithm import Fraction
from typing_extensions import Protocol

from rene._rene import (MIN_CONTOUR_VERTICES_COUNT,
                        Location,
                        Orientation,
                        Relation)
from rene.hints import (Box,
                        Empty,
                        Multipolygon,
                        Point,
                        Polygon)

_Self = TypeVar('_Self',
                contravariant=True)


class Ordered(Protocol[_Self]):
    def __lt__(self: _Self, other: _Self) -> bool:
        ...


_Ordered = TypeVar('_Ordered',
                   bound=Ordered)

_T = TypeVar('_T')


def to_boxes_ids_with_common_area(boxes: Iterable[Box],
                                  target_box: Box) -> List[int]:
    return [box_id
            for box_id, box in enumerate(boxes)
            if do_boxes_have_common_area(box, target_box)]


def to_boxes_ids_with_continuous_common_points(boxes: Iterable[Box],
                                               target_box: Box) -> List[int]:
    return [box_id
            for box_id, box in enumerate(boxes)
            if do_boxes_have_common_continuum(box, target_box)]


def do_boxes_have_common_area(first: Box, second: Box) -> bool:
    return not first.disjoint_with(second) and not first.touches(second)


def do_boxes_have_no_common_area(first: Box, second: Box) -> bool:
    return first.disjoint_with(second) or first.touches(second)


def do_boxes_have_common_continuum(first: Box, second: Box) -> bool:
    return (not first.disjoint_with(second)
            and
            (not first.touches(second)
             or (first.min_y != second.max_y and second.min_y != first.max_y)
             or (first.min_x != second.max_x and second.min_x != first.max_x)))


def do_boxes_have_no_common_continuum(first: Box, second: Box) -> bool:
    return (first.disjoint_with(second)
            or
            (first.touches(second)
             and (first.min_y == second.max_y or second.min_y == first.max_y)
             and (first.min_x == second.max_x or second.min_x == first.max_x)))


def ceil_log2(number: int) -> int:
    return number.bit_length() - (not (number & (number - 1)))


def collect_maybe_empty_polygons(
        polygons: Sequence[Polygon],
        empty_cls: Type[Empty],
        multipolygon_cls: Type[Multipolygon]
) -> Union[Empty, Multipolygon, Polygon]:
    return (collect_non_empty_polygons(polygons, multipolygon_cls)
            if polygons
            else empty_cls())


def collect_non_empty_polygons(
        polygons: Sequence[Polygon],
        multipolygon_cls: Type[Multipolygon]
) -> Union[Empty, Multipolygon, Polygon]:
    assert len(polygons) >= 1
    return polygons[0] if len(polygons) == 1 else multipolygon_cls(polygons)


def cross_multiply(first_start: Point,
                   first_end: Point,
                   second_start: Point,
                   second_end: Point) -> Fraction:
    return ((first_end.x - first_start.x) * (second_end.y - second_start.y)
            - (first_end.y - first_start.y) * (second_end.x - second_start.x))


def deduplicate(values: List[_T]) -> List[_T]:
    return [value for value, _ in groupby(values)]


def flags_to_false_indices(flags: Sequence[bool]) -> List[int]:
    return [index for index, flag in enumerate(flags) if not flag]


def flags_to_true_indices(flags: Sequence[bool]) -> List[int]:
    return [index for index, flag in enumerate(flags) if flag]


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


def merge_boxes(boxes: Iterable[Box]) -> Box:
    boxes_iterator = iter(boxes)
    first_box = next(boxes_iterator)
    max_x, min_x = first_box.max_x, first_box.min_x
    max_y, min_y = first_box.max_y, first_box.min_y
    for box in boxes_iterator:
        if box.max_x > max_x:
            max_x = box.max_x
        if box.min_x < min_x:
            min_x = box.min_x
        if box.max_y > max_y:
            max_y = box.max_y
        if box.min_y < min_y:
            min_y = box.min_y
    return type(first_box)(min_x, max_x, min_y, max_y)


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


def shrink_collinear_vertices2(vertices: Sequence[Point]) -> List[Point]:
    vertices = list(vertices)
    index = -len(vertices) + 1
    while index < 0:
        while (max(2, -index) < len(vertices)
               and orient(vertices[index + 1], vertices[index + 2],
                          vertices[index]) is Orientation.COLLINEAR):
            del vertices[index + 1]
        index += 1
    while index < len(vertices):
        while (max(2, index) < len(vertices)
               and orient(vertices[index - 1], vertices[index - 2],
                          vertices[index]) is Orientation.COLLINEAR):
            del vertices[index - 1]
        index += 1
    return vertices


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
    assert result == shrink_collinear_vertices2(vertices)
    return result


def to_boxes_have_common_area(boxes: Sequence[Box],
                              target_box: Box) -> List[bool]:
    return [do_boxes_have_common_area(box, target_box) for box in boxes]


def to_boxes_have_common_continuum(boxes: Sequence[Box],
                                   target_box: Box) -> List[bool]:
    return [do_boxes_have_common_continuum(box, target_box) for box in boxes]


def to_sign(value: Fraction) -> int:
    return 1 if value > 0 else (-1 if value else 0)


def to_sorted_pair(start: _Ordered, end: _Ordered
                   ) -> Tuple[_Ordered, _Ordered]:
    return (start, end) if start < end else (end, start)
