use std::cmp::Ordering;

use crate::locatable::{Locatable, Location};
use crate::operations::Orient;
use crate::oriented::Orientation;
use crate::seidel::edge::Edge;
use crate::seidel::trapezoid::Trapezoid;

use super::node::Node;
use super::trapezoidation::Trapezoidation;

impl Node {
    fn locate_trapezoid<'a, Point: Orient + PartialOrd>(
        &'a self,
        point: &Point,
        edges: &[Edge],
        endpoints: &[Point],
        nodes: &'a [Node],
    ) -> Option<&'a Trapezoid> {
        match self {
            Self::Leaf { trapezoid } => Some(trapezoid),
            Self::XNode {
                left_node_index,
                right_node_index,
                point_index,
            } => point
                .partial_cmp(&endpoints[*point_index])
                .and_then(|ordering| match ordering {
                    Ordering::Less => {
                        nodes[*left_node_index].locate_trapezoid(point, edges, endpoints, nodes)
                    }
                    Ordering::Greater => {
                        nodes[*right_node_index].locate_trapezoid(point, edges, endpoints, nodes)
                    }
                    Ordering::Equal => None,
                }),
            Self::YNode {
                above_node_index,
                below_node_index,
                edge_index,
            } => match edges[*edge_index].orientation_of(point, endpoints) {
                Orientation::Counterclockwise => {
                    nodes[*above_node_index].locate_trapezoid(point, edges, endpoints, nodes)
                }
                Orientation::Clockwise => {
                    nodes[*below_node_index].locate_trapezoid(point, edges, endpoints, nodes)
                }
                Orientation::Collinear => None,
            },
        }
    }
}

impl<Point: Orient + PartialOrd> Locatable<&Point> for &Trapezoidation<Point> {
    fn locate(self, point: &Point) -> Location {
        match self.get_root().locate_trapezoid(
            point,
            self.get_edges(),
            self.get_endpoints(),
            self.get_nodes(),
        ) {
            Some(trapezoid) => {
                if trapezoid.is_component {
                    Location::Interior
                } else {
                    Location::Exterior
                }
            }
            None => Location::Boundary,
        }
    }
}
