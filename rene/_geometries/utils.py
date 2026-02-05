from typing import Any

from typing_extensions import TypeIs

from rene import hints
from rene._context import Context


def is_compound(
    value: Any, /, *, context: Context[hints.ScalarT]
) -> TypeIs[hints.Compound[hints.ScalarT]]:
    return isinstance(
        value,
        (
            context.contour_cls,
            context.empty_cls,
            context.multisegment_cls,
            context.multipolygon_cls,
            context.polygon_cls,
            context.segment_cls,
        ),
    )


def is_contour(
    value: Any, /, *, context: Context[hints.ScalarT]
) -> TypeIs[hints.Contour[hints.ScalarT]]:
    return isinstance(value, context.contour_cls)


def is_empty(
    value: Any, /, *, context: Context[hints.ScalarT]
) -> TypeIs[hints.Empty[hints.ScalarT]]:
    return isinstance(value, context.empty_cls)


def is_multipolygon(
    value: Any, /, *, context: Context[hints.ScalarT]
) -> TypeIs[hints.Multipolygon[hints.ScalarT]]:
    return isinstance(value, context.multipolygon_cls)


def is_multisegment(
    value: Any, /, *, context: Context[hints.ScalarT]
) -> TypeIs[hints.Multisegment[hints.ScalarT]]:
    return isinstance(value, context.multisegment_cls)


def is_multisegmental(
    value: Any, /, *, context: Context[hints.ScalarT]
) -> TypeIs[hints.Contour[hints.ScalarT] | hints.Multisegment[hints.ScalarT]]:
    return isinstance(value, (context.contour_cls, context.multisegment_cls))


def is_polygon(
    value: Any, /, *, context: Context[hints.ScalarT]
) -> TypeIs[hints.Polygon[hints.ScalarT]]:
    return isinstance(value, context.polygon_cls)


def is_segment(
    value: Any, /, *, context: Context[hints.ScalarT]
) -> TypeIs[hints.Segment[hints.ScalarT]]:
    return isinstance(value, context.segment_cls)
