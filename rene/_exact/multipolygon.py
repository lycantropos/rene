from itertools import chain
from typing import Optional

from reprit.base import generate_repr

from rene._clipping import (intersect_multipolygon_with_polygon,
                            intersect_multipolygons,
                            subtract_multipolygons,
                            subtract_polygon_from_multipolygon,
                            symmetric_subtract_multipolygon_with_polygon,
                            symmetric_subtract_multipolygons,
                            unite_multipolygon_with_polygon,
                            unite_multipolygons)
from rene._rene import MIN_MULTIPOLYGON_POLYGONS_COUNT
from rene._utils import (collect_maybe_empty_polygons,
                         collect_non_empty_polygons)
from .context import Context


class Multipolygon:
    _context: Optional[Context] = None

    @property
    def polygons(self):
        return self._polygons[:]

    @property
    def polygons_count(self):
        return len(self._polygons)

    @property
    def segments(self):
        return list(chain.from_iterable(polygon.segments
                                        for polygon in self._polygons))

    @property
    def segments_count(self):
        return sum(polygon.segments_count for polygon in self._polygons)

    __module__ = 'rene.exact'
    __slots__ = '_polygons',

    def __new__(cls, polygons):
        if len(polygons) < MIN_MULTIPOLYGON_POLYGONS_COUNT:
            raise ValueError('Multipolygon should have at least '
                             f'{MIN_MULTIPOLYGON_POLYGONS_COUNT} polygons, '
                             f'but found {len(polygons)}.')
        self = super().__new__(cls)
        self._polygons = list(polygons)
        return self

    def __and__(self, other):
        return (
            self._context.empty_cls()
            if isinstance(other, self._context.empty_cls)
            else (
                collect_maybe_empty_polygons(intersect_multipolygons(self,
                                                                     other),
                                             self._context.empty_cls,
                                             self._context.multipolygon_cls)
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            intersect_multipolygon_with_polygon(self, other),
                            self._context.empty_cls,
                            self._context.multipolygon_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else NotImplemented
                )
            )
        )

    def __eq__(self, other):
        return (frozenset(self.polygons) == frozenset(other.polygons)
                if isinstance(other, self._context.multipolygon_cls)
                else NotImplemented)

    def __hash__(self):
        return hash(frozenset(self.polygons))

    def __or__(self, other):
        return (
            self
            if isinstance(other, self._context.empty_cls)
            else (
                collect_non_empty_polygons(unite_multipolygons(self, other),
                                           self._context.multipolygon_cls)
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_non_empty_polygons(
                            unite_multipolygon_with_polygon(self, other),
                            self._context.multipolygon_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else NotImplemented
                )
            )
        )

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __sub__(self, other):
        return (
            self
            if isinstance(other, self._context.empty_cls)
            else (
                collect_maybe_empty_polygons(subtract_multipolygons(self,
                                                                    other),
                                             self._context.empty_cls,
                                             self._context.multipolygon_cls)
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            subtract_polygon_from_multipolygon(
                                    self, other
                            ),
                            self._context.empty_cls,
                            self._context.multipolygon_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else NotImplemented
                )
            )
        )

    def __str__(self):
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.polygons))))

    def __xor__(self, other):
        return (
            self
            if isinstance(other, self._context.empty_cls)
            else (
                collect_maybe_empty_polygons(
                        symmetric_subtract_multipolygons(self, other),
                        self._context.empty_cls,
                        self._context.multipolygon_cls
                )
                if isinstance(other, self._context.multipolygon_cls)
                else (
                    collect_maybe_empty_polygons(
                            symmetric_subtract_multipolygon_with_polygon(
                                    self, other
                            ),
                            self._context.empty_cls,
                            self._context.multipolygon_cls
                    )
                    if isinstance(other, self._context.polygon_cls)
                    else NotImplemented
                )
            )
        )
