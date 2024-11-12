from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    MIN_CONTOUR_VERTICES_COUNT: int
    MIN_MULTIPOLYGON_POLYGONS_COUNT: int
    MIN_MULTISEGMENT_SEGMENTS_COUNT: int
else:
    try:
        from . import _crene
    except ImportError:
        from ._constants import (
            MIN_CONTOUR_VERTICES_COUNT,
            MIN_MULTIPOLYGON_POLYGONS_COUNT,
            MIN_MULTISEGMENT_SEGMENTS_COUNT,
        )
    else:
        MIN_CONTOUR_VERTICES_COUNT = _crene.MIN_CONTOUR_VERTICES_COUNT
        MIN_MULTIPOLYGON_POLYGONS_COUNT = (
            _crene.MIN_MULTIPOLYGON_POLYGONS_COUNT
        )
        MIN_MULTISEGMENT_SEGMENTS_COUNT = (
            _crene.MIN_MULTISEGMENT_SEGMENTS_COUNT
        )
