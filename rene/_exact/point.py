from reprit.base import generate_repr
from rithm import Fraction


class Point:
    @property
    def x(self):
        return self._x

    @property
    def y(self):
        return self._y

    __module__ = 'rene.exact'
    __slots__ = '_x', '_y'

    def __new__(cls, x, y):
        self = super().__new__(cls)
        self._x, self._y = Fraction(x), Fraction(y)
        return self

    def __eq__(self, other):
        return (self.x == other.x and self.y == other.y
                if isinstance(other, Point)
                else NotImplemented)

    def __ge__(self, other):
        return (self.x > other.x or self.x == other.x and self.y >= other.y
                if isinstance(other, Point)
                else NotImplemented)

    def __gt__(self, other):
        return (self.x > other.x or self.x == other.x and self.y > other.y
                if isinstance(other, Point)
                else NotImplemented)

    def __hash__(self):
        return hash((self.x, self.y))

    def __le__(self, other):
        return (self.x < other.x or self.x == other.x and self.y <= other.y
                if isinstance(other, Point)
                else NotImplemented)

    def __lt__(self, other):
        return (self.x < other.x or self.x == other.x and self.y < other.y
                if isinstance(other, Point)
                else NotImplemented)

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __str__(self):
        return f'{type(self).__qualname__}({self.x}, {self.y})'
