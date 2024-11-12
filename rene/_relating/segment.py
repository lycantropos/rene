from __future__ import annotations

import typing as t
from itertools import chain

from rene import Relation, hints
from rene._hints import (
    Orienteer,
    SegmentsIntersectionScale,
    SegmentsIntersector,
)
from rene._utils import to_boxes_ids_with_intersection

from . import mixed
from .segment_endpoints import (
    relate_to_contour as relate_endpoints_to_contour,
    relate_to_multisegment as relate_endpoints_to_multisegment,
    relate_to_region as relate_endpoints_to_region,
    relate_to_segment_endpoints as relate_endpoints_to_endpoints,
)
from .utils import polygon_to_segments


def relate_to_contour(
    segment: hints.Segment[hints.Scalar],
    contour: hints.Contour[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    /,
) -> Relation:
    return relate_endpoints_to_contour(
        segment.start, segment.end, contour, orienteer
    )


def relate_to_multipolygon(
    segment: hints.Segment[hints.Scalar],
    multipolygon: hints.Multipolygon[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    segment_bounding_box = segment.bounding_box
    polygons = multipolygon.polygons
    polygons_bounding_boxes = [polygon.bounding_box for polygon in polygons]
    intersecting_polygons_ids = to_boxes_ids_with_intersection(
        polygons_bounding_boxes, segment_bounding_box
    )
    if not intersecting_polygons_ids:
        return Relation.DISJOINT
    min_max_x = min(
        segment_bounding_box.max_x,
        max(
            polygons_bounding_boxes[polygon_id].max_x
            for polygon_id in intersecting_polygons_ids
        ),
    )
    intersecting_polygons = [
        polygons[polygon_id]
        for polygon_id in intersecting_polygons_ids
        if polygons_bounding_boxes[polygon_id].min_x <= min_max_x
    ]
    assert intersecting_polygons
    return mixed.LinearShapedOperation.from_segments_iterables(
        [segment],
        chain.from_iterable(
            polygon_to_segments(polygon, segment_bounding_box)
            for polygon in intersecting_polygons
        ),
        orienteer,
        segments_intersector,
    ).to_relation(True, min_max_x)


def relate_to_multiregion(
    segment: hints.Segment[hints.Scalar],
    borders: t.Sequence[hints.Contour[hints.Scalar]],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    assert len(borders) > 1, borders
    segment_bounding_box = segment.bounding_box
    regions_bounding_boxes = [border.bounding_box for border in borders]
    intersecting_borders_ids = to_boxes_ids_with_intersection(
        regions_bounding_boxes, segment_bounding_box
    )
    if not intersecting_borders_ids:
        return Relation.DISJOINT
    min_max_x = min(
        segment_bounding_box.max_x,
        max(
            regions_bounding_boxes[border_id].max_x
            for border_id in intersecting_borders_ids
        ),
    )
    return mixed.LinearShapedOperation.from_segments_iterables(
        [segment],
        [edge for border in borders for edge in border.segments],
        orienteer,
        segments_intersector,
    ).to_relation(True, min_max_x)


def relate_to_multisegment(
    segment: hints.Segment[hints.Scalar],
    multisegment: hints.Multisegment[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersection_scale: SegmentsIntersectionScale[hints.Scalar],
    /,
) -> Relation:
    return relate_endpoints_to_multisegment(
        segment.start,
        segment.end,
        multisegment,
        orienteer,
        segments_intersection_scale,
    )


def relate_to_polygon(
    segment: hints.Segment[hints.Scalar],
    polygon: hints.Polygon[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    relation_without_holes = relate_to_region(
        segment, polygon.border, False, orienteer
    )
    holes = polygon.holes
    if holes and (
        relation_without_holes is Relation.WITHIN
        or relation_without_holes is Relation.ENCLOSED
    ):
        relation_with_holes = (
            relate_to_region(segment, holes[0], True, orienteer)
            if len(holes) == 1
            else relate_to_multiregion(
                segment, holes, orienteer, segments_intersector
            )
        )
        if relation_with_holes is Relation.DISJOINT:
            return relation_without_holes
        elif relation_with_holes is Relation.TOUCH:
            return Relation.ENCLOSED
        elif relation_with_holes is Relation.ENCLOSED:
            return Relation.TOUCH
        elif relation_with_holes is Relation.WITHIN:
            return Relation.DISJOINT
        else:
            return relation_with_holes
    else:
        return relation_without_holes


def relate_to_region(
    segment: hints.Segment[hints.Scalar],
    border: hints.Contour[hints.Scalar],
    reverse_orientation: bool,
    orienteer: Orienteer[hints.Scalar],
    /,
) -> Relation:
    return relate_endpoints_to_region(
        segment.start, segment.end, border, reverse_orientation, orienteer
    )


def relate_to_segment(
    first: hints.Segment[hints.Scalar],
    second: hints.Segment[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    /,
) -> Relation:
    return relate_endpoints_to_endpoints(
        first.start, first.end, second.start, second.end, orienteer
    )
