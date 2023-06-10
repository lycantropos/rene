from __future__ import annotations

import typing as t

import typing_extensions as te
from reprit.base import generate_repr

from rene import hints
from .edge import Edge


class Trapezoid(t.Generic[hints.Scalar]):
    def is_component(self, edges: t.Sequence[Edge[hints.Scalar]], /) -> bool:
        """
        Checks if the trapezoid is a component of decomposed geometry.
        """
        return (edges[self.below_edge_index].interior_to_left
                and not edges[self.above_edge_index].interior_to_left)

    @property
    def lower_left_leaf_index(self) -> t.Optional[int]:
        return self._lower_left_leaf_index

    @property
    def lower_right_leaf_index(self) -> t.Optional[int]:
        return self._lower_right_leaf_index

    @property
    def upper_left_leaf_index(self) -> t.Optional[int]:
        return self._upper_left_leaf_index

    @property
    def upper_right_leaf_index(self) -> t.Optional[int]:
        return self._upper_right_leaf_index

    def set_as_lower_left(self, value: t.Optional[te.Self], /) -> None:
        if value is None:
            self._lower_left_leaf_index = None
        else:
            self._lower_left_leaf_index = value.leaf_index
            value._lower_right_leaf_index = self.leaf_index

    def set_as_lower_right(self, value: t.Optional[te.Self], /) -> None:
        if value is None:
            self._lower_right_leaf_index = None
        else:
            self._lower_right_leaf_index = value.leaf_index
            value._lower_left_leaf_index = self.leaf_index

    def set_as_upper_left(self, value: t.Optional[te.Self], /) -> None:
        if value is None:
            self._upper_left_leaf_index = None
        else:
            self._upper_left_leaf_index = value.leaf_index
            value._upper_right_leaf_index = self.leaf_index

    def set_as_upper_right(self, value: t.Optional[te.Self], /) -> None:
        if value is None:
            self._upper_right_leaf_index = None
        else:
            self._upper_right_leaf_index = value.leaf_index
            value._upper_left_leaf_index = self.leaf_index

    _lower_left_leaf_index: t.Optional[int]
    _lower_right_leaf_index: t.Optional[int]
    _upper_left_leaf_index: t.Optional[int]
    _upper_right_leaf_index: t.Optional[int]

    __slots__ = (
        'above_edge_index', 'below_edge_index', 'leaf_index', 'left_point',
        'right_point', '_lower_left_leaf_index', '_lower_right_leaf_index',
        '_upper_left_leaf_index', '_upper_right_leaf_index'
    )

    def __init__(self,
                 left_point: hints.Point[hints.Scalar],
                 right_point: hints.Point[hints.Scalar],
                 below_edge_index: int,
                 above_edge_index: int,
                 leaf_index: int,
                 /) -> None:
        assert left_point < right_point, 'Incorrect endpoints order'
        (
            self.above_edge_index, self.below_edge_index, self.left_point,
            self.leaf_index, self.right_point
        ) = (
            above_edge_index, below_edge_index, left_point, leaf_index,
            right_point
        )
        self._lower_left_leaf_index = self._lower_right_leaf_index = None
        self._upper_left_leaf_index = self._upper_right_leaf_index = None

    __repr__ = generate_repr(__init__)
