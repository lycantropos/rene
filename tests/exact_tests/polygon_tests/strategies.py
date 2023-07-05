from hypothesis import strategies as _st
from rithm.fraction import Fraction as _Fraction

from rene.exact import Point as _Point
from tests.exact_tests import strategies as _strategies

integers = _st.integers()
non_zero_integers = integers.filter(bool)
scalars = (integers | _st.fractions()
           | _st.builds(_Fraction, integers, non_zero_integers)
           | _st.floats(allow_infinity=False,
                        allow_nan=False))
points = _st.builds(_Point, scalars, scalars)
polygons_components = _strategies.polygons_components
polygons = _strategies.polygons
compounds = _strategies.empty_geometries | _strategies.multipolygons | polygons
