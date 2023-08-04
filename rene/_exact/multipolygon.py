from __future__ import annotations

import enum
import typing as t

import typing_extensions as te
from rithm.fraction import Fraction

from rene import (MIN_MULTIPOLYGON_POLYGONS_COUNT,
                  Location,
                  hints)
from rene._clipping import (intersect_multipolygon_with_multipolygon,
                            intersect_multipolygon_with_multisegmental,
                            intersect_multipolygon_with_polygon,
                            intersect_multipolygon_with_segment,
                            subtract_multipolygon_from_multipolygon,
                            subtract_polygon_from_multipolygon,
                            symmetric_subtract_multipolygon_from_multipolygon,
                            symmetric_subtract_polygon_from_multipolygon,
                            unite_multipolygon_with_multipolygon,
                            unite_multipolygon_with_polygon)
from rene._context import Context
from rene._utils import (collect_maybe_empty_polygons,
                         collect_maybe_empty_segments,
                         collect_non_empty_polygons)


@te.final
class Multipolygon:
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
        return _MultipolygonPolygons(self._polygons, _TOKEN)

    def locate(self, point: hints.Point[Fraction], /) -> Location:
        for polygon in self._polygons:
            location = polygon.locate(point)
            if location is not Location.EXTERIOR:
                return location
        return Location.EXTERIOR

    _context: t.ClassVar[Context[Fraction]]
    _polygons: t.Sequence[hints.Polygon[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_polygons',

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(
            cls, polygons: t.Sequence[hints.Polygon[Fraction]], /
    ) -> te.Self:
        if len(polygons) < MIN_MULTIPOLYGON_POLYGONS_COUNT:
            raise ValueError('Multipolygon should have at least '
                             f'{MIN_MULTIPOLYGON_POLYGONS_COUNT} polygons, '
                             f'but found {len(polygons)}.')
        self = super().__new__(cls)
        self._polygons = tuple(polygons)
        return self

    @t.overload
    def __and__(
            self, other: hints.Empty[Fraction], /
    ) -> hints.Empty[Fraction]:
        ...

    @t.overload
    def __and__(
            self,
            other: t.Union[
                hints.Multipolygon[Fraction], hints.Polygon[Fraction]
            ],
            /
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
                        intersect_multipolygon_with_multipolygon(
                                self, other, self._context.contour_cls,
                                self._context.polygon_cls,
                                self._context.segment_cls
                        ),
                        self._context.empty_cls,
                        self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            intersect_multipolygon_with_polygon(
                                    self, other, self._context.contour_cls,
                                    self._context.polygon_cls,
                                    self._context.segment_cls
                            ),
                            self._context.empty_cls,
                            self._context.multipolygon_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else (
                        collect_maybe_empty_segments(
                                intersect_multipolygon_with_multisegmental(
                                        self, other, self._context.segment_cls
                                ),
                                self._context.empty_cls,
                                self._context.multisegment_cls
                        )
                        if isinstance(other, (self._context.contour_cls,
                                              self._context.multisegment_cls))
                        else (
                            collect_maybe_empty_segments(
                                    intersect_multipolygon_with_segment(
                                            self, other,
                                            self._context.segment_cls
                                    ),
                                    self._context.empty_cls,
                                    self._context.multisegment_cls
                            )
                            if isinstance(other, self._context.segment_cls)
                            else NotImplemented
                        )
                    )
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
        return (frozenset(self._polygons) == frozenset(other.polygons)
                if isinstance(other, Multipolygon)
                else NotImplemented)

    def __hash__(self) -> int:
        return hash(frozenset(self._polygons))

    @t.overload
    def __or__(self, other: hints.Empty[Fraction], /) -> te.Self:
        ...

    @t.overload
    def __or__(
            self,
            other: t.Union[
                hints.Multipolygon[Fraction], hints.Polygon[Fraction]
            ],
            /
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
                        unite_multipolygon_with_multipolygon(
                                self, other, self._context.contour_cls,
                                self._context.polygon_cls,
                                self._context.segment_cls
                        ),
                        self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_non_empty_polygons(
                            unite_multipolygon_with_polygon(
                                    self, other, self._context.contour_cls,
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

    def __repr__(self) -> str:
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(repr, self._polygons))))

    def __str__(self) -> str:
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self._polygons))))

    @t.overload
    def __sub__(self, other: hints.Empty[Fraction], /) -> te.Self:
        ...

    @t.overload
    def __sub__(
            self,
            other: t.Union[
                hints.Multipolygon[Fraction], hints.Polygon[Fraction]
            ],
            /
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
                        subtract_multipolygon_from_multipolygon(
                                self, other, self._context.contour_cls,
                                self._context.polygon_cls,
                                self._context.segment_cls
                        ),
                        self._context.empty_cls, self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            subtract_polygon_from_multipolygon(
                                    self, other, self._context.contour_cls,
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

    @t.overload
    def __xor__(self, other: hints.Empty[Fraction], /) -> te.Self:
        ...

    @t.overload
    def __xor__(
            self,
            other: t.Union[
                hints.Multipolygon[Fraction], hints.Polygon[Fraction]
            ],
            /
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
                        symmetric_subtract_multipolygon_from_multipolygon(
                                self, other, self._context.contour_cls,
                                self._context.polygon_cls,
                                self._context.segment_cls
                        ),
                        self._context.empty_cls,
                        self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            symmetric_subtract_polygon_from_multipolygon(
                                    self, other, self._context.contour_cls,
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


class _Token(enum.Enum):
    VALUE = object()


_TOKEN = _Token.VALUE


@te.final
class _MultipolygonPolygons(t.Sequence[hints.Polygon[Fraction]]):
    def count(self, polygon: hints.Polygon[Fraction], /) -> int:
        return self._polygons.count(polygon)

    def index(self,
              polygon: hints.Polygon[Fraction],
              start: int = 0,
              stop: t.Optional[int] = None,
              /) -> int:
        return self._polygons.index(polygon, start,
                                    *(() if stop is None else (stop,)))

    _polygons: t.Sequence[hints.Polygon[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_polygons',

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(cls,
                polygons: t.Sequence[hints.Polygon[Fraction]],
                token: _Token,
                /) -> te.Self:
        if token is not _TOKEN:
            raise ValueError(f'{cls.__qualname__!r} is internal '
                             'and its instances should not be instantiated '
                             'outside of the library.')
        self = super().__new__(cls)
        self._polygons = polygons
        return self

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        ...

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (self._polygons == other._polygons
                if isinstance(other, _MultipolygonPolygons)
                else NotImplemented)

    @t.overload
    def __getitem__(self, item: int) -> hints.Polygon[Fraction]:
        ...

    @t.overload
    def __getitem__(self, item: slice) -> te.Self:
        ...

    def __getitem__(
            self, item: t.Union[int, slice]
    ) -> t.Union[hints.Polygon[Fraction], te.Self]:
        return (_MultipolygonPolygons(self._polygons[item], _TOKEN)
                if type(item) is slice
                else self._polygons[item])

    def __hash__(self) -> int:
        return hash(self._polygons)

    def __len__(self) -> int:
        return len(self._polygons)
