from enum import IntEnum

MIN_CONTOUR_VERTICES_COUNT = 3


class Orientation(IntEnum):
    CLOCKWISE = -1
    COLLINEAR = 0
    COUNTERCLOCKWISE = 1

    __module__ = 'rene'

    def __repr__(self):
        return (f'{type(self).__module__}.{type(self).__qualname__}'
                f'.{self.name}')

    def __str__(self):
        return f'{type(self).__qualname__}.{self.name}'
