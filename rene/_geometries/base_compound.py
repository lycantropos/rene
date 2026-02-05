from typing import Any, ClassVar, Generic

from rene import hints
from rene._context import Context


class BaseCompound(Generic[hints.ScalarT]):
    # can't use generic because of https://github.com/python/mypy/issues/5144
    _context: ClassVar[Context[Any]]
