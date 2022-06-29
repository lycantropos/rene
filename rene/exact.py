try:
    from ._cexact import (Contour,
                          Multisegment,
                          Point,
                          Polygon,
                          Segment)
except ImportError:
    from ._exact import (Contour,
                         Multisegment,
                         Point,
                         Polygon,
                         Segment)
