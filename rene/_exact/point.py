from numbers import Rational
from typing import NoReturn, TypeAlias

from rithm.fraction import Fraction
from rithm.integer import Int
from typing_extensions import Self, final

from rene._geometries.base_point import BasePoint

_Coordinate: TypeAlias = Fraction | Int | Rational | float | int


@final
class Point(BasePoint[Fraction]):
    @property
    def x(self, /) -> Fraction:
        return self._x

    @property
    def y(self, /) -> Fraction:
        return self._y

    _x: Fraction
    _y: Fraction

    __module__ = 'rene.exact'
    __slots__ = '_x', '_y'

    def __init_subclass__(cls, /) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(cls, x: _Coordinate, y: _Coordinate, /) -> Self:
        self = object.__new__(cls)
        self._x, self._y = Fraction(x), Fraction(y)
        return self
