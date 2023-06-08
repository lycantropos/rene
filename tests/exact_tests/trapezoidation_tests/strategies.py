from hypothesis import strategies as _st

from rene.exact import Trapezoidation
from tests.exact_tests import strategies as _strategies

multisegments = _strategies.multisegments
points = _strategies.points
polygons = _strategies.polygons
trapezoidations = multisegments.map(Trapezoidation.from_multisegment)
seeders = _st.integers(min_value=0).map(lambda seed: (lambda: seed))
invalid_seeds = (
        _st.integers(max_value=-1) | _st.floats() | _st.text()
)
