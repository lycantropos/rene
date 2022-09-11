from tests.exact_tests.strategies import (empty_geometries,
                                          compounds,
                                          polygons)

compounds = empty_geometries | compounds | polygons
