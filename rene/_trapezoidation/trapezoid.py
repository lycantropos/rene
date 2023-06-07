from __future__ import annotations

import typing as _t

import typing_extensions as _te
from reprit.base import generate_repr

from rene import hints as _hints
from .edge import Edge


class Trapezoid(_t.Generic[_hints.Scalar]):
    def is_component(self, edges: _t.Sequence[Edge[_hints.Scalar]]) -> bool:
        """
        Checks if the trapezoid is a component of decomposed geometry.
        """
        return (edges[self.below_edge_index].interior_to_left
                and not edges[self.above_edge_index].interior_to_left)

    @property
    def lower_left_leaf_index(self) -> _t.Optional[int]:
        return self._lower_left_leaf_index

    @property
    def lower_right_leaf_index(self) -> _t.Optional[int]:
        return self._lower_right_leaf_index

    @property
    def upper_left_leaf_index(self) -> _t.Optional[int]:
        return self._upper_left_leaf_index

    @property
    def upper_right_leaf_index(self) -> _t.Optional[int]:
        return self._upper_right_leaf_index

    def set_as_lower_left(self, value: _t.Optional[_te.Self]) -> None:
        if value is None:
            self._lower_left_leaf_index = None
        else:
            self._lower_left_leaf_index = value.leaf_index
            value._lower_right_leaf_index = self.leaf_index

    def set_as_lower_right(self, value: _t.Optional[_te.Self]) -> None:
        if value is None:
            self._lower_right_leaf_index = None
        else:
            self._lower_right_leaf_index = value.leaf_index
            value._lower_left_leaf_index = self.leaf_index

    def set_as_upper_left(self, value: _t.Optional[_te.Self]) -> None:
        if value is None:
            self._upper_left_leaf_index = None
        else:
            self._upper_left_leaf_index = value.leaf_index
            value._upper_right_leaf_index = self.leaf_index

    def set_as_upper_right(self, value: _t.Optional[_te.Self]) -> None:
        if value is None:
            self._upper_right_leaf_index = None
        else:
            self._upper_right_leaf_index = value.leaf_index
            value._upper_left_leaf_index = self.leaf_index

    __slots__ = (
        'above_edge_index', 'below_edge_index', 'leaf_index', 'left_point',
        'right_point', '_lower_left_leaf_index', '_lower_right_leaf_index',
        '_upper_left_leaf_index', '_upper_right_leaf_index'
    )

    _lower_left_leaf_index: _t.Optional[int]
    _lower_right_leaf_index: _t.Optional[int]
    _upper_left_leaf_index: _t.Optional[int]
    _upper_right_leaf_index: _t.Optional[int]

    def __init__(self,
                 left_point: _hints.Point[_hints.Scalar],
                 right_point: _hints.Point[_hints.Scalar],
                 below_edge_index: int,
                 above_edge_index: int,
                 leaf_index: int,
                 _lower_left_leaf_index: _t.Optional[int] = None,
                 _lower_right_leaf_index: _t.Optional[int] = None,
                 _upper_left_leaf_index: _t.Optional[int] = None,
                 _upper_right_leaf_index: _t.Optional[int] = None) -> None:
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
    __str__ = generate_repr(__init__,
                            argument_serializer=str)
