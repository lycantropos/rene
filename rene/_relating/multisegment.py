from rene import (Relation,
                  hints)
from .multisegmental import relate_to_multisegmental
from .segment import relate_to_multisegment as relate_segment_to_multisegment
from .segment_endpoints import (
    relate_to_contour_segments as relate_segment_to_contour_segments,
    relate_to_multisegment_segments as relate_segment_to_multisegment_segments
)


def relate_to_contour(
        multisegment: hints.Multisegment[hints.Scalar],
        contour: hints.Contour[hints.Scalar],
        /
) -> Relation:
    return relate_to_multisegmental(
            multisegment, contour, relate_segment_to_contour_segments,
            relate_segment_to_multisegment_segments
    )


def relate_to_multisegment(
        first: hints.Multisegment[hints.Scalar],
        second: hints.Multisegment[hints.Scalar],
        /
) -> Relation:
    return relate_to_multisegmental(
            first, second, relate_segment_to_multisegment_segments,
            relate_segment_to_multisegment_segments
    )


def relate_to_segment(multisegment: hints.Multisegment[hints.Scalar],
                      segment: hints.Segment[hints.Scalar],
                      /) -> Relation:
    return relate_segment_to_multisegment(segment, multisegment).complement
