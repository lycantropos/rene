try:
    from ._cexact import (Box,
                          ConstrainedDelaunayTriangulation,
                          Contour,
                          DelaunayTriangulation,
                          Empty,
                          Multipolygon,
                          Multisegment,
                          Point,
                          Polygon,
                          Segment)
except ImportError:
    from ._exact import (Box,
                         ConstrainedDelaunayTriangulation,
                         Contour,
                         DelaunayTriangulation,
                         Empty,
                         Multipolygon,
                         Multisegment,
                         Point,
                         Polygon,
                         Segment)
