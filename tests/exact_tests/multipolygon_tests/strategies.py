from hypothesis import strategies

from rene import MIN_MULTIPOLYGON_POLYGONS_COUNT
from rene.exact import Multipolygon
from tests.exact_tests.strategies import (multipolygons,
                                          multipolygons_polygons,
                                          non_zero_integers,
                                          polygons)

non_zero_integers = non_zero_integers
multipolygons_polygons = multipolygons_polygons
multipolygons_like_polygons = strategies.lists(
        polygons,
        unique=True,
        min_size=MIN_MULTIPOLYGON_POLYGONS_COUNT
)
invalid_count_multipolygons_polygons = strategies.lists(
        polygons,
        unique=True,
        max_size=MIN_MULTIPOLYGON_POLYGONS_COUNT - 1
)
multipolygons_like = strategies.builds(Multipolygon,
                                       multipolygons_like_polygons)
multipolygons = multipolygons
