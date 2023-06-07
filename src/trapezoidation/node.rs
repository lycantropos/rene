use super::edge::Edge;
use super::trapezoid::Trapezoid;

#[derive(Clone)]
pub(crate) enum Node<Point> {
    Leaf {
        trapezoid: Trapezoid<Point>,
    },
    XNode {
        point: Point,
        left_node_index: usize,
        right_node_index: usize,
    },
    YNode {
        edge_index: usize,
        below_node_index: usize,
        above_node_index: usize,
    },
}

impl<Point> Node<Point> {
    pub(super) fn new_leaf(
        left_point: Point,
        right_point: Point,
        below_edge_index: usize,
        above_edge_index: usize,
        nodes: &mut Vec<Self>,
    ) -> usize {
        let result = nodes.len();
        let node = Self::Leaf {
            trapezoid: Trapezoid::<Point>::new(
                left_point,
                right_point,
                below_edge_index,
                above_edge_index,
                result,
            ),
        };
        nodes.push(node);
        result
    }

    pub(super) fn new_x_node(
        point: Point,
        left_node_index: usize,
        right_node_index: usize,
        nodes: &mut Vec<Self>,
    ) -> usize {
        let result = nodes.len();
        let node = Self::XNode {
            point,
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
        let node = Self::YNode {
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
            Self::XNode {
                left_node_index,
                right_node_index,
                ..
            } => {
                nodes[*left_node_index]
                    .height(nodes)
                    .max(nodes[*right_node_index].height(nodes))
                    + 1
            }
            Self::YNode {
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

    pub(super) fn get_trapezoid(&self) -> &Trapezoid<Point> {
        match self {
            Self::Leaf { trapezoid } => trapezoid,
            _ => unreachable!("Only leaves have trapezoids."),
        }
    }

    pub(super) fn get_trapezoid_mut(&mut self) -> &mut Trapezoid<Point> {
        match self {
            Self::Leaf { trapezoid } => trapezoid,
            _ => unreachable!("Only leaves have trapezoids."),
        }
    }

    pub(super) fn search_intersecting_trapezoid<'a>(
        &'a self,
        edge: &Edge<Point>,
        edges: &[Edge<Point>],
        nodes: &'a [Node<Point>],
    ) -> &'a Trapezoid<Point>
    where
        Point: PartialOrd,
        Edge<Point>: PartialOrd,
    {
        match self {
            Self::Leaf { trapezoid } => trapezoid,
            Self::XNode {
                left_node_index,
                right_node_index,
                point,
            } => &nodes[if edge.left_point.lt(point) {
                *left_node_index
            } else {
                *right_node_index
            }]
            .search_intersecting_trapezoid(edge, edges, nodes),
            Self::YNode {
                above_node_index,
                below_node_index,
                edge_index,
            } => nodes[if edges[*edge_index].lt(&edge) {
                *above_node_index
            } else {
                *below_node_index
            }]
            .search_intersecting_trapezoid(edge, edges, nodes),
        }
    }
}
