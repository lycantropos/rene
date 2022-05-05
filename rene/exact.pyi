from numbers import Rational as _Rational
from typing import (Any as _Any,
                    Union as _Union,
                    overload as _overload)

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

    @_overload
    def __eq__(self, other: 'Point') -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...

    def __repr__(self) -> str:
        ...
