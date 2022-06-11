from typing import (Any,
                    Sequence)

from rithm import Fraction

from rene._rene import (MIN_CONTOUR_VERTICES_COUNT,
                        Orientation)
from .point import Point


class Contour:
    @property
    def orientation(self):
        vertices = self.vertices
        min_vertex_index = min(range(len(vertices)),
                               key=vertices.__getitem__)
        return _to_contour_orientation(vertices, min_vertex_index)

    @property
    def vertices(self):
        return self._vertices[:]

    __module__ = 'rene.exact'
    __slots__ = '_vertices',

    def __new__(cls, vertices):
        if len(vertices) < MIN_CONTOUR_VERTICES_COUNT:
            raise ValueError('Contour should have at least '
                             f'{MIN_CONTOUR_VERTICES_COUNT} vertices, '
                             f'but found {len(vertices)}.')
        self = super().__new__(cls)
        self._vertices = list(vertices)
        return self

    def __eq__(self, other):
        return (_are_non_empty_unique_sequences_rotationally_equivalent(
                self.vertices, other.vertices)
                if isinstance(other, Contour)
                else NotImplemented)

    def __hash__(self):
        vertices = self.vertices
        min_vertex_index = min(range(len(vertices)),
                               key=vertices.__getitem__)
        vertices = (vertices[min_vertex_index:min_vertex_index + 1]
                    + vertices[:min_vertex_index][::-1]
                    + vertices[:min_vertex_index:-1]
                    if (_to_contour_orientation(vertices, min_vertex_index)
                        == Orientation.CLOCKWISE)
                    else (vertices[min_vertex_index:]
                          + vertices[:min_vertex_index]))
        return hash(tuple(vertices))

    def __repr__(self):
        return (f'{type(self).__module__}.{type(self).__qualname__}'
                f'({self.vertices!r})')

    def __str__(self):
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.vertices))))


def _to_contour_orientation(vertices: Sequence[Point],
                            min_vertex_index: int) -> Orientation:
    return Orientation(_to_sign(_cross_multiply(
            vertices[min_vertex_index - 1], vertices[min_vertex_index],
            vertices[min_vertex_index - 1],
            vertices[(min_vertex_index + 1) % len(vertices)],
    )))


def _are_non_empty_unique_sequences_rotationally_equivalent(
        left: Sequence[Any], right: Sequence[Any]
) -> bool:
    assert left and right
    if len(left) != len(right):
        return False
    first_left_element = left[0]
    try:
        index = right.index(first_left_element)
    except ValueError:
        return False
    else:
        return ((left[1:len(left) - index] == right[index + 1:]
                 and left[len(left) - index:] == right[:index])
                or (left[1:index + 1] == right[:index][::-1]
                    and left[index + 1:] == right[len(right) - 1:index:-1]))


def _cross_multiply(first_start: Point,
                    first_end: Point,
                    second_start: Point,
                    second_end: Point) -> Fraction:
    return ((first_end.x - first_start.x) * (second_end.y - second_start.y)
            - ((first_end.y - first_start.y)
               * (second_end.x - second_start.x)))


def _to_sign(value: Fraction) -> int:
    return (1 if value > 0 else -1) if value else 0
