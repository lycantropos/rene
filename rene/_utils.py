from __future__ import annotations

import sys as _sys
import typing as _t
from itertools import groupby

import typing_extensions as _te

from rene import (MIN_CONTOUR_VERTICES_COUNT,
                  Location,
                  Orientation)
from rene.hints import (Box,
                        Contour,
                        Empty,
                        Multipolygon,
                        Point,
                        Polygon,
                        Scalar)


class Ordered(_te.Protocol):
    def __lt__(self, other: _te.Self, /) -> bool:
        ...


_OrderedT = _t.TypeVar('_OrderedT',
                       bound=Ordered)
_T = _t.TypeVar('_T')


def are_contour_vertices_non_degenerate(
        vertices: _t.Sequence[Point[Scalar]], /
) -> bool:
    return (all(orient(vertices[index - 1], vertices[index],
                       vertices[index + 1]) is not Orientation.COLLINEAR
                for index in range(1, len(vertices) - 1))
            and (len(vertices) <= MIN_CONTOUR_VERTICES_COUNT
                 or ((orient(vertices[-2], vertices[-1], vertices[0])
                      is not Orientation.COLLINEAR)
                     and (orient(vertices[-1], vertices[0], vertices[1])
                          is not Orientation.COLLINEAR))))


def do_boxes_have_common_area(
        first: Box[Scalar], second: Box[Scalar], /
) -> bool:
    return not first.disjoint_with(second) and not first.touches(second)


def do_boxes_have_no_common_area(
        first: Box[Scalar], second: Box[Scalar], /
) -> bool:
    return first.disjoint_with(second) or first.touches(second)


def do_boxes_have_common_continuum(
        first: Box[Scalar], second: Box[Scalar], /
) -> bool:
    return (not first.disjoint_with(second)
            and
            (not first.touches(second)
             or (first.min_y != second.max_y and second.min_y != first.max_y)
             or (first.min_x != second.max_x and second.min_x != first.max_x)))


def do_boxes_have_no_common_continuum(
        first: Box[Scalar], second: Box[Scalar], /
) -> bool:
    return (first.disjoint_with(second)
            or
            (first.touches(second)
             and (first.min_y == second.max_y or second.min_y == first.max_y)
             and (first.min_x == second.max_x or second.min_x == first.max_x)))


def ceil_log2(number: int, /) -> int:
    return number.bit_length() - (not (number & (number - 1)))


def collect_maybe_empty_polygons(
        polygons: _t.Sequence[Polygon[Scalar]],
        empty_cls: _t.Type[Empty[Scalar]],
        multipolygon_cls: _t.Type[Multipolygon[Scalar]],
        /
) -> _t.Union[Empty[Scalar], Multipolygon[Scalar], Polygon[Scalar]]:
    return (collect_non_empty_polygons(polygons, multipolygon_cls)
            if polygons
            else empty_cls())


def collect_non_empty_polygons(
        polygons: _t.Sequence[Polygon[Scalar]],
        multipolygon_cls: _t.Type[Multipolygon[Scalar]],
        /
) -> _t.Union[Multipolygon[Scalar], Polygon[Scalar]]:
    assert len(polygons) >= 1
    return polygons[0] if len(polygons) == 1 else multipolygon_cls(polygons)


def cross_multiply(first_start: Point[Scalar],
                   first_end: Point[Scalar],
                   second_start: Point[Scalar],
                   second_end: Point[Scalar],
                   /) -> Scalar:
    return ((first_end.x - first_start.x) * (second_end.y - second_start.y)
            - (first_end.y - first_start.y) * (second_end.x - second_start.x))


def deduplicate(values: _t.List[_T], /) -> _t.List[_T]:
    return [value for value, _ in groupby(values)]


def flags_to_false_indices(flags: _t.Sequence[bool], /) -> _t.List[int]:
    return [index for index, flag in enumerate(flags) if not flag]


def flags_to_true_indices(flags: _t.Sequence[bool], /) -> _t.List[int]:
    return [index for index, flag in enumerate(flags) if flag]


def intersect_crossing_segments(first_start: Point[Scalar],
                                first_end: Point[Scalar],
                                second_start: Point[Scalar],
                                second_end: Point[Scalar], /) -> Point[Scalar]:
    scale = (cross_multiply(first_start, second_start, second_start,
                            second_end)
             / cross_multiply(first_start, first_end, second_start,
                              second_end))
    return type(first_start)(
            first_start.x + (first_end.x - first_start.x) * scale,
            first_start.y + (first_end.y - first_start.y) * scale
    )


def is_even(value: int, /) -> bool:
    return value & 1 == 0


def is_odd(value: int, /) -> bool:
    return value & 1 == 1


def locate_point_in_point_point_point_circle(point: Point[Scalar],
                                             first: Point[Scalar],
                                             second: Point[Scalar],
                                             third: Point[Scalar],
                                             /) -> Location:
    first_dx, first_dy = first.x - point.x, first.y - point.y
    second_dx, second_dy = second.x - point.x, second.y - point.y
    third_dx, third_dy = third.x - point.x, third.y - point.y
    raw = to_sign(((first_dx * first_dx + first_dy * first_dy)
                   * (second_dx * third_dy - second_dy * third_dx))
                  - ((second_dx * second_dx + second_dy * second_dy)
                     * (first_dx * third_dy - first_dy * third_dx))
                  + ((third_dx * third_dx + third_dy * third_dy)
                     * (first_dx * second_dy - first_dy * second_dx)))
    return (Location.BOUNDARY
            if raw == 0
            else (Location.INTERIOR if raw > 0 else Location.EXTERIOR))


