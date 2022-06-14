from hypothesis import strategies

from rene import MIN_CONTOUR_VERTICES_COUNT
from rene.exact import (Contour,
                        Point)
from tests.strategies import (contours,
                              contours_vertices,
                              scalars)

points = strategies.builds(Point, scalars, scalars)
contours_vertices = contours_vertices
contours_like_vertices = strategies.lists(points,
                                          unique=True,
                                          min_size=MIN_CONTOUR_VERTICES_COUNT)
invalid_count_contours_vertices = strategies.lists(
        points,
        unique=True,
        max_size=MIN_CONTOUR_VERTICES_COUNT - 1
)
contours_like = strategies.builds(Contour, contours_like_vertices)
contours = contours
