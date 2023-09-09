from __future__ import annotations

from rene import (Relation,
                  hints)
from .segment_endpoints import (
    relate_to_contour as relate_endpoints_to_contour,
    relate_to_multisegment as relate_endpoints_to_multisegment,
    relate_to_region as relate_endpoints_to_region,
    relate_to_segment_endpoints as relate_endpoints_to_endpoints
)


def relate_to_contour(
        segment: hints.Segment[hints.Scalar],
        contour: hints.Contour[hints.Scalar],
        /
) -> Relation:
    return relate_endpoints_to_contour(segment.start, segment.end, contour)


def relate_to_multisegment(
        segment: hints.Segment[hints.Scalar],
        multisegment: hints.Multisegment[hints.Scalar]
) -> Relation:
    return relate_endpoints_to_multisegment(segment.start, segment.end,
                                            multisegment)


def relate_to_segment(first: hints.Segment[hints.Scalar],
                      second: hints.Segment[hints.Scalar],
                      /) -> Relation:
    return relate_endpoints_to_endpoints(first.start, first.end, second.start,
                                         second.end)


def relate_to_region(segment: hints.Segment[hints.Scalar],
                     border: hints.Contour[hints.Scalar],
                     /) -> Relation:
    return relate_endpoints_to_region(segment.start, segment.end, border)
