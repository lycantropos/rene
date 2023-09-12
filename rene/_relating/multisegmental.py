import typing as t
from itertools import chain

from rene import (Relation,
                  hints)
from rene._utils import (merge_boxes,
                         to_boxes_ids_with_intersection)
from . import (linear,
               mixed)
from .segment import (relate_to_multiregion as relate_segment_to_multiregion,
                      relate_to_region as relate_segment_to_region)

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


def relate_to_multiregion(multisegmental: _Multisegmental[hints.Scalar],
                          borders: t.Sequence[hints.Contour[hints.Scalar]],
                          reverse_shaped_orientation: bool,
                          /) -> Relation:
    assert len(borders) > 1, borders
    multisegmental_bounding_box = multisegmental.bounding_box
    borders_boxes = [border.bounding_box for border in borders]
    multiregion_bounding_box = merge_boxes(borders_boxes)
    if multisegmental_bounding_box.disjoint_with(multiregion_bounding_box):
        return Relation.DISJOINT
    multisegmental_segments = multisegmental.segments
    segments_boxes = [segment.bounding_box
                      for segment in multisegmental_segments]
    intersecting_segments_ids = to_boxes_ids_with_intersection(
            segments_boxes, multiregion_bounding_box
    )
    if not intersecting_segments_ids:
        return Relation.DISJOINT
    elif len(intersecting_segments_ids) == 1:
        multisegmental_segment = multisegmental_segments[
            intersecting_segments_ids[0]
        ]
        relation = relate_segment_to_multiregion(multisegmental_segment,
                                                 borders)
        return (Relation.TOUCH
                if relation is Relation.COMPONENT or relation is Relation.EQUAL
                else (Relation.CROSS
                      if (relation is Relation.ENCLOSED
                          or relation is Relation.WITHIN)
                      else relation))
    intersecting_borders_ids = to_boxes_ids_with_intersection(
            borders_boxes, multisegmental_bounding_box
    )
    assert intersecting_borders_ids
    max_min_x = max(
            min(segments_boxes[segment_id].min_x
                for segment_id in intersecting_segments_ids),
            min(borders_boxes[border_id].min_x
                for border_id in intersecting_borders_ids)
    )
    min_max_x = min(
            max(segments_boxes[segment_id].max_x
                for segment_id in intersecting_segments_ids),
            max(borders_boxes[border_id].max_x
                for border_id in intersecting_borders_ids)
    )
    intersecting_segments = [
        multisegmental_segments[segment_id]
        for segment_id in intersecting_segments_ids
        if (max_min_x <= segments_boxes[segment_id].max_x
            and segments_boxes[segment_id].min_x <= min_max_x)
    ]
    if not intersecting_segments:
        return Relation.DISJOINT
    intersecting_borders = [borders[border_id]
                            for border_id in intersecting_borders_ids]
    return mixed.LinearShapedOperation.from_segments_iterables(
            intersecting_segments,
            chain.from_iterable(border.segments
                                for border in intersecting_borders),
            reverse_shaped_orientation
    ).to_relation(
            len(intersecting_segments) == len(multisegmental_segments),
            min_max_x
    )


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
    relation_without_holes = relate_to_region(multisegmental, polygon.border,
                                              False)
    holes = polygon.holes
    if holes and (relation_without_holes is Relation.ENCLOSED
                  or relation_without_holes is Relation.WITHIN):
        relation_with_holes = (relate_to_region(multisegmental, holes[0], True)
                               if len(holes) == 1
                               else relate_to_multiregion(multisegmental,
                                                          holes, True))
        if relation_with_holes is Relation.DISJOINT:
            return relation_without_holes
        elif relation_with_holes is Relation.TOUCH:
            return Relation.ENCLOSED
        elif (relation_with_holes is Relation.CROSS
              or relation_with_holes is Relation.COMPONENT):
            return relation_with_holes
        elif relation_with_holes is Relation.ENCLOSED:
            return Relation.TOUCH
        else:
            assert relation_with_holes is Relation.WITHIN, relation_with_holes
            return Relation.DISJOINT
    else:
        return relation_without_holes


def relate_to_region(multisegmental: _Multisegmental[hints.Scalar],
                     border: hints.Contour[hints.Scalar],
                     reverse_region_orientation: bool,
                     /) -> Relation:
    multisegmental_bounding_box, region_bounding_box = (
        multisegmental.bounding_box, border.bounding_box
    )
    if multisegmental_bounding_box.disjoint_with(region_bounding_box):
        return Relation.DISJOINT
    multisegmental_segments = multisegmental.segments
    multisegmental_boxes = [segment.bounding_box
                            for segment in multisegmental_segments]
    intersecting_segments_ids = to_boxes_ids_with_intersection(
            multisegmental_boxes, region_bounding_box
    )
    if not intersecting_segments_ids:
        return Relation.DISJOINT
    elif len(intersecting_segments_ids) == 1:
        multisegmental_segment = multisegmental_segments[
            intersecting_segments_ids[0]
        ]
        relation = relate_segment_to_region(multisegmental_segment, border)
        return (Relation.TOUCH
                if relation is Relation.COMPONENT or relation is Relation.EQUAL
                else (Relation.CROSS
                      if (relation is Relation.ENCLOSED
                          or relation is Relation.WITHIN)
                      else relation))
    max_min_x = max(
            min(multisegmental_boxes[segment_id].min_x
                for segment_id in intersecting_segments_ids),
            region_bounding_box.min_x
    )
    min_max_x = min(
            max(multisegmental_boxes[segment_id].max_x
                for segment_id in intersecting_segments_ids),
            region_bounding_box.max_x
    )
    intersecting_segments = [
        multisegmental_segments[segment_id]
        for segment_id in intersecting_segments_ids
        if (max_min_x <= multisegmental_boxes[segment_id].max_x
            and multisegmental_boxes[segment_id].min_x <= min_max_x)
    ]
    if not intersecting_segments:
        return Relation.DISJOINT
    return mixed.LinearShapedOperation.from_segments_iterables(
            intersecting_segments, border.segments, reverse_region_orientation
    ).to_relation(
            len(intersecting_segments) == len(multisegmental_segments),
            min_max_x
    )
