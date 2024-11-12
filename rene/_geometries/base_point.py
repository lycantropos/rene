from abc import ABC, abstractmethod
from typing import Any, Generic, overload

from typing_extensions import Self

from rene import hints


class BasePoint(ABC, Generic[hints.Scalar]):
    @property
    @abstractmethod
    def x(self) -> hints.Scalar: ...

    @property
    @abstractmethod
    def y(self) -> hints.Scalar: ...

    @overload
    def __eq__(self, other: Self, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...

    def __eq__(self, other: Any, /) -> Any:
        return (
            self.x == other.x and self.y == other.y
            if isinstance(other, type(self))
            else NotImplemented
        )

    @overload
    def __ge__(self, other: Self, /) -> bool: ...

    @overload
    def __ge__(self, other: Any, /) -> Any: ...

    def __ge__(self, other: Any, /) -> Any:
        return (
            self.x > other.x or self.x == other.x and self.y >= other.y
            if isinstance(other, type(self))
            else NotImplemented
        )

    @overload
    def __gt__(self, other: Self, /) -> bool: ...

    @overload
    def __gt__(self, other: Any, /) -> Any: ...

    def __gt__(self, other: Any, /) -> Any:
        return (
            self.x > other.x or self.x == other.x and self.y > other.y
            if isinstance(other, type(self))
            else NotImplemented
        )

    def __hash__(self, /) -> int:
        return hash((self.x, self.y))

    @overload
    def __le__(self, other: Self, /) -> bool: ...

    @overload
    def __le__(self, other: Any, /) -> Any: ...

    def __le__(self, other: Any, /) -> Any:
        return (
            self.x < other.x or self.x == other.x and self.y <= other.y
            if isinstance(other, type(self))
            else NotImplemented
        )

    @overload
    def __lt__(self, other: Self, /) -> bool: ...

    @overload
    def __lt__(self, other: Any, /) -> Any: ...

    def __lt__(self, other: Any, /) -> Any:
        return (
            self.x < other.x or self.x == other.x and self.y < other.y
            if isinstance(other, type(self))
            else NotImplemented
        )

    def __repr__(self, /) -> str:
        return f'{type(self).__qualname__}({self.x!r}, {self.y!r})'

    def __str__(self, /) -> str:
        return f'{type(self).__qualname__}({self.x}, {self.y})'
