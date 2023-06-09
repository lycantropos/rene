from __future__ import annotations

import typing as _t

import typing_extensions as _te
from rithm.fraction import Fraction as _Fraction

from rene._rene import MIN_CONTOUR_VERTICES_COUNT
from rene._triangulation.constrained_delaunay import (
    ConstrainedDelaunayTriangulation as _RawConstrainedDelaunayTriangulation
)
from rene._triangulation.delaunay import (
    DelaunayTriangulation as _RawDelaunayTriangulation
)
from rene._utils import shrink_collinear_vertices
from .contour import Contour
from .point import Point
from .polygon import Polygon


class ConstrainedDelaunayTriangulation:
    @classmethod
    def from_polygon(cls,
                     polygon: Polygon) -> ConstrainedDelaunayTriangulation:
        return cls(_RawConstrainedDelaunayTriangulation.from_polygon(polygon))

    @property
    def border(self) -> Contour:
        boundary_points = self._raw.to_boundary_points()
        return Contour(boundary_points
                       if len(boundary_points) < MIN_CONTOUR_VERTICES_COUNT
                       else shrink_collinear_vertices(boundary_points))

    @property
    def triangles(self) -> _t.Sequence[Contour]:
        return [Contour(vertices)
                for vertices in self._raw.triangles_vertices()]

    _raw: _RawConstrainedDelaunayTriangulation[_Fraction]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __bool__(self) -> bool:
        return bool(self._raw)

    def __new__(
            cls, _raw: _RawConstrainedDelaunayTriangulation[_Fraction]
    ) -> _te.Self:
        self = super().__new__(cls)
        self._raw = _raw
        return self


class DelaunayTriangulation:
    @classmethod
    def from_points(cls, points: _t.Sequence[Point]) -> _te.Self:
        return cls(_RawDelaunayTriangulation.from_points(points))

    @property
    def border(self) -> Contour:
        boundary_points = self._raw.to_boundary_points()
        return Contour(boundary_points
                       if len(boundary_points) < MIN_CONTOUR_VERTICES_COUNT
                       else shrink_collinear_vertices(boundary_points))

    @property
    def triangles(self) -> _t.Sequence[Contour]:
        return [Contour(vertices)
                for vertices in self._raw.triangles_vertices()]

    _raw: _RawDelaunayTriangulation[_Fraction]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __bool__(self) -> bool:
        return bool(self._raw)

    def __new__(cls, _raw: _RawDelaunayTriangulation[_Fraction]) -> _te.Self:
        self = super().__new__(cls)
        self._raw = _raw
        return self
