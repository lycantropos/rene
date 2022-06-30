import sys
from typing import (List,
                    Optional,
                    Sequence,
                    Tuple)

from rene._rene import (Location,
                        Orientation)
from rene._utils import (ceil_log2,
                         deduplicate,
                         locate_point_in_point_point_point_circle,
                         orient)
from rene.hints import Point
from .mesh import Mesh
from .quad_edge import (QuadEdge,
                        to_opposite_edge)

UNDEFINED_EDGE = QuadEdge(sys.maxsize)


def create_triangle(mesh: Mesh,
                    left_point_index: int,
                    mid_point_index: int,
                    right_point_index: int) -> Tuple[QuadEdge, QuadEdge]:
    first_edge = mesh.create_edge(left_point_index, mid_point_index)
    second_edge = mesh.create_edge(mid_point_index, right_point_index)
    mesh.splice_edges(to_opposite_edge(first_edge), second_edge)
    orientation = orient_point_to_edge(
            mesh, first_edge, mesh.to_end(second_edge)
    )
    if orientation is Orientation.CLOCKWISE:
        third_edge = mesh.connect_edges(second_edge, first_edge)
        return to_opposite_edge(third_edge), third_edge
    elif orientation is Orientation.COLLINEAR:
        return first_edge, to_opposite_edge(second_edge)
    else:
        assert orientation is Orientation.COUNTERCLOCKWISE
        mesh.connect_edges(second_edge, first_edge)
        return first_edge, to_opposite_edge(second_edge)


class Triangulation:
    @classmethod
    def delaunay(cls, points: Sequence[Point]) -> 'Triangulation':
        endpoints = list(points)
        endpoints.sort()
        endpoints = deduplicate(endpoints)
        mesh = Mesh.from_points(endpoints)
        if len(endpoints) < 2:
            return cls(UNDEFINED_EDGE, UNDEFINED_EDGE, mesh)
        else:
            segments_count, triangles_count = to_base_cases(len(endpoints))
            sub_triangulations_sides = []
            for index in range(segments_count):
                edge = mesh.create_edge(2 * index, 2 * index + 1)
                opposite_edge = to_opposite_edge(edge)
                sub_triangulations_sides.append((edge, opposite_edge))
            offset = 2 * segments_count
            for index in range(triangles_count):
                sub_triangulations_sides.append(create_triangle(
                        mesh,
                        offset + 3 * index,
                        offset + 3 * index + 1,
                        offset + 3 * index + 2
                ))
            for _ in range(ceil_log2(len(sub_triangulations_sides))):
                merge_steps_count = len(sub_triangulations_sides) // 2
                next_sub_triangulations_sides = []
                for step in range(merge_steps_count):
                    next_sub_triangulations_sides.append(merge(
                            mesh,
                            sub_triangulations_sides[2 * step],
                            sub_triangulations_sides[2 * step + 1],
                    ))
                next_sub_triangulations_sides.extend(
                        sub_triangulations_sides[2 * merge_steps_count:])
                sub_triangulations_sides = next_sub_triangulations_sides
            assert len(sub_triangulations_sides) == 1
            left_side, right_side = sub_triangulations_sides[0]
            return cls(left_side, right_side, mesh)

    def to_boundary_edges(self) -> List[QuadEdge]:
        result = []
        if self:
            start = self.left_side
            edge = start
            while True:
                result.append(edge)
                candidate = self.mesh.to_right_from_end(edge)
                if candidate == start:
                    break
                edge = candidate
        return result

    def to_end(self, edge: QuadEdge) -> Point:
        return self.mesh.to_end(edge)

    def to_start(self, edge: QuadEdge) -> Point:
        return self.mesh.to_start(edge)

    def triangles_vertices(self) -> List[Tuple[Point, Point, Point]]:
        return to_triangles_vertices(self.mesh)

    __slots__ = 'left_side', 'mesh', 'right_side'

    def __init__(self, left_side: QuadEdge, right_side: QuadEdge, mesh: Mesh
                 ) -> None:
        self.left_side, self.mesh, self.right_side = (
            left_side, mesh, right_side
        )

    def __bool__(self) -> bool:
        result = bool(self.mesh)
        assert result is (self.left_side != UNDEFINED_EDGE)
        assert result is (self.right_side != UNDEFINED_EDGE)
        return result


def build_base_edge(
        mesh: Mesh,
        first_right_side: QuadEdge,
        second_left_side: QuadEdge,
) -> Tuple[QuadEdge, QuadEdge, QuadEdge]:
    while True:
        if orient_point_to_edge(
                mesh, first_right_side, mesh.to_start(second_left_side)
        ) is Orientation.COUNTERCLOCKWISE:
            first_right_side = mesh.to_left_from_end(first_right_side)
        elif orient_point_to_edge(
                mesh, second_left_side, mesh.to_start(first_right_side)
        ) is Orientation.CLOCKWISE:
            second_left_side = mesh.to_right_from_end(second_left_side)
        else:
            break
    return (first_right_side,
            mesh.connect_edges(to_opposite_edge(second_left_side),
                               first_right_side),
            second_left_side)


