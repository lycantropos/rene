from __future__ import annotations

from typing import Any, ClassVar, NoReturn, TYPE_CHECKING

from rithm.fraction import Fraction
from typing_extensions import Self, final

from rene._geometries.base_segment import BaseSegment

if TYPE_CHECKING:
    from rene import hints
    from rene._context import Context


@final
class Segment(BaseSegment[Fraction]):
    @property
    def end(self, /) -> hints.Point[Fraction]:
        return self._end

    @property
    def start(self, /) -> hints.Point[Fraction]:
        return self._start

    _context: ClassVar[Context[Fraction]]
    _end: hints.Point[Fraction]
    _start: hints.Point[Fraction]

    __module__ = 'rene.exact'
    __slots__ = '_end', '_start'

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(
        cls, start: hints.Point[Fraction], end: hints.Point[Fraction], /
    ) -> Self:
        self = super().__new__(cls)
        self._end, self._start = end, start
        return self
