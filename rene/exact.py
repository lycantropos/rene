try:
    from ._cexact import (Contour,
                          DelaunayTriangulation,
                          Multisegment,
                          Point,
                          Polygon,
                          Segment)
except ImportError:
    from ._exact import (ConstrainedDelaunayTriangulation,
                         Contour,
                         DelaunayTriangulation,
                         Multisegment,
                         Point,
                         Polygon,
                         Segment)
