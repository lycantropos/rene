from typing import (Any,
                    Optional,
                    Sequence)

from reprit.base import generate_repr

from rene._bentley_ottmann.base import (Intersection,
                                        sweep)
from rene._rene import (MIN_CONTOUR_VERTICES_COUNT,
                        Orientation,
                        Relation)
from rene._utils import orient
from .box import Box
from .context import Context
from .point import Point
from .segment import Segment


class Contour:
    _context: Optional[Context] = None

    @property
    def bounding_box(self):
        vertices = iter(self._vertices)
        first_vertex = next(vertices)
        min_x = max_x = first_vertex.x
        min_y = max_y = first_vertex.y
        for vertex in vertices:
            if vertex.x > max_x:
                max_x = vertex.x
            elif vertex.x < min_x:
                min_x = vertex.x
            if vertex.y > max_y:
                max_y = vertex.y
            elif vertex.y < min_y:
                min_y = vertex.y
        return Box(min_x, max_x, min_y, max_y)

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
    def segments_count(self):
        return len(self._vertices)

    @property
    def vertices(self):
        return self._vertices[:]

    @property
    def vertices_count(self):
        return len(self._vertices)

    def is_valid(self):
        if not _are_contour_vertices_non_degenerate(self.vertices):
            return False
        segments = self.segments
        if len(segments) < MIN_CONTOUR_VERTICES_COUNT:
            return False
        neighbour_segments_touches_count = 0
        for intersection in sweep(segments):
            if not _neighbour_segments_vertices_touch(intersection, segments):
                return False
            neighbour_segments_touches_count += 1
        return neighbour_segments_touches_count == len(segments)

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
        return (
            _are_non_empty_unique_sequences_rotationally_equivalent(
                    self.vertices, other.vertices
            )
            if isinstance(other, Contour)
            else NotImplemented
        )

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

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __str__(self):
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.vertices))))


def _are_contour_vertices_non_degenerate(vertices: Sequence[Point]) -> bool:
    return (all(orient(vertices[index - 1], vertices[index],
                       vertices[index + 1]) is not Orientation.COLLINEAR
                for index in range(1, len(vertices) - 1))
            and (len(vertices) <= MIN_CONTOUR_VERTICES_COUNT
                 or ((orient(vertices[-2], vertices[-1], vertices[0])
                      is not Orientation.COLLINEAR)
                     and (orient(vertices[-1], vertices[0], vertices[1])
                          is not Orientation.COLLINEAR))))


def _neighbour_segments_vertices_touch(intersection: Intersection,
                                       segments: Sequence[Segment]) -> bool:
    first_segment = segments[intersection.first_segment_id]
    second_segment = segments[intersection.second_segment_id]
    touches_at_vertices = (
            intersection.relation is Relation.TOUCH
            and (intersection.start == first_segment.start
                 or intersection.start == first_segment.end)
            and (intersection.start == second_segment.start
                 or intersection.start == second_segment.end)
    )
    neighbour_segments_intersection = (
            abs(intersection.first_segment_id
                - intersection.second_segment_id) == 1
            or (intersection.first_segment_id == len(segments) - 1
                and intersection.second_segment_id == 0)
            or (intersection.second_segment_id == len(segments) - 1
                and intersection.first_segment_id == 0)
    )
    return touches_at_vertices and neighbour_segments_intersection


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
