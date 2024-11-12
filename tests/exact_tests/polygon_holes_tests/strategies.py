import sys as _sys
from operator import attrgetter as _attrgetter

from hypothesis import strategies as _st

from tests.exact_tests import strategies as _strategies

MAX_INDEX = _sys.maxsize >> (-(-_sys.maxsize.bit_length() // 2))
assert _sys.maxsize >= MAX_INDEX * MAX_INDEX
indices = _st.integers(-MAX_INDEX, MAX_INDEX)
non_zero_indices = indices.filter(bool)
slices = _st.builds(slice, indices, indices, non_zero_indices)
polygons_holes = _strategies.polygons.map(_attrgetter('holes'))
contours = _strategies.contours
