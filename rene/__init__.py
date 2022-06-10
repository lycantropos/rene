"""Computational geometry."""

__version__ = '0.0.0'

try:
    from ._crene import (MIN_CONTOUR_VERTICES_COUNT,
                         Orientation,
                         Relation)
except ImportError:
    from ._rene import (MIN_CONTOUR_VERTICES_COUNT,
                        Orientation,
                        Relation)
