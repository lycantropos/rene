from __future__ import annotations

import enum
import typing as t

import typing_extensions as te
from rithm.fraction import Fraction

from rene import (MIN_CONTOUR_VERTICES_COUNT,
                  hints)
from rene._geometries.base_contour import BaseContour
from rene._utils import to_contour_segments


@te.final
class Contour(BaseContour[Fraction]):
    @property
    def segments(self) -> t.Sequence[hints.Segment[Fraction]]:
        return _ContourSegments(self._segments, _TOKEN)

    @property
    def vertices(self) -> t.Sequence[hints.Point[Fraction]]:
        return _ContourVertices(self._vertices, _TOKEN)

    _segments: t.Sequence[hints.Segment[Fraction]]
    _vertices: t.Sequence[hints.Point[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_segments', '_vertices'

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(
            cls, vertices: t.Sequence[hints.Point[Fraction]], /
    ) -> te.Self:
        if len(vertices) < MIN_CONTOUR_VERTICES_COUNT:
            raise ValueError('Contour should have at least '
                             f'{MIN_CONTOUR_VERTICES_COUNT} vertices, '
                             f'but found {len(vertices)}.')
        self = super().__new__(cls)
        self._vertices = tuple(vertices)
        self._segments = tuple(to_contour_segments(self._vertices,
                                                   self._context.segment_cls))
        return self


class _Token(enum.Enum):
    VALUE = object()


_TOKEN = _Token.VALUE


@te.final
class _ContourSegments(t.Sequence[hints.Segment[Fraction]]):
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
                if isinstance(other, _ContourSegments)
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
        return (_ContourSegments(self._segments[item], _TOKEN)
                if type(item) is slice
                else self._segments[item])

    def __hash__(self) -> int:
        return hash(self._segments)

    def __len__(self) -> int:
        return len(self._segments)


@te.final
class _ContourVertices(t.Sequence[hints.Point[Fraction]]):
    def count(self, point: hints.Point[Fraction], /) -> int:
        return self._vertices.count(point)

    def index(self,
              point: hints.Point[Fraction],
              start: int = 0,
              stop: t.Optional[int] = None,
              /) -> int:
        return self._vertices.index(point, start,
                                    *(() if stop is None else (stop,)))

    _vertices: t.Sequence[hints.Point[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = '_vertices',

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(cls,
                vertices: t.Sequence[hints.Point[Fraction]],
                token: _Token,
                /) -> te.Self:
        if token is not _TOKEN:
            raise ValueError(f'{cls.__qualname__!r} is internal '
                             'and its instances should not be instantiated '
                             'outside of the library.')
        self = super().__new__(cls)
        self._vertices = vertices
        return self

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        ...

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (self._vertices == other._vertices
                if isinstance(other, _ContourVertices)
                else NotImplemented)

    @t.overload
    def __getitem__(self, item: int) -> hints.Point[Fraction]:
        ...

    @t.overload
    def __getitem__(self, item: slice) -> te.Self:
        ...

    def __getitem__(
            self, item: t.Union[int, slice]
    ) -> t.Union[hints.Point[Fraction], te.Self]:
        return (_ContourVertices(self._vertices[item], _TOKEN)
                if type(item) is slice
                else self._vertices[item])

    def __hash__(self) -> int:
        return hash(self._vertices)

    def __len__(self) -> int:
        return len(self._vertices)
