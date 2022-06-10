from enum import (IntEnum,
                  unique)

MIN_CONTOUR_VERTICES_COUNT = 3


class Base(IntEnum):
    __module__ = 'rene'

    def __repr__(self):
        return (f'{type(self).__module__}.{type(self).__qualname__}'
                f'.{self.name}')

    def __str__(self):
        return f'{type(self).__qualname__}.{self.name}'


@unique
class Orientation(Base):
    CLOCKWISE = -1
    COLLINEAR = 0
    COUNTERCLOCKWISE = 1
