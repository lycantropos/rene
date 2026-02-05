from typing import Any, TypeVar, overload

from typing_extensions import Self

from rene import hints
from rene.enums import Location, Relation

from .base_compound import BaseCompound
from .utils import is_compound, is_empty

_CompoundT = TypeVar('_CompoundT', bound=hints.Compound[Any])


class BaseEmpty(BaseCompound[hints.ScalarT]):
    def locate(self, _point: hints.Point[hints.ScalarT], /) -> Location:
        return Location.EXTERIOR

    def relate_to(self, other: hints.Compound[hints.ScalarT], /) -> Relation:
        return (
            Relation.EQUAL
            if is_empty(other, context=self._context)
            else Relation.DISJOINT
        )

    @overload
    def __and__(self, other: hints.Compound[hints.ScalarT], /) -> Self: ...

    @overload
    def __and__(self, other: Any, /) -> Any: ...

    def __and__(self, other: Any, /) -> Any:
        context = self._context
        return self if is_compound(other, context=context) else NotImplemented

    def __contains__(self, point: hints.Point[hints.ScalarT], /) -> bool:
        return False

    @overload
    def __eq__(self, other: Self, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...

    def __eq__(self, other: Any, /) -> Any:
        return True if isinstance(other, type(self)) else NotImplemented

    def __hash__(self, /) -> int:
        return 0

    @overload
    def __or__(self, other: _CompoundT, /) -> _CompoundT: ...

    @overload
    def __or__(self, other: Any, /) -> Any: ...

    def __or__(self, other: Any, /) -> Any:
        context = self._context
        return other if is_compound(other, context=context) else NotImplemented

    def __repr__(self, /) -> str:
        return f'{type(self).__qualname__}()'

    @overload
    def __sub__(self, other: hints.Compound[Any], /) -> Self: ...

    @overload
    def __sub__(self, other: Any, /) -> Any: ...

    def __sub__(self, other: Any, /) -> Any:
        context = self._context
        return self if is_compound(other, context=context) else NotImplemented

    @overload
    def __xor__(self, other: Self, /) -> Self: ...

    @overload
    def __xor__(self, other: _CompoundT, /) -> _CompoundT: ...

    @overload
    def __xor__(self, other: Any, /) -> Any: ...

    def __xor__(self, other: Any, /) -> Any:
        context = self._context
        return other if is_compound(other, context=context) else NotImplemented
