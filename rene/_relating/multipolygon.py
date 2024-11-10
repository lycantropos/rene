from __future__ import annotations

import typing as t
from itertools import chain

from rene import Relation, hints
from rene._hints import Orienteer, SegmentsIntersector
from rene._utils import to_boxes_ids_with_intersection

from . import mixed, shaped
from .utils import polygon_to_segments


def relate_to_contour(
    multipolygon: hints.Multipolygon[hints.Scalar],
    contour: hints.Contour[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    return relate_to_multisegmental(
        multipolygon, contour, orienteer, segments_intersector
    )


def relate_to_multipolygon(
    first: hints.Multipolygon[hints.Scalar],
    second: hints.Multipolygon[hints.Scalar],
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
    first_polygons = first.polygons
    first_boxes = [polygon.bounding_box for polygon in first_polygons]
    first_intersecting_polygons_ids = to_boxes_ids_with_intersection(
        first_boxes, second_bounding_box
    )
    if not first_intersecting_polygons_ids:
        return Relation.DISJOINT
    second_polygons = second.polygons
    second_boxes = [polygon.bounding_box for polygon in second_polygons]
    second_intersecting_polygons_ids = to_boxes_ids_with_intersection(
        second_boxes, first_bounding_box
    )
    if not second_intersecting_polygons_ids:
        return Relation.DISJOINT
    min_max_x = min(
        max(
            first_boxes[polygon_id].max_x
            for polygon_id in first_intersecting_polygons_ids
        ),
        max(
            second_boxes[polygon_id].max_x
            for polygon_id in second_intersecting_polygons_ids
        ),
    )
    first_intersecting_polygons = [
        first_polygons[polygon_id]
        for polygon_id in first_intersecting_polygons_ids
        if first_boxes[polygon_id].min_x <= min_max_x
    ]
    if not first_intersecting_polygons:
        return Relation.DISJOINT
    second_intersecting_polygons = [
        second_polygons[polygon_id]
        for polygon_id in second_intersecting_polygons_ids
        if second_boxes[polygon_id].min_x <= min_max_x
    ]
    if not second_intersecting_polygons:
        return Relation.DISJOINT
    return shaped.Operation.from_segments_iterables(
        chain.from_iterable(
            polygon_to_segments(polygon, second_bounding_box)
            for polygon in first_intersecting_polygons
        ),
        chain.from_iterable(
            polygon_to_segments(polygon, first_bounding_box)
            for polygon in second_intersecting_polygons
        ),
        orienteer,
        segments_intersector,
    ).to_relation(
        len(first_intersecting_polygons) == len(first_polygons),
        len(second_intersecting_polygons) == len(second_polygons),
        min_max_x,
    )


def relate_to_multisegment(
    multipolygon: hints.Multipolygon[hints.Scalar],
    multisegment: hints.Multisegment[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    return relate_to_multisegmental(
        multipolygon, multisegment, orienteer, segments_intersector
    )


def relate_to_polygon(
    multipolygon: hints.Multipolygon[hints.Scalar],
    polygon: hints.Polygon[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    multipolygon_bounding_box, polygon_bounding_box = (
        multipolygon.bounding_box,
        polygon.bounding_box,
    )
    if multipolygon_bounding_box.disjoint_with(polygon_bounding_box):
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
        chain.from_iterable(
            polygon_to_segments(polygon, polygon_bounding_box)
            for polygon in intersecting_polygons
        ),
        polygon_to_segments(polygon, multipolygon_bounding_box),
        orienteer,
        segments_intersector,
    ).to_relation(len(intersecting_polygons) == len(polygons), True, min_max_x)


def relate_to_segment(
    multipolygon: hints.Multipolygon[hints.Scalar],
    segment: hints.Segment[hints.Scalar],
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
    return mixed.ShapedLinearOperation.from_segments_iterables(
        chain.from_iterable(
            polygon_to_segments(polygon, segment_bounding_box)
            for polygon in intersecting_polygons
        ),
        [segment],
        orienteer,
        segments_intersector,
    ).to_relation(True, min_max_x)


_Multisegmental = t.Union[hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar]]


def relate_to_multisegmental(
    multipolygon: hints.Multipolygon[hints.Scalar],
    multisegmental: _Multisegmental[hints.Scalar],
    orienteer: Orienteer[hints.Scalar],
    segments_intersector: SegmentsIntersector[hints.Scalar],
    /,
) -> Relation:
    multipolygon_bounding_box, multisegmental_bounding_box = (
        multipolygon.bounding_box,
        multisegmental.bounding_box,
    )
    if multipolygon_bounding_box.disjoint_with(multisegmental_bounding_box):
        return Relation.DISJOINT
    multisegmental_segments = multisegmental.segments
    multisegmental_boxes = [segment.bounding_box for segment in multisegmental_segments]
    intersecting_segments_ids = to_boxes_ids_with_intersection(
        multisegmental_boxes, multipolygon_bounding_box
    )
    if not intersecting_segments_ids:
        return Relation.DISJOINT
    elif len(intersecting_segments_ids) == 1:
        intersecting_segment = multisegmental_segments[intersecting_segments_ids[0]]
        relation = relate_to_segment(
            multipolygon, intersecting_segment, orienteer, segments_intersector
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
    multipolygon_polygons = multipolygon.polygons
    multipolygon_boxes = [polygon.bounding_box for polygon in multipolygon_polygons]
    intersecting_polygons_ids = to_boxes_ids_with_intersection(
        multipolygon_boxes, multisegmental_bounding_box
    )
    min_max_x = min(
        max(
            multipolygon_boxes[polygon_id].max_x
            for polygon_id in intersecting_polygons_ids
        ),
        max(
            multisegmental_boxes[segment_id].max_x
            for segment_id in intersecting_segments_ids
        ),
    )
    intersecting_polygons = [
        multipolygon_polygons[polygon_id]
        for polygon_id in intersecting_polygons_ids
        if multipolygon_boxes[polygon_id].min_x <= min_max_x
    ]
    assert intersecting_polygons
    intersecting_segments = [
        multisegmental_segments[segment_id]
        for segment_id in intersecting_segments_ids
        if multisegmental_boxes[segment_id].min_x <= min_max_x
    ]
    assert intersecting_segments
    return mixed.ShapedLinearOperation.from_segments_iterables(
        chain.from_iterable(
            polygon_to_segments(polygon, multisegmental_bounding_box)
            for polygon in intersecting_polygons
        ),
        intersecting_segments,
        orienteer,
        segments_intersector,
    ).to_relation(
        len(intersecting_segments_ids) == len(multisegmental_segments),
        min_max_x,
    )
