from reprit.base import generate_repr
from rithm import Fraction

from rene._rene import Relation


class Box:
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

    def covers(self, other):
        return (other.max_x < self.max_x and other.max_y < self.max_y
                and self.min_x < other.min_x and self.min_y < other.min_y)

    def disjoint_with(self, other):
        return (self.max_x < other.min_x or self.max_y < other.min_y
                or other.max_x < self.min_x or other.max_y < self.min_y)

    def enclosed_by(self, other):
        return 2 <= ((1
                      if self.max_x == other.max_x
                      else (2 if self.max_x < other.max_x else 0))
                     * (1
                        if self.max_y == other.max_y
                        else (2 if self.max_y < other.max_y else 0))
                     * (1
                        if self.min_x == other.min_x
                        else (2 if self.min_x > other.min_x else 0))
                     * (1
                        if self.min_y == other.min_y
                        else (2 if self.min_y > other.min_y else 0))) <= 8

    def encloses(self, other):
        return 2 <= ((1
                      if self.max_x == other.max_x
                      else (2 if self.max_x > other.max_x else 0))
                     * (1
                        if self.max_y == other.max_y
                        else (2 if self.max_y > other.max_y else 0))
                     * (1
                        if self.min_x == other.min_x
                        else (2 if self.min_x < other.min_x else 0))
                     * (1
                        if self.min_y == other.min_y
                        else (2 if self.min_y < other.min_y else 0))) <= 8

    def equals_to(self, other):
        return (self.min_x == other.min_x and self.max_x == other.max_x
                and self.min_y == other.min_y and self.max_y == other.max_y)

    def is_valid(self):
        return self.min_x <= self.max_x and self.min_y <= self.max_y

    def overlaps(self, other):
        if not (self.min_x < other.max_x and other.min_x < self.max_x
                and self.min_y < other.max_y and other.min_y < self.max_y):
            return False
        elif self.max_x > other.max_x:
            return (other.min_x < self.min_x
                    or other.min_y < self.min_y
                    or self.max_y < other.max_y)
        elif self.max_x < other.max_x:
            return (self.min_x < other.min_x
                    or self.min_y < other.min_y
                    or other.max_y < self.max_y)
        elif self.min_x > other.min_x:
            return self.min_y < other.min_y or other.max_y < self.max_y
        elif self.min_x < other.min_x:
            return other.min_y < self.min_y or self.max_y < other.max_y
        else:
            return ((other.min_y < self.min_y and other.max_y < self.max_y)
                    or (self.min_y < other.min_y and self.max_y < other.max_y))

    def relate_to(self, other):
        if self.max_x == other.max_x:
            if self.min_x == other.min_x:
                if self.max_y == other.max_y:
                    if self.min_y == other.min_y:
                        return Relation.EQUAL
                    elif self.min_y > other.min_y:
                        return Relation.ENCLOSED
                    else:
                        assert self.min_y < other.min_y
                        return Relation.ENCLOSES
                elif self.max_y > other.max_y:
                    if self.min_y == other.max_y:
                        return Relation.TOUCH
                    elif self.min_y > other.max_y:
                        return Relation.DISJOINT
                    else:
                        assert self.min_y < other.max_y
                        return (Relation.OVERLAP
                                if self.min_y > other.min_y
                                else Relation.ENCLOSES)
                else:
                    assert self.max_y < other.max_y
                    if self.max_y == other.min_y:
                        return Relation.TOUCH
                    elif self.max_y > other.min_y:
                        return (Relation.OVERLAP
                                if self.min_y < other.min_y
                                else Relation.ENCLOSED)
                    else:
                        assert self.max_y < other.min_y
                        return Relation.DISJOINT
            elif self.min_x > other.min_x:
                if self.max_y == other.max_y:
                    return (Relation.OVERLAP
                            if self.min_y < other.min_y
                            else Relation.ENCLOSED)
                elif self.max_y > other.max_y:
                    if self.min_y == other.max_y:
                        return Relation.TOUCH
                    elif self.min_y > other.max_y:
                        return Relation.DISJOINT
                    else:
                        assert self.min_y < other.max_y
                        return Relation.OVERLAP
                else:
                    assert self.max_y < other.max_y
                    if self.max_y == other.min_y:
                        return Relation.TOUCH
                    elif self.max_y > other.min_y:
                        return (Relation.OVERLAP
                                if self.min_y < other.min_y
                                else Relation.ENCLOSED)
                    else:
                        assert self.max_y < other.min_y
                        return Relation.DISJOINT
            else:
                assert self.min_x < other.min_x
                if self.max_y == other.max_y:
                    return (Relation.OVERLAP
                            if self.min_y > other.min_y
                            else Relation.ENCLOSES)
                elif self.max_y > other.max_y:
                    if self.min_y == other.max_y:
                        return Relation.TOUCH
                    elif self.min_y > other.max_y:
                        return Relation.DISJOINT
                    else:
                        assert self.min_y < other.max_y
                        return (Relation.OVERLAP
                                if self.min_y > other.min_y
                                else Relation.ENCLOSES)
                else:
                    assert self.max_y < other.max_y
                    if self.max_y == other.min_y:
                        return Relation.TOUCH
                    elif self.max_y > other.min_y:
                        return Relation.OVERLAP
                    else:
                        assert self.max_y < other.min_y
                        return Relation.DISJOINT
        elif self.max_x > other.max_x:
            if self.min_x == other.max_x:
                if self.max_y == other.max_y:
                    return Relation.TOUCH
                elif self.max_y > other.max_y:
                    return (Relation.DISJOINT
                            if self.min_y > other.max_y
                            else Relation.TOUCH)
                else:
                    assert self.max_y < other.max_y
                    return (Relation.DISJOINT
                            if self.max_y < other.min_y
                            else Relation.TOUCH)
            elif self.min_x > other.max_x:
                return Relation.DISJOINT
            else:
                assert self.min_x < other.max_x
                if self.min_x == other.min_x:
                    if self.max_y == other.max_y:
                        return (Relation.OVERLAP
                                if self.min_y > other.min_y
                                else Relation.ENCLOSES)
                    elif self.max_y > other.max_y:
                        if self.min_y == other.max_y:
                            return Relation.TOUCH
                        elif self.min_y > other.max_y:
                            return Relation.DISJOINT
                        else:
                            assert self.min_y < other.max_y
                            return (Relation.OVERLAP
                                    if self.min_y > other.min_y
                                    else Relation.ENCLOSES)
                    else:
                        assert self.max_y < other.max_y
                        if self.max_y == other.min_y:
                            return Relation.TOUCH
                        elif self.max_y > other.min_y:
                            return Relation.OVERLAP
                        else:
                            return Relation.DISJOINT
                elif self.min_x > other.min_x:
                    if self.max_y == other.max_y:
                        return Relation.OVERLAP
                    elif self.max_y > other.max_y:
                        if self.min_y == other.max_y:
                            return Relation.TOUCH
                        elif self.min_y > other.max_y:
                            return Relation.DISJOINT
                        else:
                            assert self.min_y < other.max_y
                            return Relation.OVERLAP
                    else:
                        assert self.max_y < other.max_y
                        if self.max_y == other.min_y:
                            return Relation.TOUCH
                        elif self.max_y > other.min_y:
                            return Relation.OVERLAP
                        else:
                            assert self.max_y < other.min_y
                            return Relation.DISJOINT
                else:
                    assert self.min_x < other.min_x
                    if self.max_y == other.max_y:
                        return (Relation.OVERLAP
                                if self.min_y > other.min_y
                                else Relation.ENCLOSES)
                    elif self.max_y > other.max_y:
                        if self.min_y == other.max_y:
                            return Relation.TOUCH
                        elif self.min_y > other.max_y:
                            return Relation.DISJOINT
                        else:
                            assert self.min_y < other.max_y
                            if self.min_y == other.min_y:
                                return Relation.ENCLOSES
                            elif self.min_y > other.min_y:
                                return Relation.OVERLAP
                            else:
                                assert self.min_y < other.min_y
                                return Relation.COVER
                    else:
                        assert self.max_y < other.max_y
                        if self.max_y == other.min_y:
                            return Relation.TOUCH
                        elif self.max_y > other.min_y:
                            return Relation.OVERLAP
                        else:
                            return Relation.DISJOINT
        else:
            assert self.max_x < other.max_x
            if self.max_x == other.min_x:
                if self.max_y == other.max_y:
                    return Relation.TOUCH
                elif self.max_y > other.max_y:
                    return (Relation.DISJOINT
                            if self.min_y > other.max_y
                            else Relation.TOUCH)
                else:
                    assert self.max_y < other.max_y
                    return (Relation.DISJOINT
                            if self.max_y < other.min_y
                            else Relation.TOUCH)
            elif self.max_x > other.min_x:
                if self.min_x == other.min_x:
                    if self.max_y == other.max_y:
                        return (Relation.OVERLAP
                                if self.min_y < other.min_y
                                else Relation.ENCLOSED)
                    elif self.max_y > other.max_y:
                        if self.min_y == other.max_y:
                            return Relation.TOUCH
                        elif self.min_y > other.max_y:
                            return Relation.DISJOINT
                        else:
                            return Relation.OVERLAP
                    else:
                        assert self.max_y < other.max_y
                        if self.max_y == other.min_y:
                            return Relation.TOUCH
                        elif self.max_y > other.min_y:
                            return (Relation.OVERLAP
                                    if self.min_y < other.min_y
                                    else Relation.ENCLOSED)
                        else:
                            assert self.max_y < other.min_y
                            return Relation.DISJOINT
                elif self.min_x > other.min_x:
                    if self.max_y == other.max_y:
                        return (Relation.OVERLAP
                                if self.min_y < other.min_y
                                else Relation.ENCLOSED)
                    elif self.max_y > other.max_y:
                        if self.min_y == other.max_y:
                            return Relation.TOUCH
                        elif self.min_y > other.max_y:
                            return Relation.DISJOINT
                        else:
                            assert self.min_y < other.max_y
                            return Relation.OVERLAP
                    else:
                        assert self.max_y < other.max_y
                        if self.max_y == other.min_y:
                            return Relation.TOUCH
                        elif self.max_y > other.min_y:
                            if self.min_y == other.min_y:
                                return Relation.ENCLOSED
                            elif self.min_y > other.min_y:
                                return Relation.WITHIN
                            else:
                                assert self.min_y < other.min_y
                                return Relation.OVERLAP
                        else:
                            assert self.max_y < other.min_y
                            return Relation.DISJOINT
                else:
                    assert self.min_x < other.min_x
                    if self.max_y == other.max_y:
                        return Relation.OVERLAP
                    elif self.max_y > other.max_y:
                        if self.min_y == other.max_y:
                            return Relation.TOUCH
                        elif self.min_y > other.max_y:
                            return Relation.DISJOINT
                        else:
                            assert self.min_y < other.max_y
                            return Relation.OVERLAP
                    else:
                        assert self.max_y < other.max_y
                        if self.max_y == other.min_y:
                            return Relation.TOUCH
                        elif self.max_y > other.min_y:
                            return Relation.OVERLAP
                        else:
                            return Relation.DISJOINT
            else:
                assert self.max_x < other.min_x
                return Relation.DISJOINT

    def touches(self, other):
        return (((self.min_x == other.max_x or self.max_x == other.min_x)
                 and (self.min_y <= other.max_y and other.min_y <= self.max_y))
                or
                ((self.min_x <= other.max_x and other.min_x <= self.max_x)
                 and (self.min_y == other.max_y or other.min_y == self.max_y)))

    def within(self, other):
        return (self.max_x < other.max_x
                and self.max_y < other.max_y
                and other.min_x < self.min_x
                and other.min_y < self.min_y)

    __module__ = 'rene.exact'
    __slots__ = '_min_x', '_max_x', '_min_y', '_max_y'

    def __new__(cls, min_x, max_x, min_y, max_y):
        self = super().__new__(cls)
        self._max_x, self._max_y, self._min_x, self._min_y = (
            Fraction(max_x), Fraction(max_y), Fraction(min_x), Fraction(min_y)
        )
        return self

    def __eq__(self, other):
        return ((self.min_x == other.min_x
                 and self.max_x == other.max_x
                 and self.min_y == other.min_y
                 and self.max_y == other.max_y)
                if isinstance(other, Box)
                else NotImplemented)

    def __hash__(self):
        return hash((self.min_x, self.max_x, self.min_y, self.max_y))

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __str__(self):
        return (f'{type(self).__qualname__}'
                f'({self.min_x}, {self.max_x}, {self.min_y}, {self.max_y})')
