import sys as _sys
from operator import attrgetter as _attrgetter

from hypothesis import strategies as _st

from tests.exact_tests import strategies as _strategies

indices = _st.integers(-_sys.maxsize - 1, _sys.maxsize)
non_zero_indices = indices.filter(bool)
slices = _st.builds(slice, indices, indices, non_zero_indices)
multisegments_segments = _strategies.multisegments.map(_attrgetter('segments'))
segments = _strategies.segments
