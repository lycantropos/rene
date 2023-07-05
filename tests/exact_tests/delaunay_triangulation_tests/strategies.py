from hypothesis import strategies as _st

from tests.exact_tests import strategies as _strategies

points = _strategies.points
points_lists = _st.lists(points,
                         min_size=1)
two_or_more_points_lists = _st.lists(points,
                                     min_size=2)
