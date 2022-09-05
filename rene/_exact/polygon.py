from itertools import chain
from typing import Any

from reprit.base import generate_repr

from rene._clipping import (intersect_polygons,
                            subtract_polygons,
                            symmetric_subtract_polygons,
                            unite_polygons)
from rene._utils import (collect_maybe_empty_polygons,
                         collect_non_empty_polygons)
from .empty import Empty


class Polygon:
    # to avoid circular imports
    _multipolygon_cls: Any = None

    @property
    def border(self):
        return self._border

    @property
    def bounding_box(self):
        return self.border.bounding_box

    @property
    def holes(self):
        return self._holes[:]

    @property
    def segments(self):
        return list(chain(self.border.segments,
                          chain.from_iterable(hole.segments
                                              for hole in self._holes)))

    @property
    def segments_count(self):
        return sum([hole.segments_count for hole in self._holes],
                   self.border.segments_count)

    __module__ = 'rene.exact'
    __slots__ = '_border', '_holes'

    def __new__(cls, border, holes):
        self = super().__new__(cls)
        self._border, self._holes = border, list(holes)
        return self

    def __and__(self, other):
        return (collect_maybe_empty_polygons(intersect_polygons(self, other),
                                             Empty, self._multipolygon_cls)
                if isinstance(other, Polygon)
                else NotImplemented)

    def __eq__(self, other):
        return ((self.border == other.border
                 and len(self.holes) == len(other.holes)
                 and frozenset(self.holes) == frozenset(other.holes))
                if isinstance(other, Polygon)
                else NotImplemented)

    def __hash__(self):
        return hash((self.border, frozenset(self.holes)))

    def __or__(self, other):
        return (collect_non_empty_polygons(unite_polygons(self, other),
                                           self._multipolygon_cls)
                if isinstance(other, Polygon)
                else NotImplemented)

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __str__(self):
        return (f'{type(self).__qualname__}({self.border}, [{{}}])'
                .format(', '.join(map(str, self.holes))))

    def __sub__(self, other):
        return (collect_maybe_empty_polygons(subtract_polygons(self, other),
                                             Empty, self._multipolygon_cls)
                if isinstance(other, Polygon)
                else NotImplemented)

    def __xor__(self, other):
        return (
            collect_maybe_empty_polygons(
                    symmetric_subtract_polygons(self, other), Empty,
                    self._multipolygon_cls
            )
            if isinstance(other, Polygon)
            else NotImplemented
        )
