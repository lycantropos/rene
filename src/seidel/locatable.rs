use crate::locatable::{Locatable, Location};
use crate::operations::Orient;
use crate::oriented::Orientation;
use crate::seidel::edge::Edge;
use crate::seidel::trapezoid::Trapezoid;

use super::node::Node;
use super::trapezoidation::Trapezoidation;

impl<Point: Orient + PartialOrd> Node<Point> {
    fn locate_trapezoid<'a>(
        &'a self,
        point: &Point,
        edges: &[Edge<Point>],
        nodes: &'a [Node<Point>],
    ) -> Option<&'a Trapezoid<Point>> {
        match self {
            Node::<Point>::Leaf { trapezoid } => Some(trapezoid),
            Node::<Point>::XNode {
                left_node_index,
                right_node_index,
                point: node_point,
            } => {
                if point < node_point {
                    nodes[*left_node_index].locate_trapezoid(point, edges, nodes)
                } else if node_point < point {
                    nodes[*right_node_index].locate_trapezoid(point, edges, nodes)
                } else {
                    None
                }
            }
            Node::<Point>::YNode {
                above_node_index,
                below_node_index,
                edge_index,
            } => match edges[*edge_index].orientation_of(point) {
                Orientation::Counterclockwise => {
                    nodes[*above_node_index].locate_trapezoid(point, edges, nodes)
                }
                Orientation::Clockwise => {
                    nodes[*below_node_index].locate_trapezoid(point, edges, nodes)
                }
                Orientation::Collinear => None,
            },
        }
    }
}

impl<Point: Orient + PartialOrd> Locatable<&Point> for &Trapezoidation<Point> {
    fn locate(self, point: &Point) -> Location {
        match self
            .get_root()
            .locate_trapezoid(point, self.get_edges(), self.get_nodes())
        {
            Some(trapezoid) => {
                if self.get_edges()[trapezoid.below_edge_index].interior_to_left
                    && !self.get_edges()[trapezoid.above_edge_index].interior_to_left
                {
                    Location::Interior
                } else {
                    Location::Exterior
                }
            }
            None => Location::Boundary,
        }
    }
}
