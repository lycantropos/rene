from __future__ import annotations

from typing import Any, NoReturn, TYPE_CHECKING

from typing_extensions import Self, final

from rene import MIN_CONTOUR_VERTICES_COUNT, hints
from rene._triangulation.constrained_delaunay import (
    ConstrainedDelaunayTriangulation as _RawConstrainedDelaunayTriangulation,
)
from rene._triangulation.delaunay import (
    DelaunayTriangulation as _RawDelaunayTriangulation,
)
from rene._utils import shrink_collinear_vertices

if TYPE_CHECKING:
    from collections.abc import Sequence

    from rithm.fraction import Fraction

    from rene._context import Context


@final
class ConstrainedDelaunayTriangulation:
    @classmethod
    def from_polygon(cls, polygon: hints.Polygon[Fraction], /) -> Self:
        return cls(
            _RawConstrainedDelaunayTriangulation.from_polygon(
                polygon, cls._context.orient
            )
        )

    @property
    def border(self, /) -> hints.Contour[Fraction]:
        boundary_points = self._raw.to_boundary_points()
        return self._context.contour_cls(
            boundary_points
            if len(boundary_points) < MIN_CONTOUR_VERTICES_COUNT
            else shrink_collinear_vertices(
                boundary_points, self._context.orient
            )
        )

    @property
    def triangles(self, /) -> Sequence[hints.Contour[Fraction]]:
        contour_cls = self._context.contour_cls
        return [
            contour_cls(vertices)
            for vertices in self._raw.triangles_vertices()
        ]

    _context: Context[Fraction]
    _raw: _RawConstrainedDelaunayTriangulation[Fraction]

    __module__ = 'rene.exact'
    __slots__ = ('_raw',)

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(
        cls, raw: _RawConstrainedDelaunayTriangulation[Fraction], /
    ) -> Self:
        self = super().__new__(cls)
        self._raw = raw
        return self

    def __bool__(self) -> bool:
        return bool(self._raw)


class DelaunayTriangulation:
    @classmethod
    def from_points(cls, points: Sequence[hints.Point[Fraction]], /) -> Self:
        return cls(
            _RawDelaunayTriangulation.from_points(points, cls._context.orient)
        )

    @property
    def border(self, /) -> hints.Contour[Fraction]:
        boundary_points = self._raw.to_boundary_points()
        return self._context.contour_cls(
            boundary_points
            if len(boundary_points) < MIN_CONTOUR_VERTICES_COUNT
            else shrink_collinear_vertices(
                boundary_points, self._context.orient
            )
        )

    @property
    def triangles(self, /) -> Sequence[hints.Contour[Fraction]]:
        contour_cls = self._context.contour_cls
        return [
            contour_cls(vertices)
            for vertices in self._raw.triangles_vertices()
        ]

    _context: Context[Fraction]
    _raw: _RawDelaunayTriangulation[Fraction]

    __module__ = 'rene.exact'
    __slots__ = ('_raw',)

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(cls, raw: _RawDelaunayTriangulation[Fraction], /) -> Self:
        self = super().__new__(cls)
        self._raw = raw
        return self

    def __bool__(self) -> bool:
        return bool(self._raw)
