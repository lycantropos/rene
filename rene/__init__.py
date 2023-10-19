"""Computational geometry."""

__version__ = '0.1.0'

try:
    from . import _crene
except ImportError:
    from ._rene import (MIN_CONTOUR_VERTICES_COUNT,
                        MIN_MULTIPOLYGON_POLYGONS_COUNT,
                        MIN_MULTISEGMENT_SEGMENTS_COUNT,
                        Location,
                        Orientation,
                        Relation)
else:
    MIN_CONTOUR_VERTICES_COUNT = _crene.MIN_CONTOUR_VERTICES_COUNT
    MIN_MULTIPOLYGON_POLYGONS_COUNT = _crene.MIN_MULTIPOLYGON_POLYGONS_COUNT
    MIN_MULTISEGMENT_SEGMENTS_COUNT = _crene.MIN_MULTISEGMENT_SEGMENTS_COUNT
    Location = _crene.Location
    Orientation = _crene.Orientation
    Relation = _crene.Relation
