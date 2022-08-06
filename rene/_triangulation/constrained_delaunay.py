from collections import deque
from itertools import (chain,
                       groupby,
                       repeat)
from operator import attrgetter
from typing import (Callable,
                    Container,
                    Iterable,
                    List,
                    Sequence,
                    Set,
                    Tuple)

from reprit.base import generate_repr

from rene._rene import (Location,
                        Orientation,
                        Relation)
from rene._utils import (locate_point_in_point_point_point_circle,
                         orient,
                         relate_segments,
                         to_sorted_pair)
from rene.hints import (Point,
                        Polygon)
from .mesh import (Mesh,
                   build_delaunay_triangulation,
                   orient_point_to_edge)
from .quad_edge import (UNDEFINED_EDGE,
                        QuadEdge,
                        to_opposite_edge)
from .vertices import (ContourVertex,
                       ContoursVertex)

BORDER_CONTOUR_INDEX = 0


class ConstrainedDelaunay:
    @classmethod
    def from_polygon(cls, polygon: Polygon) -> 'ConstrainedDelaunay':
        contours_vertices = [polygon.border.vertices,
                             *[hole.vertices for hole in polygon.holes]]
        vertices = list(chain.from_iterable(
                map(ContourVertex, repeat(index),
                    range(len(contour_vertices) - 1), contour_vertices)
                for index, contour_vertices in enumerate(contours_vertices)
        )) + [ContourVertex(index, len(contour_vertices) - 1,
                            contour_vertices[-1])
              for index, contour_vertices in enumerate(contours_vertices)]
        vertices.sort()
        merged_vertices = [
            ContoursVertex({vertex.contour_index: vertex.index
                            for vertex in same_point_vertices},
                           point)
            for point, same_point_vertices in groupby(vertices,
                                                      key=attrgetter('point'))
        ]
        mesh = Mesh.from_points(merged_vertices)
        left_side, right_side = build_delaunay_triangulation(mesh)
        contours_sizes = [len(contour_vertices)
                          for contour_vertices in contours_vertices]
        triangular_holes_indices = [
            hole_index
            for hole_index, hole_size in enumerate(contours_sizes[1:],
                                                   start=1)
            if hole_size == 3
        ]
        self = cls(left_side, right_side, mesh, triangular_holes_indices)
        self.constrain(contours_sizes, contours_vertices)
        self.bound(contours_sizes)
        self.cut(contours_vertices)
        return self

    @property
    def left_side(self) -> QuadEdge:
        return self._left_side

    @property
    def mesh(self) -> Mesh[ContoursVertex]:
        return self._mesh

    @property
    def right_side(self) -> QuadEdge:
        return self._right_side

    def bound(self, contours_sizes: List[int]) -> None:
        mesh = self.mesh
        boundary_edges = self.to_unique_boundary_edges()
        extraneous_mouths = [edge
                             for edge in boundary_edges
                             if not is_polygon_edge(mesh, edge,
                                                    contours_sizes)]
        while extraneous_mouths:
            mouth = extraneous_mouths.pop()
            candidates = mouth_edge_to_incidents(mesh, mouth)
            new_non_boundary = [candidate
                                for candidate in candidates
                                if not is_polygon_edge(mesh, candidate,
                                                       contours_sizes)]
            assert all(edge not in extraneous_mouths
                       for edge in new_non_boundary)
            self.delete_edge(mouth)
            extraneous_mouths.extend(new_non_boundary)

    def constrain(self,
                  contours_sizes: List[int],
                  contours_vertices: List[Sequence[Point]]) -> None:
        mesh = self.mesh
        has_three_boundary_edges = self.has_three_boundary_edges()
        assert (has_three_boundary_edges
                is (len(self.to_boundary_points()) == 3))
        is_inner_edge = (
            is_inner_edge_of_mesh_with_three_boundary_edges
            if has_three_boundary_edges
            else is_inner_edge_of_mesh_with_more_than_three_boundary_edges
        )
        potential_crossings = {
            edge
            for edge in mesh.to_unique_edges()
            if (is_inner_edge(mesh, edge)
                and not is_polygon_edge(mesh, edge, contours_sizes))
        }
        for (
                constraint_start, constraint_end
        ) in to_unsatisfied_constraints_endpoints(mesh, contours_vertices):
            set_constraint(mesh, constraint_start, constraint_end,
                           potential_crossings, contours_sizes, is_inner_edge)

    def cut(self, contours_vertices: List[Sequence[Point]]) -> None:
        mesh = self.mesh
        for edge in mesh.to_unique_edges():
            if is_edge_inside_hole(mesh, edge, contours_vertices):
                self.delete_edge(edge)

    def delete_edge(self, edge: QuadEdge) -> None:
        if (edge == self.right_side
                or to_opposite_edge(edge) == self.right_side):
            self._right_side = to_opposite_edge(self.mesh.to_right_from_end(
                    self.right_side
            ))
        if edge == self.left_side or to_opposite_edge(edge) == self.left_side:
            self._left_side = self.mesh.to_left_from_start(self.left_side)
        self.mesh.delete_edge(edge)

    def has_three_boundary_edges(self) -> bool:
        if self:
            start = self.left_side
            edge = start
            for edges_count in range(1, 4):
                candidate = self.mesh.to_right_from_end(edge)
                if candidate == start:
                    return edges_count == 3
                edge = candidate
        return False

    def to_boundary_points(self) -> List[Point]:
        if self:
            result = []
            start = self.left_side
            edge = start
            while True:
                result.append(self.mesh.to_start(edge).point)
                candidate = self.mesh.to_right_from_end(edge)
                if candidate == start:
                    break
                edge = candidate
            return result
        else:
            return [endpoint.point for endpoint in self.mesh.endpoints]

    def to_unique_boundary_edges(self) -> List[QuadEdge]:
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

    def triangles_vertices(self) -> List[Tuple[Point, Point, Point]]:
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
                            mesh, edge,
                            mesh.to_end(mesh.to_left_from_start(edge))
                    ) is Orientation.COUNTERCLOCKWISE
                    and
                    not (self._triangular_holes_indices
                         and are_triangular_hole_vertices(
                                    first_vertex, second_vertex, third_vertex,
                                    self._triangular_holes_indices
                            ))):
                result.append((first_vertex.point, second_vertex.point,
                               third_vertex.point))
        return result

    __slots__ = ('_left_side', '_mesh', '_right_side',
                 '_triangular_holes_indices')

    def __init__(self,
                 left_side: QuadEdge,
                 right_side: QuadEdge,
                 mesh: Mesh,
                 _triangular_holes_indices: Container[int]) -> None:
        self._left_side, self._mesh, self._right_side = (
            left_side, mesh, right_side
        )
        self._triangular_holes_indices = _triangular_holes_indices

    def __bool__(self) -> bool:
        result = bool(self.mesh)
        assert result is (self.left_side != UNDEFINED_EDGE)
        assert result is (self.right_side != UNDEFINED_EDGE)
        return result

    __repr__ = generate_repr(__init__)


