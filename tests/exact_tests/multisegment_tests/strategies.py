from hypothesis import strategies

from rene import MIN_MULTISEGMENT_SEGMENTS_COUNT
from rene.exact import Multisegment
from tests.strategies import (multisegments,
                              multisegments_segments,
                              non_zero_integers,
                              segments)

non_zero_integers = non_zero_integers
multisegments_segments = multisegments_segments
multisegments_like_segments = strategies.lists(
        segments,
        unique=True,
        min_size=MIN_MULTISEGMENT_SEGMENTS_COUNT
)
multisegments_like = strategies.builds(Multisegment,
                                       multisegments_like_segments)
multisegments = multisegments
