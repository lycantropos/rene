try:
    from ._cexact import (Contour,
                          Multisegment,
                          Point,
                          Polygon,
                          Segment,
                          Triangulation)
except ImportError:
    from ._exact import (Contour,
                         Multisegment,
                         Point,
                         Polygon,
                         Segment,
                         Triangulation)
