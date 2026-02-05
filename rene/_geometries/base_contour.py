from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Any, TYPE_CHECKING, overload

from typing_extensions import Self

from rene import hints
from rene._bentley_ottmann.base import Intersection, sweep
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
    unite_multisegmental_with_segment,
)
from rene._relating import contour
from rene._utils import (
    are_contour_vertices_non_degenerate,
    to_arg_min,
    to_contour_orientation,
)
from rene.constants import MIN_CONTOUR_VERTICES_COUNT
from rene.enums import Location, Orientation, Relation

from .base_compound import BaseCompound
from .utils import (
    is_contour,
    is_empty,
    is_multipolygon,
    is_multisegment,
    is_multisegmental,
    is_polygon,
    is_segment,
)

if TYPE_CHECKING:
    from collections.abc import Sequence


class BaseContour(ABC, BaseCompound[hints.ScalarT]):
    @property
    def bounding_box(self, /) -> hints.Box[hints.ScalarT]:
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
    def orientation(self, /) -> Orientation:
        vertices = self.vertices
        min_vertex_index = to_arg_min(vertices)
        return to_contour_orientation(
            vertices, min_vertex_index, self._context.orient
        )

    @property
    @abstractmethod
    def segments(self, /) -> Sequence[hints.Segment[hints.ScalarT]]: ...

    @property
    @abstractmethod
    def vertices(self, /) -> Sequence[hints.Point[hints.ScalarT]]: ...

    def is_valid(self, /) -> bool:
        if not are_contour_vertices_non_degenerate(
            self.vertices, self._context.orient
        ):
            return False
        segments = self.segments
        if len(segments) < MIN_CONTOUR_VERTICES_COUNT:
            return False
        neighbour_segments_touches_count = 0
        for intersection in sweep(
            segments, self._context.orient, self._context.intersect_segments
        ):
            if not _neighbour_segments_vertices_touch(intersection, segments):
                return False
            neighbour_segments_touches_count += 1
        return neighbour_segments_touches_count == len(segments)

    def locate(self, point: hints.Point[hints.ScalarT], /) -> Location:
        return (
            Location.EXTERIOR
            if all(
                segment.locate(point) is Location.EXTERIOR
                for segment in self.segments
            )
            else Location.BOUNDARY
        )

    def relate_to(self, other: hints.Compound[hints.ScalarT], /) -> Relation:
        context = self._context
        if is_contour(other, context=context):
            return contour.relate_to_contour(
                self, other, context.orient, context.intersect_segments
            )
        if is_empty(other, context=context):
            return Relation.DISJOINT
        if is_multipolygon(other, context=context):
            return contour.relate_to_multipolygon(
                self, other, context.orient, context.intersect_segments
            )
        if is_multisegment(other, context=context):
            return contour.relate_to_multisegment(
                self,
                other,
                context.orient,
                context.to_segments_intersection_scale,
                context.intersect_segments,
            )
        if is_polygon(other, context=context):
            return contour.relate_to_polygon(
                self, other, context.orient, context.intersect_segments
            )
        if is_segment(other, context=context):
            return contour.relate_to_segment(self, other, context.orient)
        raise TypeError(f'Unsupported type: {type(other)!r}.')

    @abstractmethod
    def __new__(
        cls, vertices: Sequence[hints.Point[hints.ScalarT]], /
    ) -> Self:
        raise NotImplementedError

    @overload
    def __and__(
        self, other: hints.Empty[hints.ScalarT], /
    ) -> hints.Empty[hints.ScalarT]: ...

    @overload
    def __and__(
        self,
        other: (
            hints.Contour[hints.ScalarT]
            | hints.Multipolygon[hints.ScalarT]
            | hints.Multisegment[hints.ScalarT]
            | hints.Polygon[hints.ScalarT]
            | hints.Segment[hints.ScalarT]
        ),
        /,
    ) -> (
        hints.Empty[hints.ScalarT]
        | hints.Multisegment[hints.ScalarT]
        | hints.Segment[hints.ScalarT]
    ): ...

    @overload
    def __and__(self, other: Any, /) -> Any: ...

    def __and__(self, other: Any, /) -> Any:
        context = self._context
        return (
            intersect_multisegmental_with_multisegmental(
                self,
                other,
                context.empty_cls,
                context.multisegment_cls,
                context.orient,
                context.segment_cls,
                context.intersect_segments,
            )
            if is_multisegmental(other, context=context)
            else (
                intersect_multisegmental_with_segment(
                    self,
                    other,
                    context.empty_cls,
                    context.multisegment_cls,
                    context.orient,
                    context.segment_cls,
                )
                if is_segment(other, context=context)
                else (
                    intersect_multisegmental_with_polygon(
                        self,
                        other,
                        context.empty_cls,
                        context.multisegment_cls,
                        context.orient,
                        context.segment_cls,
                        context.intersect_segments,
                    )
                    if is_polygon(other, context=context)
                    else (
                        intersect_multisegmental_with_multipolygon(
                            self,
                            other,
                            context.empty_cls,
                            context.multisegment_cls,
                            context.orient,
                            context.segment_cls,
                            context.intersect_segments,
                        )
                        if is_multipolygon(other, context=context)
                        else (
                            other
                            if is_empty(other, context=context)
                            else NotImplemented
                        )
                    )
                )
            )
        )

    def __contains__(self, point: hints.Point[hints.ScalarT], /) -> bool:
        return self.locate(point) is not Location.EXTERIOR

    @overload
    def __eq__(self, other: Self, /) -> bool:
        pass

    @overload
    def __eq__(self, other: Any, /) -> Any:
        pass

    def __eq__(self, other: Any, /) -> Any:
        return (
            _are_non_empty_unique_sequences_rotationally_equivalent(
                self.vertices, other.vertices
            )
            if is_contour(other, context=self._context)
            else NotImplemented
        )

    def __hash__(self, /) -> int:
        vertices = self.vertices
        min_vertex_index = to_arg_min(vertices)
        vertices = (
            (
                *vertices[min_vertex_index : min_vertex_index + 1],
                *vertices[:min_vertex_index][::-1],
                *vertices[:min_vertex_index:-1],
            )
            if (
                to_contour_orientation(
                    vertices, min_vertex_index, self._context.orient
                )
                is Orientation.CLOCKWISE
            )
            else (*vertices[min_vertex_index:], *vertices[:min_vertex_index])
        )
        return hash(vertices)

    @overload
    def __or__(self, other: hints.Empty[hints.ScalarT], /) -> Self: ...

    @overload
    def __or__(
        self,
        other: (
            hints.Contour[hints.ScalarT]
            | hints.Multisegment[hints.ScalarT]
            | hints.Segment[hints.ScalarT]
        ),
        /,
    ) -> hints.Multisegment[hints.ScalarT] | hints.Segment[hints.ScalarT]: ...

    @overload
    def __or__(self, other: Any, /) -> Any: ...

    def __or__(self, other: Any, /) -> Any:
        context = self._context
        return (
            unite_multisegmental_with_multisegmental(
                self,
                other,
                context.multisegment_cls,
                context.orient,
                context.segment_cls,
                context.intersect_segments,
            )
            if is_multisegmental(other, context=context)
            else (
                unite_multisegmental_with_segment(
                    self,
                    other,
                    context.multisegment_cls,
                    context.orient,
                    context.segment_cls,
                    context.intersect_segments,
                )
                if is_segment(other, context=context)
                else (
                    self
                    if is_empty(other, context=context)
                    else NotImplemented
                )
            )
        )

    def __repr__(self, /) -> str:
        return f'{type(self).__qualname__}([{{}}])'.format(
            ', '.join(map(repr, self.vertices))
        )

    def __str__(self, /) -> str:
        return f'{type(self).__qualname__}([{{}}])'.format(
            ', '.join(map(str, self.vertices))
        )

    @overload
    def __sub__(self, other: hints.Empty[hints.ScalarT], /) -> Self: ...

    @overload
    def __sub__(
        self,
        other: (
            hints.Contour[hints.ScalarT]
            | hints.Multipolygon[hints.ScalarT]
            | hints.Multisegment[hints.ScalarT]
            | hints.Polygon[hints.ScalarT]
            | hints.Segment[hints.ScalarT]
        ),
        /,
    ) -> (
        hints.Empty[hints.ScalarT]
        | hints.Multisegment[hints.ScalarT]
        | hints.Segment[hints.ScalarT]
    ): ...

    @overload
    def __sub__(self, other: Any, /) -> Any: ...

    def __sub__(self, other: Any, /) -> Any:
        context = self._context
        return (
            subtract_multisegmental_from_multisegmental(
                self,
                other,
                context.empty_cls,
                context.multisegment_cls,
                context.orient,
                context.segment_cls,
                context.intersect_segments,
            )
            if is_multisegmental(other, context=context)
            else (
                subtract_segment_from_multisegmental(
                    self,
                    other,
                    context.empty_cls,
                    context.multisegment_cls,
                    context.orient,
                    context.segment_cls,
                    context.intersect_segments,
                )
                if is_segment(other, context=context)
                else (
                    subtract_multipolygon_from_multisegmental(
                        self,
                        other,
                        context.empty_cls,
                        context.multisegment_cls,
                        context.orient,
                        context.segment_cls,
                        context.intersect_segments,
                    )
                    if is_multipolygon(other, context=context)
                    else (
                        subtract_polygon_from_multisegmental(
                            self,
                            other,
                            context.empty_cls,
                            context.multisegment_cls,
                            context.orient,
                            context.segment_cls,
                            context.intersect_segments,
                        )
                        if is_polygon(other, context=context)
                        else (
                            self
                            if is_empty(other, context=context)
                            else NotImplemented
                        )
                    )
                )
            )
        )

    @overload
    def __xor__(self, other: hints.Empty[hints.ScalarT], /) -> Self: ...

    @overload
    def __xor__(
        self,
        other: (
            hints.Contour[hints.ScalarT]
            | hints.Multisegment[hints.ScalarT]
            | hints.Segment[hints.ScalarT]
        ),
        /,
    ) -> (
        hints.Empty[hints.ScalarT]
        | hints.Multisegment[hints.ScalarT]
        | hints.Segment[hints.ScalarT]
    ): ...

    @overload
    def __xor__(self, other: Any, /) -> Any: ...

    def __xor__(self, other: Any, /) -> Any:
        context = self._context
        return (
            symmetric_subtract_multisegmental_from_multisegmental(
                self,
                other,
                context.empty_cls,
                context.multisegment_cls,
                context.orient,
                context.segment_cls,
                context.intersect_segments,
            )
            if is_multisegmental(other, context=context)
            else (
                symmetric_subtract_segment_from_multisegmental(
                    self,
                    other,
                    context.empty_cls,
                    context.multisegment_cls,
                    context.orient,
                    context.segment_cls,
                    context.intersect_segments,
                )
                if is_segment(other, context=context)
                else (
                    self
                    if is_empty(other, context=context)
                    else NotImplemented
                )
            )
        )


