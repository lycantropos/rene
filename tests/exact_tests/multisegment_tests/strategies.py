from hypothesis import strategies

from rene import MIN_MULTISEGMENT_SEGMENTS_COUNT
from rene.exact import Multisegment
from tests.strategies import (multisegments,
                              multisegments_segments,
                              non_zero_integers,
                              points)

non_zero_integers = non_zero_integers
multisegments_segments = multisegments_segments
multisegments_like_segments = strategies.lists(
        points,
        unique=True,
        min_size=MIN_MULTISEGMENT_SEGMENTS_COUNT
)
invalid_count_multisegments_segments = strategies.lists(
        points,
        unique=True,
        max_size=MIN_MULTISEGMENT_SEGMENTS_COUNT - 1
)
multisegments_like = strategies.builds(Multisegment,
                                       multisegments_like_segments)
multisegments = multisegments
