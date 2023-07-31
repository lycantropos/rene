import typing as t

import typing_extensions as te
from rithm.fraction import Fraction

from rene import (Location,
                  Relation,
                  hints)
from rene._context import Context

_CompoundT = t.TypeVar('_CompoundT',
                       bound=hints.Compound[Fraction])


@te.final
class Empty:
    def locate(self, _point: hints.Point[Fraction], /) -> Location:
        return Location.EXTERIOR

    def relate_to(
            self, other: t.Union[hints.Compound[Fraction]], /
    ) -> Relation:
        context = self._context
        if not isinstance(other,
                          (context.contour_cls, context.empty_cls,
                           context.multisegment_cls, context.multipolygon_cls,
                           context.polygon_cls, context.segment_cls)):
            raise TypeError('Expected compound geometry, '
                            f'but got {type(other)}.')
        return (Relation.EQUAL
                if isinstance(other, context.empty_cls)
                else Relation.DISJOINT)

    _context: t.ClassVar[Context[Fraction]]

    __module__ = 'rene.exact'
    __slots__ = ()

    def __init_subclass__(cls, /, **_kwargs: t.Any) -> t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(cls) -> te.Self:
        return super().__new__(cls)

    @t.overload
    def __and__(self, other: hints.Compound[Fraction], /) -> te.Self:
        ...

    @t.overload
    def __and__(self, other: t.Any, /) -> t.Any:
        ...

    def __and__(self, other: t.Any, /) -> t.Any:
        context = self._context
        return (self
                if isinstance(other,
                              (context.contour_cls, context.empty_cls,
                               context.multisegment_cls,
                               context.multipolygon_cls, context.polygon_cls,
                               context.segment_cls))
                else NotImplemented)

    def __contains__(self, point: hints.Point[Fraction], /) -> bool:
        return False

    @t.overload
    def __eq__(self, other: te.Self, /) -> bool:
        ...

    @t.overload
    def __eq__(self, other: t.Any, /) -> t.Any:
        ...

    def __eq__(self, other: t.Any, /) -> t.Any:
        return (True
                if isinstance(other, self._context.empty_cls)
                else NotImplemented)

    def __hash__(self) -> int:
        return 0

    @t.overload
    def __or__(self, other: _CompoundT, /) -> _CompoundT:
        ...

    @t.overload
    def __or__(self, other: t.Any, /) -> t.Any:
        ...

    def __or__(self, other: t.Any, /) -> t.Any:
        context = self._context
        return (other
                if isinstance(other,
                              (context.contour_cls, context.empty_cls,
                               context.multisegment_cls,
                               context.multipolygon_cls, context.polygon_cls,
                               context.segment_cls))
                else NotImplemented)

    def __repr__(self) -> str:
        return f'{type(self).__qualname__}()'

    @t.overload
    def __sub__(self, other: _CompoundT, /) -> te.Self:
        ...

    @t.overload
    def __sub__(self, other: t.Any, /) -> t.Any:
        ...

    def __sub__(self, other: t.Any, /) -> t.Any:
        context = self._context
        return (self
                if isinstance(other,
                              (context.contour_cls, context.empty_cls,
                               context.multisegment_cls,
                               context.multipolygon_cls, context.polygon_cls,
                               context.segment_cls))
                else NotImplemented)

    @t.overload
    def __xor__(self, other: te.Self, /) -> te.Self:
        ...

    @t.overload
    def __xor__(self, other: _CompoundT, /) -> _CompoundT:
        ...

    @t.overload
    def __xor__(self, other: t.Any, /) -> t.Any:
        ...

    def __xor__(self, other: t.Any, /) -> t.Any:
        context = self._context
        return (other
                if isinstance(other,
                              (context.contour_cls, context.empty_cls,
                               context.multisegment_cls,
                               context.multipolygon_cls, context.polygon_cls,
                               context.segment_cls))
                else NotImplemented)
