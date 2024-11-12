from __future__ import annotations

import enum
from collections.abc import Sequence
from typing import Any, NoReturn, overload

from rithm.fraction import Fraction
from typing_extensions import Self, final

from rene import hints
from rene._geometries.base_contour import BaseContour
from rene._utils import to_contour_segments
from rene.constants import MIN_CONTOUR_VERTICES_COUNT


@final
class Contour(BaseContour[Fraction]):
    @property
    def segments(self, /) -> Sequence[hints.Segment[Fraction]]:
        return _ContourSegments(self._segments, _TOKEN)

    @property
    def vertices(self, /) -> Sequence[hints.Point[Fraction]]:
        return _ContourVertices(self._vertices, _TOKEN)

    _segments: Sequence[hints.Segment[Fraction]]
    _vertices: Sequence[hints.Point[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_segments', '_vertices'

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(cls, vertices: Sequence[hints.Point[Fraction]], /) -> Self:
        if len(vertices) < MIN_CONTOUR_VERTICES_COUNT:
            raise ValueError(
                'Contour should have at least '
                f'{MIN_CONTOUR_VERTICES_COUNT} vertices, '
                f'but found {len(vertices)}.'
            )
        self = super().__new__(cls)
        self._vertices = tuple(vertices)
        self._segments = tuple(
            to_contour_segments(self._vertices, self._context.segment_cls)
        )
        return self


class _Token(enum.Enum):
    VALUE = object()


_TOKEN = _Token.VALUE


@final
class _ContourSegments(Sequence[hints.Segment[Fraction]]):
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
            if isinstance(other, _ContourSegments)
            else NotImplemented
        )

    @overload
    def __getitem__(self, item: int) -> hints.Segment[Fraction]: ...

    @overload
    def __getitem__(self, item: slice) -> Self: ...

    def __getitem__(self, item: int | slice) -> hints.Segment[Fraction] | Self:
        return (
            _ContourSegments(self._segments[item], _TOKEN)
            if type(item) is slice
            else self._segments[item]
        )

    def __hash__(self, /) -> int:
        return hash(self._segments)

    def __len__(self) -> int:
        return len(self._segments)


@final
class _ContourVertices(Sequence[hints.Point[Fraction]]):
    def count(self, point: hints.Point[Fraction], /) -> int:
        return self._vertices.count(point)

    def index(
        self,
        point: hints.Point[Fraction],
        start: int = 0,
        stop: int | None = None,
        /,
    ) -> int:
        return self._vertices.index(
            point, start, *(() if stop is None else (stop,))
        )

    _vertices: Sequence[hints.Point[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = ('_vertices',)

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(
        cls, vertices: Sequence[hints.Point[Fraction]], token: _Token, /
    ) -> Self:
        if token is not _TOKEN:
            raise ValueError(
                f'{cls.__qualname__!r} is internal '
                'and its instances should not be instantiated '
                'outside of the library.'
            )
        self = super().__new__(cls)
        self._vertices = vertices
        return self

    @overload
    def __eq__(self, other: Self, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...

    def __eq__(self, other: Any, /) -> Any:
        return (
            self._vertices == other._vertices
            if isinstance(other, _ContourVertices)
            else NotImplemented
        )

    @overload
    def __getitem__(self, item: int) -> hints.Point[Fraction]: ...

    @overload
    def __getitem__(self, item: slice) -> Self: ...

    def __getitem__(self, item: int | slice) -> hints.Point[Fraction] | Self:
        return (
            _ContourVertices(self._vertices[item], _TOKEN)
            if type(item) is slice
            else self._vertices[item]
        )

    def __hash__(self, /) -> int:
        return hash(self._vertices)

    def __len__(self) -> int:
        return len(self._vertices)
