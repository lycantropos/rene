class Multisegment:
    @property
    def segments(self):
        return self._segments[:]

    __module__ = 'rene.exact'
    __slots__ = '_segments',

    def __new__(cls, segments):
        self = super().__new__(cls)
        self._segments = list(segments)
        return self

    def __eq__(self, other):
        return (frozenset(self.segments) == frozenset(other.segments)
                if isinstance(other, Multisegment)
                else NotImplemented)

    def __hash__(self):
        return hash(frozenset(self.segments))

    def __repr__(self):
        return (f'{type(self).__module__}.{type(self).__qualname__}'
                f'({self.segments!r})')

    def __str__(self):
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.segments))))
