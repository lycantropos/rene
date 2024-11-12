from typing import Any, ClassVar, NoReturn, TypeVar

from rithm.fraction import Fraction
from typing_extensions import Self, final

from rene import hints
from rene._context import Context
from rene._geometries.base_empty import BaseEmpty

_CompoundT = TypeVar('_CompoundT', bound=hints.Compound[Fraction])


@final
class Empty(BaseEmpty[Fraction]):
    _context: ClassVar[Context[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = ()

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(cls) -> Self:
        return super().__new__(cls)
