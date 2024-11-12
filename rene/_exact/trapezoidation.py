from __future__ import annotations

import random
from typing import Any, ClassVar, NoReturn

from rithm.fraction import Fraction
from typing_extensions import Self, final

from rene import Location, hints
from rene._context import Context
from rene._seidel.trapezoidation import Trapezoidation as _RawTrapezoidation
from rene._utils import polygon_to_segments_count, validate_seed


@final
class Trapezoidation:
    @classmethod
    def from_multisegment(
        cls,
        multisegment: hints.Multisegment[Fraction],
        /,
        *,
        seeder: hints.Seeder | None = None,
    ) -> Self:
        seed = (
            random.randint(0, len(multisegment.segments))
            if seeder is None
            else seeder()
        )
        validate_seed(seed)
        return cls(
            _RawTrapezoidation.from_multisegment(
                multisegment, seed, cls._context.orient
            )
        )

    @classmethod
    def from_polygon(
        cls,
        polygon: hints.Polygon[Fraction],
        /,
        *,
        seeder: hints.Seeder | None = None,
    ) -> Self:
        seed = (
            random.randint(0, polygon_to_segments_count(polygon))
            if seeder is None
            else seeder()
        )
        validate_seed(seed)
        return cls(
            _RawTrapezoidation.from_polygon(polygon, seed, cls._context.orient)
        )

    @property
    def height(self, /) -> int:
        return self._raw.height

    def locate(self, point: hints.Point[Fraction], /) -> Location:
        return self._raw.locate(point)

    _context: ClassVar[Context[Fraction]]
    _raw: _RawTrapezoidation[Fraction]

    __module__ = 'rene.exact'
    __slots__ = ('_raw',)

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(cls, raw: _RawTrapezoidation[Fraction], /) -> Self:
        self = super().__new__(cls)
        self._raw = raw
        return self

    def __contains__(self, point: hints.Point[Fraction], /) -> bool:
        return self._raw.__contains__(point)
