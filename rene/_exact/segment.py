class Segment:
    @property
    def end(self):
        return self._end

    @property
    def start(self):
        return self._start

    __module__ = 'rene.exact'
    __slots__ = '_end', '_start'

    def __new__(cls, start, end):
        self = super().__new__(cls)
        self._end, self._start = end, start
        return self

    def __hash__(self):
        return hash(frozenset((self.start, self.end)))

    def __eq__(self, other):
        return (self.start == other.start and self.end == other.end
                or self.end == other.start and self.start == other.end
                if isinstance(other, Segment)
                else NotImplemented)

    def __repr__(self):
        return (f'{type(self).__module__}.{type(self).__qualname__}'
                f'({self.start!r}, {self.end!r})')

    def __str__(self):
        return f'{type(self).__qualname__}({self.start}, {self.end})'
