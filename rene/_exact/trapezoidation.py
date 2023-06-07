from __future__ import annotations

import random as _random

import typing_extensions as _te
from rithm.fraction import Fraction as _Fraction

from rene._trapezoidation.hints import Shuffler as _Shuffler
from rene._trapezoidation.trapezoidation import (
    Trapezoidation as _RawTrapezoidation
)
from .multisegment import Multisegment as _Multisegment


class Trapezoidation:
    @classmethod
    def from_multisegment(cls,
                          _multisegment: _Multisegment,
                          _shuffler: _Shuffler = _random.shuffle) -> _te.Self:
        return cls(_RawTrapezoidation.from_multisegment(_multisegment,
                                                        _shuffler))

    @property
    def height(self) -> int:
        return self._raw.height

    _raw: _RawTrapezoidation[_Fraction]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __init__(self, _raw: _RawTrapezoidation[_Fraction]) -> None:
        self._raw = _raw
