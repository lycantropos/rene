"""Computational geometry."""

__version__ = '0.0.0'

try:
    from ._rene import (Orientation,
                        MIN_CONTOUR_VERTICES_COUNT)
except ImportError:
    from enum import IntEnum as _IntEnum

    MIN_CONTOUR_VERTICES_COUNT = 3


    class Orientation(_IntEnum):
        CLOCKWISE = -1
        COLLINEAR = 0
        COUNTERCLOCKWISE = 1

        def __repr__(self):
            return f'{__name__}.{type(self).__qualname__}.{self.name}'
