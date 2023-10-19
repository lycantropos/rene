import typing as t
from abc import (ABC,
                 abstractmethod)

import typing_extensions as te

from rene import hints


class BasePoint(ABC, t.Generic[hints.Scalar]):
    @property
    @abstractmethod
    def x(self) -> hints.Scalar:
        ...

    @property
    @abstractmethod
    def y(self) -> hints.Scalar:
        ...

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        ...

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (self.x == other.x and self.y == other.y
                if isinstance(other, type(self))
                else NotImplemented)

    @t.overload
    def __ge__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __ge__(self, other: t.Any, /) -> t.Any:
        ...

    def __ge__(self, other: t.Any, /) -> t.Any:
        return (self.x > other.x
                or self.x == other.x and self.y >= other.y
                if isinstance(other, type(self))
                else NotImplemented)

    @t.overload
    def __gt__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __gt__(self, other: t.Any, /) -> t.Any:
        ...

    def __gt__(self, other: t.Any, /) -> t.Any:
        return (self.x > other.x
                or self.x == other.x and self.y > other.y
                if isinstance(other, type(self))
                else NotImplemented)

    def __hash__(self) -> int:
        return hash((self.x, self.y))

    @t.overload
    def __le__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __le__(self, other: t.Any, /) -> t.Any:
        ...

    def __le__(self, other: t.Any, /) -> t.Any:
        return (self.x < other.x
                or self.x == other.x and self.y <= other.y
                if isinstance(other, type(self))
                else NotImplemented)

    @t.overload
    def __lt__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __lt__(self, other: t.Any, /) -> t.Any:
        ...

    def __lt__(self, other: t.Any, /) -> t.Any:
        return (self.x < other.x or self.x == other.x and self.y < other.y
                if isinstance(other, type(self))
                else NotImplemented)

    def __repr__(self) -> str:
        return f'{type(self).__qualname__}({self.x!r}, {self.y!r})'

    def __str__(self) -> str:
        return f'{type(self).__qualname__}({self.x}, {self.y})'
