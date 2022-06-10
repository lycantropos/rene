try:
    from ._cexact import (Contour,
                          Point,
                          Polygon,
                          Segment)
except ImportError:
    from ._exact import (Contour,
                         Point,
                         Polygon,
                         Segment)
