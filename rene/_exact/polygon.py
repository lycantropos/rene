from __future__ import annotations

import typing as t
from itertools import chain

import typing_extensions as te
from reprit.base import generate_repr
from rithm.fraction import Fraction

from rene import (Location,
                  hints)
from rene._clipping import (intersect_polygon_with_multipolygon,
                            intersect_polygons,
                            subtract_multipolygon_from_polygon,
                            subtract_polygons,
                            symmetric_subtract_polygon_with_multipolygon,
                            symmetric_subtract_polygons,
                            unite_polygon_with_multipolygon,
                            unite_polygons)
from rene._context import Context
from rene._utils import (collect_maybe_empty_polygons,
                         collect_non_empty_polygons,
                         locate_point_in_region)


class Polygon:
    @property
    def border(self) -> hints.Contour[Fraction]:
        return self._border

    @property
    def bounding_box(self) -> hints.Box[Fraction]:
        return self._border.bounding_box

    @property
    def holes(self) -> t.Sequence[hints.Contour[Fraction]]:
        return self._holes[:]

    @property
    def holes_count(self) -> int:
        return len(self._holes)

    @property
    def segments(self) -> t.Sequence[hints.Segment[Fraction]]:
        return list(chain(self._border.segments,
                          chain.from_iterable(hole.segments
                                              for hole in self._holes)))

    @property
    def segments_count(self) -> int:
        return sum([hole.segments_count for hole in self._holes],
                   self._border.segments_count)

    def locate(self, point: hints.Point[Fraction], /) -> Location:
        location_without_holes = locate_point_in_region(self._border, point)
        if location_without_holes is Location.INTERIOR:
            for hole in self._holes:
                location_in_hole = locate_point_in_region(hole, point)
                if location_in_hole is Location.INTERIOR:
                    return Location.EXTERIOR
                elif location_in_hole is Location.BOUNDARY:
                    return Location.BOUNDARY
        return location_without_holes

    _context: t.ClassVar[Context[Fraction]]
    _border: hints.Contour[Fraction]
    _holes: t.List[hints.Contour[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_border', '_holes'

    def __new__(cls,
                border: hints.Contour[Fraction],
                holes: t.Sequence[hints.Contour[Fraction]],
                /) -> te.Self:
        self = super().__new__(cls)
        self._border, self._holes = border, list(holes)
        return self

    @t.overload
    def __and__(self, other: hints.Empty[Fraction], /) -> hints.Empty[
        Fraction]:
        ...

    @t.overload
    def __and__(
            self, other: hints.Multipolygon[Fraction], /
    ) -> t.Union[
        hints.Empty[Fraction], hints.Multipolygon[Fraction],
        hints.Polygon[Fraction]
    ]:
        ...

    @t.overload
    def __and__(
            self, other: hints.Polygon[Fraction], /
    ) -> t.Union[
        hints.Empty[Fraction], hints.Multipolygon[Fraction],
        hints.Polygon[Fraction]
    ]:
        ...

    @t.overload
    def __and__(self, other: t.Any, /) -> t.Any:
        ...

    def __and__(self, other: t.Any, /) -> t.Any:
        return (
            other
            if isinstance(other, self._context.empty_cls)
            else (
                collect_maybe_empty_polygons(
                        intersect_polygon_with_multipolygon(self, other),
                        self._context.empty_cls, self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            intersect_polygons(self, other),
                            self._context.empty_cls,
                            self._context.multipolygon_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else NotImplemented
                )
            )
        )

    def __contains__(self, point: hints.Point[Fraction], /) -> bool:
        return self.locate(point) is not Location.EXTERIOR

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        ...

    def __eq__(self, other: t.Any, /) -> t.Any:
        return ((self._border == other.border
                 and len(self._holes) == len(other.holes)
                 and frozenset(self._holes) == frozenset(other.holes))
                if isinstance(other, self._context.polygon_cls)
                else NotImplemented)

    def __hash__(self) -> int:
        return hash((self._border, frozenset(self._holes)))

    @t.overload
    def __or__(self, other: hints.Empty[Fraction], /) -> te.Self:
        ...

    @t.overload
    def __or__(
            self, other: hints.Multipolygon[Fraction], /
    ) -> t.Union[hints.Multipolygon[Fraction], hints.Polygon[Fraction]]:
        ...

    @t.overload
    def __or__(
            self, other: hints.Polygon[Fraction], /
    ) -> t.Union[hints.Multipolygon[Fraction], hints.Polygon[Fraction]]:
        ...

    @t.overload
    def __or__(self, other: t.Any, /) -> t.Any:
        ...

    def __or__(self, other: t.Any, /) -> t.Any:
        return (
            self
            if isinstance(other, self._context.empty_cls)
            else (
                collect_non_empty_polygons(
                        unite_polygon_with_multipolygon(self, other),
                        self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else
                (
                    collect_non_empty_polygons(unite_polygons(self, other),
                                               self._context.multipolygon_cls)
                    if isinstance(other, Polygon)
                    else NotImplemented
                )
            )
        )

    __repr__ = generate_repr(__new__)

    def __str__(self) -> str:
        return (f'{type(self).__qualname__}({self.border}, [{{}}])'
                .format(', '.join(map(str, self.holes))))

    @t.overload
    def __sub__(self, other: hints.Empty[Fraction], /) -> te.Self:
        ...

    @t.overload
    def __sub__(
            self, other: hints.Multipolygon[Fraction], /
    ) -> t.Union[
        hints.Empty[Fraction], hints.Multipolygon[Fraction],
        hints.Polygon[Fraction]
    ]:
        ...

    @t.overload
    def __sub__(
            self, other: hints.Polygon[Fraction], /
    ) -> t.Union[
        hints.Empty[Fraction], hints.Multipolygon[Fraction],
        hints.Polygon[Fraction]
    ]:
        ...

    @t.overload
    def __sub__(self, other: t.Any, /) -> t.Any:
        ...

    def __sub__(self, other: t.Any, /) -> t.Any:
        return (
            self
            if isinstance(other, self._context.empty_cls)
            else (
                collect_maybe_empty_polygons(
                        subtract_multipolygon_from_polygon(self, other),
                        self._context.empty_cls, self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            subtract_polygons(self, other),
                            self._context.empty_cls,
                            self._context.multipolygon_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else NotImplemented
                )
            )
        )

    @t.overload
    def __xor__(self, other: hints.Empty[Fraction], /) -> te.Self:
        ...

    @t.overload
    def __xor__(
            self, other: hints.Multipolygon[Fraction], /
    ) -> t.Union[
        hints.Empty[Fraction], hints.Multipolygon[Fraction],
        hints.Polygon[Fraction]
    ]:
        ...

    @t.overload
    def __xor__(
            self, other: hints.Polygon[Fraction], /
    ) -> t.Union[
        hints.Empty[Fraction], hints.Multipolygon[Fraction],
        hints.Polygon[Fraction]
    ]:
        ...

    @t.overload
    def __xor__(self, other: t.Any, /) -> t.Any:
        ...

    def __xor__(self, other: t.Any, /) -> t.Any:
        return (
            self
            if isinstance(other, self._context.empty_cls)
            else (
                collect_maybe_empty_polygons(
                        symmetric_subtract_polygon_with_multipolygon(self,
                                                                     other),
                        self._context.empty_cls, self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            symmetric_subtract_polygons(self, other),
                            self._context.empty_cls,
                            self._context.multipolygon_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else NotImplemented
                )
            )
        )
