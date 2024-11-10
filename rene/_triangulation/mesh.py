from __future__ import annotations

import typing as t

import typing_extensions as te

from rene import Location, Orientation, hints
from rene._hints import Orienteer
from rene._utils import (
    ceil_log2,
    is_even,
    locate_point_in_point_point_point_circle,
)

from .quad_edge import (
    QuadEdge,
    UNDEFINED_EDGE,
    to_opposite_edge,
    to_rotated_edge,
)


class Mesh(t.Generic[hints.Scalar]):
    endpoints: list[hints.Point[hints.Scalar]]
    left_from_start: list[QuadEdge]
    starts_indices: list[int]

    @classmethod
    def from_points(cls, endpoints: list[hints.Point[hints.Scalar]], /) -> te.Self:
        return cls(endpoints, [], [])

    def connect_edges(self, first: QuadEdge, second: QuadEdge, /) -> QuadEdge:
        result = self.create_edge(self.to_end_index(first), self.to_start_index(second))
        self.splice_edges(result, self.to_left_from_end(first))
        self.splice_edges(to_opposite_edge(result), second)
        return result

    def create_edge(self, start_index: int, end_index: int, /) -> QuadEdge:
        self.starts_indices.append(start_index)
        self.starts_indices.append(end_index)
        edge = QuadEdge(len(self.left_from_start))
        rotated_edge = QuadEdge(edge + 1)
        opposite_edge = QuadEdge(edge + 2)
        triple_rotated_edge = QuadEdge(edge + 3)
        self.left_from_start.append(edge)
        self.left_from_start.append(triple_rotated_edge)
        self.left_from_start.append(opposite_edge)
        self.left_from_start.append(rotated_edge)
        return edge

    def is_deleted_edge(self, edge: QuadEdge, /) -> bool:
        result = self.to_left_from_start(edge) == edge
        assert (self.to_right_from_start(edge) == edge) is result
        return result

    def delete_edge(self, edge: QuadEdge, /) -> None:
        self.splice_edges(edge, self.to_right_from_start(edge))
        opposite_edge = to_opposite_edge(edge)
        self.splice_edges(opposite_edge, self.to_right_from_start(opposite_edge))

    def splice_edges(self, first: QuadEdge, second: QuadEdge, /) -> None:
        alpha = to_rotated_edge(self.to_left_from_start(first))
        beta = to_rotated_edge(self.to_left_from_start(second))
        self.left_from_start[first], self.left_from_start[second] = (
            self.to_left_from_start(second),
            self.to_left_from_start(first),
        )
        self.left_from_start[alpha], self.left_from_start[beta] = (
            self.to_left_from_start(beta),
            self.to_left_from_start(alpha),
        )

    def swap_diagonal(self, edge: QuadEdge, /) -> None:
        """
        Swaps diagonal in a quadrilateral formed by triangles
        in both clockwise and counterclockwise order around the start.
        """
        side = self.to_right_from_start(edge)
        opposite = to_opposite_edge(edge)
        opposite_side = self.to_right_from_start(opposite)
        self.splice_edges(edge, side)
        self.splice_edges(opposite, opposite_side)
        self.splice_edges(edge, self.to_left_from_end(side))
        self.splice_edges(opposite, self.to_left_from_end(opposite_side))
        self.starts_indices[edge // 2] = self.to_end_index(side)
        self.starts_indices[opposite // 2] = self.to_end_index(opposite_side)
        assert self.to_start(edge) == self.to_end(side)
        assert self.to_end(edge) == self.to_end(opposite_side)

    def to_edges(self) -> t.Iterable[QuadEdge]:
        candidates = [
            QuadEdge(index) for index in range(0, len(self.left_from_start), 2)
        ]
        return [
            candidate for candidate in candidates if not self.is_deleted_edge(candidate)
        ]

    def to_end(self, edge: QuadEdge, /) -> hints.Point[hints.Scalar]:
        """
        aka "Dest" in L. Guibas and J. Stolfi notation.
        """
        return self.endpoints[self.to_end_index(edge)]

    def to_end_index(self, edge: QuadEdge, /) -> int:
        return self.to_start_index(to_opposite_edge(edge))

    def to_left_from_end(self, edge: QuadEdge, /) -> QuadEdge:
        """
        aka "Lnext" in L. Guibas and J. Stolfi notation.
        """
        return to_rotated_edge(
            self.to_left_from_start(to_opposite_edge(to_rotated_edge(edge)))
        )

    def to_left_from_start(self, edge: QuadEdge, /) -> QuadEdge:
        """
        aka "Onext" in L. Guibas and J. Stolfi notation.
        """
        return self.left_from_start[edge]

    def to_right_from_end(self, edge: QuadEdge, /) -> QuadEdge:
        """
        aka "Rprev" in L. Guibas and J. Stolfi notation.
        """
        return self.to_left_from_start(to_opposite_edge(edge))

    def to_right_from_start(self, edge: QuadEdge, /) -> QuadEdge:
        """
        aka "Oprev" in L. Guibas and J. Stolfi notation.
        """
        return to_rotated_edge(self.to_left_from_start(to_rotated_edge(edge)))

    def to_start(self, edge: QuadEdge, /) -> hints.Point[hints.Scalar]:
        """
        aka "Org" in L. Guibas and J. Stolfi notation.
        """
        return self.endpoints[self.to_start_index(edge)]

    def to_start_index(self, edge: QuadEdge, /) -> int:
        assert is_even(edge)
        return self.starts_indices[edge // 2]

    def to_unique_edges(self) -> t.Iterable[QuadEdge]:
        candidates = [
            QuadEdge(index) for index in range(0, len(self.left_from_start), 4)
        ]
        return [
            candidate for candidate in candidates if not self.is_deleted_edge(candidate)
        ]

    __slots__ = "endpoints", "left_from_start", "starts_indices"

    def __bool__(self) -> bool:
        return bool(self.left_from_start)

    def __init__(
        self,
        endpoints: list[hints.Point[hints.Scalar]],
        left_from_start: list[QuadEdge],
        starts_indices: list[int],
        /,
    ) -> None:
        self.endpoints, self.left_from_start, self.starts_indices = (
            endpoints,
            left_from_start,
            starts_indices,
        )


def build_base_edge(
    mesh: Mesh[hints.Scalar],
    first_right_side: QuadEdge,
    second_left_side: QuadEdge,
    orienteer: Orienteer[hints.Scalar],
    /,
) -> tuple[QuadEdge, QuadEdge, QuadEdge]:
    while True:
        if (
            orient_point_to_edge(
                mesh,
                first_right_side,
                mesh.to_start(second_left_side),
                orienteer,
            )
            is Orientation.COUNTERCLOCKWISE
        ):
            first_right_side = mesh.to_left_from_end(first_right_side)
        elif (
            orient_point_to_edge(
                mesh,
                second_left_side,
                mesh.to_start(first_right_side),
                orienteer,
            )
            is Orientation.CLOCKWISE
        ):
            second_left_side = mesh.to_right_from_end(second_left_side)
        else:
            break
    return (
        first_right_side,
        mesh.connect_edges(to_opposite_edge(second_left_side), first_right_side),
        second_left_side,
    )


def build_delaunay_triangulation(
    mesh: Mesh[hints.Scalar], orienteer: Orienteer[hints.Scalar], /
) -> tuple[QuadEdge, QuadEdge]:
    if len(mesh.endpoints) < 2:
        left_side = right_side = UNDEFINED_EDGE
    else:
        segments_count, triangles_count = to_base_cases(len(mesh.endpoints))
        sub_triangulations_sides = []
        for index in range(segments_count):
            edge = mesh.create_edge(2 * index, 2 * index + 1)
            opposite_edge = to_opposite_edge(edge)
            sub_triangulations_sides.append((edge, opposite_edge))
        offset = 2 * segments_count
        for index in range(triangles_count):
            sub_triangulations_sides.append(
                create_triangle(
                    mesh,
                    offset + 3 * index,
                    offset + 3 * index + 1,
                    offset + 3 * index + 2,
                    orienteer,
                )
            )
        for _ in range(ceil_log2(len(sub_triangulations_sides))):
            merge_steps_count = len(sub_triangulations_sides) // 2
            next_sub_triangulations_sides = []
            for step in range(merge_steps_count):
                next_sub_triangulations_sides.append(
                    merge(
                        mesh,
                        sub_triangulations_sides[2 * step],
                        sub_triangulations_sides[2 * step + 1],
                        orienteer,
                    )
                )
            next_sub_triangulations_sides.extend(
                sub_triangulations_sides[2 * merge_steps_count :]
            )
            sub_triangulations_sides = next_sub_triangulations_sides
        assert len(sub_triangulations_sides) == 1
        left_side, right_side = sub_triangulations_sides[0]
    return left_side, right_side


def create_triangle(
    mesh: Mesh[hints.Scalar],
    left_point_index: int,
    mid_point_index: int,
    right_point_index: int,
    orienteer: Orienteer[hints.Scalar],
    /,
) -> tuple[QuadEdge, QuadEdge]:
    first_edge = mesh.create_edge(left_point_index, mid_point_index)
    second_edge = mesh.create_edge(mid_point_index, right_point_index)
    mesh.splice_edges(to_opposite_edge(first_edge), second_edge)
    orientation = orient_point_to_edge(
        mesh, first_edge, mesh.to_end(second_edge), orienteer
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


def find_left_candidate(
    mesh: Mesh[hints.Scalar],
    base_edge: QuadEdge,
    orienteer: Orienteer[hints.Scalar],
    /,
) -> QuadEdge | None:
    result = mesh.to_left_from_start(to_opposite_edge(base_edge))
    if (
        orient_point_to_edge(mesh, base_edge, mesh.to_end(result), orienteer)
        is not Orientation.CLOCKWISE
    ):
        return None
    else:
        while (
            orient_point_to_edge(
                mesh,
                base_edge,
                mesh.to_end(mesh.to_left_from_start(result)),
                orienteer,
            )
            is Orientation.CLOCKWISE
            and locate_point_in_point_point_point_circle(
                mesh.to_end(mesh.to_left_from_start(result)),
                mesh.to_end(base_edge),
                mesh.to_start(base_edge),
                mesh.to_end(result),
            )
            is Location.INTERIOR
        ):
            next_candidate = mesh.to_left_from_start(result)
            mesh.delete_edge(result)
            result = next_candidate
    return result


def find_right_candidate(
    mesh: Mesh[hints.Scalar],
    base_edge: QuadEdge,
    orienteer: Orienteer[hints.Scalar],
    /,
) -> QuadEdge | None:
    result = mesh.to_right_from_start(base_edge)
    if (
        orient_point_to_edge(mesh, base_edge, mesh.to_end(result), orienteer)
        is not Orientation.CLOCKWISE
    ):
        return None
    else:
        while (
            orient_point_to_edge(
                mesh,
                base_edge,
                mesh.to_end(mesh.to_right_from_start(result)),
                orienteer,
            )
            is Orientation.CLOCKWISE
            and locate_point_in_point_point_point_circle(
                mesh.to_end(mesh.to_right_from_start(result)),
                mesh.to_end(base_edge),
                mesh.to_start(base_edge),
                mesh.to_end(result),
            )
            is Location.INTERIOR
        ):
            next_candidate = mesh.to_right_from_start(result)
            mesh.delete_edge(result)
            result = next_candidate
    return result


def merge(
    mesh: Mesh[hints.Scalar],
    first_sides: tuple[QuadEdge, QuadEdge],
    second_sides: tuple[QuadEdge, QuadEdge],
    orienteer: Orienteer[hints.Scalar],
    /,
) -> tuple[QuadEdge, QuadEdge]:
    first_left_side, first_right_side = first_sides
    second_left_side, second_right_side = second_sides
    first_right_side, base_edge, second_left_side = build_base_edge(
        mesh, first_right_side, second_left_side, orienteer
    )
    rise_bubble(mesh, base_edge, orienteer)
    left_side = (
        to_opposite_edge(base_edge)
        if mesh.to_start(first_left_side) == mesh.to_start(first_right_side)
        else first_left_side
    )
    right_side = (
        base_edge
        if (mesh.to_start(second_left_side) == mesh.to_start(second_right_side))
        else second_right_side
    )
    return left_side, right_side


def orient_point_to_edge(
    mesh: Mesh[hints.Scalar],
    base_edge: QuadEdge,
    point: hints.Point[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    /,
) -> Orientation:
    return orienteer(mesh.to_start(base_edge), mesh.to_end(base_edge), point)


def rise_bubble(
    mesh: Mesh[hints.Scalar],
    base_edge: QuadEdge,
    orienteer: Orienteer[hints.Scalar],
    /,
) -> None:
    while True:
        left_candidate, right_candidate = (
            find_left_candidate(mesh, base_edge, orienteer),
            find_right_candidate(mesh, base_edge, orienteer),
        )
        if left_candidate is not None:
            if right_candidate is not None:
                if (
                    locate_point_in_point_point_point_circle(
                        mesh.to_end(right_candidate),
                        mesh.to_end(left_candidate),
                        mesh.to_end(base_edge),
                        mesh.to_start(base_edge),
                    )
                    is Location.INTERIOR
                ):
                    base_edge = mesh.connect_edges(
                        right_candidate, to_opposite_edge(base_edge)
                    )
                else:
                    base_edge = mesh.connect_edges(
                        to_opposite_edge(base_edge),
                        to_opposite_edge(left_candidate),
                    )
            else:
                base_edge = mesh.connect_edges(
                    to_opposite_edge(base_edge),
                    to_opposite_edge(left_candidate),
                )
        elif right_candidate is not None:
            base_edge = mesh.connect_edges(right_candidate, to_opposite_edge(base_edge))
        else:
            break


def to_base_cases(points_count: int, /) -> tuple[int, int]:
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