def angle_contains_point(vertex: Point,
                         first_ray_point: Point,
                         second_ray_point: Point,
                         angle_orientation: Orientation,
                         point: Point) -> bool:
    assert angle_orientation is not Orientation.COLLINEAR
    first_half_orientation = orient(vertex, first_ray_point, point)
    second_half_orientation = orient(second_ray_point, vertex, point)
    return (second_half_orientation is angle_orientation
            if first_half_orientation is Orientation.COLLINEAR
            else (first_half_orientation is angle_orientation
                  if second_half_orientation is Orientation.COLLINEAR
                  else (first_half_orientation
                        is angle_orientation
                        is second_half_orientation)))


def are_polygon_edge_endpoints(start: ContoursVertex,
                               end: ContoursVertex,
                               contours_sizes: List[int]) -> bool:
    return any(are_polygon_edge_indices(start.indices[contour_index],
                                        end.indices[contour_index],
                                        contours_sizes[contour_index])
               for contour_index in start.indices.keys() & end.indices.keys())


def are_polygon_edge_indices(start_index: int,
                             end_index: int,
                             contour_size: int) -> bool:
    return (abs(start_index - end_index) == 1
            or (start_index == 0 and end_index == contour_size - 1)
            or (start_index == contour_size - 1 and end_index == 0))


