from __future__ import annotations

import typing as t
from collections import deque
from itertools import (chain,
                       groupby)
from operator import attrgetter

import typing_extensions as te

from rene import (Location,
                  Orientation,
                  Relation,
                  hints)
from rene._relating import segment_endpoints
from rene._utils import (locate_point_in_point_point_point_circle,
                         orient)
from .mesh import (Mesh,
                   build_delaunay_triangulation,
                   orient_point_to_edge)
from .quad_edge import (UNDEFINED_EDGE,
                        QuadEdge,
                        to_opposite_edge)
from .vertices import (ContourVertex,
                       PolygonVertexPosition)

BORDER_CONTOUR_INDEX = 0


class ConstrainedDelaunayTriangulation(t.Generic[hints.Scalar]):
    @classmethod
    def from_polygon(cls, polygon: hints.Polygon[hints.Scalar], /) -> te.Self:
        contours_vertices = [polygon.border.vertices,
                             *[hole.vertices for hole in polygon.holes]]
        vertices = list(chain.from_iterable(
                (
                    ContourVertex(index, vertex_index, vertex)
                    for vertex_index, vertex
                    in zip(range(len(contour_vertices) - 1), contour_vertices)
                )
                for index, contour_vertices in enumerate(contours_vertices)
        )) + [ContourVertex(index, len(contour_vertices) - 1,
                            contour_vertices[-1])
              for index, contour_vertices in enumerate(contours_vertices)]
        vertices.sort()
        polygon_vertices_positions: t.List[
            t.List[PolygonVertexPosition]
        ] = []
        points: t.List[hints.Point[hints.Scalar]] = []
        for point, same_point_vertices in groupby(vertices,
                                                  key=attrgetter('point')):
            points.append(point)
            polygon_vertices_positions.append(
                    [PolygonVertexPosition(vertex.contour_index, vertex.index)
                     for vertex in same_point_vertices]
            )
        mesh = Mesh.from_points(points)
        left_side, right_side = build_delaunay_triangulation(mesh)
        contours_sizes = [len(contour_vertices)
                          for contour_vertices in contours_vertices]
        triangular_holes_indices = [
            hole_index
            for hole_index, hole_size in enumerate(contours_sizes[1:],
                                                   start=1)
            if hole_size == 3
        ]
        self = cls(left_side, right_side, mesh, polygon_vertices_positions,
                   triangular_holes_indices)
        self.constrain(contours_sizes, contours_vertices)
        self.bound(contours_sizes)
        self.cut(contours_vertices)
        return self

    @property
    def left_side(self) -> QuadEdge:
        return self._left_side

    @property
    def mesh(self) -> Mesh[hints.Scalar]:
        return self._mesh

    @property
    def right_side(self) -> QuadEdge:
        return self._right_side

    def bound(self, contours_sizes: t.List[int], /) -> None:
        mesh = self.mesh
        boundary_edges = self.to_unique_boundary_edges()
        extraneous_mouths = [
            edge
            for edge in boundary_edges
            if not is_polygon_edge(mesh, edge, contours_sizes,
                                   self._polygon_vertices_positions)
        ]
        while extraneous_mouths:
            mouth = extraneous_mouths.pop()
            first_candidate, second_candidate = mouth_edge_to_incidents(
                    mesh, mouth
            )
            self.delete_edge(mouth)
            if not is_polygon_edge(mesh, first_candidate, contours_sizes,
                                   self._polygon_vertices_positions):
                extraneous_mouths.append(first_candidate)
            if not is_polygon_edge(mesh, second_candidate, contours_sizes,
                                   self._polygon_vertices_positions):
                extraneous_mouths.append(second_candidate)

    def constrain(
            self,
            contours_sizes: t.List[int],
            contours_vertices: t.List[t.Sequence[hints.Point[hints.Scalar]]],
            /
    ) -> None:
        mesh = self.mesh
        contours_constraints_flags = to_contours_constraints_flags(
                mesh, contours_sizes, self._polygon_vertices_positions
        )
        for edge in mesh.to_edges():
            vertex_start_index = mesh.to_start_index(edge)
            vertex_point = mesh.endpoints[vertex_start_index]
            vertex_positions = self._polygon_vertices_positions[
                vertex_start_index
            ]
            for vertex_position in vertex_positions:
                contour_index, vertex_index = (vertex_position.contour_index,
                                               vertex_position.index)
                contour_size = contours_sizes[contour_index]
                next_vertex_index = (vertex_index + 1) % contour_size
                constraint_index = to_constraint_index(vertex_index,
                                                       next_vertex_index)
                if not contours_constraints_flags[contour_index][
                    constraint_index
                ]:
                    next_vertex_point = contours_vertices[contour_index][
                        next_vertex_index
                    ]
                    angle_base_edge = to_angle_containing_constraint_base(
                            mesh, edge, next_vertex_point
                    )
                    crossings = detect_crossings(
                            mesh, angle_base_edge, vertex_point,
                            next_vertex_point
                    )
                    if crossings:
                        set_constraint(mesh, vertex_point, next_vertex_point,
                                       crossings)
                    contours_constraints_flags[contour_index][
                        constraint_index
                    ] = True

    def cut(self,
            contours_vertices: t.List[t.Sequence[hints.Point[hints.Scalar]]],
            /) -> None:
        mesh = self.mesh
        for edge in mesh.to_unique_edges():
            if is_edge_inside_hole(mesh, edge, contours_vertices,
                                   self._polygon_vertices_positions):
                self.delete_edge(edge)

    def delete_edge(self, edge: QuadEdge, /) -> None:
        if (edge == self.right_side
                or to_opposite_edge(edge) == self.right_side):
            self._right_side = to_opposite_edge(self.mesh.to_right_from_end(
                    self.right_side
            ))
        if edge == self.left_side or to_opposite_edge(edge) == self.left_side:
            self._left_side = self.mesh.to_left_from_start(self.left_side)
        self.mesh.delete_edge(edge)

    def to_boundary_points(self) -> t.List[hints.Point[hints.Scalar]]:
        edge_to_start = self.mesh.to_start
        return [edge_to_start(edge)
                for edge in self.to_unique_boundary_edges()]

    def to_unique_boundary_edges(self) -> t.List[QuadEdge]:
        if self:
            result = []
            start = self.left_side
            edge = start
            while True:
                result.append(edge)
                candidate = self.mesh.to_right_from_end(edge)
                if candidate == start:
                    break
                edge = candidate
            return result
        else:
            return list(self.mesh.to_unique_edges())

    def triangles_vertices(
            self
    ) -> t.List[t.Tuple[
        hints.Point[hints.Scalar], hints.Point[hints.Scalar],
        hints.Point[hints.Scalar]
    ]]:
        mesh = self.mesh
        result = []
        for edge in mesh.to_edges():
            first_vertex = mesh.to_start(edge)
            second_vertex = mesh.to_end(edge)
            third_vertex = mesh.to_end(mesh.to_left_from_start(edge))
            if (first_vertex < second_vertex
                    and first_vertex < third_vertex
                    and third_vertex == mesh.to_end(
                            mesh.to_right_from_start(to_opposite_edge(edge))
                    )
                    and orient_point_to_edge(
                            mesh, edge, third_vertex
                    ) is Orientation.COUNTERCLOCKWISE
                    and
                    not (self._triangular_holes_indices
                         and are_triangular_hole_vertices(
                                    self._polygon_vertices_positions[
                                        mesh.to_start_index(edge)
                                    ],
                                    self._polygon_vertices_positions[
                                        mesh.to_end_index(edge)
                                    ],
                                    self._polygon_vertices_positions[
                                        mesh.to_end_index(
                                                mesh.to_left_from_start(edge)
                                        )
                                    ],
                                    self._triangular_holes_indices
                            ))):
                result.append((first_vertex, second_vertex, third_vertex))
        return result

    __slots__ = ('_left_side', '_mesh', '_polygon_vertices_positions',
                 '_right_side', '_triangular_holes_indices')

    def __init__(
            self,
            left_side: QuadEdge,
            right_side: QuadEdge,
            mesh: Mesh[hints.Scalar],
            _polygon_vertices_positions: t.List[
                t.List[PolygonVertexPosition]
            ],
            _triangular_holes_indices: t.Container[int],
            /
    ) -> None:
        (self._left_side, self._mesh, self._polygon_vertices_positions,
         self._right_side, self._triangular_holes_indices) = (
            left_side, mesh, _polygon_vertices_positions, right_side,
            _triangular_holes_indices
        )

    def __bool__(self) -> bool:
        result = bool(self.mesh)
        assert result is (self.left_side != UNDEFINED_EDGE)
        assert result is (self.right_side != UNDEFINED_EDGE)
        return result


