import typing as _t
from numbers import Rational

import typing_extensions as _te
from reprit.base import generate_repr
from rithm.fraction import Fraction
from rithm.integer import Int

_Coordinate = _t.Union[Fraction, Int, Rational, float, int]


class Point:
    @property
    def x(self) -> Fraction:
        return self._x

    @property
    def y(self) -> Fraction:
        return self._y

    _x: Fraction
    _y: Fraction

    __module__ = 'rene.exact'
    __slots__ = '_x', '_y'

    def __new__(cls, x: _Coordinate, y: _Coordinate) -> _te.Self:
        self = super().__new__(cls)
        self._x, self._y = Fraction(x), Fraction(y)
        return self

    @_t.overload
    def __eq__(self, other: _te.Self) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __eq__(self, other: _t.Any) -> _t.Any:
        return (self.x == other.x and self.y == other.y
                if isinstance(other, Point)
                else NotImplemented)

    @_t.overload
    def __ge__(self, other: _te.Self) -> bool:
        ...

    @_t.overload
    def __ge__(self, other: _t.Any) -> _t.Any:
        ...

    def __ge__(self, other: _t.Any) -> _t.Any:
        return (self.x > other.x or self.x == other.x and self.y >= other.y
                if isinstance(other, Point)
                else NotImplemented)

    @_t.overload
    def __gt__(self, other: _te.Self) -> bool:
        ...

    @_t.overload
    def __gt__(self, other: _t.Any) -> _t.Any:
        ...

    def __gt__(self, other: _t.Any) -> _t.Any:
        return (self.x > other.x or self.x == other.x and self.y > other.y
                if isinstance(other, Point)
                else NotImplemented)

    def __hash__(self) -> int:
        return hash((self.x, self.y))

    @_t.overload
    def __le__(self, other: _te.Self) -> bool:
        ...

    @_t.overload
    def __le__(self, other: _t.Any) -> _t.Any:
        ...

    def __le__(self, other: _t.Any) -> _t.Any:
        return (self.x < other.x or self.x == other.x and self.y <= other.y
                if isinstance(other, Point)
                else NotImplemented)

    @_t.overload
    def __lt__(self, other: _te.Self) -> bool:
        ...

    @_t.overload
    def __lt__(self, other: _t.Any) -> _t.Any:
        ...

    def __lt__(self, other: _t.Any) -> _t.Any:
        return (self.x < other.x or self.x == other.x and self.y < other.y
                if isinstance(other, Point)
                else NotImplemented)

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __str__(self) -> str:
        return f'{type(self).__qualname__}({self.x}, {self.y})'
