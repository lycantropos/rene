from collections import deque
from itertools import (chain,
                       groupby,
                       repeat)
from operator import attrgetter
from typing import (Container,
                    List,
                    Sequence,
                    Tuple)

from reprit.base import generate_repr

from rene._rene import (Location,
                        Orientation,
                        Relation)
from rene._utils import (locate_point_in_point_point_point_circle,
                         orient,
                         relate_segments)
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
            first_candidate, second_candidate = mouth_edge_to_incidents(
                    mesh, mouth
            )
            self.delete_edge(mouth)
            if not is_polygon_edge(mesh, first_candidate, contours_sizes):
                extraneous_mouths.append(first_candidate)
            if not is_polygon_edge(mesh, second_candidate, contours_sizes):
                extraneous_mouths.append(second_candidate)

    def constrain(self,
                  contours_sizes: List[int],
                  contours_vertices: List[Sequence[Point]]) -> None:
        mesh = self.mesh
        contours_constraints_flags = to_contours_constraints_flags(
                mesh, contours_sizes
        )
        for edge in mesh.to_edges():
            vertex = mesh.to_start(edge)
            for contour_index, vertex_index in vertex.indices.items():
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
                            mesh, angle_base_edge, vertex.point,
                            next_vertex_point
                    )
                    if crossings:
                        set_constraint(mesh, vertex.point, next_vertex_point,
                                       crossings)
                    contours_constraints_flags[contour_index][
                        constraint_index
                    ] = True
            assert mesh.to_start(edge) is vertex
        assert all(map(all, contours_constraints_flags))

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

    def to_boundary_points(self) -> List[Point]:
        edge_to_start = self.mesh.to_start
        return [edge_to_start(edge).point
                for edge in self.to_unique_boundary_edges()]

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


def are_polygon_edge_indices(first: int,
                             second: int,
                             contour_size: int) -> bool:
    return (abs(first - second) == 1
            or (first == 0 and second == contour_size - 1)
            or (first == contour_size - 1 and second == 0))


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


def detect_crossings(mesh: Mesh[ContoursVertex],
                     base_edge: QuadEdge,
                     constraint_start: Point,
                     constraint_end: Point) -> List[QuadEdge]:
    candidate = mesh.to_left_from_end(base_edge)
    result = []
    while mesh.to_start(candidate).point != constraint_end:
        last_crossing = candidate
        assert relate_segments(
                mesh.to_start(last_crossing).point,
                mesh.to_end(last_crossing).point,
                constraint_start, constraint_end
        ) is Relation.CROSS
        result.append(last_crossing)
        candidate = mesh.to_right_from_start(last_crossing)
        if ((orient_point_to_edge(mesh, candidate, constraint_end)
             is not Orientation.CLOCKWISE)
                or (orient(constraint_start, constraint_end,
                           mesh.to_end(candidate).point)
                    is Orientation.CLOCKWISE)):
            candidate = to_opposite_edge(mesh.to_right_from_end(last_crossing))
    assert all(to_opposite_edge(edge) not in result for edge in result)
    assert all(edge in result or to_opposite_edge(edge) in result
               for edge in mesh.to_unique_edges()
               if relate_segments(mesh.to_start(edge).point,
                                  mesh.to_end(edge).point, constraint_start,
                                  constraint_end) is Relation.CROSS)
    return result


def is_edge_inside_hole(mesh: Mesh,
                        edge: QuadEdge,
                        contours_vertices: List[Sequence[Point]]) -> bool:
    start, end = mesh.to_start(edge), mesh.to_end(edge)
    common_holes_indices = (start.indices.keys() & end.indices.keys()
                            - {BORDER_CONTOUR_INDEX})
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
        start_angle_orientation = orient(start.point, prior_to_start_point,
                                         next_to_start_point)
        end_angle_orientation = orient(end.point, prior_to_end_point,
                                       next_to_end_point)
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


def mouth_edge_to_incidents(mesh: Mesh,
                            edge: QuadEdge) -> Tuple[QuadEdge, QuadEdge]:
    left_from_start = mesh.to_left_from_start(edge)
    assert orient_point_to_edge(
            mesh, edge, mesh.to_end(left_from_start)
    ) is Orientation.COUNTERCLOCKWISE
    return left_from_start, mesh.to_right_from_end(left_from_start)


def to_contours_constraints_flags(
        mesh: Mesh,
        contours_sizes: List[int]
) -> List[List[bool]]:
    result = [[False] * contour_size for contour_size in contours_sizes]
    for edge in mesh.to_unique_edges():
        start, end = mesh.to_start(edge), mesh.to_end(edge)
        for contour_index in start.indices.keys() & end.indices.keys():
            start_index, end_index = (start.indices[contour_index],
                                      end.indices[contour_index])
            if are_polygon_edge_indices(start_index, end_index,
                                        contours_sizes[contour_index]):
                result[contour_index][
                    to_constraint_index(start_index, end_index)
                ] = True
    return result


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


def is_polygon_edge(mesh: Mesh[ContoursVertex],
                    edge: QuadEdge,
                    contours_sizes: List[int]) -> bool:
    return are_polygon_edge_endpoints(mesh.to_start(edge), mesh.to_end(edge),
                                      contours_sizes)


def resolve_crossings(mesh: Mesh[ContoursVertex],
                      constraint_start: Point,
                      constraint_end: Point,
                      crossings: List[QuadEdge]) -> List[QuadEdge]:
    result = []
    crossings_queue = deque(crossings,
                            maxlen=len(crossings))
    while crossings_queue:
        crossing = crossings_queue.popleft()
        if is_convex_quadrilateral_diagonal(mesh, crossing):
            mesh.swap_diagonal(crossing)
            relation = relate_segments(mesh.to_start(crossing).point,
                                       mesh.to_end(crossing).point,
                                       constraint_start, constraint_end)
            if relation is Relation.CROSS:
                crossings_queue.append(crossing)
            elif relation is not Relation.EQUAL:
                result.append(crossing)
        else:
            crossings_queue.append(crossing)
    return result


def restore_delaunay_criterion(mesh: Mesh, candidates: List[QuadEdge]) -> None:
    while True:
        next_target_edges = []
        edges_to_swap = []
        for edge in candidates:
            (edges_to_swap
             if edge_should_be_swapped(mesh, edge)
             else next_target_edges).append(edge)
        if not edges_to_swap:
            break
        for edge in edges_to_swap:
            mesh.swap_diagonal(edge)
        candidates = next_target_edges


def set_constraint(mesh: Mesh[ContoursVertex],
                   constraint_start: Point,
                   constraint_end: Point,
                   crossings: List[QuadEdge]) -> None:
    new_edges = resolve_crossings(mesh, constraint_start, constraint_end,
                                  crossings)
    restore_delaunay_criterion(mesh, new_edges)


def to_angle_containing_constraint_base(mesh: Mesh[ContoursVertex],
                                        edge: QuadEdge,
                                        constraint_end: Point) -> QuadEdge:
    if mesh.to_end(edge).point != constraint_end:
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


def to_constraint_index(first_vertex_index: int,
                        second_vertex_index: int) -> int:
    return (max(first_vertex_index, second_vertex_index)
            if abs(second_vertex_index - first_vertex_index) == 1
            else 0)
