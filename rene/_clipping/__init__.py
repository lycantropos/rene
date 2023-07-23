from . import (difference,
               intersection,
               symmetric_difference,
               union)

intersect_polygon_with_polygon = intersection.intersect_polygon_with_polygon
intersect_polygon_with_polygons = intersection.intersect_polygon_with_polygons
intersect_polygons_with_polygon = intersection.intersect_polygons_with_polygon
intersect_polygons_with_polygons = (
    intersection.intersect_polygons_with_polygons
)
intersect_segment_with_segment = intersection.intersect_segment_with_segment
intersect_segment_with_segments = intersection.intersect_segment_with_segments
intersect_segments_with_segment = intersection.intersect_segments_with_segment
intersect_segments_with_segments = (
    intersection.intersect_segments_with_segments
)

subtract_polygon_from_polygon = difference.subtract_polygon_from_polygon
subtract_polygon_from_polygons = difference.subtract_polygon_from_polygons
subtract_polygons_from_polygon = difference.subtract_polygons_from_polygon
subtract_polygons_from_polygons = difference.subtract_polygons_from_polygons

symmetric_subtract_polygon_from_polygon = (
    symmetric_difference.symmetric_subtract_polygon_from_polygon
)
symmetric_subtract_polygon_from_polygons = (
    symmetric_difference.symmetric_subtract_polygon_from_polygons
)
symmetric_subtract_polygons_from_polygon = (
    symmetric_difference.symmetric_subtract_polygons_from_polygon
)
symmetric_subtract_polygons_from_polygons = (
    symmetric_difference.symmetric_subtract_polygons_from_polygons
)
symmetric_subtract_segment_from_segment = (
    symmetric_difference.symmetric_subtract_segment_from_segment
)
symmetric_subtract_segment_from_segments = (
    symmetric_difference.symmetric_subtract_segment_from_segments
)
symmetric_subtract_segments_from_segment = (
    symmetric_difference.symmetric_subtract_segments_from_segment
)
symmetric_subtract_segments_from_segments = (
    symmetric_difference.symmetric_subtract_segments_from_segments
)

unite_polygon_with_polygon = union.unite_polygon_with_polygon
unite_polygon_with_polygons = union.unite_polygon_with_polygons
unite_polygons_with_polygon = union.unite_polygons_with_polygon
unite_polygons_with_polygons = union.unite_polygons_with_polygons
