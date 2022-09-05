from typing import (Sequence,
                    Union)

from reprit.base import generate_repr

from rene._clipping.intersection import intersect_polygons
from rene._clipping.union import unite_polygons
from .empty import Empty
from .multipolygon import Multipolygon


class Polygon:
    @property
    def border(self):
        return self._border

    @property
    def bounding_box(self):
        return self.border.bounding_box

    @property
    def holes(self):
        return self._holes[:]

    __module__ = 'rene.exact'
    __slots__ = '_border', '_holes'

    def __new__(cls, border, holes):
        self = super().__new__(cls)
        self._border, self._holes = border, list(holes)
        return self

    def __and__(self, other):
        return (collect_maybe_empty_polygons(intersect_polygons(self, other))
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
        return (collect_non_empty_polygons(unite_polygons(self, other))
                if isinstance(other, Polygon)
                else NotImplemented)

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __str__(self):
        return (f'{type(self).__qualname__}({self.border}, [{{}}])'
                .format(', '.join(map(str, self.holes))))


def collect_maybe_empty_polygons(
        polygons: Sequence[Polygon]
) -> Union[Empty, Multipolygon, Polygon]:
    return collect_non_empty_polygons(polygons) if polygons else Empty()


def collect_non_empty_polygons(
        polygons: Sequence[Polygon]
) -> Union[Empty, Multipolygon, Polygon]:
    assert len(polygons) >= 1
    return polygons[0] if len(polygons) == 1 else Multipolygon(polygons)
