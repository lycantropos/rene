from rene._context import Context as _Context
from .box import Box
from .contour import Contour
from .empty import Empty
from .multipolygon import Multipolygon
from .multisegment import Multisegment
from .point import Point
from .polygon import Polygon
from .segment import Segment
from .trapezoidation import Trapezoidation
from .triangulation import (ConstrainedDelaunayTriangulation,
                            DelaunayTriangulation)

ConstrainedDelaunayTriangulation._context = Contour._context = \
    DelaunayTriangulation._context = Empty._context = Multipolygon._context = \
    Multisegment._context = Polygon._context = Segment._context = \
    Trapezoidation._context = _Context(box_cls=Box,
                                       contour_cls=Contour,
                                       empty_cls=Empty,
                                       multipolygon_cls=Multipolygon,
                                       multisegment_cls=Multisegment,
                                       point_cls=Point,
                                       polygon_cls=Polygon,
                                       segment_cls=Segment)
