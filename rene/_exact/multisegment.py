from __future__ import annotations

import enum
from collections.abc import Sequence
from typing import Any, ClassVar, NoReturn, overload

from rithm.fraction import Fraction
from typing_extensions import Self, final

from rene import MIN_MULTISEGMENT_SEGMENTS_COUNT, hints
from rene._context import Context
from rene._geometries.base_multisegment import BaseMultisegment


@final
class Multisegment(BaseMultisegment[Fraction]):
    @property
    def segments(self, /) -> Sequence[hints.Segment[Fraction]]:
        return _MultisegmentSegments(self._segments, _TOKEN)

    _context: ClassVar[Context[Fraction]]
    _segments: Sequence[hints.Segment[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = ('_segments',)

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(cls, segments: Sequence[hints.Segment[Fraction]], /) -> Self:
        if len(segments) < MIN_MULTISEGMENT_SEGMENTS_COUNT:
            raise ValueError(
                'Multisegment should have at least '
                f'{MIN_MULTISEGMENT_SEGMENTS_COUNT} segments, '
                f'but found {len(segments)}.'
            )
        self = super().__new__(cls)
        self._segments = tuple(segments)
        return self


class _Token(enum.Enum):
    VALUE = object()


_TOKEN = _Token.VALUE


@final
class _MultisegmentSegments(Sequence[hints.Segment[Fraction]]):
    def count(self, segment: hints.Segment[Fraction], /) -> int:
        return self._segments.count(segment)

    def index(
        self,
        segment: hints.Segment[Fraction],
        start: int = 0,
        stop: int | None = None,
        /,
    ) -> int:
        return self._segments.index(
            segment, start, *(() if stop is None else (stop,))
        )

    _segments: Sequence[hints.Segment[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = ('_segments',)

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(
        cls, segments: Sequence[hints.Segment[Fraction]], token: _Token, /
    ) -> Self:
        if token is not _TOKEN:
            raise ValueError(
                f'{cls.__qualname__!r} is internal '
                'and its instances should not be instantiated '
                'outside of the library.'
            )
        self = super().__new__(cls)
        self._segments = segments
        return self

    @overload
    def __eq__(self, other: Self, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...

    def __eq__(self, other: Any, /) -> Any:
        return (
            self._segments == other._segments
            if isinstance(other, _MultisegmentSegments)
            else NotImplemented
        )

    @overload
    def __getitem__(self, item: int) -> hints.Segment[Fraction]: ...

    @overload
    def __getitem__(self, item: slice) -> Self: ...

    def __getitem__(self, item: int | slice) -> hints.Segment[Fraction] | Self:
        return (
            _MultisegmentSegments(self._segments[item], _TOKEN)
            if type(item) is slice
            else self._segments[item]
        )

    def __hash__(self, /) -> int:
        return hash(self._segments)

    def __len__(self) -> int:
        return len(self._segments)
