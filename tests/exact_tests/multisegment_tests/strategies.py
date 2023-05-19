from hypothesis import strategies

from rene import MIN_MULTISEGMENT_SEGMENTS_COUNT
from rene.exact import Multisegment
from tests.exact_tests import strategies as _strategies

points = _strategies.points
non_zero_integers = _strategies.non_zero_integers
multisegments_segments = _strategies.multisegments_segments
multisegments_like_segments = strategies.lists(
        _strategies.segments,
        unique=True,
        min_size=MIN_MULTISEGMENT_SEGMENTS_COUNT
)
multisegments_like = strategies.builds(Multisegment,
                                       multisegments_like_segments)
multisegments = _strategies.multisegments
