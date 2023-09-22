from __future__ import annotations

import typing as t
from itertools import chain

from rene import (Relation,
                  hints)
from rene._utils import to_boxes_ids_with_intersection
from . import mixed


def relate_to_contour(polygon: hints.Polygon[hints.Scalar],
                      contour: hints.Contour[hints.Scalar],
                      /) -> Relation:
    return relate_to_multisegmental(polygon, contour)


def relate_to_multisegment(polygon: hints.Polygon[hints.Scalar],
                           multisegment: hints.Multisegment[hints.Scalar],
                           /) -> Relation:
    return relate_to_multisegmental(polygon, multisegment)


def relate_to_segment(polygon: hints.Polygon[hints.Scalar],
                      segment: hints.Segment[hints.Scalar],
                      /) -> Relation:
    polygon_bounding_box, segment_bounding_box = (polygon.bounding_box,
                                                  segment.bounding_box)
    if polygon_bounding_box.disjoint_with(segment_bounding_box):
        return Relation.DISJOINT
    min_max_x = min(segment_bounding_box.max_x, polygon_bounding_box.max_x)
    return mixed.ShapedLinearOperation.from_segments_iterables(
            chain(
                    polygon.border.segments,
                    chain.from_iterable(
                            hole.segments
                            for hole in polygon.holes
                            if not hole.bounding_box.disjoint_with(
                                    segment_bounding_box
                            )
                    )
            ),
            [segment]
    ).to_relation(True, min_max_x)


_Multisegmental = t.Union[
    hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar]
]


def relate_to_multisegmental(polygon: hints.Polygon[hints.Scalar],
                             multisegmental: _Multisegmental[hints.Scalar],
                             /) -> Relation:
    polygon_bounding_box, multisegmental_bounding_box = (
        polygon.bounding_box, multisegmental.bounding_box
    )
    if polygon_bounding_box.disjoint_with(multisegmental_bounding_box):
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
        relation = relate_to_segment(polygon, intersecting_segment)
        return (Relation.TOUCH
                if relation is Relation.COMPONENT
                else (Relation.CROSS
                      if (relation is Relation.ENCLOSED
                          or relation is Relation.WITHIN)
                      else relation))
    min_max_x = min(polygon_bounding_box.max_x,
                    max(multisegmental_boxes[segment_id].max_x
                        for segment_id in intersecting_segments_ids))
    return mixed.ShapedLinearOperation.from_segments_iterables(
            chain(
                    polygon.border.segments,
                    chain.from_iterable(
                            hole.segments
                            for hole in polygon.holes
                            if not hole.bounding_box.disjoint_with(
                                    multisegmental_bounding_box
                            )
                    )
            ),
            [multisegmental_segments[segment_id]
             for segment_id in intersecting_segments_ids]
    ).to_relation(
            len(intersecting_segments_ids) == len(multisegmental_segments),
            min_max_x
    )
