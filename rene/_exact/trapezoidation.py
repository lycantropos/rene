from __future__ import annotations

import random
import typing as _t

import typing_extensions as _te
from rithm.fraction import Fraction as _Fraction

from rene import Location
from rene._trapezoidation.trapezoidation import (
    Trapezoidation as _RawTrapezoidation
)
from rene._utils import validate_seed as _validate_seed
from rene.hints import Seeder as _Seeder
from .multisegment import Multisegment as _Multisegment
from .point import Point


class Trapezoidation:
    @classmethod
    def from_multisegment(cls,
                          _multisegment: _Multisegment,
                          *,
                          seeder: _t.Optional[_Seeder] = None) -> _te.Self:
        seed = (random.randint(0, _multisegment.segments_count)
                if seeder is None
                else seeder())
        _validate_seed(seed)
        return cls(_RawTrapezoidation.from_multisegment(_multisegment, seed))

    @property
    def height(self) -> int:
        return self._raw.height

    def locate(self, point: Point) -> Location:
        return self._raw.locate(point)

    _raw: _RawTrapezoidation[_Fraction]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __init__(self, _raw: _RawTrapezoidation[_Fraction]) -> None:
        self._raw = _raw

    def __contains__(self, point: Point) -> bool:
        return self._raw.__contains__(point)