def angle_contains_point(vertex: hints.Point[hints.Scalar],
                         first_ray_point: hints.Point[hints.Scalar],
                         second_ray_point: hints.Point[hints.Scalar],
                         angle_orientation: Orientation,
                         point: hints.Point[hints.Scalar],
                         /) -> bool:
    assert angle_orientation is not Orientation.COLLINEAR
    first_half_orientation = orient(vertex, first_ray_point, point)
    second_half_orientation = orient(second_ray_point, vertex, point)
    return ((first_half_orientation is Orientation.COLLINEAR
             or first_half_orientation is angle_orientation)
            and (second_half_orientation is Orientation.COLLINEAR
                 or second_half_orientation is angle_orientation))


def are_polygon_edge_indices(
        first: int, second: int, contour_size: int, /
) -> bool:
    return (abs(first - second) == 1
            or (first == 0 and second == contour_size - 1)
            or (first == contour_size - 1 and second == 0))


def are_triangular_hole_vertices(
        first_positions: t.List[PolygonVertexPosition],
        second_positions: t.List[PolygonVertexPosition],
        third_positions: t.List[PolygonVertexPosition],
        triangular_holes_indices: t.Container[int],
        /
) -> bool:
    first_contours_indices = [position.contour_index
                              for position in first_positions]
    second_contours_indices = [position.contour_index
                               for position in second_positions]
    third_contours_indices = [position.contour_index
                              for position in third_positions]
    return any((contour_index != BORDER_CONTOUR_INDEX
                and contour_index in second_contours_indices
                and contour_index in third_contours_indices
                and contour_index in triangular_holes_indices)
               for contour_index in first_contours_indices)


