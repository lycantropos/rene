from __future__ import annotations

import typing as t
from itertools import chain

import typing_extensions as te
from reprit.base import generate_repr
from rithm.fraction import Fraction

from rene import (MIN_MULTIPOLYGON_POLYGONS_COUNT,
                  Location,
                  hints)
from rene._clipping import (intersect_polygons_with_polygon,
                            intersect_polygons_with_polygons,
                            subtract_polygon_from_polygons,
                            subtract_polygons_from_polygons,
                            symmetric_subtract_polygon_from_polygons,
                            symmetric_subtract_polygons_from_polygons,
                            unite_polygons_with_polygon,
                            unite_polygons_with_polygons)
from rene._context import Context
from rene._utils import (collect_maybe_empty_polygons,
                         collect_non_empty_polygons)


class Multipolygon:
    _context: t.ClassVar[Context[Fraction]]

    @property
    def bounding_box(self) -> hints.Box[Fraction]:
        polygons = iter(self._polygons)
        first_polygon_bounding_box = next(polygons).bounding_box
        min_x, max_x, min_y, max_y = (first_polygon_bounding_box.min_x,
                                      first_polygon_bounding_box.max_x,
                                      first_polygon_bounding_box.min_y,
                                      first_polygon_bounding_box.max_y)
        for polygon in polygons:
            polygon_bounding_box = polygon.bounding_box
            if polygon_bounding_box.max_x > max_x:
                max_x = polygon_bounding_box.max_x
            if polygon_bounding_box.min_x < min_x:
                min_x = polygon_bounding_box.min_x
            if polygon_bounding_box.max_y > max_y:
                max_y = polygon_bounding_box.max_y
            if polygon_bounding_box.min_y < min_y:
                min_y = polygon_bounding_box.min_y
        return self._context.box_cls(min_x, max_x, min_y, max_y)

    @property
    def polygons(self) -> t.Sequence[hints.Polygon[Fraction]]:
        return self._polygons[:]

    @property
    def polygons_count(self) -> int:
        return len(self._polygons)

    @property
    def segments(self) -> t.Sequence[hints.Segment[Fraction]]:
        return list(chain.from_iterable(polygon.segments
                                        for polygon in self._polygons))

    @property
    def segments_count(self) -> int:
        return sum(polygon.segments_count for polygon in self._polygons)

    def locate(self, point: hints.Point[Fraction], /) -> Location:
        for polygon in self._polygons:
            location = polygon.locate(point)
            if location is not Location.EXTERIOR:
                return location
        return Location.EXTERIOR

    _polygons: t.Sequence[hints.Polygon[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_polygons',

    def __new__(
            cls, polygons: t.Sequence[hints.Polygon[Fraction]], /
    ) -> te.Self:
        if len(polygons) < MIN_MULTIPOLYGON_POLYGONS_COUNT:
            raise ValueError('Multipolygon should have at least '
                             f'{MIN_MULTIPOLYGON_POLYGONS_COUNT} polygons, '
                             f'but found {len(polygons)}.')
        self = super().__new__(cls)
        self._polygons = list(polygons)
        return self

    @t.overload
    def __and__(
            self, other: hints.Empty[Fraction], /
    ) -> hints.Empty[Fraction]:
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
            self._context.empty_cls()
            if isinstance(other, self._context.empty_cls)
            else (
                collect_maybe_empty_polygons(
                        intersect_polygons_with_polygons(
                                self.polygons, other.polygons,
                                self._context.contour_cls,
                                self._context.polygon_cls,
                                self._context.segment_cls
                        ),
                        self._context.empty_cls,
                        self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            intersect_polygons_with_polygon(
                                    self.polygons, other,
                                    self._context.contour_cls,
                                    self._context.polygon_cls,
                                    self._context.segment_cls
                            ),
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
        return (frozenset(self.polygons) == frozenset(other.polygons)
                if isinstance(other, self._context.multipolygon_cls)
                else NotImplemented)

    def __hash__(self) -> int:
        return hash(frozenset(self.polygons))

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
                        unite_polygons_with_polygons(
                                self.polygons, other.polygons,
                                self._context.contour_cls,
                                self._context.polygon_cls,
                                self._context.segment_cls
                        ),
                        self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_non_empty_polygons(
                            unite_polygons_with_polygon(
                                    self.polygons, other,
                                    self._context.contour_cls,
                                    self._context.polygon_cls,
                                    self._context.segment_cls
                            ),
                            self._context.multipolygon_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else NotImplemented
                )
            )
        )

    __repr__ = generate_repr(__new__)

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
                        subtract_polygons_from_polygons(
                                self.polygons, other.polygons,
                                self._context.contour_cls,
                                self._context.polygon_cls,
                                self._context.segment_cls
                        ),
                        self._context.empty_cls, self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            subtract_polygon_from_polygons(
                                    self.polygons, other,
                                    self._context.contour_cls,
                                    self._context.polygon_cls,
                                    self._context.segment_cls
                            ),
                            self._context.empty_cls,
                            self._context.multipolygon_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else NotImplemented
                )
            )
        )

    def __str__(self) -> str:
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.polygons))))

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
                        symmetric_subtract_polygons_from_polygons(
                                self.polygons, other.polygons,
                                self._context.contour_cls,
                                self._context.polygon_cls,
                                self._context.segment_cls
                        ),
                        self._context.empty_cls,
                        self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            symmetric_subtract_polygon_from_polygons(
                                    self.polygons, other,
                                    self._context.contour_cls,
                                    self._context.polygon_cls,
                                    self._context.segment_cls
                            ),
                            self._context.empty_cls,
                            self._context.multipolygon_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else NotImplemented
                )
            )
        )
