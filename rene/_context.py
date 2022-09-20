from typing import (Generic,
                    Type)

from reprit import serializers
from reprit.base import generate_repr

from .hints import (Box,
                    Contour,
                    Empty,
                    Multipolygon,
                    Multisegment,
                    Point,
                    Polygon,
                    Scalar,
                    Segment)


class Context(Generic[Scalar]):
    @property
    def box_cls(self) -> Type[Box[Scalar]]:
        return self._box_cls

    @property
    def contour_cls(self) -> Type[Contour[Scalar]]:
        return self._contour_cls

    @property
    def empty_cls(self) -> Type[Empty]:
        return self._empty_cls

    @property
    def multipolygon_cls(self) -> Type[Multipolygon[Scalar]]:
        return self._multipolygon_cls

    @property
    def multisegment_cls(self) -> Type[Multisegment[Scalar]]:
        return self._multisegment_cls

    @property
    def point_cls(self) -> Type[Point[Scalar]]:
        return self._point_cls

    @property
    def polygon_cls(self) -> Type[Polygon[Scalar]]:
        return self._polygon_cls

    @property
    def segment_cls(self) -> Type[Segment[Scalar]]:
        return self._segment_cls

    _box_cls: Type[Box[Scalar]]
    _contour_cls: Type[Contour[Scalar]]
    _empty_cls: Type[Empty]
    _multipolygon_cls: Type[Multipolygon[Scalar]]
    _multisegment_cls: Type[Multisegment[Scalar]]
    _point_cls: Type[Point[Scalar]]
    _polygon_cls: Type[Polygon[Scalar]]
    _segment_cls: Type[Segment[Scalar]]

    __module__ = 'rene.exact'
    __slots__ = (
        '_box_cls', '_contour_cls', '_empty_cls', '_multipolygon_cls',
        '_multisegment_cls', '_point_cls', '_polygon_cls', '_segment_cls'
    )

    def __new__(cls,
                *,
                box_cls: Type[Box[Scalar]],
                contour_cls: Type[Contour[Scalar]],
                empty_cls: Type[Empty],
                multipolygon_cls: Type[Multipolygon[Scalar]],
                multisegment_cls: Type[Multisegment[Scalar]],
                point_cls: Type[Point[Scalar]],
                polygon_cls: Type[Polygon[Scalar]],
                segment_cls: Type[Segment[Scalar]]) -> 'Context[Scalar]':
        self = super().__new__(cls)
        (
            self._box_cls, self._empty_cls, self._multipolygon_cls,
            self._multisegment_cls, self._point_cls, self._polygon_cls,
            self._segment_cls
        ) = (box_cls, empty_cls, multipolygon_cls, multisegment_cls, point_cls,
             polygon_cls, segment_cls)
        return self

    __repr__ = generate_repr(__new__,
                             argument_serializer=serializers.complex_,
                             with_module_name=True)
