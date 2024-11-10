import typing as t

import typing_extensions as te
from rithm.fraction import Fraction

from rene import hints
from rene._context import Context
from rene._geometries.base_empty import BaseEmpty

_CompoundT = t.TypeVar("_CompoundT", bound=hints.Compound[Fraction])


@te.final
class Empty(BaseEmpty[Fraction]):
    _context: t.ClassVar[Context[Fraction]]

    __module__ = "rene.exact"
    __slots__ = ()

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f"type {cls.__qualname__!r} " "is not an acceptable base type")

    def __new__(cls) -> te.Self:
        return super().__new__(cls)
