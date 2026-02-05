from collections.abc import Callable, Sequence
from itertools import chain
from typing import TypeAlias

from rene import hints
from rene._hints import Orienteer, SegmentsIntersector
from rene._utils import to_boxes_ids_with_intersection
from rene.enums import Relation

from . import linear, mixed
from .segment import (
    relate_to_multipolygon as relate_segment_to_multipolygon,
    relate_to_polygon as relate_segment_to_polygon,
)
from .utils import polygon_to_segments

_Multisegmental: TypeAlias = (
    hints.Contour[hints.ScalarT] | hints.Multisegment[hints.ScalarT]
)
_EndpointsToSegmentsRelater: TypeAlias = Callable[
    [
        hints.Point[hints.ScalarT],
        hints.Point[hints.ScalarT],
        Sequence[hints.Segment[hints.ScalarT]],
    ],
    Relation,
]


def relate_to_multipolygon(
    multisegmental: _Multisegmental[hints.ScalarT],
    multipolygon: hints.Multipolygon[hints.ScalarT],
    orienteer: Orienteer[hints.ScalarT],
    segments_intersector: SegmentsIntersector[hints.ScalarT],
    /,
) -> Relation:
    multisegmental_bounding_box, multipolygon_bounding_box = (
        multisegmental.bounding_box,
        multipolygon.bounding_box,
    )
    if multisegmental_bounding_box.disjoint_with(multipolygon_bounding_box):
        return Relation.DISJOINT
    multisegmental_segments = multisegmental.segments
    multisegmental_boxes = [
        segment.bounding_box for segment in multisegmental_segments
    ]
    intersecting_segments_ids = to_boxes_ids_with_intersection(
        multisegmental_boxes, multipolygon_bounding_box
    )
    if not intersecting_segments_ids:
        return Relation.DISJOINT
    if len(intersecting_segments_ids) == 1:
        intersecting_segment = multisegmental_segments[
            intersecting_segments_ids[0]
        ]
        relation = relate_segment_to_multipolygon(
            intersecting_segment, multipolygon, orienteer, segments_intersector
        )
        return (
            Relation.TOUCH
            if relation is Relation.COMPONENT
            else (
                Relation.CROSS
                if (
                    relation is Relation.ENCLOSED
                    or relation is Relation.WITHIN
                )
                else relation
            )
        )
    multipolygon_polygons = multipolygon.polygons
    multipolygon_boxes = [
        polygon.bounding_box for polygon in multipolygon_polygons
    ]
    intersecting_polygons_ids = to_boxes_ids_with_intersection(
        multipolygon_boxes, multisegmental_bounding_box
    )
    if not intersecting_polygons_ids:
        return Relation.DISJOINT
    min_max_x = min(
        max(
            multisegmental_boxes[segment_id].max_x
            for segment_id in intersecting_segments_ids
        ),
        max(
            multipolygon_boxes[polygon_id].max_x
            for polygon_id in intersecting_polygons_ids
        ),
    )
    intersecting_segments = [
        multisegmental_segments[segment_id]
        for segment_id in intersecting_segments_ids
        if multisegmental_boxes[segment_id].min_x <= min_max_x
    ]
    assert intersecting_segments
    intersecting_polygons = [
        multipolygon_polygons[polygon_id]
        for polygon_id in intersecting_polygons_ids
        if multipolygon_boxes[polygon_id].min_x <= min_max_x
    ]
    assert intersecting_polygons
    return mixed.LinearShapedOperation.from_segments_iterables(
        intersecting_segments,
        chain.from_iterable(
            polygon_to_segments(polygon, multisegmental_bounding_box)
            for polygon in intersecting_polygons
        ),
        orienteer,
        segments_intersector,
    ).to_relation(
        linear_is_subset_of_shaped=len(intersecting_segments_ids)
        == len(multisegmental_segments),
        min_max_x=min_max_x,
    )


