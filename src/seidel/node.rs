use super::edge::Edge;
use super::trapezoid::Trapezoid;
use crate::operations::Orient;

#[derive(Clone)]
pub(crate) enum Node {
    Leaf {
        trapezoid: Trapezoid,
    },
    X {
        point_index: usize,
        left_node_index: usize,
        right_node_index: usize,
    },
    Y {
        edge_index: usize,
        below_node_index: usize,
        above_node_index: usize,
    },
}

impl Node {
    pub(super) fn new_leaf(
        left_point_index: usize,
        right_point_index: usize,
        below_edge_index: usize,
        above_edge_index: usize,
        edges: &[Edge],
        nodes: &mut Vec<Self>,
    ) -> usize {
        let result = nodes.len();
        let node = Self::Leaf {
            trapezoid: Trapezoid::new(
                edges[below_edge_index].interior_to_left
                    && !edges[above_edge_index].interior_to_left,
                left_point_index,
                right_point_index,
                below_edge_index,
                above_edge_index,
                result,
            ),
        };
        nodes.push(node);
        result
    }

    pub(super) fn new_x_node(
        point_index: usize,
        left_node_index: usize,
        right_node_index: usize,
        nodes: &mut Vec<Self>,
    ) -> usize {
        let result = nodes.len();
        let node = Self::X {
            point_index,
            left_node_index,
            right_node_index,
        };
        nodes.push(node);
        result
    }

    pub(super) fn new_y_node(
        edge_index: usize,
        below_node_index: usize,
        above_node_index: usize,
        nodes: &mut Vec<Self>,
    ) -> usize {
        let result = nodes.len();
        let node = Self::Y {
            edge_index,
            below_node_index,
            above_node_index,
        };
        nodes.push(node);
        result
    }

    pub(super) fn height(&self, nodes: &[Self]) -> usize {
        match self {
            Self::Leaf { .. } => 0,
            Self::X {
                left_node_index,
                right_node_index,
                ..
            } => {
                nodes[*left_node_index]
                    .height(nodes)
                    .max(nodes[*right_node_index].height(nodes))
                    + 1
            }
            Self::Y {
                above_node_index,
                below_node_index,
                ..
            } => {
                nodes[*above_node_index]
                    .height(nodes)
                    .max(nodes[*below_node_index].height(nodes))
                    + 1
            }
        }
    }

    pub(super) fn get_trapezoid(&self) -> &Trapezoid {
        match self {
            Self::Leaf { trapezoid } => trapezoid,
            _ => unreachable!("Only leaves have trapezoids."),
        }
    }

    pub(super) fn get_trapezoid_mut(&mut self) -> &mut Trapezoid {
        match self {
            Self::Leaf { trapezoid } => trapezoid,
            _ => unreachable!("Only leaves have trapezoids."),
        }
    }

    pub(super) fn search_intersecting_trapezoid<'a, Point: PartialOrd>(
        &'a self,
        edge: &Edge,
        edges: &[Edge],
        endpoints: &[Point],
        nodes: &'a [Node],
    ) -> &'a Trapezoid
    where
        for<'b> &'b Point: Orient,
    {
        match self {
            Self::Leaf { trapezoid } => trapezoid,
            Self::X {
                left_node_index,
                right_node_index,
                point_index,
            } => nodes[if endpoints[edge.left_point_index]
                .lt(&endpoints[*point_index])
            {
                *left_node_index
            } else {
                *right_node_index
            }]
            .search_intersecting_trapezoid(edge, edges, endpoints, nodes),
            Self::Y {
                above_node_index,
                below_node_index,
                edge_index,
            } => nodes[if edges[*edge_index].is_under(edge, endpoints) {
                *above_node_index
            } else {
                *below_node_index
            }]
            .search_intersecting_trapezoid(edge, edges, endpoints, nodes),
        }
    }
}
