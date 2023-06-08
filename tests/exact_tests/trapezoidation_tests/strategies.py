from rene.exact import Trapezoidation
from tests.exact_tests import strategies as _strategies

multisegments = _strategies.multisegments
points = _strategies.points
trapezoidations = multisegments.map(Trapezoidation.from_multisegment)
