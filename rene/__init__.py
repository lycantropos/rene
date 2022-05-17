"""Computational geometry."""

__version__ = '0.0.0'

try:
    from ._rene import (MIN_CONTOUR_VERTICES_COUNT,
                        Orientation)
except ImportError:
    from enum import IntEnum as _IntEnum

    MIN_CONTOUR_VERTICES_COUNT = 3


    class Orientation(_IntEnum):
        CLOCKWISE = -1
        COLLINEAR = 0
        COUNTERCLOCKWISE = 1

        def __repr__(self):
            return (f'{type(self).__module__}.{type(self).__qualname__}'
                    f'.{self.name}')

        def __str__(self):
            return f'{type(self).__qualname__}.{self.name}'
