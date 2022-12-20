"""Computational geometry."""

__version__ = '0.1.0-alpha'

try:
    from ._crene import (MIN_CONTOUR_VERTICES_COUNT,
                         MIN_MULTIPOLYGON_POLYGONS_COUNT,
                         MIN_MULTISEGMENT_SEGMENTS_COUNT,
                         Location,
                         Orientation,
                         Relation)
except ImportError:
    from ._rene import (MIN_CONTOUR_VERTICES_COUNT,
                        MIN_MULTIPOLYGON_POLYGONS_COUNT,
                        MIN_MULTISEGMENT_SEGMENTS_COUNT,
                        Location,
                        Orientation,
                        Relation)
