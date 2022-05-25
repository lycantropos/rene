from functools import partial
from operator import (attrgetter,
                      itemgetter)

from ground.base import (Context,
                         Mode)
from hypothesis import strategies
from hypothesis_geometry import planar
from rithm import (Fraction,
                   Int)

from rene.exact import (Contour,
                        Point,
                        Polygon,
                        Segment)

context = Context(contour_cls=Contour,
                  point_cls=Point,
                  polygon_cls=Polygon,
                  segment_cls=Segment,
                  mode=Mode.PLAIN)
integers = strategies.builds(Int, strategies.integers())
non_zero_integers = strategies.builds(Int,
                                      strategies.integers(max_value=-1)
                                      | strategies.integers(min_value=1))
scalars = integers | strategies.builds(Fraction, integers, non_zero_integers)
to_contours = partial(planar.contours, scalars, scalars,
                      context=context)
to_polygons = partial(planar.polygons, scalars, scalars,
                      context=context)
contours = to_contours()
contours_vertices = contours.map(attrgetter('vertices')).map(itemgetter(0))
polygons = to_polygons()
polygons_components = polygons.map(attrgetter('border', 'holes'))
