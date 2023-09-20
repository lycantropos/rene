from rene import (Relation,
                  hints)
from .multisegmental import (
    relate_to_multipolygon as relate_multisegmental_to_multipolygon,
    relate_to_multisegmental as relate_multisegmental_to_multisegmental,
    relate_to_polygon as relate_multisegmental_to_polygon
)
from .segment import relate_to_multisegment as relate_segment_to_multisegment
from .segment_endpoints import (
    relate_to_contour_segments as relate_segment_to_contour_segments,
    relate_to_multisegment_segments as relate_segment_to_multisegment_segments
)


def relate_to_contour(multisegment: hints.Multisegment[hints.Scalar],
                      contour: hints.Contour[hints.Scalar],
                      /) -> Relation:
    return relate_multisegmental_to_multisegmental(
            multisegment, contour, relate_segment_to_contour_segments,
            relate_segment_to_multisegment_segments
    )


def relate_to_multipolygon(multisegment: hints.Multisegment[hints.Scalar],
                           multipolygon: hints.Multipolygon[hints.Scalar],
                           /) -> Relation:
    return relate_multisegmental_to_multipolygon(multisegment, multipolygon)


def relate_to_multisegment(first: hints.Multisegment[hints.Scalar],
                           second: hints.Multisegment[hints.Scalar],
                           /) -> Relation:
    return relate_multisegmental_to_multisegmental(
            first, second, relate_segment_to_multisegment_segments,
            relate_segment_to_multisegment_segments
    )


def relate_to_polygon(multisegment: hints.Multisegment[hints.Scalar],
                      polygon: hints.Polygon[hints.Scalar],
                      /) -> Relation:
    return relate_multisegmental_to_polygon(multisegment, polygon)


def relate_to_segment(multisegment: hints.Multisegment[hints.Scalar],
                      segment: hints.Segment[hints.Scalar],
                      /) -> Relation:
    return relate_segment_to_multisegment(segment, multisegment).complement
