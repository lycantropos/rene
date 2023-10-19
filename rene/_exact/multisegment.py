from __future__ import annotations

import enum
import typing as t

import typing_extensions as te
from rithm.fraction import Fraction

from rene import (MIN_MULTISEGMENT_SEGMENTS_COUNT,
                  hints)
from rene._context import Context
from rene._geometries.base_multisegment import BaseMultisegment


@te.final
class Multisegment(BaseMultisegment[Fraction]):
    @property
    def segments(self) -> t.Sequence[hints.Segment[Fraction]]:
        return _MultisegmentSegments(self._segments, _TOKEN)

    _context: t.ClassVar[Context[Fraction]]
    _segments: t.Sequence[hints.Segment[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_segments',

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(
            cls, segments: t.Sequence[hints.Segment[Fraction]], /
    ) -> te.Self:
        if len(segments) < MIN_MULTISEGMENT_SEGMENTS_COUNT:
            raise ValueError('Multisegment should have at least '
                             f'{MIN_MULTISEGMENT_SEGMENTS_COUNT} segments, '
                             f'but found {len(segments)}.')
        self = super().__new__(cls)
        self._segments = tuple(segments)
        return self


class _Token(enum.Enum):
    VALUE = object()


_TOKEN = _Token.VALUE


@te.final
class _MultisegmentSegments(t.Sequence[hints.Segment[Fraction]]):
    def count(self, segment: hints.Segment[Fraction], /) -> int:
        return self._segments.count(segment)

    def index(self,
              segment: hints.Segment[Fraction],
              start: int = 0,
              stop: t.Optional[int] = None,
              /) -> int:
        return self._segments.index(segment, start,
                                    *(() if stop is None else (stop,)))

    _segments: t.Sequence[hints.Segment[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_segments',

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(cls,
                segments: t.Sequence[hints.Segment[Fraction]],
                token: _Token,
                /) -> te.Self:
        if token is not _TOKEN:
            raise ValueError(f'{cls.__qualname__!r} is internal '
                             'and its instances should not be instantiated '
                             'outside of the library.')
        self = super().__new__(cls)
        self._segments = segments
        return self

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        ...

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (self._segments == other._segments
                if isinstance(other, _MultisegmentSegments)
                else NotImplemented)

    @t.overload
    def __getitem__(self, item: int) -> hints.Segment[Fraction]:
        ...

    @t.overload
    def __getitem__(self, item: slice) -> te.Self:
        ...

    def __getitem__(
            self, item: t.Union[int, slice]
    ) -> t.Union[hints.Segment[Fraction], te.Self]:
        return (_MultisegmentSegments(self._segments[item], _TOKEN)
                if type(item) is slice
                else self._segments[item])

    def __hash__(self) -> int:
        return hash(self._segments)

    def __len__(self) -> int:
        return len(self._segments)
