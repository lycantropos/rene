from hypothesis import strategies as _st

from tests.exact_tests import strategies as _strategies

integers = _st.integers()
non_zero_integers = integers.filter(bool)
scalars = _strategies.scalars_strategies.flatmap(lambda strategy: strategy)
points = _strategies.points
