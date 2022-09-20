from typing import Optional

from reprit.base import generate_repr

from rene._bentley_ottmann.base import sweep
from rene._context import Context
from rene._rene import (MIN_MULTISEGMENT_SEGMENTS_COUNT,
                        Relation)


class Multisegment:
    _context: Optional[Context] = None

    @property
    def segments(self):
        return self._segments[:]

    @property
    def segments_count(self):
        return len(self._segments)

    def is_valid(self):
        segments = self.segments
        return all(intersection.relation is Relation.TOUCH
                   for intersection in sweep(segments))

    __module__ = 'rene.exact'
    __slots__ = '_segments',

    def __new__(cls, segments):
        if len(segments) < MIN_MULTISEGMENT_SEGMENTS_COUNT:
            raise ValueError('Multisegment should have at least '
                             f'{MIN_MULTISEGMENT_SEGMENTS_COUNT} segments, '
                             f'but found {len(segments)}.')
        self = super().__new__(cls)
        self._segments = list(segments)
        return self

    def __eq__(self, other):
        return (frozenset(self.segments) == frozenset(other.segments)
                if isinstance(other, Multisegment)
                else NotImplemented)

    def __hash__(self):
        return hash(frozenset(self.segments))

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __str__(self):
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.segments))))