def find_left_candidate(mesh: Mesh, base_edge: QuadEdge) -> Optional[QuadEdge]:
    result = mesh.to_left_from_start(to_opposite_edge(base_edge))
    if (orient_point_to_edge(mesh, base_edge, mesh.to_end(result))
            is not Orientation.CLOCKWISE):
        return None
    else:
        while (
                orient_point_to_edge(
                        mesh, base_edge,
                        mesh.to_end(mesh.to_left_from_start(result))
                ) is Orientation.CLOCKWISE
                and
                locate_point_in_point_point_point_circle(
                        mesh.to_end(mesh.to_left_from_start(result)),
                        mesh.to_end(base_edge),
                        mesh.to_start(base_edge),
                        mesh.to_end(result)
                ) is Location.INTERIOR
        ):
            next_candidate = mesh.to_left_from_start(result)
            mesh.delete_edge(result)
            result = next_candidate
    return result


def find_right_candidate(mesh: Mesh, base_edge: QuadEdge
                         ) -> Optional[QuadEdge]:
    result = mesh.to_right_from_start(base_edge)
    if (orient_point_to_edge(mesh, base_edge, mesh.to_end(result))
            is not Orientation.CLOCKWISE):
        return None
    else:
        while (
                orient_point_to_edge(
                        mesh, base_edge,
                        mesh.to_end(mesh.to_right_from_start(result))
                ) is Orientation.CLOCKWISE
                and
                locate_point_in_point_point_point_circle(
                        mesh.to_end(mesh.to_right_from_start(result)),
                        mesh.to_end(base_edge),
                        mesh.to_start(base_edge),
                        mesh.to_end(result)
                ) is Location.INTERIOR
        ):
            next_candidate = mesh.to_right_from_start(result)
            mesh.delete_edge(result)
            result = next_candidate
    return result


def merge(mesh: Mesh,
          first_sides: Tuple[QuadEdge, QuadEdge],
          second_sides: Tuple[QuadEdge, QuadEdge]
          ) -> Tuple[QuadEdge, QuadEdge]:
    first_left_side, first_right_side = first_sides
    second_left_side, second_right_side = second_sides
    first_right_side, base_edge, second_left_side = build_base_edge(
            mesh, first_right_side, second_left_side
    )
    rise_bubble(mesh, base_edge)
    left_side = (
        to_opposite_edge(base_edge)
        if mesh.to_start(first_left_side) == mesh.to_start(first_right_side)
        else first_left_side
    )
    right_side = (
        base_edge
        if (mesh.to_start(second_left_side)
            == mesh.to_start(second_right_side))
        else second_right_side
    )
    return left_side, right_side


def orient_point_to_edge(mesh: Mesh, base_edge: QuadEdge, point: Point
                         ) -> Orientation:
    return orient(mesh.to_start(base_edge), mesh.to_end(base_edge), point)


def rise_bubble(mesh: Mesh, base_edge: QuadEdge) -> None:
    while True:
        left_candidate, right_candidate = (
            find_left_candidate(mesh, base_edge),
            find_right_candidate(mesh, base_edge)
        )
        if left_candidate is not None:
            if right_candidate is not None:
                if locate_point_in_point_point_point_circle(
                        mesh.to_end(right_candidate),
                        mesh.to_end(left_candidate),
                        mesh.to_end(base_edge),
                        mesh.to_start(base_edge)
                ) is Location.INTERIOR:
                    base_edge = mesh.connect_edges(right_candidate,
                                                   to_opposite_edge(base_edge))
                else:
                    base_edge = mesh.connect_edges(
                            to_opposite_edge(base_edge),
                            to_opposite_edge(left_candidate)
                    )
            else:
                base_edge = mesh.connect_edges(
                        to_opposite_edge(base_edge),
                        to_opposite_edge(left_candidate)
                )
        elif right_candidate is not None:
            base_edge = mesh.connect_edges(right_candidate,
                                           to_opposite_edge(base_edge))
        else:
            break


def to_base_cases(points_count: int) -> Tuple[int, int]:
    """
    Searches solution of linear diophantine equation
        2 * segments_count + 3 * triangles_count == points_count
    where `points_count >= 2`
    """
    assert points_count >= 2
    triangles_count, rest_points = divmod(points_count, 3)
    if rest_points == 0:
        return 0, triangles_count
    elif rest_points == 1:
        return 2, triangles_count - 1
    else:
        return 1, triangles_count


def to_triangles_vertices(mesh: Mesh) -> List[Tuple[Point, Point, Point]]:
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
                        mesh, edge, mesh.to_end(mesh.to_left_from_start(edge))
                ) is Orientation.COUNTERCLOCKWISE):
            result.append((first_vertex, second_vertex, third_vertex))
    return result
