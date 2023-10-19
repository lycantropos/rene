import typing as t

from rene import hints
from rene._context import Context


class BaseCompound(t.Generic[hints.Scalar]):
    # can't use generic because of https://github.com/python/mypy/issues/5144
    _context: t.ClassVar[Context[t.Any]]
