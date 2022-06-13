from typing import (Any,
                    Sequence)

from rene._rene import (MIN_CONTOUR_VERTICES_COUNT,
                        Orientation,
                        Relation)
from .bentley_ottmann.base import sweep
from .point import Point
from .segment import Segment
from .utils import orient


class Contour:
    @property
    def orientation(self):
        vertices = self.vertices
        min_vertex_index = min(range(len(vertices)),
                               key=vertices.__getitem__)
        return _to_contour_orientation(vertices, min_vertex_index)

    @property
    def segments(self):
        result = [Segment(self._vertices[index], self._vertices[index + 1])
                  for index in range(len(self.vertices) - 1)]
        result.append(Segment(self._vertices[-1], self._vertices[0]))
        return result

    @property
    def vertices(self):
        return self._vertices[:]

    def is_valid(self):
        segments = self.segments
        if len(segments) < MIN_CONTOUR_VERTICES_COUNT:
            return False
        intersections = iter(sweep(segments))
        intersection = next(intersections)
        if intersection.relation is not Relation.TOUCH:
            return False
        else:
            segment_id = intersection.first_segment_id
            has_second_tangent = False
            for intersection in intersections:
                if intersection.relation is not Relation.TOUCH:
                    return False
                elif intersection.first_segment_id == segment_id:
                    if has_second_tangent:
                        return False
                    has_second_tangent = True
                else:
                    assert intersection.second_segment_id != segment_id
                    if not has_second_tangent:
                        return False
                    segment_id = intersection.first_segment_id
                    has_second_tangent = False
            return True

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
    return orient(vertices[min_vertex_index - 1], vertices[min_vertex_index],
                  vertices[(min_vertex_index + 1) % len(vertices)])


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
