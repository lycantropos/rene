from rene._delaunay.triangulation import Triangulation as RawTriangulation
from rene._rene import MIN_CONTOUR_VERTICES_COUNT
from rene._utils import shrink_collinear_vertices
from .contour import Contour


class Triangulation:
    @classmethod
    def delaunay(cls, points):
        return cls(RawTriangulation.delaunay(points))

    def boundary(self):
        boundary_points = self._raw.to_boundary_points()
        return Contour(boundary_points
                       if boundary_points < MIN_CONTOUR_VERTICES_COUNT
                       else shrink_collinear_vertices(boundary_points))

    def triangles(self):
        return [Contour(vertices)
                for vertices in self._raw.triangles_vertices()]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __bool__(self):
        return bool(self._raw)

    def __init__(self, _raw: RawTriangulation) -> None:
        self._raw = _raw
