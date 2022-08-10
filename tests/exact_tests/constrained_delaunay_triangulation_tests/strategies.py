from hypothesis import strategies

from tests.strategies import (points,
                              polygons)

points = points
points_lists = strategies.lists(points,
                                min_size=1)
two_or_more_points_lists = strategies.lists(points,
                                            min_size=2)
polygons = polygons
