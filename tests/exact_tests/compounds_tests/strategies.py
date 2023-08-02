from tests.exact_tests import strategies as _strategies

compounds = (_strategies.empty_geometries | _strategies.multipolygons
             | _strategies.polygons)
points = _strategies.points
