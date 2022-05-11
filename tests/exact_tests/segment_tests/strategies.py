from operator import ne

from hypothesis import strategies
from rithm import Fraction

from rene.exact import (Point,
                        Segment)
from tests.utils import pack

integers = strategies.integers()
non_zero_integers = integers.filter(bool)
scalars = (integers | strategies.fractions()
           | strategies.builds(Fraction, integers, non_zero_integers)
           | strategies.floats(allow_infinity=False,
                               allow_nan=False))
points = strategies.builds(Point, scalars, scalars)
segments_endpoints = strategies.tuples(points, points).filter(pack(ne))
segments = segments_endpoints.map(pack(Segment))
