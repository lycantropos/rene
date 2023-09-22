from __future__ import annotations

import typing as t
from itertools import chain

from rene import (Relation,
                  hints)
from rene._utils import to_boxes_ids_with_intersection
from . import mixed


def relate_to_contour(multipolygon: hints.Multipolygon[hints.Scalar],
                      contour: hints.Contour[hints.Scalar],
                      /) -> Relation:
    return relate_to_multisegmental(multipolygon, contour)


def relate_to_multisegment(multipolygon: hints.Multipolygon[hints.Scalar],
                           multisegment: hints.Multisegment[hints.Scalar],
                           /) -> Relation:
    return relate_to_multisegmental(multipolygon, multisegment)


def relate_to_segment(multipolygon: hints.Multipolygon[hints.Scalar],
                      segment: hints.Segment[hints.Scalar],
                      /) -> Relation:
    segment_bounding_box = segment.bounding_box
    polygons = multipolygon.polygons
    polygons_bounding_boxes = [polygon.bounding_box for polygon in polygons]
    intersecting_polygons_ids = to_boxes_ids_with_intersection(
            polygons_bounding_boxes, segment_bounding_box
    )
    if not intersecting_polygons_ids:
        return Relation.DISJOINT
    min_max_x = min(segment_bounding_box.max_x,
                    max(polygons_bounding_boxes[polygon_id].max_x
                        for polygon_id in intersecting_polygons_ids))
    return mixed.ShapedLinearOperation.from_segments_iterables(
            chain.from_iterable(
                    chain(
                            polygon.border.segments,
                            chain.from_iterable(
                                    hole.segments
                                    for hole in polygon.holes
                                    if not hole.bounding_box.disjoint_with(
                                            segment_bounding_box
                                    )
                            )
                    )
                    for polygon in polygons
            ),
            [segment]
    ).to_relation(True, min_max_x)


_Multisegmental = t.Union[
    hints.Contour[hints.Scalar], hints.Multisegment[hints.Scalar]
]


def relate_to_multisegmental(multipolygon: hints.Multipolygon[hints.Scalar],
                             multisegmental: _Multisegmental[hints.Scalar],
                             /) -> Relation:
    multipolygon_bounding_box, multisegmental_bounding_box = (
        multipolygon.bounding_box, multisegmental.bounding_box
    )
    if multipolygon_bounding_box.disjoint_with(multisegmental_bounding_box):
        return Relation.DISJOINT
    multisegmental_segments = multisegmental.segments
    multisegmental_boxes = [segment.bounding_box
                            for segment in multisegmental_segments]
    intersecting_segments_ids = to_boxes_ids_with_intersection(
            multisegmental_boxes, multipolygon_bounding_box
    )
    if not intersecting_segments_ids:
        return Relation.DISJOINT
    elif len(intersecting_segments_ids) == 1:
        intersecting_segment = multisegmental_segments[
            intersecting_segments_ids[0]
        ]
        relation = relate_to_segment(multipolygon, intersecting_segment)
        return (Relation.TOUCH
                if relation is Relation.COMPONENT
                else (Relation.CROSS
                      if (relation is Relation.ENCLOSED
                          or relation is Relation.WITHIN)
                      else relation))
    multipolygon_polygons = multipolygon.polygons
    multipolygon_boxes = [polygon.bounding_box
                          for polygon in multipolygon_polygons]
    intersecting_polygons_ids = to_boxes_ids_with_intersection(
            multipolygon_boxes, multisegmental_bounding_box
    )
    min_max_x = min(
            max(multipolygon_boxes[polygon_id].max_x
                for polygon_id in intersecting_polygons_ids),
            max(multisegmental_boxes[segment_id].max_x
                for segment_id in intersecting_segments_ids),
    )
    return mixed.ShapedLinearOperation.from_segments_iterables(
            chain.from_iterable(
                    chain(
                            polygon.border.segments,
                            chain.from_iterable(
                                    hole.segments
                                    for hole in polygon.holes
                                    if not hole.bounding_box.disjoint_with(
                                            multisegmental_bounding_box
                                    )
                            )
                    )
                    for polygon in [
                        multipolygon_polygons[polygon_id]
                        for polygon_id in intersecting_polygons_ids
                    ]
            ),
            [multisegmental_segments[segment_id]
             for segment_id in intersecting_segments_ids]
    ).to_relation(
            len(intersecting_segments_ids) == len(multisegmental_segments),
            min_max_x
    )