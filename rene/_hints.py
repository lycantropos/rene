from __future__ import annotations

import typing as t

import typing_extensions as te

from rene import (Orientation,
                  hints)

_Key = t.TypeVar('_Key',
                 contravariant=True)
_Value = t.TypeVar('_Value',
                   covariant=True)


class Map(te.Protocol[_Key, _Value]):
    def __getitem__(self, key: _Key, /) -> _Value:
        ...


Orienteer = t.Callable[
    [
        hints.Point[hints.Scalar], hints.Point[hints.Scalar],
        hints.Point[hints.Scalar]
    ],
    Orientation
]
SegmentsIntersector = t.Callable[
    [
        hints.Point[hints.Scalar], hints.Point[hints.Scalar],
        hints.Point[hints.Scalar], hints.Point[hints.Scalar]
    ],
    hints.Point[hints.Scalar]
]
SegmentsIntersectionScale = t.Callable[
    [
        hints.Point[hints.Scalar], hints.Point[hints.Scalar],
        hints.Point[hints.Scalar], hints.Point[hints.Scalar]
    ],
    hints.Scalar
]