def detect_crossings(mesh: Mesh[hints.Scalar],
                     base_edge: QuadEdge,
                     constraint_start: hints.Point[hints.Scalar],
                     constraint_end: hints.Point[hints.Scalar],
                     /) -> t.List[QuadEdge]:
    candidate = mesh.to_left_from_end(base_edge)
    result = []
    while mesh.to_start(candidate) != constraint_end:
        last_crossing = candidate
        assert segment_endpoints.relate_to_segment_endpoints(
                mesh.to_start(last_crossing), mesh.to_end(last_crossing),
                constraint_start, constraint_end
        ) is Relation.CROSS
        result.append(last_crossing)
        candidate = mesh.to_right_from_start(last_crossing)
        if ((orient_point_to_edge(mesh, candidate, constraint_end)
             is not Orientation.CLOCKWISE)
                or (orient(constraint_start, constraint_end,
                           mesh.to_end(candidate))
                    is Orientation.CLOCKWISE)):
            candidate = to_opposite_edge(mesh.to_right_from_end(last_crossing))
    assert all(to_opposite_edge(edge) not in result for edge in result)
    assert all(
            edge in result or to_opposite_edge(edge) in result
            for edge in mesh.to_unique_edges()
            if (
                    segment_endpoints.relate_to_segment_endpoints(
                            mesh.to_start(edge), mesh.to_end(edge),
                            constraint_start, constraint_end
                    ) is Relation.CROSS
            )
    )
    return result


def intersect_polygon_vertices_positions(
        first: t.List[PolygonVertexPosition],
        second: t.List[PolygonVertexPosition],
        *,
        with_border: bool
) -> t.List[t.Tuple[PolygonVertexPosition, PolygonVertexPosition]]:
    return list(_intersect_polygon_vertices_positions(first, second, False,
                                                      with_border)
                if len(first) < len(second)
                else _intersect_polygon_vertices_positions(second, first, True,
                                                           with_border))


def _intersect_polygon_vertices_positions(
        first: t.List[PolygonVertexPosition],
        second: t.List[PolygonVertexPosition],
        reverse: bool,
        with_border: bool,
        /
) -> t.Iterable[t.Tuple[PolygonVertexPosition, PolygonVertexPosition]]:
    assert len(first) <= len(second)
    for first_position in first:
        if with_border or first_position.contour_index != BORDER_CONTOUR_INDEX:
            try:
                second_position = next(
                        candidate
                        for candidate in second
                        if (candidate.contour_index
                            == first_position.contour_index)
                )
            except StopIteration:
                pass
            else:
                yield ((second_position, first_position)
                       if reverse
                       else (first_position, second_position))


