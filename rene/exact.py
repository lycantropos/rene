try:
    from ._exact import (Contour,
                         Point,
                         Segment)
except ImportError:
    from typing import (Any as _Any,
                        Sequence as _Sequence)

    from rithm import Fraction as _Fraction


    class Contour:
        @property
        def vertices(self):
            return self._vertices[:]

        __slots__ = '_vertices',

        def __new__(cls, vertices):
            self = super().__new__(cls)
            self._vertices = list(vertices)
            return self

        def __eq__(self, other):
            return (_are_non_empty_unique_sequences_rotationally_equivalent(
                    self.vertices, other.vertices)
                    if isinstance(other, Contour)
                    else NotImplemented)

        def __repr__(self):
            return f'{__name__}.{type(self).__qualname__}({self.vertices!r})'

        def __str__(self):
            return (f'{type(self).__qualname__}([{{}}])'
                    .format(', '.join(map(str, self.vertices))))


    def _are_non_empty_unique_sequences_rotationally_equivalent(
            left: _Sequence[_Any], right: _Sequence[_Any]
    ) -> bool:
        assert left and right
        if len(left) != len(right):
            return False
        first_left_element = left[0]
        try:
            index = right.index(first_left_element)
        except ValueError:
            return False
        else:
            return ((left[1:len(left) - index] == right[index + 1:]
                     and left[len(left) - index:] == right[:index])
                    or (left[:len(left) - index - 1:-1] == right[:index]
                        and (left[len(left) - index - 1:0:-1]
                             == right[index + 1:])))


    class Point:
        @property
        def x(self):
            return self._x

        @property
        def y(self):
            return self._y

        __slots__ = '_x', '_y'

        def __new__(cls, x, y):
            self = super().__new__(cls)
            self._x, self._y = (_Fraction(x)
                                if isinstance(x, float)
                                else _Fraction(x.numerator, x.denominator),
                                _Fraction(y)
                                if isinstance(y, float)
                                else _Fraction(y.numerator, y.denominator))
            return self

        def __eq__(self, other):
            return (self.x == other.x and self.y == other.y
                    if isinstance(other, Point)
                    else NotImplemented)

        def __hash__(self):
            return hash((self.x, self.y))

        def __repr__(self):
            return (f'{__name__}.{type(self).__qualname__}'
                    f'({self.x!r}, {self.y!r})')

        def __str__(self):
            return f'{type(self).__qualname__}({self.x}, {self.y})'


    class Segment:
        @property
        def end(self):
            return self._end

        @property
        def start(self):
            return self._start

        __slots__ = '_end', '_start'

        def __new__(cls, start, end):
            self = super().__new__(cls)
            self._end, self._start = end, start
            return self

        def __eq__(self, other):
            return (self.start == other.start and self.end == other.end
                    or self.end == other.start and self.start == other.end
                    if isinstance(other, Segment)
                    else NotImplemented)

        def __repr__(self):
            return (f'{__name__}.{type(self).__qualname__}'
                    f'({self.start!r}, {self.end!r})')

        def __str__(self):
            return f'{type(self).__qualname__}({self.start}, {self.end})'
