class Polygon:
    @property
    def border(self):
        return self._border

    @property
    def holes(self):
        return self._holes[:]

    __module__ = 'rene.exact'
    __slots__ = '_border', '_holes'

    def __new__(cls, border, holes):
        self = super().__new__(cls)
        self._border, self._holes = border, list(holes)
        return self

    def __eq__(self, other):
        return ((self.border == other.border
                 and len(self.holes) == len(other.holes)
                 and frozenset(self.holes) == frozenset(other.holes))
                if isinstance(other, Polygon)
                else NotImplemented)

    def __hash__(self):
        return hash((self.border, frozenset(self.holes)))

    def __repr__(self):
        return (f'{type(self).__module__}.{type(self).__qualname__}'
                f'({self.border!r}, {self.holes!r})')

    def __str__(self):
        return (f'{type(self).__qualname__}({self.border}, [{{}}])'
                .format(', '.join(map(str, self.holes))))
