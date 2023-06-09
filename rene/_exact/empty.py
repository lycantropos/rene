import typing as _t

import typing_extensions as _te
from reprit.base import generate_repr as _generate_repr
from rithm.fraction import Fraction as _Fraction

from rene import (Location as _Location,
                  Relation as _Relation,
                  hints as _hints)
from rene._context import Context as _Context


class Empty:
    def locate(self, _point: _hints.Point[_Fraction]) -> _Location:
        return _Location.EXTERIOR

    def relate_to(self,
                  _other: _t.Union[_hints.Compound[_Fraction]]) -> _Relation:
        context = self._context
        if not isinstance(_other,
                          (context.contour_cls, context.empty_cls,
                           context.multisegment_cls, context.multipolygon_cls,
                           context.polygon_cls, context.segment_cls)):
            raise TypeError('Expected compound geometry, '
                            f'but got {type(_other)}.')
        return (_Relation.EQUAL
                if isinstance(_other, context.empty_cls)
                else _Relation.DISJOINT)

    _context: _t.ClassVar[_Context[_Fraction]]

    __module__ = 'rene.exact'
    __slots__ = ()

    def __new__(cls) -> _te.Self:
        return super().__new__(cls)

    @_t.overload
    def __and__(
            self,
            other: _t.Union[
                _te.Self, _hints.Multipolygon[_Fraction],
                _hints.Polygon[_Fraction]
            ]
    ) -> _te.Self:
        ...

    @_t.overload
    def __and__(self, other: _t.Any) -> _t.Any:
        ...

    def __and__(self, other: _t.Any) -> _t.Any:
        return (self
                if isinstance(other, (self._context.empty_cls,
                                      self._context.polygon_cls,
                                      self._context.multipolygon_cls))
                else NotImplemented)

    def __contains__(self, point: _hints.Point[_Fraction]) -> bool:
        return False

    @_t.overload
    def __eq__(self, other: _te.Self) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __eq__(self, other: _t.Any) -> _t.Any:
        return (True
                if isinstance(other, self._context.empty_cls)
                else NotImplemented)

    def __hash__(self) -> int:
        return 0

    @_t.overload
    def __or__(self, other: _te.Self) -> _te.Self:
        ...

    @_t.overload
    def __or__(
            self, other: _hints.Multipolygon[_Fraction]
    ) -> _hints.Multipolygon[_Fraction]:
        ...

    @_t.overload
    def __or__(self,
               other: _hints.Polygon[_Fraction]) -> _hints.Polygon[_Fraction]:
        ...

    @_t.overload
    def __or__(self, other: _t.Any) -> _t.Any:
        ...

    def __or__(self, other: _t.Any) -> _t.Any:
        return (other
                if isinstance(other, (self._context.empty_cls,
                                      self._context.polygon_cls,
                                      self._context.multipolygon_cls))
                else NotImplemented)

    __repr__ = _generate_repr(__new__,
                              with_module_name=True)

    def __str__(self) -> str:
        return f'{type(self).__qualname__}()'

    @_t.overload
    def __sub__(
            self,
            other: _t.Union[
                _te.Self, _hints.Multipolygon[_Fraction],
                _hints.Polygon[_Fraction]
            ]
    ) -> _te.Self:
        ...

    @_t.overload
    def __sub__(self, other: _t.Any) -> _t.Any:
        ...

    def __sub__(self, other: _t.Any) -> _t.Any:
        return (self
                if isinstance(other, (self._context.empty_cls,
                                      self._context.polygon_cls,
                                      self._context.multipolygon_cls))
                else NotImplemented)

    @_t.overload
    def __xor__(self, other: _te.Self) -> _te.Self:
        ...

    @_t.overload
    def __xor__(
            self, other: _hints.Multipolygon[_Fraction]
    ) -> _hints.Multipolygon[_Fraction]:
        ...

    @_t.overload
    def __xor__(self,
                other: _hints.Polygon[_Fraction]) -> _hints.Polygon[_Fraction]:
        ...

    @_t.overload
    def __xor__(self, other: _t.Any) -> _t.Any:
        ...

    def __xor__(self, other: _t.Any) -> _t.Any:
        return (other
                if isinstance(other, (self._context.empty_cls,
                                      self._context.polygon_cls,
                                      self._context.multipolygon_cls))
                else NotImplemented)
