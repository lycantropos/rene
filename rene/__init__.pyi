from typing import (Any as _Any,
                    overload as _overload)

__version__: str

MIN_CONTOUR_VERTICES_COUNT: int = ...


class Orientation:
    CLOCKWISE: 'Orientation' = ...
    COLLINEAR: 'Orientation' = ...
    COUNTERCLOCKWISE: 'Orientation' = ...

    @property
    def value(self) -> int:
        ...

    @_overload
    def __eq__(self, other: 'Orientation') -> bool:
        ...

    @_overload
    def __eq__(self, other: _Any) -> _Any:
        ...

    def __repr__(self) -> str:
        ...

    def __str__(self) -> str:
        ...