def is_edge_inside_hole(
        mesh: Mesh[hints.Scalar],
        edge: QuadEdge,
        contours_vertices: t.List[t.Sequence[hints.Point[hints.Scalar]]],
        polygon_vertices_positions: t.List[t.List[PolygonVertexPosition]],
        /
) -> bool:
    start_index, end_index = mesh.to_start_index(edge), mesh.to_end_index(edge)
    start, end = mesh.endpoints[start_index], mesh.endpoints[end_index]
    common_holes_positions = intersect_polygon_vertices_positions(
            polygon_vertices_positions[start_index],
            polygon_vertices_positions[end_index],
            with_border=False
    )
    assert len(common_holes_positions) <= 1
    return (bool(common_holes_positions)
            and is_segment_inside_hole(start, end, *common_holes_positions[0],
                                       contours_vertices))


def is_segment_inside_hole(
        start: hints.Point[hints.Scalar],
        end: hints.Point[hints.Scalar],
        start_position: PolygonVertexPosition,
        end_position: PolygonVertexPosition,
        contours_vertices: t.List[t.Sequence[hints.Point[hints.Scalar]]],
        /
) -> bool:
    assert start_position.contour_index == end_position.contour_index
    hole_vertices = contours_vertices[start_position.contour_index]
    hole_size = len(hole_vertices)
    start_vertex_index = start_position.index
    end_vertex_index = end_position.index
    if are_polygon_edge_indices(start_vertex_index, end_vertex_index,
                                hole_size):
        return False
    prior_to_start_point = hole_vertices[start_vertex_index - 1]
    prior_to_end_point = hole_vertices[end_vertex_index - 1]
    next_to_start_point = hole_vertices[(start_vertex_index + 1) % hole_size]
    next_to_end_point = hole_vertices[(end_vertex_index + 1) % hole_size]
    start_angle_orientation = orient(start, prior_to_start_point,
                                     next_to_start_point)
    end_angle_orientation = orient(end, prior_to_end_point,
                                   next_to_end_point)
    return (((end_angle_orientation is Orientation.COUNTERCLOCKWISE)
             is angle_contains_point(end, prior_to_end_point,
                                     next_to_end_point, end_angle_orientation,
                                     start))
            and ((start_angle_orientation is Orientation.COUNTERCLOCKWISE)
                 is angle_contains_point(start, prior_to_start_point,
                                         next_to_start_point,
                                         start_angle_orientation, end)))


def mouth_edge_to_incidents(
        mesh: Mesh[hints.Scalar], edge: QuadEdge, /
) -> t.Tuple[QuadEdge, QuadEdge]:
    left_from_start = mesh.to_left_from_start(edge)
    assert orient_point_to_edge(
            mesh, edge, mesh.to_end(left_from_start)
    ) is Orientation.COUNTERCLOCKWISE
    return left_from_start, mesh.to_right_from_end(left_from_start)


def to_contours_constraints_flags(
        mesh: Mesh[hints.Scalar],
        contours_sizes: t.List[int],
        polygon_vertices_positions: t.List[t.List[PolygonVertexPosition]],
        /
) -> t.List[t.List[bool]]:
    result = [[False] * contour_size for contour_size in contours_sizes]
    for edge in mesh.to_unique_edges():
        common_positions = intersect_polygon_vertices_positions(
                polygon_vertices_positions[mesh.to_start_index(edge)],
                polygon_vertices_positions[mesh.to_end_index(edge)],
                with_border=True
        )
        for start_position, end_position in common_positions:
            assert start_position.contour_index == end_position.contour_index
            contour_index = start_position.contour_index
            start_index, end_index = (start_position.index, end_position.index)
            if are_polygon_edge_indices(start_index, end_index,
                                        contours_sizes[contour_index]):
                result[contour_index][
                    to_constraint_index(start_index, end_index)
                ] = True
    return result


def edge_should_be_swapped(
        mesh: Mesh[hints.Scalar], edge: QuadEdge, /
) -> bool:
    return (is_convex_quadrilateral_diagonal(mesh, edge)
            and
            (locate_point_in_point_point_point_circle(
                    mesh.to_end(mesh.to_right_from_start(edge)),
                    mesh.to_start(edge), mesh.to_end(edge),
                    mesh.to_end(mesh.to_left_from_start(edge))
            )
             is Location.INTERIOR
             or (locate_point_in_point_point_point_circle(
                            mesh.to_end(mesh.to_left_from_start(edge)),
                            mesh.to_end(edge), mesh.to_start(edge),
                            mesh.to_end(mesh.to_right_from_start(edge))
                    )
                 is Location.INTERIOR)))


