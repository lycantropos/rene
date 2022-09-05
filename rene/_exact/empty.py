from typing import Optional

from reprit.base import generate_repr

from .context import Context


class Empty:
    _context: Optional[Context] = None

    __module__ = 'rene.exact'
    __slots__ = ()

    def __new__(cls):
        return super().__new__(cls)

    def __and__(self, other):
        return (self
                if isinstance(other, (self._context.empty_cls,
                                      self._context.polygon_cls,
                                      self._context.multipolygon_cls))
                else NotImplemented)

    def __eq__(self, other):
        return (True
                if isinstance(other, self._context.empty_cls)
                else NotImplemented)

    def __hash__(self):
        return 0

    def __or__(self, other):
        return (other
                if isinstance(other, (self._context.empty_cls,
                                      self._context.polygon_cls,
                                      self._context.multipolygon_cls))
                else NotImplemented)

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __str__(self):
        return f'{type(self).__qualname__}()'
