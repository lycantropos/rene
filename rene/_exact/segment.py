from typing import Optional

from reprit.base import generate_repr

from .context import Context


class Segment:
    _context: Optional[Context] = None

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

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __str__(self):
        return f'{type(self).__qualname__}({self.start}, {self.end})'
