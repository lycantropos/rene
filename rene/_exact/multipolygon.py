from __future__ import annotations

import typing as t
from itertools import chain

import typing_extensions as te
from reprit.base import generate_repr
from rithm.fraction import Fraction

from rene import (MIN_MULTIPOLYGON_POLYGONS_COUNT,
                  Location,
                  hints)
from rene._clipping import (intersect_multipolygon_with_polygon,
                            intersect_multipolygons,
                            subtract_multipolygons,
                            subtract_polygon_from_multipolygon,
                            symmetric_subtract_multipolygon_with_polygon,
                            symmetric_subtract_multipolygons,
                            unite_multipolygon_with_polygon,
                            unite_multipolygons)
from rene._context import Context
from rene._utils import (collect_maybe_empty_polygons,
                         collect_non_empty_polygons)


class Multipolygon:
    _context: t.ClassVar[Context[Fraction]]

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
                        intersect_multipolygons(self, other,
                                                self._context.segment_cls),
                        self._context.empty_cls,
                        self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            intersect_multipolygon_with_polygon(
                                    self, other, self._context.segment_cls
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
                        unite_multipolygons(self, other,
                                            self._context.segment_cls),
                        self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_non_empty_polygons(
                            unite_multipolygon_with_polygon(
                                    self, other, self._context.segment_cls
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
                        subtract_multipolygons(self, other,
                                               self._context.segment_cls),
                        self._context.empty_cls, self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            subtract_polygon_from_multipolygon(
                                    self, other, self._context.segment_cls
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
                        symmetric_subtract_multipolygons(
                                self, other, self._context.segment_cls
                        ),
                        self._context.empty_cls,
                        self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            symmetric_subtract_multipolygon_with_polygon(
                                    self, other, self._context.segment_cls
                            ),
                            self._context.empty_cls,
                            self._context.multipolygon_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else NotImplemented
                )
            )
        )
