from __future__ import annotations

from typing import Generic, TYPE_CHECKING

from typing_extensions import Self

from rene import hints
from rene._utils import deduplicate
from rene.enums import Orientation

from .mesh import Mesh, build_delaunay_triangulation, orient_point_to_edge
from .quad_edge import QuadEdge, to_opposite_edge

if TYPE_CHECKING:
    from collections.abc import Sequence

    from rene._hints import Orienteer


class DelaunayTriangulation(Generic[hints.ScalarT]):
    @classmethod
    def from_points(
        cls,
        points: Sequence[hints.Point[hints.ScalarT]],
        orienteer: Orienteer[hints.ScalarT],
        /,
    ) -> Self:
        endpoints = list(points)
        endpoints.sort()
        mesh = Mesh.from_points(deduplicate(endpoints))
        left_side, right_side = build_delaunay_triangulation(mesh, orienteer)
        return cls(left_side, right_side, mesh, orienteer)

    @property
    def left_side(self, /) -> QuadEdge:
        return self._left_side

    @property
    def mesh(self, /) -> Mesh[hints.ScalarT]:
        return self._mesh

    @property
    def right_side(self, /) -> QuadEdge:
        return self._right_side

    def to_boundary_points(self, /) -> list[hints.Point[hints.ScalarT]]:
        if self:
            result = []
            start = self.left_side
            edge = start
            while True:
                result.append(self.mesh.to_start(edge))
                candidate = self.mesh.to_right_from_end(edge)
                if candidate == start:
                    break
                edge = candidate
            return result
        return self.mesh.endpoints

    def triangles_vertices(
        self,
    ) -> list[
        tuple[
            hints.Point[hints.ScalarT],
            hints.Point[hints.ScalarT],
            hints.Point[hints.ScalarT],
        ]
    ]:
        mesh = self.mesh
        result = []
        for edge in mesh.to_edges():
            first_vertex = mesh.to_start(edge)
            second_vertex = mesh.to_end(edge)
            third_vertex = mesh.to_end(mesh.to_left_from_start(edge))
            if (
                first_vertex < second_vertex
                and first_vertex < third_vertex
                and (
                    third_vertex
                    == mesh.to_end(
                        mesh.to_right_from_start(to_opposite_edge(edge))
                    )
                )
                and (
                    orient_point_to_edge(
                        mesh, edge, third_vertex, self._orienteer
                    )
                    is Orientation.COUNTERCLOCKWISE
                )
            ):
                result.append((first_vertex, second_vertex, third_vertex))
        return result

    _left_side: QuadEdge
    _mesh: Mesh[hints.ScalarT]
    _orienteer: Orienteer[hints.ScalarT]
    _right_side: QuadEdge

    __slots__ = '_left_side', '_mesh', '_orienteer', '_right_side'

    def __new__(
        cls,
        left_side: QuadEdge,
        right_side: QuadEdge,
        mesh: Mesh[hints.ScalarT],
        orienteer: Orienteer[hints.ScalarT],
        /,
    ) -> Self:
        self = super().__new__(cls)
        self._left_side, self._mesh, self._orienteer, self._right_side = (
            left_side,
            mesh,
            orienteer,
            right_side,
        )
        return self
