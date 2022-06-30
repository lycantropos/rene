from reprit.base import generate_repr

from rene._rene import (MIN_MULTISEGMENT_SEGMENTS_COUNT,
                        Relation)
from .bentley_ottmann.base import sweep


class Multisegment:
    @property
    def segments(self):
        return self._segments[:]

    def is_valid(self):
        segments = self.segments
        return (len(segments) >= MIN_MULTISEGMENT_SEGMENTS_COUNT
                and all(intersection.relation is Relation.TOUCH
                        for intersection in sweep(segments)))

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

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __str__(self):
        return (f'{type(self).__qualname__}([{{}}])'
                .format(', '.join(map(str, self.segments))))
