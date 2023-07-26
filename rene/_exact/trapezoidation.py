from __future__ import annotations

import random
import typing as t

import typing_extensions as te
from rithm.fraction import Fraction

from rene import (Location,
                  hints)
from rene._context import Context
from rene._seidel.trapezoidation import Trapezoidation as _RawTrapezoidation
from rene._utils import validate_seed


class Trapezoidation:
    @classmethod
    def from_multisegment(cls,
                          multisegment: hints.Multisegment[Fraction],
                          /,
                          *,
                          seeder: t.Optional[hints.Seeder] = None) -> te.Self:
        seed = (random.randint(0, multisegment.segments_count)
                if seeder is None
                else seeder())
        validate_seed(seed)
        return cls(_RawTrapezoidation.from_multisegment(multisegment, seed))

    @classmethod
    def from_polygon(cls,
                     polygon: hints.Polygon[Fraction],
                     /,
                     *,
                     seeder: t.Optional[hints.Seeder] = None) -> te.Self:
        seed = (random.randint(0,
                               polygon.border.segments_count
                               + sum(hole.segments_count
                                     for hole in polygon.holes))
                if seeder is None
                else seeder())
        validate_seed(seed)
        return cls(_RawTrapezoidation.from_polygon(polygon, seed))

    @property
    def height(self) -> int:
        return self._raw.height

    def locate(self, point: hints.Point[Fraction], /) -> Location:
        return self._raw.locate(point)

    _context: t.ClassVar[Context[Fraction]]
    _raw: _RawTrapezoidation[Fraction]

    __module__ = 'rene.exact'
    __slots__ = '_raw',

    def __new__(cls, raw: _RawTrapezoidation[Fraction], /) -> te.Self:
        self = super().__new__(cls)
        self._raw = raw
        return self

    def __contains__(self, point: hints.Point[Fraction], /) -> bool:
        return self._raw.__contains__(point)
