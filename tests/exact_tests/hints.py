from typing import TypeVar

from rene import exact as _exact

Compound = (
    _exact.Contour
    | _exact.Empty
    | _exact.Multipolygon
    | _exact.Multisegment
    | _exact.Polygon
    | _exact.Segment
)
# here and after we use `TypeVar` instead of `Union` because of
# https://github.com/python/mypy/issues/6478
CompoundT = TypeVar(
    'CompoundT',
    _exact.Contour,
    _exact.Empty,
    _exact.Multipolygon,
    _exact.Multisegment,
    _exact.Polygon,
    _exact.Segment,
)
ClosedIdempotentCompoundT = TypeVar(
    'ClosedIdempotentCompoundT',
    _exact.Empty,
    _exact.Multipolygon,
    _exact.Multisegment,
    _exact.Polygon,
    _exact.Segment,
)
IdempotentMaybeLinearCompound = (
    _exact.Empty | _exact.Multisegment | _exact.Segment
)
MaybeLinearCompound = (
    _exact.Contour | _exact.Empty | _exact.Multisegment | _exact.Segment
)
MaybeShapedCompound = _exact.Empty | _exact.Multipolygon | _exact.Polygon
ClosedCompoundsPairT = TypeVar(
    'ClosedCompoundsPairT',
    tuple[MaybeLinearCompound, MaybeLinearCompound],
    tuple[MaybeShapedCompound, MaybeShapedCompound],
)
ClosedCompoundsTripletT = TypeVar(
    'ClosedCompoundsTripletT',
    tuple[MaybeLinearCompound, MaybeLinearCompound, MaybeLinearCompound],
    tuple[MaybeShapedCompound, MaybeShapedCompound, MaybeShapedCompound],
)
ClosedIdempotentCompoundsPairT = TypeVar(
    'ClosedIdempotentCompoundsPairT',
    tuple[IdempotentMaybeLinearCompound, IdempotentMaybeLinearCompound],
    tuple[MaybeShapedCompound, MaybeShapedCompound],
)
ClosedIdempotentCompoundsTripletT = TypeVar(
    'ClosedIdempotentCompoundsTripletT',
    tuple[
        IdempotentMaybeLinearCompound,
        IdempotentMaybeLinearCompound,
        IdempotentMaybeLinearCompound,
    ],
    tuple[MaybeShapedCompound, MaybeShapedCompound, MaybeShapedCompound],
)
