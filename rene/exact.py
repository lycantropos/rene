try:
    from ._cexact import (ConstrainedDelaunayTriangulation,
                          Contour,
                          DelaunayTriangulation,
                          Empty,
                          Multipolygon,
                          Multisegment,
                          Point,
                          Polygon,
                          Segment)
except ImportError:
    from ._exact import (ConstrainedDelaunayTriangulation,
                         Contour,
                         DelaunayTriangulation,
                         Empty,
                         Multipolygon,
                         Multisegment,
                         Point,
                         Polygon,
                         Segment)
