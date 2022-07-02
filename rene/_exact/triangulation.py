from rene._delaunay.triangulation import Triangulation as RawTriangulation
from rene._utils import shrink_collinear_vertices
from .contour import Contour


class Triangulation:
    @classmethod
    def delaunay(cls, points):
        return cls(RawTriangulation.delaunay(points))

    def boundary(self):
        boundary_points = self._raw.to_boundary_points()
        return Contour(shrink_collinear_vertices(boundary_points)
                       if boundary_points
                       else boundary_points)

    def triangles(self):
        return [Contour(vertices)
                for vertices in self._raw.triangles_vertices()]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __init__(self, _raw: RawTriangulation) -> None:
        self._raw = _raw
