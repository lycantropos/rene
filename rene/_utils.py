from itertools import groupby
from typing import (List,
                    Sequence,
                    Tuple,
                    TypeVar)

from rithm import Fraction
from typing_extensions import Protocol

from rene._rene import (MIN_CONTOUR_VERTICES_COUNT,
                        Location,
                        Orientation)
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