def _are_non_empty_unique_sequences_rotationally_equivalent(
    left: Sequence[Any], right: Sequence[Any], /
) -> bool:
    assert len(left) > 0, left
    assert len(right) > 0, right
    if len(left) != len(right):
        return False
    first_left_element = left[0]
    try:
        index = right.index(first_left_element)
    except ValueError:
        return False
    else:
        return (
            left[1 : len(left) - index] == right[index + 1 :]
            and left[len(left) - index :] == right[:index]
        ) or (
            left[1 : index + 1] == right[:index][::-1]
            and left[index + 1 :] == right[len(right) - 1 : index : -1]
        )


def _neighbour_segments_vertices_touch(
    intersection: Intersection[hints.ScalarT],
    segments: Sequence[hints.Segment[hints.ScalarT]],
    /,
) -> bool:
    first_segment = segments[intersection.first_segment_id]
    second_segment = segments[intersection.second_segment_id]
    touches_at_vertices = (
        intersection.relation is Relation.TOUCH
        and (
            intersection.start == first_segment.start
            or intersection.start == first_segment.end
        )
        and (
            intersection.start == second_segment.start
            or intersection.start == second_segment.end
        )
    )
    neighbour_segments_intersection = (
        (
            abs(intersection.first_segment_id - intersection.second_segment_id)
            == 1
        )
        or (
            intersection.first_segment_id == len(segments) - 1
            and intersection.second_segment_id == 0
        )
        or (
            intersection.second_segment_id == len(segments) - 1
            and intersection.first_segment_id == 0
        )
    )
    return touches_at_vertices and neighbour_segments_intersection
