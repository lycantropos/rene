from rithm import Fraction

from rene._rene import Orientation
from .point import Point


def cross_multiply(first_start: Point,
                   first_end: Point,
                   second_start: Point,
                   second_end: Point) -> Fraction:
    return ((first_end.x - first_start.x) * (second_end.y - second_start.y)
            - (first_end.y - first_start.y) * (second_end.x - second_start.x))


def to_sign(value: Fraction) -> int:
    return 1 if value > 0 else (-1 if value else 0)


def orient(vertex: Point,
           first_ray_point: Point,
           second_ray_point: Point) -> Orientation:
    return Orientation(to_sign(cross_multiply(vertex, first_ray_point, vertex,
                                              second_ray_point)))
