from typing import Type

from rene.hints import (Box,
                        Contour,
                        Empty,
                        Multipolygon,
                        Multisegment,
                        Point,
                        Polygon,
                        Segment)


class Context:
    _box_cls: Type[Box]
    _contour_cls: Type[Contour]
    _empty_cls: Type[Empty]
    _multipolygon_cls: Type[Multipolygon]
    _multisegment_cls: Type[Multisegment]
    _point_cls: Type[Point]
    _polygon_cls: Type[Polygon]
    _segment_cls: Type[Segment]

    __slots__ = (
        '_box_cls', '_contour_cls', '_empty_cls', '_multipolygon_cls',
        '_multisegment_cls', '_point_cls', '_polygon_cls', '_segment_cls'
    )

    def __new__(cls,
                *,
                box_cls: Type[Box],
                contour_cls: Type[Contour],
                empty_cls: Type[Empty],
                multipolygon_cls: Type[Multipolygon],
                multisegment_cls: Type[Multisegment],
                point_cls: Type[Point],
                polygon_cls: Type[Polygon],
                segment_cls: Type[Segment]) -> 'Context':
        self = super().__new__(cls)
        (
            self._box_cls, self._empty_cls, self._multipolygon_cls,
            self._multisegment_cls, self._point_cls, self._polygon_cls,
            self._segment_cls
        ) = (box_cls, empty_cls, multipolygon_cls, multisegment_cls, point_cls,
             polygon_cls, segment_cls)
        return self

    @property
    def box_cls(self) -> Type[Box]:
        return self._box_cls

    @property
    def contour_cls(self) -> Type[Contour]:
        return self._contour_cls

    @property
    def empty_cls(self) -> Type[Empty]:
        return self._empty_cls

    @property
    def multipolygon_cls(self) -> Type[Multipolygon]:
        return self._multipolygon_cls

    @property
    def multisegment_cls(self) -> Type[Multisegment]:
        return self._multisegment_cls

    @property
    def point_cls(self) -> Type[Point]:
        return self._point_cls

    @property
    def polygon_cls(self) -> Type[Polygon]:
        return self._polygon_cls

    @property
    def segment_cls(self) -> Type[Segment]:
        return self._segment_cls
