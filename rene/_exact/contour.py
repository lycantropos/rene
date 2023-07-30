from __future__ import annotations

import typing as t

import typing_extensions as te
from reprit.base import generate_repr
from rithm.fraction import Fraction

from rene import (MIN_CONTOUR_VERTICES_COUNT,
                  Location,
                  Orientation,
                  Relation,
                  hints)
from rene._bentley_ottmann.base import (Intersection,
                                        sweep)
from rene._clipping import (intersect_segments_with_segment,
                            intersect_segments_with_segments,
                            symmetric_subtract_segment_from_segments,
                            symmetric_subtract_segments_from_segments)
from rene._context import Context
from rene._utils import (are_contour_vertices_non_degenerate,
                         collect_maybe_empty_segments,
                         to_arg_min,
                         to_contour_orientation,
                         to_contour_segments)


@te.final
class Contour:
    @property
    def bounding_box(self) -> hints.Box[Fraction]:
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
        vertices = self._vertices
        min_vertex_index = to_arg_min(vertices)
        return to_contour_orientation(vertices, min_vertex_index)

    @property
    def segments(self) -> t.Sequence[hints.Segment[Fraction]]:
        return to_contour_segments(self._vertices, self._context.segment_cls)

    @property
    def segments_count(self) -> int:
        return len(self._vertices)

    @property
    def vertices(self) -> t.Sequence[hints.Point[Fraction]]:
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

    def locate(self, point: hints.Point[Fraction], /) -> Location:
        return (Location.EXTERIOR
                if all(segment.locate(point) is Location.EXTERIOR
                       for segment in self.segments)
                else Location.BOUNDARY)

    _context: t.ClassVar[Context[Fraction]]
    _vertices: t.List[hints.Point[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_vertices',

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(
            cls, vertices: t.Sequence[hints.Point[Fraction]], /
    ) -> Contour:
        if len(vertices) < MIN_CONTOUR_VERTICES_COUNT:
            raise ValueError('Contour should have at least '
                             f'{MIN_CONTOUR_VERTICES_COUNT} vertices, '
                             f'but found {len(vertices)}.')
        self = super().__new__(cls)
        self._vertices = list(vertices)
        return self

    @t.overload
    def __and__(
            self, other: hints.Empty[Fraction], /
    ) -> hints.Empty[Fraction]:
        ...

    @t.overload
    def __and__(
            self,
            other: t.Union[
                hints.Contour[hints.Scalar], hints.Multisegment[Fraction],
                hints.Segment[Fraction]
            ],
            /
    ) -> t.Union[
        hints.Empty[Fraction], hints.Multisegment[Fraction],
        hints.Segment[Fraction]
    ]:
        ...

    @t.overload
    def __and__(self, other: t.Any, /) -> t.Any:
        ...

    def __and__(self, other: t.Any, /) -> t.Any:
        return (
            collect_maybe_empty_segments(
                    intersect_segments_with_segments(
                            self.segments, other.segments,
                            self._context.segment_cls
                    ),
                    self._context.empty_cls, self._context.multisegment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                collect_maybe_empty_segments(
                        intersect_segments_with_segment(
                                self.segments, other, self._context.segment_cls
                        ),
                        self._context.empty_cls, self._context.multisegment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else NotImplemented)
        )

    def __contains__(self, point: hints.Point[Fraction], /) -> bool:
        return self.locate(point) is not Location.EXTERIOR

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool:
        pass

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        pass

    def __eq__(self, other: t.Any, /) -> t.Any:
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

    def __xor__(self, other: t.Any, /) -> t.Any:
        return (
            collect_maybe_empty_segments(
                    symmetric_subtract_segments_from_segments(
                            self.segments, other.segments,
                            self._context.segment_cls
                    ),
                    self._context.empty_cls, self._context.multisegment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                symmetric_subtract_segment_from_segments(
                        self.segments, other, self._context.segment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else NotImplemented
            )
        )


def _neighbour_segments_vertices_touch(
        intersection: Intersection[Fraction],
        segments: t.Sequence[hints.Segment[Fraction]],
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
        left: t.Sequence[t.Any], right: t.Sequence[t.Any], /
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
