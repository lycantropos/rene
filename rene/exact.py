try:
    from ._cexact import (Box,
                          ConstrainedDelaunayTriangulation,
                          Contour,
                          DelaunayTriangulation,
                          Empty,
                          Multipolygon,
                          Multisegment,
                          Point,
                          Polygon,
                          Segment,
                          Trapezoidation as _RawTrapezoidation)
except ImportError:
    from ._exact import (Box,
                         ConstrainedDelaunayTriangulation,
                         Contour,
                         DelaunayTriangulation,
                         Empty,
                         Multipolygon,
                         Multisegment,
                         Point,
                         Polygon,
                         Segment,
                         Trapezoidation)
else:
    import random as _random
    import typing as _t

    import typing_extensions as _te

    from ._crene import Location as _Location
    from ._utils import (
        polygon_to_segments_count as _polygon_to_segments_count,
        validate_seed as _validate_seed
    )
    from .hints import Seeder as _Seeder


    @_te.final
    class Trapezoidation:
        @classmethod
        def from_multisegment(cls,
                              multisegment: Multisegment,
                              /,
                              *,
                              seeder: _t.Optional[_Seeder] = None) -> _te.Self:
            seed = (_random.randint(0, len(multisegment.segments))
                    if seeder is None
                    else seeder())
            _validate_seed(seed)
            return cls(_RawTrapezoidation.from_multisegment(multisegment,
                                                            seed))

        @classmethod
        def from_polygon(cls,
                         polygon: Polygon,
                         /,
                         *,
                         seeder: _t.Optional[_Seeder] = None) -> _te.Self:
            seed = (_random.randint(0, _polygon_to_segments_count(polygon))
                    if seeder is None
                    else seeder())
            _validate_seed(seed)
            return cls(_RawTrapezoidation.from_polygon(polygon, seed))

        @property
        def height(self) -> int:
            return self._raw.height

        def locate(self, point: Point, /) -> _Location:
            return self._raw.locate(point)

        _raw: _RawTrapezoidation

        __slots__ = '_raw',

        def __init_subclass__(cls, /, **_kwargs: _t.Any) -> _t.NoReturn:
            raise TypeError(f'type {cls.__qualname__!r} '
                            'is not an acceptable base type')

        def __new__(cls, raw: _RawTrapezoidation, /) -> _te.Self:
            self = super().__new__(cls)
            self._raw = raw
            return self

        def __contains__(self, point: Point, /) -> bool:
            return self._raw.__contains__(point)