def locate_point_in_region(
        border: Contour[Scalar], point: Point[Scalar], /
) -> Location:
    is_point_inside = False
    point_y = point.y
    for edge in border.segments:
        if (locate_point_in_segment(edge.start, edge.end, point)
                is Location.BOUNDARY):
            return Location.BOUNDARY
        start, end = edge.start, edge.end
        if ((start.y > point_y) is not (end.y > point_y)
                and ((end.y > start.y)
                     is (orient(start, end, point)
                         is Orientation.COUNTERCLOCKWISE))):
            is_point_inside = not is_point_inside
    return Location.INTERIOR if is_point_inside else Location.EXTERIOR


def locate_point_in_segment(start: Point[Scalar],
                            end: Point[Scalar],
                            point: Point[Scalar],
                            /) -> Location:
    return (Location.BOUNDARY
            if (start == point
                or end == point
                or ((start.x <= point.x <= end.x
                     if start.x <= end.x
                     else end.x < point.x < start.x)
                    and (start.y <= point.y <= end.y
                         if start.y <= end.y
                         else end.y < point.y < start.y)
                    and orient(start, end, point) is Orientation.COLLINEAR))
            else Location.EXTERIOR)


def merge_boxes(boxes: _t.Iterable[Box[Scalar]], /) -> Box[Scalar]:
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


def orient(vertex: Point[Scalar],
           first_ray_point: Point[Scalar],
           second_ray_point: Point[Scalar],
           /) -> Orientation:
    raw = to_sign(cross_multiply(vertex, first_ray_point, vertex,
                                 second_ray_point))
    return (Orientation.COLLINEAR
            if raw == 0
            else (Orientation.COUNTERCLOCKWISE
                  if raw > 0
                  else Orientation.CLOCKWISE))


def point_vertex_line_divides_angle(point: Point[Scalar],
                                    vertex: Point[Scalar],
                                    first_ray_point: Point[Scalar],
                                    second_ray_point: Point[Scalar],
                                    /) -> bool:
    return (orient(vertex, first_ray_point, point)
            is orient(vertex, point, second_ray_point))


def shrink_collinear_vertices(
        vertices: _t.Sequence[Point[Scalar]], /
) -> _t.List[Point[Scalar]]:
    assert len(vertices) >= MIN_CONTOUR_VERTICES_COUNT
    result = [vertices[0]]
    for index in range(1, len(vertices) - 1):
        if (orient(result[-1], vertices[index], vertices[index + 1])
                is not Orientation.COLLINEAR):
            result.append(vertices[index])
    if (orient(result[-1], vertices[-1], result[0])
            is not Orientation.COLLINEAR):
        result.append(vertices[-1])
    return result


def to_boxes_have_common_area(
        boxes: _t.Sequence[Box[Scalar]], target_box: Box[Scalar], /
) -> _t.List[bool]:
    return [do_boxes_have_common_area(box, target_box) for box in boxes]


def to_boxes_have_common_continuum(
        boxes: _t.Sequence[Box[Scalar]], target_box: Box[Scalar], /
) -> _t.List[bool]:
    return [do_boxes_have_common_continuum(box, target_box) for box in boxes]


def to_boxes_ids_with_common_area(
        boxes: _t.Iterable[Box[Scalar]], target_box: Box[Scalar], /
) -> _t.List[int]:
    return [box_id
            for box_id, box in enumerate(boxes)
            if do_boxes_have_common_area(box, target_box)]


def to_boxes_ids_with_continuous_common_points(
        boxes: _t.Iterable[Box[Scalar]], target_box: Box[Scalar], /
) -> _t.List[int]:
    return [box_id
            for box_id, box in enumerate(boxes)
            if do_boxes_have_common_continuum(box, target_box)]


def to_arg_min(values: _t.Sequence[_OrderedT], /) -> int:
    return min(range(len(values)),
               key=values.__getitem__)


def to_contour_orientation(
        vertices: _t.Sequence[Point[Scalar]], min_vertex_index: int, /
) -> Orientation:
    return orient(vertices[min_vertex_index - 1], vertices[min_vertex_index],
                  vertices[(min_vertex_index + 1) % len(vertices)])


def to_sign(value: _t.Any, /) -> int:
    return 1 if value > 0 else (-1 if value else 0)


def to_sorted_pair(first: _OrderedT,
                   second: _OrderedT, /) -> _t.Tuple[_OrderedT, _OrderedT]:
    return (first, second) if first < second else (second, first)


def permute(values: _t.MutableSequence[_T], seed: int, /) -> None:
    """
    Based on "Ranking and unranking permutations in linear time"
    by W. Myrvold, F. Ruskey

    Time complexity: O(values.len())
    Memory complexity: O(1)

    More at: http://webhome.cs.uvic.ca/~ruskey/Publications/RankPerm/MyrvoldRuskey.pdf
    """
    for step in range(len(values), 0, -1):
        values[seed % step], values[step - 1] = (values[step - 1],
                                                 values[seed % step])
        seed //= step


def validate_seed(
        seed: _t.Any, _max_usize_value: int = (_sys.maxsize << 1) + 1, /
) -> None:
    error_type: _t.Type[Exception]
    if not isinstance(seed, int):
        error_type = TypeError
    elif seed < 0:
        error_type = ValueError
    elif seed > _max_usize_value:
        error_type = OverflowError
    else:
        return
    raise error_type('Seed should be an integer '
                     f'from range({0}, {_max_usize_value}), but got "{seed}".')
