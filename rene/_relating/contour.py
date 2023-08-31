from rene import (Relation,
                  hints)
from .multisegmental import relate_to_multisegmental
from .segment import (
    relate_to_contour_segments as relate_segment_to_contour_segments,
    relate_to_multisegment_segments as relate_segment_to_multisegment_segments,
)


def relate_to_contour(
        first: hints.Contour[hints.Scalar],
        second: hints.Contour[hints.Scalar],
        /
) -> Relation:
    return relate_to_multisegmental(
            first, second, relate_segment_to_contour_segments,
            relate_segment_to_contour_segments
    )


def relate_to_multisegment(
        contour: hints.Contour[hints.Scalar],
        multisegment: hints.Multisegment[hints.Scalar],
        /
) -> Relation:
    return relate_to_multisegmental(
            contour, multisegment, relate_segment_to_multisegment_segments,
            relate_segment_to_contour_segments
    )
