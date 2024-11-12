from __future__ import annotations

import typing as t
from numbers import Rational

import typing_extensions as te
from rithm.fraction import Fraction
from rithm.integer import Int

from rene._geometries.base_box import BaseBox

_Coordinate = t.Union[Fraction, Int, Rational, float, int]


@te.final
class Box(BaseBox[Fraction]):
    @property
    def max_x(self, /) -> Fraction:
        return self._max_x

    @property
    def max_y(self, /) -> Fraction:
        return self._max_y

    @property
    def min_x(self, /) -> Fraction:
        return self._min_x

    @property
    def min_y(self, /) -> Fraction:
        return self._min_y

    _max_x: Fraction
    _max_y: Fraction
    _min_x: Fraction
    _min_y: Fraction

    __module__ = 'rene.exact'
    __slots__ = '_min_x', '_max_x', '_min_y', '_max_y'

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(
        cls,
        min_x: _Coordinate,
        max_x: _Coordinate,
        min_y: _Coordinate,
        max_y: _Coordinate,
        /,
    ) -> te.Self:
        self = super().__new__(cls)
        self._max_x, self._max_y, self._min_x, self._min_y = (
            Fraction(max_x),
            Fraction(max_y),
            Fraction(min_x),
            Fraction(min_y),
        )
        return self
