pub(crate) type QuadEdge = usize;

pub(super) fn to_opposite_edge(edge: QuadEdge) -> QuadEdge {
    ((edge >> 2) << 2) + ((edge + 2) & 3)
}

pub(super) fn to_rotated_edge(edge: QuadEdge) -> QuadEdge {
    ((edge >> 2) << 2) + ((edge + 1) & 3)
}