def are_triangular_hole_vertices(
        first: ContoursVertex,
        second: ContoursVertex,
        third: ContoursVertex,
        triangular_holes_indices: Container[int]
) -> bool:
    assert first.point != second.point
    assert second.point != third.point
    assert third.point != first.point
    common_holes_indices = (first.indices.keys() & second.indices.keys()
                            & third.indices.keys() - {BORDER_CONTOUR_INDEX})
    return any(hole_index in triangular_holes_indices
               for hole_index in common_holes_indices)


def is_edge_inside_hole(mesh: Mesh,
                        edge: QuadEdge,
                        contours_vertices: List[Sequence[Point]]) -> bool:
    start, end = mesh.to_start(edge), mesh.to_end(edge)
    common_holes_indices = (
            start.indices.keys() & end.indices.keys()
            - {BORDER_CONTOUR_INDEX}
    )
    assert len(common_holes_indices) <= 1
    for hole_index in common_holes_indices:
        hole_vertices = contours_vertices[hole_index]
        hole_size = len(hole_vertices)
        start_index = start.indices[hole_index]
        end_index = end.indices[hole_index]
        if are_polygon_edge_indices(start_index, end_index, hole_size):
            return False
        prior_to_start_point = hole_vertices[start_index - 1]
        prior_to_end_point = hole_vertices[end_index - 1]
        next_to_start_point = hole_vertices[(start_index + 1) % hole_size]
        next_to_end_point = hole_vertices[(end_index + 1) % hole_size]
        end_angle_orientation = orient(end.point, prior_to_end_point,
                                       next_to_end_point)
        start_angle_orientation = orient(start.point, prior_to_start_point,
                                         next_to_start_point)
        if (((end_angle_orientation is Orientation.COUNTERCLOCKWISE)
             is angle_contains_point(end.point, prior_to_end_point,
                                     next_to_end_point, end_angle_orientation,
                                     start.point))
                and ((start_angle_orientation is Orientation.COUNTERCLOCKWISE)
                     is angle_contains_point(start.point, prior_to_start_point,
                                             next_to_start_point,
                                             start_angle_orientation,
                                             end.point))):
            return True
    return False


def mouth_edge_to_incidents(mesh: Mesh, edge: QuadEdge) -> Iterable[QuadEdge]:
    left_from_start = mesh.to_left_from_start(edge)
    assert orient_point_to_edge(
            mesh, edge, mesh.to_end(left_from_start)
    ) is Orientation.COUNTERCLOCKWISE
    return [left_from_start, mesh.to_right_from_end(left_from_start)]


def to_unsatisfied_constraints_endpoints(
        mesh: Mesh[ContoursVertex],
        contours_vertices: List[Sequence[Point]]
) -> Iterable[Tuple[Point, Point]]:
    are_constraints_satisfied = [[False] * len(contour_vertices)
                                 for contour_vertices in contours_vertices]
    for edge in mesh.to_unique_edges():
        edge_start, edge_end = mesh.to_start(edge), mesh.to_end(edge)
        common_contours_indices = (edge_start.indices.keys()
                                   & edge_end.indices.keys())
        for contour_index in common_contours_indices:
            edge_start_index = edge_start.indices[contour_index]
            edge_end_index = edge_end.indices[contour_index]
            if are_polygon_edge_indices(edge_start_index, edge_end_index,
                                        len(contours_vertices[contour_index])):
                are_constraints_satisfied[contour_index][
                    min(edge_start_index, edge_end_index)
                    if abs(edge_end_index - edge_start_index) == 1
                    else -1
                ] = True
    for contour_index, are_contour_constraints_satisfied in enumerate(
            are_constraints_satisfied
    ):
        for index, is_constraint_satisfied in enumerate(
                are_contour_constraints_satisfied
        ):
            if not is_constraint_satisfied:
                contour_vertices = contours_vertices[contour_index]
                constraint_start = contour_vertices[index]
                constraint_end = contour_vertices[(index + 1)
                                                  % len(contour_vertices)]
                yield constraint_start, constraint_end


