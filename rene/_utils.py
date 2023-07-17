from __future__ import annotations

import sys
import typing as t
from itertools import groupby

import typing_extensions as te

from rene import (MIN_CONTOUR_VERTICES_COUNT,
                  Location,
                  Orientation,
                  hints)


class Ordered(te.Protocol):
    def __lt__(self, other: te.Self, /) -> bool:
        ...


_OrderedT = t.TypeVar('_OrderedT',
                      bound=Ordered)
_T = t.TypeVar('_T')


def are_contour_vertices_non_degenerate(
        vertices: t.Sequence[hints.Point[hints.Scalar]], /
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
        first: hints.Box[hints.Scalar], second: hints.Box[hints.Scalar], /
) -> bool:
    return not first.disjoint_with(second) and not first.touches(second)


def do_boxes_have_no_common_area(
        first: hints.Box[hints.Scalar], second: hints.Box[hints.Scalar], /
) -> bool:
    return first.disjoint_with(second) or first.touches(second)


def do_boxes_have_common_continuum(
        first: hints.Box[hints.Scalar], second: hints.Box[hints.Scalar], /
) -> bool:
    return (not first.disjoint_with(second)
            and
            (not first.touches(second)
             or (first.min_y != second.max_y and second.min_y != first.max_y)
             or (first.min_x != second.max_x and second.min_x != first.max_x)))


def do_boxes_have_no_common_continuum(
        first: hints.Box[hints.Scalar], second: hints.Box[hints.Scalar], /
) -> bool:
    return (first.disjoint_with(second)
            or
            (first.touches(second)
             and (first.min_y == second.max_y or second.min_y == first.max_y)
             and (first.min_x == second.max_x or second.min_x == first.max_x)))


def ceil_log2(number: int, /) -> int:
    return number.bit_length() - (not (number & (number - 1)))


def collect_maybe_empty_polygons(
        polygons: t.Sequence[hints.Polygon[hints.Scalar]],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multipolygon[hints.Scalar],
    hints.Polygon[hints.Scalar]
]:
    return (collect_non_empty_polygons(polygons, multipolygon_cls)
            if polygons
            else empty_cls())


