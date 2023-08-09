import typing as _t

from rene import exact as _exact

Compound = _t.Union[
        _exact.Contour, _exact.Empty, _exact.Multipolygon,
        _exact.Multisegment, _exact.Polygon, _exact.Segment
]
CompoundT = _t.TypeVar(
        'CompoundT', _exact.Contour, _exact.Empty, _exact.Multipolygon,
        _exact.Multisegment, _exact.Polygon, _exact.Segment
)
MaybeLinearCompound = _t.Union[
    _exact.Contour, _exact.Empty, _exact.Multisegment, _exact.Segment
]
MaybeShapedCompound = _t.Union[
    _exact.Empty, _exact.Multipolygon, _exact.Polygon
]
ClosedCompoundsPair = _t.Union[
    # _t.Tuple[MaybeLinearCompound, MaybeLinearCompound],
    _t.Tuple[MaybeShapedCompound, MaybeShapedCompound]
]
ClosedCompoundsTriplet = _t.Union[
    # _t.Tuple[MaybeLinearCompound, MaybeLinearCompound, MaybeLinearCompound],
    _t.Tuple[MaybeShapedCompound, MaybeShapedCompound, MaybeShapedCompound]
]
