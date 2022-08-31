from hypothesis import strategies

from rene.exact import Box
from tests.strategies import (boxes,
                              boxes_limits,
                              non_zero_integers,
                              scalars_strategies)

non_zero_integers = non_zero_integers
boxes_limits = boxes_limits
boxes_like = scalars_strategies.flatmap(
        lambda scalars: strategies.builds(Box, scalars, scalars, scalars,
                                          scalars)
)
boxes = boxes