def is_convex_quadrilateral_diagonal(
        mesh: Mesh[hints.Scalar], edge: QuadEdge, /
) -> bool:
    return (orient_point_to_edge(mesh, mesh.to_left_from_end(edge),
                                 mesh.to_start(edge))
            is orient_point_to_edge(mesh, mesh.to_right_from_start(edge),
                                    mesh.to_end(edge))
            is Orientation.COUNTERCLOCKWISE
            is orient_point_to_edge(
                    mesh,
                    to_opposite_edge(mesh.to_right_from_end(edge)),
                    mesh.to_end(mesh.to_left_from_start(edge))
            )
            is orient_point_to_edge(
                    mesh,
                    to_opposite_edge(mesh.to_left_from_start(edge)),
                    mesh.to_end(mesh.to_right_from_start(edge))
            ))


def is_polygon_edge(
        mesh: Mesh[hints.Scalar],
        edge: QuadEdge,
        contours_sizes: t.List[int],
        polygon_vertices_positions: t.List[t.List[PolygonVertexPosition]],
        /
) -> bool:
    common_positions = intersect_polygon_vertices_positions(
            polygon_vertices_positions[mesh.to_start_index(edge)],
            polygon_vertices_positions[mesh.to_end_index(edge)],
            with_border=True
    )
    return any(are_polygon_edge_indices(start_position.index,
                                        end_position.index,
                                        contours_sizes[
                                            start_position.contour_index
                                        ])
               for start_position, end_position in common_positions)


def resolve_crossings(mesh: Mesh[hints.Scalar],
                      constraint_start: hints.Point[hints.Scalar],
                      constraint_end: hints.Point[hints.Scalar],
                      crossings: t.List[QuadEdge],
                      /) -> t.List[QuadEdge]:
    result = []
    crossings_queue = deque(crossings,
                            maxlen=len(crossings))
    while crossings_queue:
        crossing = crossings_queue.popleft()
        if is_convex_quadrilateral_diagonal(mesh, crossing):
            mesh.swap_diagonal(crossing)
            relation = segment_endpoints.relate_to_segment_endpoints(
                    mesh.to_start(crossing), mesh.to_end(crossing),
                    constraint_start, constraint_end
            )
            if relation is Relation.CROSS:
                crossings_queue.append(crossing)
            elif relation is not Relation.EQUAL:
                result.append(crossing)
        else:
            crossings_queue.append(crossing)
    return result


def restore_delaunay_criterion(
        mesh: Mesh[hints.Scalar], candidates: t.List[QuadEdge], /
) -> None:
    while True:
        next_target_edges: t.List[QuadEdge] = []
        edges_to_swap: t.List[QuadEdge] = []
        for edge in candidates:
            (edges_to_swap
             if edge_should_be_swapped(mesh, edge)
             else next_target_edges).append(edge)
        if not edges_to_swap:
            break
        for edge in edges_to_swap:
            mesh.swap_diagonal(edge)
        candidates = next_target_edges


def set_constraint(mesh: Mesh[hints.Scalar],
                   constraint_start: hints.Point[hints.Scalar],
                   constraint_end: hints.Point[hints.Scalar],
                   crossings: t.List[QuadEdge],
                   /) -> None:
    new_edges = resolve_crossings(mesh, constraint_start, constraint_end,
                                  crossings)
    restore_delaunay_criterion(mesh, new_edges)


def to_angle_containing_constraint_base(
        mesh: Mesh[hints.Scalar],
        edge: QuadEdge,
        constraint_end: hints.Point[hints.Scalar],
        /
) -> QuadEdge:
    if mesh.to_end(edge) != constraint_end:
        orientation = orient_point_to_edge(mesh, edge, constraint_end)
        if orientation is Orientation.COUNTERCLOCKWISE:
            while True:
                candidate = mesh.to_left_from_start(edge)
                orientation = orient_point_to_edge(mesh, candidate,
                                                   constraint_end)
                if orientation is Orientation.CLOCKWISE:
                    break
                edge = candidate
        else:
            while True:
                edge = mesh.to_right_from_start(edge)
                orientation = orient_point_to_edge(mesh, edge, constraint_end)
                if orientation is not Orientation.CLOCKWISE:
                    break
    return edge


def to_constraint_index(
        first_vertex_index: int, second_vertex_index: int, /
) -> int:
    return (max(first_vertex_index, second_vertex_index)
            if abs(second_vertex_index - first_vertex_index) == 1
            else 0)
