import sys as _sys
from operator import attrgetter as _attrgetter

from hypothesis import strategies as _st

from tests.exact_tests import strategies as _strategies

MAX_INDEX = _sys.maxsize >> (-(-_sys.maxsize.bit_length() // 2))
assert MAX_INDEX * MAX_INDEX <= _sys.maxsize
indices = _st.integers(-MAX_INDEX, MAX_INDEX)
non_zero_indices = indices.filter(bool)
slices = _st.builds(slice, indices, indices, non_zero_indices)
multipolygons_polygons = _strategies.multipolygons.map(_attrgetter('polygons'))
polygons = _strategies.polygons
