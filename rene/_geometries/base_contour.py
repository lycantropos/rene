from __future__ import annotations

import typing as t
from abc import (ABC,
                 abstractmethod)

import typing_extensions as te

from rene import (MIN_CONTOUR_VERTICES_COUNT,
                  Location,
                  Orientation,
                  Relation,
                  hints)
from rene._bentley_ottmann.base import (Intersection,
                                        sweep)
from rene._clipping import (
    intersect_multisegmental_with_multipolygon,
    intersect_multisegmental_with_multisegmental,
    intersect_multisegmental_with_polygon,
    intersect_multisegmental_with_segment,
    subtract_multipolygon_from_multisegmental,
    subtract_multisegmental_from_multisegmental,
    subtract_polygon_from_multisegmental,
    subtract_segment_from_multisegmental,
    symmetric_subtract_multisegmental_from_multisegmental,
    symmetric_subtract_segment_from_multisegmental,
    unite_multisegmental_with_multisegmental,
    unite_multisegmental_with_segment
)
from .base_compound import BaseCompound
from rene._relating import contour
from rene._utils import (are_contour_vertices_non_degenerate,
                         to_arg_min,
                         to_contour_orientation)


class BaseContour(ABC, BaseCompound[hints.Scalar]):
    @property
    def bounding_box(self) -> hints.Box[hints.Scalar]:
        vertices = iter(self.vertices)
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
    @abstractmethod
    def segments(self) -> t.Sequence[hints.Segment[hints.Scalar]]:
        ...

    @property
    @abstractmethod
    def vertices(self) -> t.Sequence[hints.Point[hints.Scalar]]:
        ...

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

    def locate(self, point: hints.Point[hints.Scalar], /) -> Location:
        return (Location.EXTERIOR
                if all(segment.locate(point) is Location.EXTERIOR
                       for segment in self.segments)
                else Location.BOUNDARY)

    def relate_to(self, other: hints.Compound[hints.Scalar], /) -> Relation:
        if isinstance(other, self._context.contour_cls):
            return contour.relate_to_contour(self, other)
        elif isinstance(other, self._context.multisegment_cls):
            return contour.relate_to_multisegment(self, other)
        elif isinstance(other, self._context.segment_cls):
            return contour.relate_to_segment(self, other)
        elif isinstance(other, self._context.polygon_cls):
            return contour.relate_to_polygon(self, other)
        elif isinstance(other, self._context.multipolygon_cls):
            return contour.relate_to_multipolygon(self, other)
        elif isinstance(other, self._context.empty_cls):
            return Relation.DISJOINT
        else:
            raise TypeError(f'Unsupported type: {type(other)!r}.')

    @t.overload
    def __and__(
            self, other: hints.Empty[hints.Scalar], /
    ) -> hints.Empty[hints.Scalar]:
        ...

    @t.overload
    def __and__(
            self,
            other: t.Union[
                hints.Contour[hints.Scalar], hints.Multipolygon[hints.Scalar],
                hints.Multisegment[hints.Scalar], hints.Polygon[hints.Scalar],
                hints.Segment[hints.Scalar]
            ],
            /
    ) -> t.Union[
        hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
        hints.Segment[hints.Scalar]
    ]:
        ...

    @t.overload
    def __and__(self, other: t.Any, /) -> t.Any:
        ...

    def __and__(self, other: t.Any, /) -> t.Any:
        return (
            intersect_multisegmental_with_multisegmental(
                    self, other, self._context.empty_cls,
                    self._context.multisegment_cls, self._context.segment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                intersect_multisegmental_with_segment(
                        self, other, self._context.empty_cls,
                        self._context.multisegment_cls,
                        self._context.segment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else (
                    intersect_multisegmental_with_polygon(
                            self, other, self._context.empty_cls,
                            self._context.multisegment_cls,
                            self._context.segment_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else (
                        intersect_multisegmental_with_multipolygon(
                                self, other, self._context.empty_cls,
                                self._context.multisegment_cls,
                                self._context.segment_cls
                        )
                        if isinstance(other, self._context.multipolygon_cls)
                        else (other
                              if isinstance(other, self._context.empty_cls)
                              else NotImplemented)
                    )
                )
            )
        )

    def __contains__(self, point: hints.Point[hints.Scalar], /) -> bool:
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
            if isinstance(other, self._context.contour_cls)
            else NotImplemented
        )

    def __hash__(self) -> int:
        vertices = self.vertices
        min_vertex_index = to_arg_min(vertices)
        vertices = ((*vertices[min_vertex_index:min_vertex_index + 1],
                     *vertices[:min_vertex_index][::-1],
                     *vertices[:min_vertex_index:-1])
                    if (to_contour_orientation(vertices, min_vertex_index)
                        is Orientation.CLOCKWISE)
                    else (*vertices[min_vertex_index:],
                          *vertices[:min_vertex_index]))
        return hash(vertices)

    @t.overload
    def __or__(self, other: hints.Empty[hints.Scalar], /) -> te.Self:
        ...

    @t.overload
    def __or__(
            self,
            other: t.Union[
                hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar],
                hints.Segment[hints.Scalar]
            ],
            /
    ) -> t.Union[
        hints.Multisegment[hints.Scalar], hints.Segment[hints.Scalar]
    ]:
        ...

    @t.overload
    def __or__(self, other: t.Any, /) -> t.Any:
        ...

    def __or__(self, other: t.Any, /) -> t.Any:
        return (
            unite_multisegmental_with_multisegmental(
                    self, other, self._context.multisegment_cls,
                    self._context.segment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                unite_multisegmental_with_segment(
                        self, other, self._context.multisegment_cls,
                        self._context.segment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else (self
                      if isinstance(other, self._context.empty_cls)
                      else NotImplemented)
            )
        )

    def __repr__(self) -> str:
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(repr, self.vertices))))

    def __str__(self) -> str:
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.vertices))))

    @t.overload
    def __sub__(self, other: hints.Empty[hints.Scalar], /) -> te.Self:
        ...

    @t.overload
    def __sub__(
            self,
            other: t.Union[
                hints.Contour[hints.Scalar], hints.Multipolygon[hints.Scalar],
                hints.Multisegment[hints.Scalar], hints.Polygon[hints.Scalar],
                hints.Segment[hints.Scalar]
            ],
            /
    ) -> t.Union[
        hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
        hints.Segment[hints.Scalar]
    ]:
        ...

    @t.overload
    def __sub__(self, other: t.Any, /) -> t.Any:
        ...

    def __sub__(self, other: t.Any, /) -> t.Any:
        return (
            subtract_multisegmental_from_multisegmental(
                    self, other, self._context.empty_cls,
                    self._context.multisegment_cls, self._context.segment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                subtract_segment_from_multisegmental(
                        self, other, self._context.empty_cls,
                        self._context.multisegment_cls,
                        self._context.segment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else (
                    subtract_multipolygon_from_multisegmental(
                            self, other, self._context.empty_cls,
                            self._context.multisegment_cls,
                            self._context.segment_cls
                    )
                    if isinstance(other, self._context.multipolygon_cls)
                    else (
                        subtract_polygon_from_multisegmental(
                                self, other, self._context.empty_cls,
                                self._context.multisegment_cls,
                                self._context.segment_cls
                        )
                        if isinstance(other, self._context.polygon_cls)
                        else (self
                              if isinstance(other, self._context.empty_cls)
                              else NotImplemented)
                    )
                )
            )
        )

    @t.overload
    def __xor__(self, other: hints.Empty[hints.Scalar], /) -> te.Self:
        ...

    @t.overload
    def __xor__(
            self,
            other: t.Union[
                hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar],
                hints.Segment[hints.Scalar]
            ],
            /
    ) -> t.Union[
        hints.Empty[hints.Scalar], hints.Multisegment[hints.Scalar],
        hints.Segment[hints.Scalar]
    ]:
        ...

    @t.overload
    def __xor__(self, other: t.Any, /) -> t.Any:
        ...

    def __xor__(self, other: t.Any, /) -> t.Any:
        return (
            symmetric_subtract_multisegmental_from_multisegmental(
                    self, other, self._context.empty_cls,
                    self._context.multisegment_cls, self._context.segment_cls
            )
            if isinstance(other, (self._context.contour_cls,
                                  self._context.multisegment_cls))
            else (
                symmetric_subtract_segment_from_multisegmental(
                        self, other, self._context.empty_cls,
                        self._context.multisegment_cls,
                        self._context.segment_cls
                )
                if isinstance(other, self._context.segment_cls)
                else (self
                      if isinstance(other, self._context.empty_cls)
                      else NotImplemented)
            )
        )


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


def _neighbour_segments_vertices_touch(
        intersection: Intersection[hints.Scalar],
        segments: t.Sequence[hints.Segment[hints.Scalar]],
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
