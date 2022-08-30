from rithm import Fraction


class Box:
    __slots__ = '_min_x', '_max_x', '_min_y', '_max_y'

    def __new__(cls, min_x, max_x, min_y, max_y):
        self = super().__new__(cls)
        self._max_x, self._max_y, self._min_x, self._min_y = (
            Fraction(max_x), Fraction(max_y), Fraction(min_x), Fraction(min_y)
        )
        return self

    @property
    def max_x(self):
        return self._max_x

    @property
    def max_y(self):
        return self._max_y

    @property
    def min_x(self):
        return self._min_x

    @property
    def min_y(self):
        return self._min_y
