from typing import Any, TypeVar, overload

from typing_extensions import Self

from rene import Location, Relation, hints

from .base_compound import BaseCompound

_CompoundT = TypeVar('_CompoundT', bound=hints.Compound[Any])


class BaseEmpty(BaseCompound[hints.Scalar]):
    def locate(self, _point: hints.Point[hints.Scalar], /) -> Location:
        return Location.EXTERIOR

    def relate_to(self, other: hints.Compound[hints.Scalar], /) -> Relation:
        context = self._context
        if not isinstance(
            other,
            (
                context.contour_cls,
                context.empty_cls,
                context.multisegment_cls,
                context.multipolygon_cls,
                context.polygon_cls,
                context.segment_cls,
            ),
        ):
            raise TypeError(
                f'Expected compound geometry, but got {type(other)}.'
            )
        return (
            Relation.EQUAL
            if isinstance(other, context.empty_cls)
            else Relation.DISJOINT
        )

    @overload
    def __and__(self, other: hints.Compound[hints.Scalar], /) -> Self: ...

    @overload
    def __and__(self, other: Any, /) -> Any: ...

    def __and__(self, other: Any, /) -> Any:
        context = self._context
        return (
            self
            if isinstance(
                other,
                (
                    context.contour_cls,
                    context.empty_cls,
                    context.multisegment_cls,
                    context.multipolygon_cls,
                    context.polygon_cls,
                    context.segment_cls,
                ),
            )
            else NotImplemented
        )

    def __contains__(self, point: hints.Point[hints.Scalar], /) -> bool:
        return False

    @overload
    def __eq__(self, other: Self, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...

    def __eq__(self, other: Any, /) -> Any:
        return True if isinstance(other, type(self)) else NotImplemented

    def __hash__(self, /) -> int:
        return 0

    @overload
    def __or__(self, other: _CompoundT, /) -> _CompoundT: ...

    @overload
    def __or__(self, other: Any, /) -> Any: ...

    def __or__(self, other: Any, /) -> Any:
        context = self._context
        return (
            other
            if isinstance(
                other,
                (
                    context.contour_cls,
                    context.empty_cls,
                    context.multisegment_cls,
                    context.multipolygon_cls,
                    context.polygon_cls,
                    context.segment_cls,
                ),
            )
            else NotImplemented
        )

    def __repr__(self, /) -> str:
        return f'{type(self).__qualname__}()'

    @overload
    def __sub__(self, other: _CompoundT, /) -> Self: ...

    @overload
    def __sub__(self, other: Any, /) -> Any: ...

    def __sub__(self, other: Any, /) -> Any:
        context = self._context
        return (
            self
            if isinstance(
                other,
                (
                    context.contour_cls,
                    context.empty_cls,
                    context.multisegment_cls,
                    context.multipolygon_cls,
                    context.polygon_cls,
                    context.segment_cls,
                ),
            )
            else NotImplemented
        )

    @overload
    def __xor__(self, other: Self, /) -> Self: ...

    @overload
    def __xor__(self, other: _CompoundT, /) -> _CompoundT: ...

    @overload
    def __xor__(self, other: Any, /) -> Any: ...

    def __xor__(self, other: Any, /) -> Any:
        context = self._context
        return (
            other
            if isinstance(
                other,
                (
                    context.contour_cls,
                    context.empty_cls,
                    context.multisegment_cls,
                    context.multipolygon_cls,
                    context.polygon_cls,
                    context.segment_cls,
                ),
            )
            else NotImplemented
        )
