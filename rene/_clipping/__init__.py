from . import (difference,
               intersection,
               symmetric_difference,
               union)

intersect_multipolygon_with_multipolygon = (
    intersection.intersect_multipolygon_with_multipolygon
)
intersect_multipolygon_with_multisegmental = (
    intersection.intersect_multipolygon_with_multisegmental
)
intersect_multipolygon_with_polygon = (
    intersection.intersect_multipolygon_with_polygon
)
intersect_multipolygon_with_segment = (
    intersection.intersect_multipolygon_with_segment
)
intersect_multisegmental_with_multipolygon = (
    intersection.intersect_multisegmental_with_multipolygon
)
intersect_multisegmental_with_multisegmental = (
    intersection.intersect_multisegmental_with_multisegmental
)
intersect_multisegmental_with_polygon = (
    intersection.intersect_multisegmental_with_polygon
)
intersect_multisegmental_with_segment = (
    intersection.intersect_multisegmental_with_segment
)
intersect_polygon_with_multipolygon = (
    intersection.intersect_polygon_with_multipolygon
)
intersect_polygon_with_multisegmental = (
    intersection.intersect_polygon_with_multisegmental
)
intersect_polygon_with_polygon = intersection.intersect_polygon_with_polygon
intersect_polygon_with_segment = intersection.intersect_polygon_with_segment
intersect_segment_with_multipolygon = (
    intersection.intersect_segment_with_multipolygon
)
intersect_segment_with_multisegmental = (
    intersection.intersect_segment_with_multisegmental
)
intersect_segment_with_polygon = intersection.intersect_segment_with_polygon
intersect_segment_with_segment = intersection.intersect_segment_with_segment

subtract_multipolygon_from_multipolygon = (
    difference.subtract_multipolygon_from_multipolygon
)
subtract_multipolygon_from_polygon = (
    difference.subtract_multipolygon_from_polygon
)
subtract_multisegmental_from_multisegmental = (
    difference.subtract_multisegmental_from_multisegmental
)
subtract_multisegmental_from_segment = (
    difference.subtract_multisegmental_from_segment
)
subtract_polygon_from_multipolygon = (
    difference.subtract_polygon_from_multipolygon
)
subtract_polygon_from_polygon = difference.subtract_polygon_from_polygon
subtract_segment_from_multisegmental = (
    difference.subtract_segment_from_multisegmental
)
subtract_segment_from_segment = difference.subtract_segment_from_segment

symmetric_subtract_polygon_from_polygon = (
    symmetric_difference.symmetric_subtract_polygon_from_polygon
)
symmetric_subtract_polygon_from_multipolygon = (
    symmetric_difference.symmetric_subtract_polygon_from_multipolygon
)
symmetric_subtract_multipolygon_from_polygon = (
    symmetric_difference.symmetric_subtract_multipolygon_from_polygon
)
symmetric_subtract_multipolygon_from_multipolygon = (
    symmetric_difference.symmetric_subtract_multipolygon_from_multipolygon
)
symmetric_subtract_segment_from_segment = (
    symmetric_difference.symmetric_subtract_segment_from_segment
)
symmetric_subtract_segment_from_multisegmental = (
    symmetric_difference.symmetric_subtract_segment_from_multisegmental
)
symmetric_subtract_multisegmental_from_segment = (
    symmetric_difference.symmetric_subtract_multisegmental_from_segment
)
symmetric_subtract_multisegmental_from_multisegmental = (
    symmetric_difference.symmetric_subtract_multisegmental_from_multisegmental
)

unite_multipolygon_with_multipolygon = (
    union.unite_multipolygon_with_multipolygon
)
unite_multipolygon_with_polygon = union.unite_multipolygon_with_polygon
unite_multisegmental_with_multisegmental = (
    union.unite_multisegmental_with_multisegmental
)
unite_multisegmental_with_segment = union.unite_multisegmental_with_segment
unite_polygon_with_multipolygon = union.unite_polygon_with_multipolygon
unite_polygon_with_polygon = union.unite_polygon_with_polygon
unite_segment_with_multisegmental = union.unite_segment_with_multisegmental
unite_segment_with_segment = union.unite_segment_with_segment
