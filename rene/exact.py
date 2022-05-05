try:
    from ._exact import Point
except ImportError:
    from rithm import Fraction as _Fraction


    class Point:
        __slots__ = '_x', '_y'

        def __new__(cls, x, y):
            self = super().__new__(cls)
            self._x, self._y = (_Fraction(x)
                                if isinstance(x, float)
                                else _Fraction(x.numerator, x.denominator),
                                _Fraction(y)
                                if isinstance(y, float)
                                else _Fraction(y.numerator, y.denominator))
            return self

        @property
        def x(self):
            return self._x

        @property
        def y(self):
            return self._y
