from collections.abc import Iterator
from typing import Any


def has_two_or_more_elements(
    iterator: Iterator[Any], /, _sentinel: object = object()
) -> bool:
    return (
        next(iterator, _sentinel) is not _sentinel
        and next(iterator, _sentinel) is not _sentinel
    )
