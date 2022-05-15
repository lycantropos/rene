"""Computational geometry."""

__version__ = '0.0.0'

try:
    from ._rene import Orientation
except ImportError:
    from enum import IntEnum as _IntEnum


    class Orientation(_IntEnum):
        CLOCKWISE = -1
        COLLINEAR = 0
        COUNTERCLOCKWISE = 1

        def __repr__(self):
            return f'{__name__}.{type(self).__qualname__}.{self.name}'
