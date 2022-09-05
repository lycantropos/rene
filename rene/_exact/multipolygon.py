from reprit.base import generate_repr

from rene._rene import MIN_MULTIPOLYGON_POLYGONS_COUNT


class Multipolygon:
    @property
    def polygons(self):
        return self._polygons[:]

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

    def __eq__(self, other):
        return (frozenset(self.polygons) == frozenset(other.polygons)
                if isinstance(other, Multipolygon)
                else NotImplemented)

    def __hash__(self):
        return hash(frozenset(self.polygons))

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __str__(self):
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.polygons))))
