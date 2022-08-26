try:
    from ._cexact import (ConstrainedDelaunayTriangulation,
                          Contour,
                          DelaunayTriangulation,
                          Multipolygon,
                          Multisegment,
                          Point,
                          Polygon,
                          Segment)
except ImportError:
    from ._exact import (ConstrainedDelaunayTriangulation,
                         Contour,
                         DelaunayTriangulation,
                         Multipolygon,
                         Multisegment,
                         Point,
                         Polygon,
                         Segment)
