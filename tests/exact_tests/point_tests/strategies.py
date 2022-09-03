from hypothesis import strategies

from tests.exact_tests.strategies import (points,
                                          scalars_strategies)

integers = strategies.integers()
non_zero_integers = integers.filter(bool)
scalars = scalars_strategies.flatmap(lambda strategy: strategy)
points = points
