from hypothesis import strategies

from rene import MIN_CONTOUR_VERTICES_COUNT
from rene.exact import Contour
from tests.exact_tests import strategies as _strategies

points = _strategies.points
non_zero_integers = _strategies.non_zero_integers
contours_vertices = _strategies.contours_vertices
contours_like_vertices = strategies.lists(points,
                                          unique=True,
                                          min_size=MIN_CONTOUR_VERTICES_COUNT)
invalid_count_contours_vertices = strategies.lists(
        points,
        unique=True,
        max_size=MIN_CONTOUR_VERTICES_COUNT - 1
)
contours_like = strategies.builds(Contour, contours_like_vertices)
contours = _strategies.contours
