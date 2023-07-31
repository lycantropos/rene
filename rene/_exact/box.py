from __future__ import annotations

import typing as t
from numbers import Rational

import typing_extensions as te
from rithm.fraction import Fraction
from rithm.integer import Int

from rene import Relation

_Coordinate = t.Union[Fraction, Int, Rational, float, int]


@te.final
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

    def covers(self, other: te.Self, /) -> bool:
        return (other.max_x < self._max_x and other.max_y < self._max_y
                and self._min_x < other.min_x and self._min_y < other.min_y)

    def disjoint_with(self, other: te.Self, /) -> bool:
        return (self._max_x < other.min_x or self._max_y < other.min_y
                or other.max_x < self._min_x or other.max_y < self._min_y)

    def enclosed_by(self, other: te.Self, /) -> bool:
        return 2 <= ((1
                      if self._max_x == other.max_x
                      else (2 if self._max_x < other.max_x else 0))
                     * (1
                        if self._max_y == other.max_y
                        else (2 if self._max_y < other.max_y else 0))
                     * (1
                        if self._min_x == other.min_x
                        else (2 if self._min_x > other.min_x else 0))
                     * (1
                        if self._min_y == other.min_y
                        else (2 if self._min_y > other.min_y else 0))) <= 8

    def encloses(self, other: te.Self, /) -> bool:
        return 2 <= ((1
                      if self._max_x == other.max_x
                      else (2 if self._max_x > other.max_x else 0))
                     * (1
                        if self._max_y == other.max_y
                        else (2 if self._max_y > other.max_y else 0))
                     * (1
                        if self._min_x == other.min_x
                        else (2 if self._min_x < other.min_x else 0))
                     * (1
                        if self._min_y == other.min_y
                        else (2 if self._min_y < other.min_y else 0))) <= 8

    def equals_to(self, other: te.Self, /) -> bool:
        return (self._min_x == other.min_x and self._max_x == other.max_x
                and self._min_y == other.min_y and self._max_y == other.max_y)

    def is_valid(self) -> bool:
        return self._min_x <= self._max_x and self._min_y <= self._max_y

    def overlaps(self, other: te.Self, /) -> bool:
        if not (self._min_x < other.max_x and other.min_x < self._max_x
                and self._min_y < other.max_y and other.min_y < self._max_y):
            return False
        elif self._max_x > other.max_x:
            return (other.min_x < self._min_x
                    or other.min_y < self._min_y
                    or self._max_y < other.max_y)
        elif self._max_x < other.max_x:
            return (self._min_x < other.min_x
                    or self._min_y < other.min_y
                    or other.max_y < self._max_y)
        elif self._min_x > other.min_x:
            return self._min_y < other.min_y or other.max_y < self._max_y
        elif self._min_x < other.min_x:
            return other.min_y < self._min_y or self._max_y < other.max_y
        else:
            return ((other.min_y < self._min_y and other.max_y < self._max_y)
                    or
                    (self._min_y < other.min_y and self._max_y < other.max_y))

    def relate_to(self, other: te.Self, /) -> Relation:
        if self._max_x == other.max_x:
            if self._min_x == other.min_x:
                if self._max_y == other.max_y:
                    if self._min_y == other.min_y:
                        return Relation.EQUAL
                    elif self._min_y > other.min_y:
                        return Relation.ENCLOSED
                    else:
                        assert self._min_y < other.min_y
                        return Relation.ENCLOSES
                elif self._max_y > other.max_y:
                    if self._min_y == other.max_y:
                        return Relation.TOUCH
                    elif self._min_y > other.max_y:
                        return Relation.DISJOINT
                    else:
                        assert self._min_y < other.max_y
                        return (Relation.OVERLAP
                                if self._min_y > other.min_y
                                else Relation.ENCLOSES)
                else:
                    assert self._max_y < other.max_y
                    if self._max_y == other.min_y:
                        return Relation.TOUCH
                    elif self._max_y > other.min_y:
                        return (Relation.OVERLAP
                                if self._min_y < other.min_y
                                else Relation.ENCLOSED)
                    else:
                        assert self._max_y < other.min_y
                        return Relation.DISJOINT
            elif self._min_x > other.min_x:
                if self._max_y == other.max_y:
                    return (Relation.OVERLAP
                            if self._min_y < other.min_y
                            else Relation.ENCLOSED)
                elif self._max_y > other.max_y:
                    if self._min_y == other.max_y:
                        return Relation.TOUCH
                    elif self._min_y > other.max_y:
                        return Relation.DISJOINT
                    else:
                        assert self._min_y < other.max_y
                        return Relation.OVERLAP
                else:
                    assert self._max_y < other.max_y
                    if self._max_y == other.min_y:
                        return Relation.TOUCH
                    elif self._max_y > other.min_y:
                        return (Relation.OVERLAP
                                if self._min_y < other.min_y
                                else Relation.ENCLOSED)
                    else:
                        assert self._max_y < other.min_y
                        return Relation.DISJOINT
            else:
                assert self._min_x < other.min_x
                if self._max_y == other.max_y:
                    return (Relation.OVERLAP
                            if self._min_y > other.min_y
                            else Relation.ENCLOSES)
                elif self._max_y > other.max_y:
                    if self._min_y == other.max_y:
                        return Relation.TOUCH
                    elif self._min_y > other.max_y:
                        return Relation.DISJOINT
                    else:
                        assert self._min_y < other.max_y
                        return (Relation.OVERLAP
                                if self._min_y > other.min_y
                                else Relation.ENCLOSES)
                else:
                    assert self._max_y < other.max_y
                    if self._max_y == other.min_y:
                        return Relation.TOUCH
                    elif self._max_y > other.min_y:
                        return Relation.OVERLAP
                    else:
                        assert self._max_y < other.min_y
                        return Relation.DISJOINT
        elif self._max_x > other.max_x:
            if self._min_x == other.max_x:
                if self._max_y == other.max_y:
                    return Relation.TOUCH
                elif self._max_y > other.max_y:
                    return (Relation.DISJOINT
                            if self._min_y > other.max_y
                            else Relation.TOUCH)
                else:
                    assert self._max_y < other.max_y
                    return (Relation.DISJOINT
                            if self._max_y < other.min_y
                            else Relation.TOUCH)
            elif self._min_x > other.max_x:
                return Relation.DISJOINT
            else:
                assert self._min_x < other.max_x
                if self._min_x == other.min_x:
                    if self._max_y == other.max_y:
                        return (Relation.OVERLAP
                                if self._min_y > other.min_y
                                else Relation.ENCLOSES)
                    elif self._max_y > other.max_y:
                        if self._min_y == other.max_y:
                            return Relation.TOUCH
                        elif self._min_y > other.max_y:
                            return Relation.DISJOINT
                        else:
                            assert self._min_y < other.max_y
                            return (Relation.OVERLAP
                                    if self._min_y > other.min_y
                                    else Relation.ENCLOSES)
                    else:
                        assert self._max_y < other.max_y
                        if self._max_y == other.min_y:
                            return Relation.TOUCH
                        elif self._max_y > other.min_y:
                            return Relation.OVERLAP
                        else:
                            return Relation.DISJOINT
                elif self._min_x > other.min_x:
                    if self._max_y == other.max_y:
                        return Relation.OVERLAP
                    elif self._max_y > other.max_y:
                        if self._min_y == other.max_y:
                            return Relation.TOUCH
                        elif self._min_y > other.max_y:
                            return Relation.DISJOINT
                        else:
                            assert self._min_y < other.max_y
                            return Relation.OVERLAP
                    else:
                        assert self._max_y < other.max_y
                        if self._max_y == other.min_y:
                            return Relation.TOUCH
                        elif self._max_y > other.min_y:
                            return Relation.OVERLAP
                        else:
                            assert self._max_y < other.min_y
                            return Relation.DISJOINT
                else:
                    assert self._min_x < other.min_x
                    if self._max_y == other.max_y:
                        return (Relation.OVERLAP
                                if self._min_y > other.min_y
                                else Relation.ENCLOSES)
                    elif self._max_y > other.max_y:
                        if self._min_y == other.max_y:
                            return Relation.TOUCH
                        elif self._min_y > other.max_y:
                            return Relation.DISJOINT
                        else:
                            assert self._min_y < other.max_y
                            if self._min_y == other.min_y:
                                return Relation.ENCLOSES
                            elif self._min_y > other.min_y:
                                return Relation.OVERLAP
                            else:
                                assert self._min_y < other.min_y
                                return Relation.COVER
                    else:
                        assert self._max_y < other.max_y
                        if self._max_y == other.min_y:
                            return Relation.TOUCH
                        elif self._max_y > other.min_y:
                            return Relation.OVERLAP
                        else:
                            return Relation.DISJOINT
        else:
            assert self._max_x < other.max_x
            if self._max_x == other.min_x:
                if self._max_y == other.max_y:
                    return Relation.TOUCH
                elif self._max_y > other.max_y:
                    return (Relation.DISJOINT
                            if self._min_y > other.max_y
                            else Relation.TOUCH)
                else:
                    assert self._max_y < other.max_y
                    return (Relation.DISJOINT
                            if self._max_y < other.min_y
                            else Relation.TOUCH)
            elif self._max_x > other.min_x:
                if self._min_x == other.min_x:
                    if self._max_y == other.max_y:
                        return (Relation.OVERLAP
                                if self._min_y < other.min_y
                                else Relation.ENCLOSED)
                    elif self._max_y > other.max_y:
                        if self._min_y == other.max_y:
                            return Relation.TOUCH
                        elif self._min_y > other.max_y:
                            return Relation.DISJOINT
                        else:
                            return Relation.OVERLAP
                    else:
                        assert self._max_y < other.max_y
                        if self._max_y == other.min_y:
                            return Relation.TOUCH
                        elif self._max_y > other.min_y:
                            return (Relation.OVERLAP
                                    if self._min_y < other.min_y
                                    else Relation.ENCLOSED)
                        else:
                            assert self._max_y < other.min_y
                            return Relation.DISJOINT
                elif self._min_x > other.min_x:
                    if self._max_y == other.max_y:
                        return (Relation.OVERLAP
                                if self._min_y < other.min_y
                                else Relation.ENCLOSED)
                    elif self._max_y > other.max_y:
                        if self._min_y == other.max_y:
                            return Relation.TOUCH
                        elif self._min_y > other.max_y:
                            return Relation.DISJOINT
                        else:
                            assert self._min_y < other.max_y
                            return Relation.OVERLAP
                    else:
                        assert self._max_y < other.max_y
                        if self._max_y == other.min_y:
                            return Relation.TOUCH
                        elif self._max_y > other.min_y:
                            if self._min_y == other.min_y:
                                return Relation.ENCLOSED
                            elif self._min_y > other.min_y:
                                return Relation.WITHIN
                            else:
                                assert self._min_y < other.min_y
                                return Relation.OVERLAP
                        else:
                            assert self._max_y < other.min_y
                            return Relation.DISJOINT
                else:
                    assert self._min_x < other.min_x
                    if self._max_y == other.max_y:
                        return Relation.OVERLAP
                    elif self._max_y > other.max_y:
                        if self._min_y == other.max_y:
                            return Relation.TOUCH
                        elif self._min_y > other.max_y:
                            return Relation.DISJOINT
                        else:
                            assert self._min_y < other.max_y
                            return Relation.OVERLAP
                    else:
                        assert self._max_y < other.max_y
                        if self._max_y == other.min_y:
                            return Relation.TOUCH
                        elif self._max_y > other.min_y:
                            return Relation.OVERLAP
                        else:
                            return Relation.DISJOINT
            else:
                assert self._max_x < other.min_x
                return Relation.DISJOINT

    def touches(self, other: te.Self, /) -> bool:
        return (((self._min_x == other.max_x or self._max_x == other.min_x)
                 and
                 (self._min_y <= other.max_y and other.min_y <= self._max_y))
                or
                ((self._min_x <= other.max_x and other.min_x <= self._max_x)
                 and
                 (self._min_y == other.max_y or other.min_y == self._max_y)))

    def within(self, other: te.Self, /) -> bool:
        return (self._max_x < other.max_x
                and self._max_y < other.max_y
                and other.min_x < self._min_x
                and other.min_y < self._min_y)

    _max_x: Fraction
    _max_y: Fraction
    _min_x: Fraction
    _min_y: Fraction

    __module__ = 'rene.exact'
    __slots__ = '_min_x', '_max_x', '_min_y', '_max_y'

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(cls,
                min_x: _Coordinate,
                max_x: _Coordinate,
                min_y: _Coordinate,
                max_y: _Coordinate,
                /) -> te.Self:
        self = super().__new__(cls)
        self._max_x, self._max_y, self._min_x, self._min_y = (
            Fraction(max_x), Fraction(max_y), Fraction(min_x), Fraction(min_y)
        )
        return self

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        ...

    def __eq__(self, other: t.Any, /) -> t.Any:
        return ((self._min_x == other.min_x
                 and self._max_x == other.max_x
                 and self._min_y == other.min_y
                 and self._max_y == other.max_y)
                if isinstance(other, Box)
                else NotImplemented)

    def __hash__(self) -> int:
        return hash((self._min_x, self._max_x, self._min_y, self._max_y))

    def __repr__(self) -> str:
        return (f'{type(self).__qualname__}({self._min_x!r}, {self._max_x!r}, '
                f'{self._min_y!r}, {self._max_y!r})')

    def __str__(self) -> str:
        return (f'{type(self).__qualname__}({self._min_x}, {self._max_x}, '
                f'{self._min_y}, {self._max_y})')
