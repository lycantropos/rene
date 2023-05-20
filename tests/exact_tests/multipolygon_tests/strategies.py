from hypothesis import strategies

from rene import MIN_MULTIPOLYGON_POLYGONS_COUNT
from rene.exact import Multipolygon
from tests.exact_tests import strategies as _strategies

points = _strategies.points
non_zero_integers = _strategies.non_zero_integers
multipolygons_polygons = _strategies.multipolygons_polygons
multipolygons_like_polygons = strategies.lists(
        _strategies.polygons,
        unique=True,
        min_size=MIN_MULTIPOLYGON_POLYGONS_COUNT
)
invalid_count_multipolygons_polygons = strategies.lists(
        _strategies.polygons,
        unique=True,
        max_size=MIN_MULTIPOLYGON_POLYGONS_COUNT - 1
)
multipolygons_like = strategies.builds(Multipolygon,
                                       multipolygons_like_polygons)
multipolygons = _strategies.multipolygons
compounds = _strategies.empty_geometries | multipolygons | _strategies.polygons
