import typing as t
from itertools import chain

from rene import (Relation,
                  hints)
from rene._utils import to_boxes_ids_with_intersection
from . import (linear,
               mixed)
from .segment import relate_to_polygon as relate_segment_to_polygon

_Multisegmental = t.Union[
    hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar]
]
_EndpointsToSegmentsRelater = t.Callable[
    [
        hints.Point[hints.Scalar], hints.Point[hints.Scalar],
        t.Sequence[hints.Segment[hints.Scalar]]
    ],
    Relation
]


def relate_to_multisegmental(
        first: _Multisegmental[hints.Scalar],
        second: _Multisegmental[hints.Scalar],
        first_to_second_relater: _EndpointsToSegmentsRelater[hints.Scalar],
        second_to_first_relater: _EndpointsToSegmentsRelater[hints.Scalar],
        /
) -> Relation:
    first_bounding_box, second_bounding_box = (first.bounding_box,
                                               second.bounding_box)
    if first_bounding_box.disjoint_with(second_bounding_box):
        return Relation.DISJOINT
    first_segments = first.segments
    first_boxes = [segment.bounding_box for segment in first_segments]
    first_intersecting_segments_ids = to_boxes_ids_with_intersection(
            first_boxes, second_bounding_box
    )
    if not first_intersecting_segments_ids:
        return Relation.DISJOINT
    second_segments = second.segments
    if len(first_intersecting_segments_ids) == 1:
        first_segment = first_segments[first_intersecting_segments_ids[0]]
        relation = first_to_second_relater(
                first_segment.start, first_segment.end, second_segments
        )
        return (Relation.OVERLAP
                if relation is Relation.COMPONENT or relation is Relation.EQUAL
                else relation)
    second_boxes = [segment.bounding_box for segment in second_segments]
    second_intersecting_segments_ids = to_boxes_ids_with_intersection(
            second_boxes, first_bounding_box
    )
    if not second_intersecting_segments_ids:
        return Relation.DISJOINT
    elif len(second_intersecting_segments_ids) == 1:
        second_segment = second_segments[second_intersecting_segments_ids[0]]
        relation = second_to_first_relater(
                second_segment.start, second_segment.end,
                [first_segments[segment_id]
                 for segment_id in first_intersecting_segments_ids]
        )
        return (Relation.OVERLAP
                if (relation is Relation.COMPONENT
                    or ((relation is Relation.COMPOSITE
                         or relation is Relation.EQUAL)
                        and (len(first_intersecting_segments_ids)
                             != len(first_segments))))
                else relation.complement)
    max_min_x = max(
            min(first_boxes[segment_id].min_x
                for segment_id in first_intersecting_segments_ids),
            min(second_boxes[segment_id].min_x
                for segment_id in second_intersecting_segments_ids)
    )
    min_max_x = min(
            max(first_boxes[segment_id].max_x
                for segment_id in first_intersecting_segments_ids),
            max(second_boxes[segment_id].max_x
                for segment_id in second_intersecting_segments_ids)
    )
    first_intersecting_segments = [
        first_segments[segment_id]
        for segment_id in first_intersecting_segments_ids
        if (max_min_x <= first_boxes[segment_id].max_x
            and first_boxes[segment_id].min_x <= min_max_x)
    ]
    if not first_intersecting_segments:
        return Relation.DISJOINT
    second_intersecting_segments = [
        second_segments[segment_id]
        for segment_id in second_intersecting_segments_ids
        if (max_min_x <= second_boxes[segment_id].max_x
            and second_boxes[segment_id].min_x <= min_max_x)
    ]
    assert second_intersecting_segments
    return linear.Operation.from_segments_iterables(
            first_intersecting_segments, second_intersecting_segments
    ).to_relation(
            len(first_intersecting_segments) == len(first_segments),
            len(second_intersecting_segments) == len(second_segments),
            min_max_x
    )


def relate_to_polygon(multisegmental: _Multisegmental[hints.Scalar],
                      polygon: hints.Polygon[hints.Scalar],
                      /) -> Relation:
    border = polygon.border
    multisegmental_bounding_box, polygon_bounding_box = (
        multisegmental.bounding_box, border.bounding_box
    )
    if multisegmental_bounding_box.disjoint_with(polygon_bounding_box):
        return Relation.DISJOINT
    multisegmental_segments = multisegmental.segments
    multisegmental_boxes = [segment.bounding_box
                            for segment in multisegmental_segments]
    intersecting_segments_ids = to_boxes_ids_with_intersection(
            multisegmental_boxes, polygon_bounding_box
    )
    if not intersecting_segments_ids:
        return Relation.DISJOINT
    elif len(intersecting_segments_ids) == 1:
        intersecting_segment = multisegmental_segments[
            intersecting_segments_ids[0]
        ]
        relation = relate_segment_to_polygon(intersecting_segment, polygon)
        return (relation
                if (len(intersecting_segments_ids)
                    == len(multisegmental_segments))
                else (Relation.TOUCH
                      if relation is Relation.COMPONENT
                      else (Relation.CROSS
                            if (relation is Relation.ENCLOSED
                                or relation is Relation.WITHIN)
                            else relation)))
    min_max_x = min(
            max(multisegmental_boxes[segment_id].max_x
                for segment_id in intersecting_segments_ids),
            polygon_bounding_box.max_x
    )
    intersecting_segments = [multisegmental_segments[segment_id]
                             for segment_id in intersecting_segments_ids]
    holes = polygon.holes
    holes_boxes = [hole.bounding_box for hole in holes]
    intersecting_holes_ids = to_boxes_ids_with_intersection(
            holes_boxes, multisegmental_bounding_box
    )
    return mixed.LinearShapedOperation.from_segments_iterables(
            intersecting_segments,
            chain(border.segments,
                  chain.from_iterable(holes[hole_id].segments
                                      for hole_id in intersecting_holes_ids))
            if intersecting_holes_ids
            else border.segments
    ).to_relation(
            len(intersecting_segments) == len(multisegmental_segments),
            min_max_x
    )
