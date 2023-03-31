from __future__ import annotations

import typing as _t
from numbers import Rational

import typing_extensions as _te
from reprit.base import generate_repr
from rithm.fraction import Fraction
from rithm.integer import Int

from rene import Relation

_Coordinate = _t.Union[Fraction, Int, Rational, float, int]


@_te.final
class Box:
    @property
    def max_x(self) -> Fraction:
        return self._max_x

    @property
    def max_y(self) -> Fraction:
        return self._max_y

    @property
    def min_x(self) -> Fraction:
        return self._min_x

    @property
    def min_y(self) -> Fraction:
        return self._min_y

    def covers(self, other: _te.Self) -> bool:
        return (other.max_x < self.max_x and other.max_y < self.max_y
                and self.min_x < other.min_x and self.min_y < other.min_y)

    def disjoint_with(self, other: _te.Self) -> bool:
        return (self.max_x < other.min_x or self.max_y < other.min_y
                or other.max_x < self.min_x or other.max_y < self.min_y)

    def enclosed_by(self, other: _te.Self) -> bool:
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

    def encloses(self, other: _te.Self) -> bool:
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

    def equals_to(self, other: _te.Self) -> bool:
        return (self.min_x == other.min_x and self.max_x == other.max_x
                and self.min_y == other.min_y and self.max_y == other.max_y)

    def is_valid(self) -> bool:
        return self.min_x <= self.max_x and self.min_y <= self.max_y

    def overlaps(self, other: _te.Self) -> bool:
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

    def relate_to(self, other: _te.Self) -> Relation:
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

    def touches(self, other: _te.Self) -> bool:
        return (((self.min_x == other.max_x or self.max_x == other.min_x)
                 and (self.min_y <= other.max_y and other.min_y <= self.max_y))
                or
                ((self.min_x <= other.max_x and other.min_x <= self.max_x)
                 and (self.min_y == other.max_y or other.min_y == self.max_y)))

    def within(self, other: _te.Self) -> bool:
        return (self.max_x < other.max_x
                and self.max_y < other.max_y
                and other.min_x < self.min_x
                and other.min_y < self.min_y)

    _max_x: Fraction
    _max_y: Fraction
    _min_x: Fraction
    _min_y: Fraction

    __module__ = 'rene.exact'
    __slots__ = '_min_x', '_max_x', '_min_y', '_max_y'

    def __new__(cls,
                min_x: _Coordinate,
                max_x: _Coordinate,
                min_y: _Coordinate,
                max_y: _Coordinate) -> _te.Self:
        self = super().__new__(cls)
        self._max_x, self._max_y, self._min_x, self._min_y = (
            Fraction(max_x), Fraction(max_y), Fraction(min_x), Fraction(min_y)
        )
        return self

    @_t.overload
    def __eq__(self, other: _te.Self) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __eq__(self, other: _t.Any) -> _t.Any:
        return ((self.min_x == other.min_x
                 and self.max_x == other.max_x
                 and self.min_y == other.min_y
                 and self.max_y == other.max_y)
                if isinstance(other, Box)
                else NotImplemented)

    def __hash__(self) -> int:
        return hash((self.min_x, self.max_x, self.min_y, self.max_y))

    __repr__ = generate_repr(__new__)

    def __str__(self) -> str:
        return (f'{type(self).__qualname__}'
                f'({self.min_x}, {self.max_x}, {self.min_y}, {self.max_y})')
