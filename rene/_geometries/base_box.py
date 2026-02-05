from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Any, Generic, overload

from typing_extensions import Self

from rene import hints
from rene.enums import Relation


class BaseBox(ABC, Generic[hints.ScalarT]):
    @property
    @abstractmethod
    def max_x(self, /) -> hints.ScalarT: ...

    @property
    @abstractmethod
    def max_y(self, /) -> hints.ScalarT: ...

    @property
    @abstractmethod
    def min_x(self, /) -> hints.ScalarT: ...

    @property
    @abstractmethod
    def min_y(self, /) -> hints.ScalarT: ...

    def covers(self, other: Self, /) -> bool:
        return (
            other.max_x < self.max_x
            and other.max_y < self.max_y
            and self.min_x < other.min_x
            and self.min_y < other.min_y
        )

    def disjoint_with(self, other: Self, /) -> bool:
        return (
            self.max_x < other.min_x
            or self.max_y < other.min_y
            or other.max_x < self.min_x
            or other.max_y < self.min_y
        )

    def enclosed_by(self, other: Self, /) -> bool:
        return (
            2
            <= (
                (
                    1
                    if self.max_x == other.max_x
                    else (2 if self.max_x < other.max_x else 0)
                )
                * (
                    1
                    if self.max_y == other.max_y
                    else (2 if self.max_y < other.max_y else 0)
                )
                * (
                    1
                    if self.min_x == other.min_x
                    else (2 if self.min_x > other.min_x else 0)
                )
                * (
                    1
                    if self.min_y == other.min_y
                    else (2 if self.min_y > other.min_y else 0)
                )
            )
            <= 8
        )

    def encloses(self, other: Self, /) -> bool:
        return (
            2
            <= (
                (
                    1
                    if self.max_x == other.max_x
                    else (2 if self.max_x > other.max_x else 0)
                )
                * (
                    1
                    if self.max_y == other.max_y
                    else (2 if self.max_y > other.max_y else 0)
                )
                * (
                    1
                    if self.min_x == other.min_x
                    else (2 if self.min_x < other.min_x else 0)
                )
                * (
                    1
                    if self.min_y == other.min_y
                    else (2 if self.min_y < other.min_y else 0)
                )
            )
            <= 8
        )

    def equals_to(self, other: Self, /) -> bool:
        return (
            self.min_x == other.min_x
            and self.max_x == other.max_x
            and self.min_y == other.min_y
            and self.max_y == other.max_y
        )

    def is_valid(self, /) -> bool:
        return self.min_x <= self.max_x and self.min_y <= self.max_y

    def overlaps(self, other: Self, /) -> bool:
        if not (
            self.min_x < other.max_x
            and other.min_x < self.max_x
            and self.min_y < other.max_y
            and other.min_y < self.max_y
        ):
            return False
        if self.max_x > other.max_x:
            return (
                other.min_x < self.min_x
                or other.min_y < self.min_y
                or self.max_y < other.max_y
            )
        if self.max_x < other.max_x:
            return (
                self.min_x < other.min_x
                or self.min_y < other.min_y
                or other.max_y < self.max_y
            )
        if self.min_x > other.min_x:
            return self.min_y < other.min_y or other.max_y < self.max_y
        if self.min_x < other.min_x:
            return other.min_y < self.min_y or self.max_y < other.max_y
        return (other.min_y < self.min_y and other.max_y < self.max_y) or (
            self.min_y < other.min_y and self.max_y < other.max_y
        )

    def relate_to(self, other: Self, /) -> Relation:
        if self.max_x == other.max_x:
            if self.min_x == other.min_x:
                if self.max_y == other.max_y:
                    if self.min_y == other.min_y:
                        return Relation.EQUAL
                    if self.min_y > other.min_y:
                        return Relation.ENCLOSED
                    assert self.min_y < other.min_y
                    return Relation.ENCLOSES
                if self.max_y > other.max_y:
                    if self.min_y == other.max_y:
                        return Relation.TOUCH
                    if self.min_y > other.max_y:
                        return Relation.DISJOINT
                    assert self.min_y < other.max_y
                    return (
                        Relation.OVERLAP
                        if self.min_y > other.min_y
                        else Relation.ENCLOSES
                    )
                assert self.max_y < other.max_y
                if self.max_y == other.min_y:
                    return Relation.TOUCH
                if self.max_y > other.min_y:
                    return (
                        Relation.OVERLAP
                        if self.min_y < other.min_y
                        else Relation.ENCLOSED
                    )
                assert self.max_y < other.min_y
                return Relation.DISJOINT
            if self.min_x > other.min_x:
                if self.max_y == other.max_y:
                    return (
                        Relation.OVERLAP
                        if self.min_y < other.min_y
                        else Relation.ENCLOSED
                    )
                if self.max_y > other.max_y:
                    if self.min_y == other.max_y:
                        return Relation.TOUCH
                    if self.min_y > other.max_y:
                        return Relation.DISJOINT
                    assert self.min_y < other.max_y
                    return Relation.OVERLAP
                assert self.max_y < other.max_y
                if self.max_y == other.min_y:
                    return Relation.TOUCH
                if self.max_y > other.min_y:
                    return (
                        Relation.OVERLAP
                        if self.min_y < other.min_y
                        else Relation.ENCLOSED
                    )
                assert self.max_y < other.min_y
                return Relation.DISJOINT
            assert self.min_x < other.min_x
            if self.max_y == other.max_y:
                return (
                    Relation.OVERLAP
                    if self.min_y > other.min_y
                    else Relation.ENCLOSES
                )
            if self.max_y > other.max_y:
                if self.min_y == other.max_y:
                    return Relation.TOUCH
                if self.min_y > other.max_y:
                    return Relation.DISJOINT
                assert self.min_y < other.max_y
                return (
                    Relation.OVERLAP
                    if self.min_y > other.min_y
                    else Relation.ENCLOSES
                )
            assert self.max_y < other.max_y
            if self.max_y == other.min_y:
                return Relation.TOUCH
            if self.max_y > other.min_y:
                return Relation.OVERLAP
            assert self.max_y < other.min_y
            return Relation.DISJOINT
        if self.max_x > other.max_x:
            if self.min_x == other.max_x:
                if self.max_y == other.max_y:
                    return Relation.TOUCH
                if self.max_y > other.max_y:
                    return (
                        Relation.DISJOINT
                        if self.min_y > other.max_y
                        else Relation.TOUCH
                    )
                assert self.max_y < other.max_y
                return (
                    Relation.DISJOINT
                    if self.max_y < other.min_y
                    else Relation.TOUCH
                )
            if self.min_x > other.max_x:
                return Relation.DISJOINT
            assert self.min_x < other.max_x
            if self.min_x == other.min_x:
                if self.max_y == other.max_y:
                    return (
                        Relation.OVERLAP
                        if self.min_y > other.min_y
                        else Relation.ENCLOSES
                    )
                if self.max_y > other.max_y:
                    if self.min_y == other.max_y:
                        return Relation.TOUCH
                    if self.min_y > other.max_y:
                        return Relation.DISJOINT
                    assert self.min_y < other.max_y
                    return (
                        Relation.OVERLAP
                        if self.min_y > other.min_y
                        else Relation.ENCLOSES
                    )
                assert self.max_y < other.max_y
                if self.max_y == other.min_y:
                    return Relation.TOUCH
                if self.max_y > other.min_y:
                    return Relation.OVERLAP
                return Relation.DISJOINT
            if self.min_x > other.min_x:
                if self.max_y == other.max_y:
                    return Relation.OVERLAP
                if self.max_y > other.max_y:
                    if self.min_y == other.max_y:
                        return Relation.TOUCH
                    if self.min_y > other.max_y:
                        return Relation.DISJOINT
                    assert self.min_y < other.max_y
                    return Relation.OVERLAP
                assert self.max_y < other.max_y
                if self.max_y == other.min_y:
                    return Relation.TOUCH
                if self.max_y > other.min_y:
                    return Relation.OVERLAP
                assert self.max_y < other.min_y
                return Relation.DISJOINT
            assert self.min_x < other.min_x
            if self.max_y == other.max_y:
                return (
                    Relation.OVERLAP
                    if self.min_y > other.min_y
                    else Relation.ENCLOSES
                )
            if self.max_y > other.max_y:
                if self.min_y == other.max_y:
                    return Relation.TOUCH
                if self.min_y > other.max_y:
                    return Relation.DISJOINT
                assert self.min_y < other.max_y
                if self.min_y == other.min_y:
                    return Relation.ENCLOSES
                if self.min_y > other.min_y:
                    return Relation.OVERLAP
                assert self.min_y < other.min_y
                return Relation.COVER
            assert self.max_y < other.max_y
            if self.max_y == other.min_y:
                return Relation.TOUCH
            if self.max_y > other.min_y:
                return Relation.OVERLAP
            return Relation.DISJOINT
        assert self.max_x < other.max_x
        if self.max_x == other.min_x:
            if self.max_y == other.max_y:
                return Relation.TOUCH
            if self.max_y > other.max_y:
                return (
                    Relation.DISJOINT
                    if self.min_y > other.max_y
                    else Relation.TOUCH
                )
            assert self.max_y < other.max_y
            return (
                Relation.DISJOINT
                if self.max_y < other.min_y
                else Relation.TOUCH
            )
        if self.max_x > other.min_x:
            if self.min_x == other.min_x:
                if self.max_y == other.max_y:
                    return (
                        Relation.OVERLAP
                        if self.min_y < other.min_y
                        else Relation.ENCLOSED
                    )
                if self.max_y > other.max_y:
                    if self.min_y == other.max_y:
                        return Relation.TOUCH
                    if self.min_y > other.max_y:
                        return Relation.DISJOINT
                    return Relation.OVERLAP
                assert self.max_y < other.max_y
                if self.max_y == other.min_y:
                    return Relation.TOUCH
                if self.max_y > other.min_y:
                    return (
                        Relation.OVERLAP
                        if self.min_y < other.min_y
                        else Relation.ENCLOSED
                    )
                assert self.max_y < other.min_y
                return Relation.DISJOINT
            if self.min_x > other.min_x:
                if self.max_y == other.max_y:
                    return (
                        Relation.OVERLAP
                        if self.min_y < other.min_y
                        else Relation.ENCLOSED
                    )
                if self.max_y > other.max_y:
                    if self.min_y == other.max_y:
                        return Relation.TOUCH
                    if self.min_y > other.max_y:
                        return Relation.DISJOINT
                    assert self.min_y < other.max_y
                    return Relation.OVERLAP
                assert self.max_y < other.max_y
                if self.max_y == other.min_y:
                    return Relation.TOUCH
                if self.max_y > other.min_y:
                    if self.min_y == other.min_y:
                        return Relation.ENCLOSED
                    if self.min_y > other.min_y:
                        return Relation.WITHIN
                    assert self.min_y < other.min_y
                    return Relation.OVERLAP
                assert self.max_y < other.min_y
                return Relation.DISJOINT
            assert self.min_x < other.min_x
            if self.max_y == other.max_y:
                return Relation.OVERLAP
            if self.max_y > other.max_y:
                if self.min_y == other.max_y:
                    return Relation.TOUCH
                if self.min_y > other.max_y:
                    return Relation.DISJOINT
                assert self.min_y < other.max_y
                return Relation.OVERLAP
            assert self.max_y < other.max_y
            if self.max_y == other.min_y:
                return Relation.TOUCH
            if self.max_y > other.min_y:
                return Relation.OVERLAP
            return Relation.DISJOINT
        assert self.max_x < other.min_x
        return Relation.DISJOINT

    def touches(self, other: Self, /) -> bool:
        return (
            (self.min_x == other.max_x or self.max_x == other.min_x)
            and (self.min_y <= other.max_y and other.min_y <= self.max_y)
        ) or (
            (self.min_x <= other.max_x and other.min_x <= self.max_x)
            and (self.min_y == other.max_y or other.min_y == self.max_y)
        )

    def within(self, other: Self, /) -> bool:
        return (
            self.max_x < other.max_x
            and self.max_y < other.max_y
            and other.min_x < self.min_x
            and other.min_y < self.min_y
        )

    @overload
    def __eq__(self, other: Self, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...

    def __eq__(self, other: Any, /) -> Any:
        return (
            (
                self.min_x == other.min_x
                and self.max_x == other.max_x
                and self.min_y == other.min_y
                and self.max_y == other.max_y
            )
            if isinstance(other, type(self))
            else NotImplemented
        )

    def __hash__(self, /) -> int:
        return hash((self.min_x, self.max_x, self.min_y, self.max_y))

    def __repr__(self, /) -> str:
        return (
            f'{type(self).__qualname__}({self.min_x!r}, {self.max_x!r}, '
            f'{self.min_y!r}, {self.max_y!r})'
        )

    def __str__(self, /) -> str:
        return (
            f'{type(self).__qualname__}({self.min_x}, {self.max_x}, '
            f'{self.min_y}, {self.max_y})'
        )
