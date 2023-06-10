from __future__ import annotations

import random
import typing as _t

import typing_extensions as _te
from rithm.fraction import Fraction as _Fraction

from rene import Location
from rene._seidel.trapezoidation import (
    Trapezoidation as _RawTrapezoidation
)
from rene._utils import validate_seed as _validate_seed
from rene.hints import Seeder as _Seeder
from .multisegment import Multisegment as _Multisegment
from .point import Point as _Point
from .polygon import Polygon as _Polygon


class Trapezoidation:
    @classmethod
    def from_multisegment(cls,
                          multisegment: _Multisegment,
                          /,
                          *,
                          seeder: _t.Optional[_Seeder] = None) -> _te.Self:
        seed = (random.randint(0, multisegment.segments_count)
                if seeder is None
                else seeder())
        _validate_seed(seed)
        return cls(_RawTrapezoidation.from_multisegment(multisegment, seed))

    @classmethod
    def from_polygon(cls,
                     polygon: _Polygon,
                     /,
                     *,
                     seeder: _t.Optional[_Seeder] = None) -> _te.Self:
        seed = (random.randint(0, polygon.segments_count)
                if seeder is None
                else seeder())
        _validate_seed(seed)
        return cls(_RawTrapezoidation.from_polygon(polygon, seed))

    @property
    def height(self) -> int:
        return self._raw.height

    def locate(self, point: _Point, /) -> Location:
        return self._raw.locate(point)

    _raw: _RawTrapezoidation[_Fraction]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __new__(cls, raw: _RawTrapezoidation[_Fraction], /) -> _te.Self:
        self = super().__new__(cls)
        self._raw = raw
        return self

    def __contains__(self, point: _Point, /) -> bool:
        return self._raw.__contains__(point)
