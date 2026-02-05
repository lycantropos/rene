from __future__ import annotations


class Trapezoid:
    @property
    def lower_left_node_index(self, /) -> int | None:
        return self._lower_left_node_index

    @property
    def lower_right_node_index(self, /) -> int | None:
        return self._lower_right_node_index

    @property
    def upper_left_node_index(self, /) -> int | None:
        return self._upper_left_node_index

    @property
    def upper_right_node_index(self, /) -> int | None:
        return self._upper_right_node_index

    def reset_lower_left(self, /) -> None:
        self._lower_left_node_index = None

    def reset_lower_right(self, /) -> None:
        self._lower_right_node_index = None

    def reset_upper_left(self, /) -> None:
        self._upper_left_node_index = None

    def reset_upper_right(self, /) -> None:
        self._upper_right_node_index = None

    def set_as_lower_left(self, value: Trapezoid) -> None:
        self._lower_left_node_index = value.leaf_index
        value._lower_right_node_index = self.leaf_index

    def set_as_lower_right(self, value: Trapezoid) -> None:
        self._lower_right_node_index = value.leaf_index
        value._lower_left_node_index = self.leaf_index

    def set_as_upper_left(self, value: Trapezoid) -> None:
        self._upper_left_node_index = value.leaf_index
        value._upper_right_node_index = self.leaf_index

    def set_as_upper_right(self, value: Trapezoid) -> None:
        self._upper_right_node_index = value.leaf_index
        value._upper_left_node_index = self.leaf_index

    _lower_left_node_index: int | None
    _lower_right_node_index: int | None
    _upper_left_node_index: int | None
    _upper_right_node_index: int | None

    __slots__ = (
        '_lower_left_node_index',
        '_lower_right_node_index',
        '_upper_left_node_index',
        '_upper_right_node_index',
        'above_edge_index',
        'below_edge_index',
        'is_component',
        'leaf_index',
        'left_point_index',
        'right_point_index',
    )

    def __init__(
        self,
        /,
        *,
        is_component: bool,
        left_point_index: int,
        right_point_index: int,
        below_edge_index: int,
        above_edge_index: int,
        leaf_index: int,
    ) -> None:
        (
            self.above_edge_index,
            self.below_edge_index,
            self.is_component,
            self.left_point_index,
            self.leaf_index,
            self.right_point_index,
        ) = (
            above_edge_index,
            below_edge_index,
            is_component,
            left_point_index,
            leaf_index,
            right_point_index,
        )
        self._lower_left_node_index = self._lower_right_node_index = None
        self._upper_left_node_index = self._upper_right_node_index = None
