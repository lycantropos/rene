from __future__ import annotations

import typing as t

import typing_extensions as te
from rithm.fraction import Fraction

from rene import (MIN_CONTOUR_VERTICES_COUNT,
                  hints)
from rene._context import Context
from rene._triangulation.constrained_delaunay import (
    ConstrainedDelaunayTriangulation as _RawConstrainedDelaunayTriangulation
)
from rene._triangulation.delaunay import (
    DelaunayTriangulation as _RawDelaunayTriangulation
)
from rene._utils import shrink_collinear_vertices


@te.final
class ConstrainedDelaunayTriangulation:
    @classmethod
    def from_polygon(cls, polygon: hints.Polygon[Fraction], /) -> te.Self:
        return cls(_RawConstrainedDelaunayTriangulation.from_polygon(polygon))

    @property
    def border(self) -> hints.Contour[Fraction]:
        boundary_points = self._raw.to_boundary_points()
        return self._context.contour_cls(
                boundary_points
                if len(boundary_points) < MIN_CONTOUR_VERTICES_COUNT
                else shrink_collinear_vertices(boundary_points)
        )

    @property
    def triangles(self) -> t.Sequence[hints.Contour[Fraction]]:
        contour_cls = self._context.contour_cls
        return [contour_cls(vertices)
                for vertices in self._raw.triangles_vertices()]

    _context: Context[Fraction]
    _raw: _RawConstrainedDelaunayTriangulation[Fraction]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(
            cls, raw: _RawConstrainedDelaunayTriangulation[Fraction], /
    ) -> te.Self:
        self = super().__new__(cls)
        self._raw = raw
        return self

    def __bool__(self) -> bool:
        return bool(self._raw)


class DelaunayTriangulation:
    @classmethod
    def from_points(
            cls, points: t.Sequence[hints.Point[Fraction]], /
    ) -> te.Self:
        return cls(_RawDelaunayTriangulation.from_points(points))

    @property
    def border(self) -> hints.Contour[Fraction]:
        boundary_points = self._raw.to_boundary_points()
        return self._context.contour_cls(
                boundary_points
                if len(boundary_points) < MIN_CONTOUR_VERTICES_COUNT
                else shrink_collinear_vertices(boundary_points)
        )

    @property
    def triangles(self) -> t.Sequence[hints.Contour[Fraction]]:
        contour_cls = self._context.contour_cls
        return [contour_cls(vertices)
                for vertices in self._raw.triangles_vertices()]

    _context: Context[Fraction]
    _raw: _RawDelaunayTriangulation[Fraction]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(cls, raw: _RawDelaunayTriangulation[Fraction], /) -> te.Self:
        self = super().__new__(cls)
        self._raw = raw
        return self

    def __bool__(self) -> bool:
        return bool(self._raw)
