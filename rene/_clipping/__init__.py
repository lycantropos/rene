from .difference import (subtract_multipolygon_from_polygon,
                         subtract_multipolygons,
                         subtract_polygon_from_multipolygon,
                         subtract_polygons)
from .intersection import (intersect_multipolygon_with_polygon,
                           intersect_multipolygons,
                           intersect_polygon_with_multipolygon,
                           intersect_polygons)
from .symmetric_difference import (
    symmetric_subtract_multipolygon_with_polygon,
    symmetric_subtract_multipolygons,
    symmetric_subtract_polygon_with_multipolygon,
    symmetric_subtract_polygons
)
from .union import (unite_multipolygon_with_polygon,
                    unite_multipolygons,
                    unite_polygon_with_multipolygon,
                    unite_polygons)
