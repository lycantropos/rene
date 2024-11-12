from __future__ import annotations

from typing import Callable, TypeVar

from typing_extensions import Protocol

from rene import Orientation, hints

_Key = TypeVar('_Key', contravariant=True)
_Value = TypeVar('_Value', covariant=True)


class Map(Protocol[_Key, _Value]):
    def __getitem__(self, key: _Key, /) -> _Value: ...


Orienteer = Callable[
    [
        hints.Point[hints.Scalar],
        hints.Point[hints.Scalar],
        hints.Point[hints.Scalar],
    ],
    Orientation,
]
SegmentsIntersector = Callable[
    [
        hints.Point[hints.Scalar],
        hints.Point[hints.Scalar],
        hints.Point[hints.Scalar],
        hints.Point[hints.Scalar],
    ],
    hints.Point[hints.Scalar],
]
SegmentsIntersectionScale = Callable[
    [
        hints.Point[hints.Scalar],
        hints.Point[hints.Scalar],
        hints.Point[hints.Scalar],
        hints.Point[hints.Scalar],
    ],
    hints.Scalar,
]
