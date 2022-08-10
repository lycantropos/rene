from rene._rene import MIN_CONTOUR_VERTICES_COUNT
from rene._triangulation.constrained_delaunay import (
    ConstrainedDelaunayTriangulation as _RawConstrainedDelaunayTriangulation
)
from rene._triangulation.delaunay import (
    DelaunayTriangulation as _RawDelaunayTriangulation
)
from rene._utils import shrink_collinear_vertices
from .contour import Contour


class ConstrainedDelaunayTriangulation:
    @classmethod
    def from_polygon(cls, polygon):
        return cls(_RawConstrainedDelaunayTriangulation.from_polygon(polygon))

    @property
    def border(self):
        boundary_points = self._raw.to_boundary_points()
        return Contour(boundary_points
                       if len(boundary_points) < MIN_CONTOUR_VERTICES_COUNT
                       else shrink_collinear_vertices(boundary_points))

    @property
    def triangles(self):
        return [Contour(vertices)
                for vertices in self._raw.triangles_vertices()]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __bool__(self):
        return bool(self._raw)

    def __init__(self, _raw: _RawConstrainedDelaunayTriangulation) -> None:
        self._raw = _raw


class DelaunayTriangulation:
    @classmethod
    def from_points(cls, points):
        return cls(_RawDelaunayTriangulation.from_points(points))

    @property
    def border(self):
        boundary_points = self._raw.to_boundary_points()
        return Contour(boundary_points
                       if len(boundary_points) < MIN_CONTOUR_VERTICES_COUNT
                       else shrink_collinear_vertices(boundary_points))

    @property
    def triangles(self):
        return [Contour(vertices)
                for vertices in self._raw.triangles_vertices()]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __bool__(self):
        return bool(self._raw)

    def __init__(self, _raw: _RawDelaunayTriangulation) -> None:
        self._raw = _raw
