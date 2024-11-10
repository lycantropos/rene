import typing as t


def has_two_or_more_elements(
    iterator: t.Iterator[t.Any], /, _sentinel: object = object()
) -> bool:
    return (
        next(iterator, _sentinel) is not _sentinel
        and next(iterator, _sentinel) is not _sentinel
    )
