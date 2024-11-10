from __future__ import annotations

import typing as t
from itertools import chain

from rene import Relation, hints
from rene._hints import Orienteer, SegmentsIntersector
from rene._utils import to_boxes_ids_with_intersection

from . import mixed, shaped
from .utils import polygon_to_segments


def relate_to_contour(
    polygon: hints.Polygon[hints.Scalar],
    contour: hints.Contour[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    return relate_to_multisegmental(polygon, contour, orienteer, segments_intersector)


def relate_to_multipolygon(
    polygon: hints.Polygon[hints.Scalar],
    multipolygon: hints.Multipolygon[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    polygon_bounding_box, multipolygon_bounding_box = (
        polygon.bounding_box,
        multipolygon.bounding_box,
    )
    if polygon_bounding_box.disjoint_with(multipolygon_bounding_box):
        return Relation.DISJOINT
    polygons = multipolygon.polygons
    polygons_bounding_boxes = [polygon.bounding_box for polygon in polygons]
    intersecting_polygons_ids = to_boxes_ids_with_intersection(
        polygons_bounding_boxes, polygon_bounding_box
    )
    if not intersecting_polygons_ids:
        return Relation.DISJOINT
    min_max_x = min(
        polygon_bounding_box.max_x,
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
    return shaped.Operation.from_segments_iterables(
        polygon_to_segments(polygon, multipolygon_bounding_box),
        chain.from_iterable(
            polygon_to_segments(polygon, polygon_bounding_box)
            for polygon in intersecting_polygons
        ),
        orienteer,
        segments_intersector,
    ).to_relation(True, len(intersecting_polygons) == len(polygons), min_max_x)


def relate_to_multisegment(
    polygon: hints.Polygon[hints.Scalar],
    multisegment: hints.Multisegment[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    return relate_to_multisegmental(
        polygon, multisegment, orienteer, segments_intersector
    )


def relate_to_polygon(
    first: hints.Polygon[hints.Scalar],
    second: hints.Polygon[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    first_bounding_box, second_bounding_box = (
        first.bounding_box,
        second.bounding_box,
    )
    if first_bounding_box.disjoint_with(second_bounding_box):
        return Relation.DISJOINT
    min_max_x = min(first_bounding_box.max_x, second_bounding_box.max_x)
    return shaped.Operation.from_segments_iterables(
        polygon_to_segments(first, second_bounding_box),
        polygon_to_segments(second, first_bounding_box),
        orienteer,
        segments_intersector,
    ).to_relation(True, True, min_max_x)


def relate_to_segment(
    polygon: hints.Polygon[hints.Scalar],
    segment: hints.Segment[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    polygon_bounding_box, segment_bounding_box = (
        polygon.bounding_box,
        segment.bounding_box,
    )
    if polygon_bounding_box.disjoint_with(segment_bounding_box):
        return Relation.DISJOINT
    min_max_x = min(segment_bounding_box.max_x, polygon_bounding_box.max_x)
    return mixed.ShapedLinearOperation.from_segments_iterables(
        polygon_to_segments(polygon, segment_bounding_box),
        [segment],
        orienteer,
        segments_intersector,
    ).to_relation(True, min_max_x)


_Multisegmental = t.Union[hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar]]


def relate_to_multisegmental(
    polygon: hints.Polygon[hints.Scalar],
    multisegmental: _Multisegmental[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    polygon_bounding_box, multisegmental_bounding_box = (
        polygon.bounding_box,
        multisegmental.bounding_box,
    )
    if polygon_bounding_box.disjoint_with(multisegmental_bounding_box):
        return Relation.DISJOINT
    multisegmental_segments = multisegmental.segments
    multisegmental_boxes = [segment.bounding_box for segment in multisegmental_segments]
    intersecting_segments_ids = to_boxes_ids_with_intersection(
        multisegmental_boxes, polygon_bounding_box
    )
    if not intersecting_segments_ids:
        return Relation.DISJOINT
    elif len(intersecting_segments_ids) == 1:
        intersecting_segment = multisegmental_segments[intersecting_segments_ids[0]]
        relation = relate_to_segment(
            polygon, intersecting_segment, orienteer, segments_intersector
        )
        return (
            Relation.TOUCH
            if relation is Relation.COMPONENT
            else (
                Relation.CROSS
                if (relation is Relation.ENCLOSED or relation is Relation.WITHIN)
                else relation
            )
        )
    min_max_x = min(
        polygon_bounding_box.max_x,
        max(
            multisegmental_boxes[segment_id].max_x
            for segment_id in intersecting_segments_ids
        ),
    )
    return mixed.ShapedLinearOperation.from_segments_iterables(
        polygon_to_segments(polygon, multisegmental_bounding_box),
        [
            multisegmental_segments[segment_id]
            for segment_id in intersecting_segments_ids
        ],
        orienteer,
        segments_intersector,
    ).to_relation(
        len(intersecting_segments_ids) == len(multisegmental_segments),
        min_max_x,
    )