def set_constraint(mesh: Mesh[ContoursVertex],
                   constraint_start: Point,
                   constraint_end: Point,
                   potential_crossings: Set[QuadEdge],
                   contours_sizes: List[int],
                   is_inner_edge: Callable[[Mesh, QuadEdge], bool]) -> None:
    crossings = [
        edge
        for edge in potential_crossings
        if relate_segments(mesh.to_start(edge).point, mesh.to_end(edge).point,
                           constraint_start, constraint_end) is Relation.CROSS
    ]
    potential_crossings.difference_update(crossings)
    new_edges = resolve_crossings(
            mesh, crossings, constraint_start, constraint_end
    )
    constraint_endpoints = to_sorted_pair(constraint_start, constraint_end)
    set_criterion(
            mesh,
            {edge
             for edge in new_edges
             if (to_sorted_pair(mesh.to_start(edge).point,
                                mesh.to_end(edge).point)
                 != constraint_endpoints)}
    )
    potential_crossings.update(edge
                               for edge in new_edges
                               if (is_inner_edge(mesh, edge)
                                   and not is_polygon_edge(mesh, edge,
                                                           contours_sizes)))


def edge_should_be_swapped(mesh: Mesh, edge: QuadEdge) -> bool:
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


def is_convex_quadrilateral_diagonal(mesh: Mesh, edge: QuadEdge) -> bool:
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


def is_inner_edge_of_mesh_with_more_than_three_boundary_edges(
        mesh: Mesh, edge: QuadEdge
) -> bool:
    return (
            mesh.to_right_from_end(mesh.to_right_from_end(
                    mesh.to_right_from_end(edge)
            )) == edge
            and
            mesh.to_left_from_end(mesh.to_left_from_end(
                    mesh.to_left_from_end(edge)
            )) == edge
    )


def is_inner_edge_of_mesh_with_three_boundary_edges(mesh: Mesh,
                                                    edge: QuadEdge) -> bool:
    return (orient_point_to_edge(mesh, edge,
                                 mesh.to_end(mesh.to_left_from_end(edge)))
            is not
            orient_point_to_edge(mesh, edge,
                                 mesh.to_end(mesh.to_right_from_end(edge))))


def is_polygon_edge(mesh: Mesh[ContoursVertex],
                    edge: QuadEdge,
                    contours_sizes: List[int]) -> bool:
    return are_polygon_edge_endpoints(mesh.to_start(edge), mesh.to_end(edge),
                                      contours_sizes)


def resolve_crossings(mesh: Mesh,
                      crossings: List[QuadEdge],
                      constraint_start: Point,
                      constraint_end: Point) -> List[QuadEdge]:
    result = []
    crossings_queue = deque(crossings,
                            maxlen=len(crossings))
    while crossings_queue:
        edge = crossings_queue.popleft()
        if is_convex_quadrilateral_diagonal(mesh, edge):
            mesh.swap_diagonal(edge)
            if relate_segments(
                    mesh.to_start(edge).point, mesh.to_end(edge).point,
                    constraint_start, constraint_end
            ) is Relation.CROSS:
                crossings_queue.append(edge)
            else:
                result.append(edge)
        else:
            assert not is_convex_quadrilateral_diagonal(mesh,
                                                        to_opposite_edge(edge))
            crossings_queue.append(edge)
    return result


def set_criterion(mesh: Mesh, target_edges: Set[QuadEdge]) -> None:
    while True:
        edges_to_swap = [
            edge
            for edge in target_edges
            if edge_should_be_swapped(mesh, edge)
        ]
        if not edges_to_swap:
            break
        for edge in edges_to_swap:
            mesh.swap_diagonal(edge)
        target_edges.difference_update(edges_to_swap)
