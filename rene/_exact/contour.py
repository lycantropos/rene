from __future__ import annotations

import typing as _t

import typing_extensions as _te
from reprit.base import generate_repr
from rithm.fraction import Fraction

from rene import (MIN_CONTOUR_VERTICES_COUNT,
                  Location,
                  Orientation,
                  Relation,
                  hints as _hints)
from rene._bentley_ottmann.base import (Intersection,
                                        sweep)
from rene._context import Context
from rene._utils import (are_contour_vertices_non_degenerate,
                         to_arg_min,
                         to_contour_orientation)


class Contour:
    @property
    def bounding_box(self) -> _hints.Box[Fraction]:
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
        return self._context.box_cls(min_x, max_x, min_y, max_y)

    @property
    def orientation(self) -> Orientation:
        vertices = self.vertices
        min_vertex_index = to_arg_min(vertices)
        return to_contour_orientation(vertices, min_vertex_index)

    @property
    def segments(self) -> _t.Sequence[_hints.Segment[Fraction]]:
        segment_cls = self._context.segment_cls
        result = [segment_cls(self._vertices[index], self._vertices[index + 1])
                  for index in range(len(self.vertices) - 1)]
        result.append(segment_cls(self._vertices[-1],
                                  self._vertices[0]))
        return result

    @property
    def segments_count(self) -> int:
        return len(self._vertices)

    @property
    def vertices(self) -> _t.Sequence[_hints.Point[Fraction]]:
        return self._vertices[:]

    @property
    def vertices_count(self) -> int:
        return len(self._vertices)

    def is_valid(self) -> bool:
        if not are_contour_vertices_non_degenerate(self.vertices):
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

    def locate(self, point: _hints.Point[Fraction], /) -> Location:
        return (Location.EXTERIOR
                if all(segment.locate(point) is Location.EXTERIOR
                       for segment in self.segments)
                else Location.BOUNDARY)

    _context: _t.ClassVar[Context[Fraction]]
    _vertices: _t.List[_hints.Point[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_vertices',

    def __new__(
            cls, vertices: _t.Sequence[_hints.Point[Fraction]], /
    ) -> Contour:
        if len(vertices) < MIN_CONTOUR_VERTICES_COUNT:
            raise ValueError('Contour should have at least '
                             f'{MIN_CONTOUR_VERTICES_COUNT} vertices, '
                             f'but found {len(vertices)}.')
        self = super().__new__(cls)
        self._vertices = list(vertices)
        return self

    def __contains__(self, point: _hints.Point[Fraction], /) -> bool:
        return self.locate(point) is not Location.EXTERIOR

    @_t.overload
    def __eq__(self, other: _te.Self, /) -> bool:
        pass

    @_t.overload
    def __eq__(self, other: _t.Any, /) -> _t.Any:
        pass

    def __eq__(self, other: _t.Any, /) -> _t.Any:
        return (
            _are_non_empty_unique_sequences_rotationally_equivalent(
                    self.vertices, other.vertices
            )
            if isinstance(other, Contour)
            else NotImplemented
        )

    def __hash__(self) -> int:
        vertices = self._vertices
        min_vertex_index = to_arg_min(vertices)
        vertices = (vertices[min_vertex_index:min_vertex_index + 1]
                    + vertices[:min_vertex_index][::-1]
                    + vertices[:min_vertex_index:-1]
                    if (to_contour_orientation(vertices, min_vertex_index)
                        is Orientation.CLOCKWISE)
                    else (vertices[min_vertex_index:]
                          + vertices[:min_vertex_index]))
        return hash(tuple(vertices))

    __repr__ = generate_repr(__new__)

    def __str__(self) -> str:
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.vertices))))


def _neighbour_segments_vertices_touch(
        intersection: Intersection[Fraction],
        segments: _t.Sequence[_hints.Segment[Fraction]],
        /
) -> bool:
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


def _are_non_empty_unique_sequences_rotationally_equivalent(
        left: _t.Sequence[_t.Any], right: _t.Sequence[_t.Any], /
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
