from rene._delaunay.triangulation import Triangulation as RawTriangulation
from rene._rene import MIN_CONTOUR_VERTICES_COUNT
from rene._utils import shrink_collinear_vertices
from .contour import Contour


class Triangulation:
    @classmethod
    def delaunay(cls, points):
        return cls(RawTriangulation.delaunay(points))

    def boundary(self):
        vertices = [self._raw.to_start(edge)
                    for edge in self._raw.to_boundary_edges()]
        return Contour(vertices
                       if len(vertices) < MIN_CONTOUR_VERTICES_COUNT
                       else shrink_collinear_vertices(vertices))

    def triangles(self):
        return [Contour(vertices)
                for vertices in self._raw.triangles_vertices()]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __init__(self, _raw: RawTriangulation) -> None:
        self._raw = _raw
