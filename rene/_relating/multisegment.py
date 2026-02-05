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
from .segment import relate_to_multisegment as relate_segment_to_multisegment
from .segment_endpoints import (
    relate_to_contour_segments as relate_segment_to_contour_segments,
    relate_to_multisegment_segments as relate_segment_to_multisegment_segments,
)


def relate_to_contour(
    multisegment: hints.Multisegment[hints.ScalarT],
    contour: hints.Contour[hints.ScalarT],
    orienteer: Orienteer[hints.ScalarT],
    segments_intersection_scale: SegmentsIntersectionScale[hints.ScalarT],
    segments_intersector: SegmentsIntersector[hints.ScalarT],
    /,
) -> Relation:
    return relate_multisegmental_to_multisegmental(
        multisegment,
        contour,
        lambda start, end, contour_segments: (
            relate_segment_to_contour_segments(
                start, end, contour_segments, orienteer
            )
        ),
        lambda start, end, multisegment_segments: (
            relate_segment_to_multisegment_segments(
                start,
                end,
                multisegment_segments,
                orienteer,
                segments_intersection_scale,
            )
        ),
        orienteer,
        segments_intersector,
    )


def relate_to_multipolygon(
    multisegment: hints.Multisegment[hints.ScalarT],
    multipolygon: hints.Multipolygon[hints.ScalarT],
    orienteer: Orienteer[hints.ScalarT],
    segments_intersector: SegmentsIntersector[hints.ScalarT],
    /,
) -> Relation:
    return relate_multisegmental_to_multipolygon(
        multisegment, multipolygon, orienteer, segments_intersector
    )


def relate_to_multisegment(
    first: hints.Multisegment[hints.ScalarT],
    second: hints.Multisegment[hints.ScalarT],
    orienteer: Orienteer[hints.ScalarT],
    segments_intersection_scale: SegmentsIntersectionScale[hints.ScalarT],
    segments_intersector: SegmentsIntersector[hints.ScalarT],
    /,
) -> Relation:
    return relate_multisegmental_to_multisegmental(
        first,
        second,
        lambda start, end, multisegment_segments: (
            relate_segment_to_multisegment_segments(
                start,
                end,
                multisegment_segments,
                orienteer,
                segments_intersection_scale,
            )
        ),
        lambda start, end, multisegment_segments: (
            relate_segment_to_multisegment_segments(
                start,
                end,
                multisegment_segments,
                orienteer,
                segments_intersection_scale,
            )
        ),
        orienteer,
        segments_intersector,
    )


def relate_to_polygon(
    multisegment: hints.Multisegment[hints.ScalarT],
    polygon: hints.Polygon[hints.ScalarT],
    orienteer: Orienteer[hints.ScalarT],
    segments_intersector: SegmentsIntersector[hints.ScalarT],
    /,
) -> Relation:
    return relate_multisegmental_to_polygon(
        multisegment, polygon, orienteer, segments_intersector
    )


def relate_to_segment(
    multisegment: hints.Multisegment[hints.ScalarT],
    segment: hints.Segment[hints.ScalarT],
    orienteer: Orienteer[hints.ScalarT],
    segments_intersection_scale: SegmentsIntersectionScale[hints.ScalarT],
    /,
) -> Relation:
    return relate_segment_to_multisegment(
        segment, multisegment, orienteer, segments_intersection_scale
    ).complement
