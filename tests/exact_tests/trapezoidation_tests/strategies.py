import sys as _sys

from hypothesis import strategies as _st

from rene.exact import Trapezoidation as _Trapezoidation
from tests.exact_tests import strategies as _strategies

_MAX_USIZE_VALUE = (_sys.maxsize << 1) + 1

multisegments = _strategies.multisegments
points = _strategies.points
polygons = _strategies.polygons
trapezoidations = (multisegments.map(_Trapezoidation.from_multisegment)
                   | polygons.map(_Trapezoidation.from_polygon))
seeders = _st.integers(
        min_value=0, max_value=_MAX_USIZE_VALUE
).map(lambda seed: (lambda: seed))
invalid_seeds = (_st.integers(max_value=-1)
                 | _st.integers(min_value=_MAX_USIZE_VALUE + 1)
                 | _st.floats()
                 | _st.text())
