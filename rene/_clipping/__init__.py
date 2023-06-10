from . import (difference,
               intersection,
               symmetric_difference,
               union)

subtract_multipolygon_from_polygon = (
    difference.subtract_multipolygon_from_polygon
)
subtract_multipolygons = difference.subtract_multipolygons
subtract_polygon_from_multipolygon = (
    difference.subtract_polygon_from_multipolygon
)
subtract_polygons = difference.subtract_polygons

intersect_multipolygon_with_polygon = (
    intersection.intersect_multipolygon_with_polygon
)
intersect_multipolygons = intersection.intersect_multipolygons
intersect_polygon_with_multipolygon = (
    intersection.intersect_polygon_with_multipolygon
)
intersect_polygons = intersection.intersect_polygons

symmetric_subtract_multipolygon_with_polygon = (
    symmetric_difference.symmetric_subtract_multipolygon_with_polygon
)
symmetric_subtract_multipolygons = (
    symmetric_difference.symmetric_subtract_multipolygons
)
symmetric_subtract_polygon_with_multipolygon = (
    symmetric_difference.symmetric_subtract_polygon_with_multipolygon
)
symmetric_subtract_polygons = symmetric_difference.symmetric_subtract_polygons

unite_multipolygon_with_polygon = union.unite_multipolygon_with_polygon
unite_multipolygons = union.unite_multipolygons
unite_polygon_with_multipolygon = union.unite_polygon_with_multipolygon
unite_polygons = union.unite_polygons
