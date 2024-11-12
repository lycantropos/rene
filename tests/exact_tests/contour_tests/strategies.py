from hypothesis import strategies as _st

from rene.constants import (
    MIN_CONTOUR_VERTICES_COUNT as _MIN_CONTOUR_VERTICES_COUNT,
)
from rene.exact import Contour as _Contour
from tests.exact_tests import strategies as _strategies

points = _strategies.points
non_zero_integers = _strategies.non_zero_integers
contours_vertices = _strategies.contours_vertices
contours_like_vertices = _st.lists(
    points, unique=True, min_size=_MIN_CONTOUR_VERTICES_COUNT
)
invalid_count_contours_vertices = _st.lists(
    points, unique=True, max_size=_MIN_CONTOUR_VERTICES_COUNT - 1
)
contours_like = _st.builds(_Contour, contours_like_vertices)
contours = _strategies.contours
