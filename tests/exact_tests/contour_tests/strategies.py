from hypothesis import strategies
from rithm import Fraction

from rene.exact import (Contour,
                        Point)

integers = strategies.integers()
non_zero_integers = integers.filter(bool)
scalars = (integers | strategies.fractions()
           | strategies.builds(Fraction, integers, non_zero_integers)
           | strategies.floats(allow_infinity=False,
                               allow_nan=False))
points = strategies.builds(Point, scalars, scalars)
contours_vertices = strategies.lists(points,
                                     unique=True,
                                     min_size=3)
contours = strategies.builds(Contour, contours_vertices)
