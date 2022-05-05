from numbers import Rational as _Rational
from typing import Union as _Union

from rithm import Fraction as _Fraction


class Point:
    @property
    def x(self) -> _Fraction:
        ...

    @property
    def y(self) -> _Fraction:
        ...

    def __new__(cls, x: _Union[_Rational, float], y: _Union[_Rational, float]
                ) -> 'Point':
        ...

    def __repr__(self) -> str:
        ...
