from . import (difference as _difference,
               intersection as _intersection,
               symmetric_difference as _symmetric_difference,
               union as _union)

subtract_multipolygon_from_polygon = (
    _difference.subtract_multipolygon_from_polygon
)
subtract_multipolygons = _difference.subtract_multipolygons
subtract_polygon_from_multipolygon = (
    _difference.subtract_polygon_from_multipolygon
)
subtract_polygons = _difference.subtract_polygons

intersect_multipolygon_with_polygon = (
    _intersection.intersect_multipolygon_with_polygon
)
intersect_multipolygons = _intersection.intersect_multipolygons
intersect_polygon_with_multipolygon = (
    _intersection.intersect_polygon_with_multipolygon
)
intersect_polygons = _intersection.intersect_polygons

symmetric_subtract_multipolygon_with_polygon = (
    _symmetric_difference.symmetric_subtract_multipolygon_with_polygon
)
symmetric_subtract_multipolygons = (
    _symmetric_difference.symmetric_subtract_multipolygons
)
symmetric_subtract_polygon_with_multipolygon = (
    _symmetric_difference.symmetric_subtract_polygon_with_multipolygon
)
symmetric_subtract_polygons = _symmetric_difference.symmetric_subtract_polygons

unite_multipolygon_with_polygon = _union.unite_multipolygon_with_polygon
unite_multipolygons = _union.unite_multipolygons
unite_polygon_with_multipolygon = _union.unite_polygon_with_multipolygon
unite_polygons = _union.unite_polygons
