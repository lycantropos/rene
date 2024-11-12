from rene import hints
from rene._hints import (
    Orienteer,
    SegmentsIntersectionScale,
    SegmentsIntersector,
)
from rene.enums import Relation

from .multisegmental import (
    relate_to_multipolygon as relate_multisegmental_to_multipolygon,
    relate_to_multisegmental as relate_multisegmental_to_multisegmental,
    relate_to_polygon as relate_multisegmental_to_polygon,
)
from .segment import relate_to_contour as relate_segment_to_contour
from .segment_endpoints import (
    relate_to_contour_segments as relate_segment_endpoints_to_contour_segments,
    relate_to_multisegment_segments as relate_segment_endpoints_to_multisegment_segments,  # noqa: E501
)


def relate_to_contour(
    first: hints.Contour[hints.Scalar],
    second: hints.Contour[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    return relate_multisegmental_to_multisegmental(
        first,
        second,
        lambda start, end, contour_segments: (
            relate_segment_endpoints_to_contour_segments(
                start, end, contour_segments, orienteer
            )
        ),
        lambda start, end, contour_segments: (
            relate_segment_endpoints_to_contour_segments(
                start, end, contour_segments, orienteer
            )
        ),
        orienteer,
        segments_intersector,
    )


def relate_to_multipolygon(
    contour: hints.Contour[hints.Scalar],
    multipolygon: hints.Multipolygon[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    return relate_multisegmental_to_multipolygon(
        contour, multipolygon, orienteer, segments_intersector
    )


def relate_to_multisegment(
    contour: hints.Contour[hints.Scalar],
    multisegment: hints.Multisegment[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersection_scale: SegmentsIntersectionScale[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    return relate_multisegmental_to_multisegmental(
        contour,
        multisegment,
        lambda start, end, multisegment_segments: (
            relate_segment_endpoints_to_multisegment_segments(
                start,
                end,
                multisegment_segments,
                orienteer,
                segments_intersection_scale,
            )
        ),
        lambda start, end, contour_segments: (
            relate_segment_endpoints_to_contour_segments(
                start, end, contour_segments, orienteer
            )
        ),
        orienteer,
        segments_intersector,
    )


def relate_to_polygon(
    contour: hints.Contour[hints.Scalar],
    polygon: hints.Polygon[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    return relate_multisegmental_to_polygon(
        contour, polygon, orienteer, segments_intersector
    )


def relate_to_segment(
    contour: hints.Contour[hints.Scalar],
    segment: hints.Segment[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    /,
) -> Relation:
    return relate_segment_to_contour(segment, contour, orienteer).complement