def collect_maybe_empty_segments(
        segments: t.Sequence[hints.Segment[hints.Scalar]],
        empty_cls: t.Type[hints.Empty[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        /
) -> t.Union[
    hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
    hints.Segment[hints.Scalar]
]:
    return (collect_non_empty_segments(segments, multisegment_cls)
            if segments
            else empty_cls())


def collect_non_empty_polygons(
        polygons: t.Sequence[hints.Polygon[hints.Scalar]],
        multipolygon_cls: t.Type[hints.Multipolygon[hints.Scalar]],
        /
) -> t.Union[hints.Multipolygon[hints.Scalar], hints.Polygon[hints.Scalar]]:
    assert len(polygons) >= 1
    return polygons[0] if len(polygons) == 1 else multipolygon_cls(polygons)


def collect_non_empty_segments(
        segments: t.Sequence[hints.Segment[hints.Scalar]],
        multisegment_cls: t.Type[hints.Multisegment[hints.Scalar]],
        /
) -> t.Union[hints.Multisegment[hints.Scalar], hints.Segment[hints.Scalar]]:
    assert len(segments) >= 1
    return segments[0] if len(segments) == 1 else multisegment_cls(segments)


def cross_multiply(first_start: hints.Point[hints.Scalar],
                   first_end: hints.Point[hints.Scalar],
                   second_start: hints.Point[hints.Scalar],
                   second_end: hints.Point[hints.Scalar],
                   /) -> hints.Scalar:
    return ((first_end.x - first_start.x) * (second_end.y - second_start.y)
            - (first_end.y - first_start.y) * (second_end.x - second_start.x))


def deduplicate(values: t.List[_T], /) -> t.List[_T]:
    return [value for value, _ in groupby(values)]


def flags_to_false_indices(flags: t.Sequence[bool], /) -> t.List[int]:
    return [index for index, flag in enumerate(flags) if not flag]


def flags_to_true_indices(flags: t.Sequence[bool], /) -> t.List[int]:
    return [index for index, flag in enumerate(flags) if flag]


def to_segments_intersection_point(first_start: hints.Point[hints.Scalar],
                                   first_end: hints.Point[hints.Scalar],
                                   second_start: hints.Point[hints.Scalar],
                                   second_end: hints.Point[hints.Scalar],
                                   /) -> hints.Point[hints.Scalar]:
    scale = to_segments_intersection_scale(first_start, first_end,
                                           second_start, second_end)
    return type(first_start)(
            first_start.x + (first_end.x - first_start.x) * scale,
            first_start.y + (first_end.y - first_start.y) * scale
    )


def to_segments_intersection_scale(first_start: hints.Point[hints.Scalar],
                                   first_end: hints.Point[hints.Scalar],
                                   second_start: hints.Point[hints.Scalar],
                                   second_end: hints.Point[hints.Scalar],
                                   /) -> hints.Scalar:
    return (cross_multiply(first_start, second_start, second_start, second_end)
            / cross_multiply(first_start, first_end, second_start, second_end))


def is_even(value: int, /) -> bool:
    return value & 1 == 0


def is_odd(value: int, /) -> bool:
    return value & 1 == 1


def locate_point_in_point_point_point_circle(point: hints.Point[hints.Scalar],
                                             first: hints.Point[hints.Scalar],
                                             second: hints.Point[hints.Scalar],
                                             third: hints.Point[hints.Scalar],
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
        border: hints.Contour[hints.Scalar], point: hints.Point[hints.Scalar],
        /
) -> Location:
    is_point_inside = False
    point_y = point.y
    for edge in border.segments:
        end, start = edge.end, edge.start
        if locate_point_in_segment(start, end, point) is Location.BOUNDARY:
            return Location.BOUNDARY
        if ((start.y > point_y) is not (end.y > point_y)
                and ((end.y > start.y)
                     is (orient(start, end, point)
                         is Orientation.COUNTERCLOCKWISE))):
            is_point_inside = not is_point_inside
    return Location.INTERIOR if is_point_inside else Location.EXTERIOR


def locate_point_in_segment(start: hints.Point[hints.Scalar],
                            end: hints.Point[hints.Scalar],
                            point: hints.Point[hints.Scalar],
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


def merge_boxes(boxes: t.Iterable[hints.Box[hints.Scalar]],
                /) -> hints.Box[hints.Scalar]:
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


def orient(vertex: hints.Point[hints.Scalar],
           first_ray_point: hints.Point[hints.Scalar],
           second_ray_point: hints.Point[hints.Scalar],
           /) -> Orientation:
    raw = to_sign(cross_multiply(vertex, first_ray_point, vertex,
                                 second_ray_point))
    return (Orientation.COLLINEAR
            if raw == 0
            else (Orientation.COUNTERCLOCKWISE
                  if raw > 0
                  else Orientation.CLOCKWISE))


def permute(values: t.MutableSequence[_T], seed: int, /) -> None:
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


def point_vertex_line_divides_angle(
        point: hints.Point[hints.Scalar],
        vertex: hints.Point[hints.Scalar],
        first_ray_point: hints.Point[hints.Scalar],
        second_ray_point: hints.Point[hints.Scalar],
        /
) -> bool:
    return (orient(vertex, first_ray_point, point)
            is orient(vertex, point, second_ray_point))


def polygon_to_correctly_oriented_segments(
        polygon: hints.Polygon[hints.Scalar],
        segment_cls: t.Type[hints.Segment[hints.Scalar]]
) -> t.Iterable[hints.Segment[hints.Scalar]]:
    yield from to_oriented_segments(polygon.border.vertices,
                                    Orientation.COUNTERCLOCKWISE, segment_cls)
    for hole in polygon.holes:
        yield from to_oriented_segments(hole.vertices, Orientation.CLOCKWISE,
                                        segment_cls)


def rotate_sequence(value: t.Sequence[_T]) -> t.List[_T]:
    return [value[0], *value[:0:-1]]


def shrink_collinear_vertices(
        vertices: t.Sequence[hints.Point[hints.Scalar]], /
) -> t.List[hints.Point[hints.Scalar]]:
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


def to_arg_min(values: t.Sequence[_OrderedT], /) -> int:
    return min(range(len(values)),
               key=values.__getitem__)


def to_boxes_have_common_area(
        boxes: t.Sequence[hints.Box[hints.Scalar]],
        target_box: hints.Box[hints.Scalar],
        /
) -> t.List[bool]:
    return [do_boxes_have_common_area(box, target_box) for box in boxes]


def to_boxes_have_common_continuum(
        boxes: t.Sequence[hints.Box[hints.Scalar]],
        target_box: hints.Box[hints.Scalar],
        /
) -> t.List[bool]:
    return [do_boxes_have_common_continuum(box, target_box) for box in boxes]


def to_boxes_ids_with_common_area(
        boxes: t.Iterable[hints.Box[hints.Scalar]],
        target_box: hints.Box[hints.Scalar],
        /
) -> t.List[int]:
    return [box_id
            for box_id, box in enumerate(boxes)
            if do_boxes_have_common_area(box, target_box)]


def to_boxes_ids_with_common_continuum(
        boxes: t.Iterable[hints.Box[hints.Scalar]],
        target_box: hints.Box[hints.Scalar],
        /
) -> t.List[int]:
    return [box_id
            for box_id, box in enumerate(boxes)
            if do_boxes_have_common_continuum(box, target_box)]


def to_contour_orientation(vertices: t.Sequence[hints.Point[hints.Scalar]],
                           min_vertex_index: int,
                           /) -> Orientation:
    return orient(vertices[min_vertex_index - 1], vertices[min_vertex_index],
                  vertices[(min_vertex_index + 1) % len(vertices)])


def to_contour_segments(vertices: t.Sequence[hints.Point[hints.Scalar]],
                        segment_cls: t.Type[hints.Segment[hints.Scalar]],
                        /) -> t.List[hints.Segment[hints.Scalar]]:
    result = [segment_cls(vertices[index], vertices[index + 1])
              for index in range(len(vertices) - 1)]
    result.append(segment_cls(vertices[-1], vertices[0]))
    return result


def to_oriented_segments(vertices: t.Sequence[hints.Point[hints.Scalar]],
                         target_orientation: Orientation,
                         segment_cls: t.Type[hints.Segment[hints.Scalar]],
                         /) -> t.List[hints.Segment[hints.Scalar]]:
    return to_contour_segments(
            vertices
            if (to_contour_orientation(vertices, to_arg_min(vertices))
                is target_orientation)
            else rotate_sequence(vertices),
            segment_cls
    )


def to_sign(value: t.Any, /) -> int:
    return 1 if value > 0 else (-1 if value else 0)


def to_sorted_pair(
        first: _OrderedT, second: _OrderedT, /
) -> t.Tuple[_OrderedT, _OrderedT]:
    return (first, second) if first < second else (second, first)


_T1 = t.TypeVar('_T1')
_T2 = t.TypeVar('_T2')


@t.overload
def unwrap_or_else(
        value: None, function: t.Callable[[], _T2], /
) -> _T2:
    ...


@t.overload
def unwrap_or_else(
        value: _T1, function: t.Callable[[], _T2], /
) -> _T1:
    ...


def unwrap_or_else(
        value: t.Optional[_T1], function: t.Callable[[], _T2], /
) -> t.Union[_T1, _T2]:
    return function() if value is None else value


def validate_seed(
        seed: t.Any, _max_usize_value: int = (sys.maxsize << 1) + 1, /
) -> None:
    error_type: t.Type[Exception]
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
