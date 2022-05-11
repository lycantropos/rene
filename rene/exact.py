try:
    from ._exact import (Point,
                         Segment)
except ImportError:
    from rithm import Fraction as _Fraction


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

        def __repr__(self):
            return (f'{__name__}.{type(self).__qualname__}'
                    f'({self.x!r}, {self.y!r})')


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
            self._end, self._start = Point(end), Point(start)
            return self

        def __eq__(self, other):
            return (self.start == other.start and self.end == other.end
                    if isinstance(other, Segment)
                    else NotImplemented)

        def __repr__(self):
            return (f'{__name__}.{type(self).__qualname__}'
                    f'({self.start!r}, {self.end!r})')
