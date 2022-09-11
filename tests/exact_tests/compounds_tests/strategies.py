from tests.exact_tests.strategies import (empty_geometries,
                                          multipolygons,
                                          polygons)

compounds = empty_geometries | multipolygons | polygons
