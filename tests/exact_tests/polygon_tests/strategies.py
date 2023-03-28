from hypothesis import strategies
from rithm.fraction import Fraction

from rene.exact import Point
from tests.exact_tests.strategies import (empty_geometries,
                                          multipolygons,
                                          polygons,
                                          polygons_components)

integers = strategies.integers()
non_zero_integers = integers.filter(bool)
scalars = (integers | strategies.fractions()
           | strategies.builds(Fraction, integers, non_zero_integers)
           | strategies.floats(allow_infinity=False,
                               allow_nan=False))
points = strategies.builds(Point, scalars, scalars)
polygons_components = polygons_components
polygons = polygons
compounds = empty_geometries | multipolygons | polygons
