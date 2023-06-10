from __future__ import annotations

import random
import typing as t

import typing_extensions as te
from rithm.fraction import Fraction

from rene import Location
from rene._seidel.trapezoidation import Trapezoidation as _RawTrapezoidation
from rene._utils import validate_seed
from rene.hints import Seeder
from .multisegment import Multisegment
from .point import Point
from .polygon import Polygon


class Trapezoidation:
    @classmethod
    def from_multisegment(cls,
                          multisegment: Multisegment,
                          /,
                          *,
                          seeder: t.Optional[Seeder] = None) -> te.Self:
        seed = (random.randint(0, multisegment.segments_count)
                if seeder is None
                else seeder())
        validate_seed(seed)
        return cls(_RawTrapezoidation.from_multisegment(multisegment, seed))

    @classmethod
    def from_polygon(cls,
                     polygon: Polygon,
                     /,
                     *,
                     seeder: t.Optional[Seeder] = None) -> te.Self:
        seed = (random.randint(0, polygon.segments_count)
                if seeder is None
                else seeder())
        validate_seed(seed)
        return cls(_RawTrapezoidation.from_polygon(polygon, seed))

    @property
    def height(self) -> int:
        return self._raw.height

    def locate(self, point: Point, /) -> Location:
        return self._raw.locate(point)

    _raw: _RawTrapezoidation[Fraction]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __new__(cls, raw: _RawTrapezoidation[Fraction], /) -> te.Self:
        self = super().__new__(cls)
        self._raw = raw
        return self

    def __contains__(self, point: Point, /) -> bool:
        return self._raw.__contains__(point)
