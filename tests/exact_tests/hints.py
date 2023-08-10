import typing as _t

from rene import exact as _exact

Compound = _t.Union[
    _exact.Contour, _exact.Empty, _exact.Multipolygon,
    _exact.Multisegment, _exact.Polygon, _exact.Segment
]
# here and after we use `TypeVar` instead of `Union` because of
# https://github.com/python/mypy/issues/6478
CompoundT = _t.TypeVar(
        'CompoundT', _exact.Contour, _exact.Empty, _exact.Multipolygon,
        _exact.Multisegment, _exact.Polygon, _exact.Segment
)
ClosedIdempotentCompoundT = _t.TypeVar(
        'ClosedIdempotentCompoundT', _exact.Empty, _exact.Multipolygon,
        _exact.Multisegment, _exact.Polygon, _exact.Segment
)
IdempotentMaybeLinearCompound = _t.Union[
    _exact.Empty, _exact.Multisegment, _exact.Segment
]
MaybeLinearCompound = _t.Union[
    _exact.Contour, _exact.Empty, _exact.Multisegment, _exact.Segment
]
MaybeShapedCompound = _t.Union[
    _exact.Empty, _exact.Multipolygon, _exact.Polygon
]
ClosedCompoundsPairT = _t.TypeVar(
        'ClosedCompoundsPairT',
        _t.Tuple[MaybeLinearCompound, MaybeLinearCompound],
        _t.Tuple[MaybeShapedCompound, MaybeShapedCompound]
)
ClosedCompoundsTripletT = _t.TypeVar(
        'ClosedCompoundsTripletT',
        _t.Tuple[
            MaybeLinearCompound, MaybeLinearCompound, MaybeLinearCompound
        ],
        _t.Tuple[MaybeShapedCompound, MaybeShapedCompound, MaybeShapedCompound]
)
ClosedIdempotentCompoundsPairT = _t.TypeVar(
        'ClosedIdempotentCompoundsPairT',
        _t.Tuple[IdempotentMaybeLinearCompound, IdempotentMaybeLinearCompound],
        _t.Tuple[MaybeShapedCompound, MaybeShapedCompound]
)
ClosedIdempotentCompoundsTripletT = _t.TypeVar(
        'ClosedIdempotentCompoundsTripletT',
        _t.Tuple[
            IdempotentMaybeLinearCompound, IdempotentMaybeLinearCompound,
            IdempotentMaybeLinearCompound
        ],
        _t.Tuple[MaybeShapedCompound, MaybeShapedCompound, MaybeShapedCompound]
)