def relate_to_multisegmental(
    first: _Multisegmental[hints.ScalarT],
    second: _Multisegmental[hints.ScalarT],
    first_to_second_relater: _EndpointsToSegmentsRelater[hints.ScalarT],
    second_to_first_relater: _EndpointsToSegmentsRelater[hints.ScalarT],
    orienteer: Orienteer[hints.ScalarT],
    segments_intersector: SegmentsIntersector[hints.ScalarT],
    /,
) -> Relation:
    first_bounding_box, second_bounding_box = (
        first.bounding_box,
        second.bounding_box,
    )
    if first_bounding_box.disjoint_with(second_bounding_box):
        return Relation.DISJOINT
    first_segments = first.segments
    first_boxes = [segment.bounding_box for segment in first_segments]
    first_intersecting_segments_ids = to_boxes_ids_with_intersection(
        first_boxes, second_bounding_box
    )
    if not first_intersecting_segments_ids:
        return Relation.DISJOINT
    if len(first_intersecting_segments_ids) == 1:
        first_segment = first_segments[first_intersecting_segments_ids[0]]
        relation = first_to_second_relater(
            first_segment.start, first_segment.end, second.segments
        )
        return (
            Relation.OVERLAP
            if relation is Relation.COMPONENT or relation is Relation.EQUAL
            else relation
        )
    second_segments = second.segments
    second_boxes = [segment.bounding_box for segment in second_segments]
    second_intersecting_segments_ids = to_boxes_ids_with_intersection(
        second_boxes, first_bounding_box
    )
    if not second_intersecting_segments_ids:
        return Relation.DISJOINT
    if len(second_intersecting_segments_ids) == 1:
        second_segment = second_segments[second_intersecting_segments_ids[0]]
        relation = second_to_first_relater(
            second_segment.start,
            second_segment.end,
            [
                first_segments[segment_id]
                for segment_id in first_intersecting_segments_ids
            ],
        )
        return (
            Relation.OVERLAP
            if (
                relation is Relation.COMPONENT
                or (
                    (
                        relation is Relation.COMPOSITE
                        or relation is Relation.EQUAL
                    )
                    and (
                        len(first_intersecting_segments_ids)
                        != len(first_segments)
                    )
                )
            )
            else relation.complement
        )
    max_min_x = max(
        min(
            first_boxes[segment_id].min_x
            for segment_id in first_intersecting_segments_ids
        ),
        min(
            second_boxes[segment_id].min_x
            for segment_id in second_intersecting_segments_ids
        ),
    )
    min_max_x = min(
        max(
            first_boxes[segment_id].max_x
            for segment_id in first_intersecting_segments_ids
        ),
        max(
            second_boxes[segment_id].max_x
            for segment_id in second_intersecting_segments_ids
        ),
    )
    first_intersecting_segments = [
        first_segments[segment_id]
        for segment_id in first_intersecting_segments_ids
        if (
            max_min_x <= first_boxes[segment_id].max_x
            and first_boxes[segment_id].min_x <= min_max_x
        )
    ]
    if not first_intersecting_segments:
        return Relation.DISJOINT
    second_intersecting_segments = [
        second_segments[segment_id]
        for segment_id in second_intersecting_segments_ids
        if (
            max_min_x <= second_boxes[segment_id].max_x
            and second_boxes[segment_id].min_x <= min_max_x
        )
    ]
    assert second_intersecting_segments
    return linear.Operation.from_segments_iterables(
        first_intersecting_segments,
        second_intersecting_segments,
        orienteer,
        segments_intersector,
    ).to_relation(
        first_is_subset=(
            len(first_intersecting_segments) == len(first_segments)
        ),
        second_is_subset=(
            len(second_intersecting_segments) == len(second_segments)
        ),
        min_max_x=min_max_x,
    )


def relate_to_polygon(
    multisegmental: _Multisegmental[hints.ScalarT],
    polygon: hints.Polygon[hints.ScalarT],
    orienteer: Orienteer[hints.ScalarT],
    segments_intersector: SegmentsIntersector[hints.ScalarT],
    /,
) -> Relation:
    multisegmental_bounding_box, polygon_bounding_box = (
        multisegmental.bounding_box,
        polygon.bounding_box,
    )
    if multisegmental_bounding_box.disjoint_with(polygon_bounding_box):
        return Relation.DISJOINT
    multisegmental_segments = multisegmental.segments
    multisegmental_boxes = [
        segment.bounding_box for segment in multisegmental_segments
    ]
    intersecting_segments_ids = to_boxes_ids_with_intersection(
        multisegmental_boxes, polygon_bounding_box
    )
    if not intersecting_segments_ids:
        return Relation.DISJOINT
    if len(intersecting_segments_ids) == 1:
        intersecting_segment = multisegmental_segments[
            intersecting_segments_ids[0]
        ]
        relation = relate_segment_to_polygon(
            intersecting_segment, polygon, orienteer, segments_intersector
        )
        return (
            Relation.TOUCH
            if relation is Relation.COMPONENT
            else (
                Relation.CROSS
                if (
                    relation is Relation.ENCLOSED
                    or relation is Relation.WITHIN
                )
                else relation
            )
        )
    min_max_x = min(
        max(
            multisegmental_boxes[segment_id].max_x
            for segment_id in intersecting_segments_ids
        ),
        polygon_bounding_box.max_x,
    )
    intersecting_segments = [
        multisegmental_segments[segment_id]
        for segment_id in intersecting_segments_ids
    ]
    return mixed.LinearShapedOperation.from_segments_iterables(
        intersecting_segments,
        chain(
            polygon.border.segments,
            chain.from_iterable(
                hole.segments
                for hole in polygon.holes
                if not hole.bounding_box.disjoint_with(
                    multisegmental_bounding_box
                )
            ),
        ),
        orienteer,
        segments_intersector,
    ).to_relation(
        linear_is_subset_of_shaped=len(intersecting_segments)
        == len(multisegmental_segments),
        min_max_x=min_max_x,
    )
