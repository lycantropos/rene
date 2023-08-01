from __future__ import annotations

from typing import (Generic,
                    Type)

import typing_extensions as te

from . import hints


class Context(Generic[hints.Scalar]):
    @property
    def box_cls(self) -> Type[hints.Box[hints.Scalar]]:
        return self._box_cls

    @property
    def contour_cls(self) -> Type[hints.Contour[hints.Scalar]]:
        return self._contour_cls

    @property
    def empty_cls(self) -> Type[hints.Empty[hints.Scalar]]:
        return self._empty_cls

    @property
    def multipolygon_cls(self) -> Type[hints.Multipolygon[hints.Scalar]]:
        return self._multipolygon_cls

    @property
    def multisegment_cls(self) -> Type[hints.Multisegment[hints.Scalar]]:
        return self._multisegment_cls

    @property
    def point_cls(self) -> Type[hints.Point[hints.Scalar]]:
        return self._point_cls

    @property
    def polygon_cls(self) -> Type[hints.Polygon[hints.Scalar]]:
        return self._polygon_cls

    @property
    def segment_cls(self) -> Type[hints.Segment[hints.Scalar]]:
        return self._segment_cls

    _box_cls: Type[hints.Box[hints.Scalar]]
    _contour_cls: Type[hints.Contour[hints.Scalar]]
    _empty_cls: Type[hints.Empty[hints.Scalar]]
    _multipolygon_cls: Type[hints.Multipolygon[hints.Scalar]]
    _multisegment_cls: Type[hints.Multisegment[hints.Scalar]]
    _point_cls: Type[hints.Point[hints.Scalar]]
    _polygon_cls: Type[hints.Polygon[hints.Scalar]]
    _segment_cls: Type[hints.Segment[hints.Scalar]]

    __module__ = 'rene.exact'
    __slots__ = (
        '_box_cls', '_contour_cls', '_empty_cls', '_multipolygon_cls',
        '_multisegment_cls', '_point_cls', '_polygon_cls', '_segment_cls'
    )

    def __new__(cls,
                *,
                box_cls: Type[hints.Box[hints.Scalar]],
                contour_cls: Type[hints.Contour[hints.Scalar]],
                empty_cls: Type[hints.Empty[hints.Scalar]],
                multipolygon_cls: Type[hints.Multipolygon[hints.Scalar]],
                multisegment_cls: Type[hints.Multisegment[hints.Scalar]],
                point_cls: Type[hints.Point[hints.Scalar]],
                polygon_cls: Type[hints.Polygon[hints.Scalar]],
                segment_cls: Type[hints.Segment[hints.Scalar]]) -> te.Self:
        self = super().__new__(cls)
        (
            self._box_cls, self._contour_cls, self._empty_cls,
            self._multipolygon_cls, self._multisegment_cls, self._point_cls,
            self._polygon_cls, self._segment_cls
        ) = (box_cls, contour_cls, empty_cls, multipolygon_cls,
             multisegment_cls, point_cls, polygon_cls, segment_cls)
        return self
