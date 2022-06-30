from typing import (Tuple,
                    TypeVar)

from rithm import Fraction
from typing_extensions import Protocol

from rene._rene import Orientation
from rene.hints import Point

_Self = TypeVar('_Self',
                contravariant=True)


class Ordered(Protocol[_Self]):
    def __lt__(self: _Self, other: _Self) -> bool:
        ...


_Ordered = TypeVar('_Ordered',
                   bound=Ordered)


def cross_multiply(first_start: Point,
                   first_end: Point,
                   second_start: Point,
                   second_end: Point) -> Fraction:
    return ((first_end.x - first_start.x) * (second_end.y - second_start.y)
            - (first_end.y - first_start.y) * (second_end.x - second_start.x))


def orient(vertex: Point,
           first_ray_point: Point,
           second_ray_point: Point) -> Orientation:
    return Orientation(to_sign(cross_multiply(vertex, first_ray_point, vertex,
                                              second_ray_point)))


def to_sign(value: Fraction) -> int:
    return 1 if value > 0 else (-1 if value else 0)


def to_sorted_pair(start: _Ordered, end: _Ordered
                   ) -> Tuple[_Ordered, _Ordered]:
    return (start, end) if start < end else (end, start)
