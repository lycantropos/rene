use std::ops::{Add, Sub};

use traiter::numbers::{Unitary, Zeroable};

use crate::bounded;
use crate::geometries::{Multisegment, Segment};
use crate::operations::{to_arg_min, Orient};
use crate::oriented::{Orientation, Oriented};
use crate::traits::{Contoural, Multisegmental, Polygonal, Segmental};
use crate::trapezoidation::node::Node;
use crate::trapezoidation::trapezoid::Trapezoid;

use super::edge::Edge;

#[derive(Clone)]
pub(crate) struct Trapezoidation<Point> {
    edges: Vec<Edge<Point>>,
    nodes: Vec<Node<Point>>,
}

impl<Point> Trapezoidation<Point> {
    pub(super) fn get_root(&self) -> &Node<Point> {
        &self.nodes[0]
    }

    pub(super) fn get_edges(&self) -> &[Edge<Point>] {
        &self.edges
    }

    pub(super) fn get_nodes(&self) -> &[Node<Point>] {
        &self.nodes
    }

    pub(crate) fn height(&self) -> usize {
        self.get_root().height(self.get_nodes())
    }
}

impl<Point> Trapezoidation<Point> {
    pub(crate) fn from_multisegment<
        Multisegment: bounded::Bounded<Scalar> + Multisegmental<Segment = Segment>,
        Scalar,
        Segment: Segmental<Endpoint = Point>,
        Shuffler: FnOnce(&mut Vec<Edge<Point>>),
    >(
        multisegment: &Multisegment,
        shuffler: Shuffler,
    ) -> Self
    where
        Point: Clone + From<(Scalar, Scalar)> + Orient + PartialOrd,
        Scalar: Clone + Unitary + Zeroable,
        for<'b> &'b Scalar:
            Add<Scalar, Output = Scalar> + Sub<Scalar, Output = Scalar> + Sub<Output = Scalar>,
    {
        let segments = multisegment.segments();
        let mut edges = Vec::<Edge<Point>>::with_capacity(segments.len());
        for segment in segments {
            let (start, end) = (segment.start(), segment.end());
            edges.push(if start < end {
                Edge::<Point> {
                    left_point: start,
                    right_point: end,
                    interior_to_left: false,
                }
            } else {
                Edge::<Point> {
                    left_point: end,
                    right_point: start,
                    interior_to_left: false,
                }
            })
        }
        shuffler(&mut edges);
        Self::from_box_with_edges(multisegment.to_bounding_box(), edges)
    }

    pub(crate) fn from_polygon<
        Polygon: bounded::Bounded<Scalar> + Polygonal<Contour = Contour>,
        Scalar,
        Contour: Contoural<Segment = Segment> + Oriented,
        Segment: Segmental<Endpoint = Point>,
        Shuffler: FnOnce(&mut Vec<Edge<Point>>),
    >(
        polygon: &Polygon,
        shuffler: Shuffler,
    ) -> Self
    where
        Point: Clone + From<(Scalar, Scalar)> + Orient + PartialOrd,
        Scalar: Clone + Unitary + Zeroable,
        for<'b> &'b Scalar:
            Add<Scalar, Output = Scalar> + Sub<Scalar, Output = Scalar> + Sub<Output = Scalar>,
    {
        let (border, holes) = (polygon.border(), polygon.holes());
        let mut edges = Vec::<Edge<Point>>::with_capacity(
            border.segments_count()
                + holes
                    .iter()
                    .map(Multisegmental::segments_count)
                    .sum::<usize>(),
        );
        Self::populate_edges_from_contour(border, &mut edges);
        for hole in holes {
            Self::populate_edges_from_contour(hole, &mut edges);
        }
        shuffler(&mut edges);
        Self::from_box_with_edges(polygon.to_bounding_box(), edges)
    }

    fn populate_edges_from_contour<
        Contour: Contoural<Segment = Segment> + Oriented,
        Segment: Segmental<Endpoint = Point>,
    >(
        contour: Contour,
        edges: &mut Vec<Edge<Point>>,
    ) where
        Point: PartialOrd,
    {
        let is_border_positively_oriented =
            contour.to_orientation() == Orientation::Counterclockwise;
        for segment in contour.segments() {
            let (start, end) = (segment.start(), segment.end());
            edges.push(if start < end {
                Edge::<Point> {
                    left_point: start,
                    right_point: end,
                    interior_to_left: is_border_positively_oriented,
                }
            } else {
                Edge::<Point> {
                    left_point: end,
                    right_point: start,
                    interior_to_left: !is_border_positively_oriented,
                }
            })
        }
    }

    fn from_box_with_edges<Scalar>(box_: bounded::Box<Scalar>, mut edges: Vec<Edge<Point>>) -> Self
    where
        Point: Clone + From<(Scalar, Scalar)> + Orient + PartialOrd,
        Scalar: Clone + Unitary + Zeroable,
        for<'b> &'b Scalar:
            Add<Scalar, Output = Scalar> + Sub<Scalar, Output = Scalar> + Sub<Output = Scalar>,
    {
        debug_assert!(!edges.is_empty());
        let mut nodes = Vec::<Node<Point>>::new();
        let first_leaf_index = Self::leaf_from_box_with_edges(box_, &mut edges, &mut nodes);
        debug_assert_eq!(first_leaf_index, 0usize);
        Self::add_edge_to_single_trapezoid(0usize, 0usize, &edges, &mut nodes);
        for edge_index in 1..edges.len() - 2 {
            Self::add_edge(edge_index, &edges, &mut nodes)
        }
        Self { edges, nodes }
    }

    fn leaf_from_box_with_edges<Scalar>(
        box_: bounded::Box<Scalar>,
        edges: &mut Vec<Edge<Point>>,
        nodes: &mut Vec<Node<Point>>,
    ) -> usize
    where
        Point: Clone + From<(Scalar, Scalar)> + Orient + PartialOrd,
        Scalar: Clone + Unitary + Zeroable,
        for<'b> &'b Scalar:
            Add<Scalar, Output = Scalar> + Sub<Scalar, Output = Scalar> + Sub<Output = Scalar>,
    {
        let (min_x, min_y, max_x, max_y) = (
            box_.get_min_x(),
            box_.get_min_y(),
            box_.get_max_x(),
            box_.get_max_y(),
        );
        let (delta_x, delta_y) = (max_x - min_x, max_y - min_y);
        let (min_x, max_x) = if delta_x.is_zero() {
            // handle vertical case
            (min_x - Scalar::one(), max_x + Scalar::one())
        } else {
            (min_x - &delta_x, max_x + delta_x)
        };
        let (min_y, max_y) = if delta_y.is_zero() {
            // handle horizontal case
            (min_y - Scalar::one(), max_y + Scalar::one())
        } else {
            (min_y - &delta_y, max_y + delta_y)
        };
        let below_edge_index = edges.len();
        edges.push(Edge::<Point> {
            left_point: Point::from((min_x.clone(), min_y.clone())),
            right_point: Point::from((max_x.clone(), min_y.clone())),
            interior_to_left: false,
        });
        let above_edge_index = edges.len();
        edges.push(Edge::<Point> {
            left_point: Point::from((min_x.clone(), max_y.clone())),
            right_point: Point::from((max_x.clone(), max_y)),
            interior_to_left: true,
        });
        Node::<Point>::new_leaf(
            Point::from((min_x, min_y.clone())),
            Point::from((max_x, min_y)),
            below_edge_index,
            above_edge_index,
            nodes,
        )
    }
}

impl<Point: Orient + PartialOrd + Clone> Trapezoidation<Point> {
    fn add_edge(edge_index: usize, edges: &[Edge<Point>], nodes: &mut Vec<Node<Point>>) {
        let trapezoids_leaves_indices =
            Self::find_intersecting_trapezoids_leaves_indices(edge_index, edges, nodes);
        debug_assert!(!trapezoids_leaves_indices.is_empty());
        if let [trapezoid_leaf_index] = trapezoids_leaves_indices.as_slice() {
            Self::add_edge_to_single_trapezoid(edge_index, *trapezoid_leaf_index, edges, nodes);
        } else {
            let (
                first_trapezoid_leaf_index,
                middle_trapezoids_leaves_indices,
                last_trapezoid_leaf_index,
            ) = if let [first_trapezoid_leaf_index, middle_trapezoids_leaves_indices @ .., last_trapezoid_leaf_index] =
                trapezoids_leaves_indices.as_slice()
            {
                (
                    *first_trapezoid_leaf_index,
                    middle_trapezoids_leaves_indices,
                    *last_trapezoid_leaf_index,
                )
            } else {
                unreachable!("Edge intersects either single or multiple trapezoids.")
            };
            let (mut prev_above_leaf_index, mut prev_below_leaf_index) =
                Self::add_edge_to_first_trapezoid(
                    edge_index,
                    first_trapezoid_leaf_index,
                    edges,
                    nodes,
                );
            for &middle_trapezoid_leaf_index in middle_trapezoids_leaves_indices {
                (prev_above_leaf_index, prev_below_leaf_index) = Self::add_edge_to_middle_trapezoid(
                    edge_index,
                    middle_trapezoid_leaf_index,
                    prev_above_leaf_index,
                    prev_below_leaf_index,
                    nodes,
                );
            }
            Self::add_edge_to_last_trapezoid(
                edge_index,
                last_trapezoid_leaf_index,
                prev_above_leaf_index,
                prev_below_leaf_index,
                edges,
                nodes,
            );
        }
    }

    fn add_edge_to_first_trapezoid(
        edge_index: usize,
        trapezoid_leaf_index: usize,
        edges: &[Edge<Point>],
        nodes: &mut Vec<Node<Point>>,
    ) -> (usize, usize) {
        let edge = &edges[edge_index];
        let (above_leaf_index, below_leaf_index) = (
            Node::<Point>::new_leaf(
                edge.left_point.clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .right_point
                    .clone(),
                edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).above_edge_index,
                nodes,
            ),
            Node::<Point>::new_leaf(
                edge.left_point.clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .right_point
                    .clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes).below_edge_index,
                edge_index,
                nodes,
            ),
        );
        let mut replacement_node_index =
            Node::<Point>::new_y_node(edge_index, below_leaf_index, above_leaf_index, nodes);
        if edge.left_point == Self::get_trapezoid(trapezoid_leaf_index, nodes).left_point {
            Self::maybe_set_as_upper_left(
                above_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_upper_left_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_lower_left(
                below_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_lower_left_leaf_index(),
                nodes,
            );
        } else {
            let left_leaf_index = Node::<Point>::new_leaf(
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .left_point
                    .clone(),
                edge.left_point.clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes).below_edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).above_edge_index,
                nodes,
            );
            Self::maybe_set_as_lower_left(
                left_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_lower_left_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_upper_left(
                left_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_upper_left_leaf_index(),
                nodes,
            );
            Self::set_as_lower_right(left_leaf_index, below_leaf_index, nodes);
            Self::set_as_upper_right(left_leaf_index, above_leaf_index, nodes);
            replacement_node_index = Node::<Point>::new_x_node(
                edge.left_point.clone(),
                left_leaf_index,
                replacement_node_index,
                nodes,
            );
        }
        Self::maybe_set_as_upper_right(
            above_leaf_index,
            Self::get_trapezoid(trapezoid_leaf_index, nodes).get_upper_right_leaf_index(),
            nodes,
        );
        Self::maybe_set_as_lower_right(
            below_leaf_index,
            Self::get_trapezoid(trapezoid_leaf_index, nodes).get_lower_right_leaf_index(),
            nodes,
        );
        Self::replace_node(trapezoid_leaf_index, replacement_node_index, nodes);
        (above_leaf_index, below_leaf_index)
    }

    fn add_edge_to_middle_trapezoid(
        edge_index: usize,
        trapezoid_leaf_index: usize,
        prev_above_leaf_index: usize,
        prev_below_leaf_index: usize,
        nodes: &mut Vec<Node<Point>>,
    ) -> (usize, usize) {
        let above_leaf_index = if Self::get_trapezoid(prev_above_leaf_index, nodes).above_edge_index
            == Self::get_trapezoid(trapezoid_leaf_index, nodes).above_edge_index
        {
            Self::get_trapezoid_mut(prev_above_leaf_index, nodes).right_point =
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .right_point
                    .clone();
            prev_above_leaf_index
        } else {
            let above_leaf_index = Node::<Point>::new_leaf(
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .left_point
                    .clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .right_point
                    .clone(),
                edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).above_edge_index,
                nodes,
            );
            Self::maybe_set_as_upper_left(
                above_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_upper_left_leaf_index(),
                nodes,
            );
            Self::set_as_lower_left(above_leaf_index, prev_above_leaf_index, nodes);
            above_leaf_index
        };
        let below_leaf_index = if Self::get_trapezoid(prev_below_leaf_index, nodes).below_edge_index
            == Self::get_trapezoid(trapezoid_leaf_index, nodes).below_edge_index
        {
            Self::get_trapezoid_mut(prev_below_leaf_index, nodes).right_point =
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .right_point
                    .clone();
            prev_below_leaf_index
        } else {
            let below_leaf_index = Node::<Point>::new_leaf(
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .left_point
                    .clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .right_point
                    .clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes).below_edge_index,
                edge_index,
                nodes,
            );
            Self::set_as_upper_left(below_leaf_index, prev_below_leaf_index, nodes);
            Self::maybe_set_as_lower_left(
                below_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_lower_left_leaf_index(),
                nodes,
            );
            below_leaf_index
        };
        Self::maybe_set_as_upper_right(
            above_leaf_index,
            Self::get_trapezoid(trapezoid_leaf_index, nodes).get_upper_right_leaf_index(),
            nodes,
        );
        Self::maybe_set_as_lower_right(
            below_leaf_index,
            Self::get_trapezoid(trapezoid_leaf_index, nodes).get_lower_right_leaf_index(),
            nodes,
        );
        {
            let replacement_node_index =
                Node::<Point>::new_y_node(edge_index, below_leaf_index, above_leaf_index, nodes);
            Self::replace_node(trapezoid_leaf_index, replacement_node_index, nodes)
        };
        (above_leaf_index, below_leaf_index)
    }

    fn add_edge_to_last_trapezoid(
        edge_index: usize,
        trapezoid_leaf_index: usize,
        prev_above_leaf_index: usize,
        prev_below_leaf_index: usize,
        edges: &[Edge<Point>],
        nodes: &mut Vec<Node<Point>>,
    ) {
        let edge = &edges[edge_index];
        let above_leaf_index = if Self::get_trapezoid(prev_above_leaf_index, nodes).above_edge_index
            == Self::get_trapezoid(trapezoid_leaf_index, nodes).above_edge_index
        {
            Self::get_trapezoid_mut(prev_above_leaf_index, nodes).right_point =
                edge.right_point.clone();
            prev_above_leaf_index
        } else {
            let above_leaf_index = Node::<Point>::new_leaf(
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .left_point
                    .clone(),
                edge.right_point.clone(),
                edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).above_edge_index,
                nodes,
            );
            Self::maybe_set_as_upper_left(
                above_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_upper_left_leaf_index(),
                nodes,
            );
            Self::set_as_lower_left(above_leaf_index, prev_above_leaf_index, nodes);
            above_leaf_index
        };
        let below_leaf_index = if Self::get_trapezoid(prev_below_leaf_index, nodes).below_edge_index
            == Self::get_trapezoid(trapezoid_leaf_index, nodes).below_edge_index
        {
            Self::get_trapezoid_mut(prev_below_leaf_index, nodes).right_point =
                edge.right_point.clone();
            prev_below_leaf_index
        } else {
            let below_leaf_index = Node::<Point>::new_leaf(
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .left_point
                    .clone(),
                edge.right_point.clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes).below_edge_index,
                edge_index,
                nodes,
            );
            Self::maybe_set_as_lower_left(
                below_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_lower_left_leaf_index(),
                nodes,
            );
            Self::set_as_upper_left(below_leaf_index, prev_below_leaf_index, nodes);
            below_leaf_index
        };
        let mut replacement_node_index =
            Node::<Point>::new_y_node(edge_index, below_leaf_index, above_leaf_index, nodes);
        if edge.right_point == Self::get_trapezoid(trapezoid_leaf_index, nodes).right_point {
            Self::maybe_set_as_upper_right(
                above_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_upper_right_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_lower_right(
                below_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_lower_right_leaf_index(),
                nodes,
            );
        } else {
            let right_leaf_index = Node::<Point>::new_leaf(
                edge.right_point.clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .right_point
                    .clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes).below_edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).above_edge_index,
                nodes,
            );
            Self::maybe_set_as_lower_right(
                right_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_lower_right_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_upper_right(
                right_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_upper_right_leaf_index(),
                nodes,
            );
            Self::set_as_lower_left(right_leaf_index, below_leaf_index, nodes);
            Self::set_as_upper_left(right_leaf_index, above_leaf_index, nodes);
            replacement_node_index = Node::<Point>::new_x_node(
                edge.right_point.clone(),
                replacement_node_index,
                right_leaf_index,
                nodes,
            );
        }
        Self::replace_node(trapezoid_leaf_index, replacement_node_index, nodes);
    }

    fn add_edge_to_single_trapezoid(
        edge_index: usize,
        trapezoid_leaf_index: usize,
        edges: &[Edge<Point>],
        nodes: &mut Vec<Node<Point>>,
    ) where
        Point: PartialOrd + Clone,
    {
        let edge = &edges[edge_index];
        let (above_leaf_index, below_leaf_index) = (
            Node::<Point>::new_leaf(
                edge.left_point.clone(),
                edge.right_point.clone(),
                edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).above_edge_index,
                nodes,
            ),
            Node::<Point>::new_leaf(
                edge.left_point.clone(),
                edge.right_point.clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes).below_edge_index,
                edge_index,
                nodes,
            ),
        );
        let mut replacement_node_index =
            Node::<Point>::new_y_node(edge_index, below_leaf_index, above_leaf_index, nodes);
        if edge.right_point == Self::get_trapezoid(trapezoid_leaf_index, nodes).right_point {
            Self::maybe_set_as_upper_right(
                above_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_upper_right_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_lower_right(
                below_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_lower_right_leaf_index(),
                nodes,
            );
        } else {
            let right_leaf_index = Node::<Point>::new_leaf(
                edge.right_point.clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .right_point
                    .clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes).below_edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).above_edge_index,
                nodes,
            );
            Self::maybe_set_as_lower_right(
                right_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_lower_right_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_upper_right(
                right_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_upper_right_leaf_index(),
                nodes,
            );
            Self::set_as_lower_left(right_leaf_index, below_leaf_index, nodes);
            Self::set_as_upper_left(right_leaf_index, above_leaf_index, nodes);
            replacement_node_index = Node::<Point>::new_x_node(
                edge.right_point.clone(),
                replacement_node_index,
                right_leaf_index,
                nodes,
            );
        }
        if edge.left_point == Self::get_trapezoid(trapezoid_leaf_index, nodes).left_point {
            Self::maybe_set_as_upper_left(
                above_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_upper_left_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_lower_left(
                below_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_lower_left_leaf_index(),
                nodes,
            );
        } else {
            let left_leaf_index = Node::<Point>::new_leaf(
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .left_point
                    .clone(),
                edge.left_point.clone(),
                Self::get_trapezoid(trapezoid_leaf_index, nodes).below_edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).above_edge_index,
                nodes,
            );
            Self::maybe_set_as_lower_left(
                left_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_lower_left_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_upper_left(
                left_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes).get_upper_left_leaf_index(),
                nodes,
            );
            Self::set_as_lower_right(left_leaf_index, below_leaf_index, nodes);
            Self::set_as_upper_right(left_leaf_index, above_leaf_index, nodes);
            replacement_node_index = Node::<Point>::new_x_node(
                edge.left_point.clone(),
                left_leaf_index,
                replacement_node_index,
                nodes,
            );
        }
        Self::replace_node(trapezoid_leaf_index, replacement_node_index, nodes);
    }

    #[inline]
    fn maybe_set_as_lower_left(
        leaf_index: usize,
        lower_left_leaf_index: Option<usize>,
        nodes: &mut Vec<Node<Point>>,
    ) {
        match lower_left_leaf_index {
            Some(lower_left_leaf_index) => {
                Self::set_as_lower_left(leaf_index, lower_left_leaf_index, nodes)
            }
            None => Self::get_trapezoid_mut(leaf_index, nodes).reset_lower_left(),
        }
    }

    #[inline]
    fn maybe_set_as_lower_right(
        leaf_index: usize,
        lower_right_leaf_index: Option<usize>,
        nodes: &mut Vec<Node<Point>>,
    ) {
        match lower_right_leaf_index {
            Some(lower_right_leaf_index) => {
                Self::set_as_lower_right(leaf_index, lower_right_leaf_index, nodes)
            }
            None => Self::get_trapezoid_mut(leaf_index, nodes).reset_lower_right(),
        }
    }

    #[inline]
    fn maybe_set_as_upper_left(
        leaf_index: usize,
        upper_left_leaf_index: Option<usize>,
        nodes: &mut Vec<Node<Point>>,
    ) {
        match upper_left_leaf_index {
            Some(upper_left_leaf_index) => {
                Self::set_as_upper_left(leaf_index, upper_left_leaf_index, nodes)
            }
            None => Self::get_trapezoid_mut(leaf_index, nodes).reset_upper_left(),
        }
    }

    #[inline]
    fn maybe_set_as_upper_right(
        leaf_index: usize,
        upper_right_leaf_index: Option<usize>,
        nodes: &mut Vec<Node<Point>>,
    ) {
        match upper_right_leaf_index {
            Some(upper_right_leaf_index) => {
                Self::set_as_upper_right(leaf_index, upper_right_leaf_index, nodes)
            }
            None => Self::get_trapezoid_mut(leaf_index, nodes).reset_upper_right(),
        }
    }

    #[inline]
    fn set_as_lower_left(
        leaf_index: usize,
        lower_left_leaf_index: usize,
        nodes: &mut Vec<Node<Point>>,
    ) {
        unsafe { &mut (*(Self::get_trapezoid_mut(leaf_index, nodes) as *mut Trapezoid<Point>)) }
            .set_as_lower_left(Self::get_trapezoid_mut(lower_left_leaf_index, nodes));
    }

    #[inline]
    fn set_as_lower_right(
        leaf_index: usize,
        lower_right_index: usize,
        nodes: &mut Vec<Node<Point>>,
    ) {
        unsafe { &mut (*(Self::get_trapezoid_mut(leaf_index, nodes) as *mut Trapezoid<Point>)) }
            .set_as_lower_right(Self::get_trapezoid_mut(lower_right_index, nodes));
    }

    #[inline]
    fn set_as_upper_left(
        leaf_index: usize,
        upper_left_leaf_index: usize,
        nodes: &mut Vec<Node<Point>>,
    ) {
        unsafe { &mut (*(Self::get_trapezoid_mut(leaf_index, nodes) as *mut Trapezoid<Point>)) }
            .set_as_upper_left(Self::get_trapezoid_mut(upper_left_leaf_index, nodes));
    }

    #[inline]
    fn set_as_upper_right(
        leaf_index: usize,
        upper_right_index: usize,
        nodes: &mut Vec<Node<Point>>,
    ) {
        unsafe { &mut (*(Self::get_trapezoid_mut(leaf_index, nodes) as *mut Trapezoid<Point>)) }
            .set_as_upper_right(Self::get_trapezoid_mut(upper_right_index, nodes));
    }

    #[inline]
    fn get_trapezoid(leaf_index: usize, nodes: &[Node<Point>]) -> &Trapezoid<Point> {
        nodes[leaf_index].get_trapezoid()
    }

    #[inline]
    fn get_trapezoid_mut(leaf_index: usize, nodes: &mut [Node<Point>]) -> &mut Trapezoid<Point> {
        nodes[leaf_index].get_trapezoid_mut()
    }

    fn find_intersecting_trapezoids_leaves_indices(
        edge_index: usize,
        edges: &[Edge<Point>],
        nodes: &[Node<Point>],
    ) -> Vec<usize> {
        let edge = &edges[edge_index];
        let mut trapezoid = nodes[0].search_intersecting_trapezoid(edge, edges, nodes);
        let mut result = vec![trapezoid.leaf_index()];
        while trapezoid.right_point < edge.right_point {
            let leaf_index = if edge.orientation_of(&trapezoid.right_point)
                == Orientation::Clockwise
            {
                match trapezoid.get_upper_right_leaf_index() {
                    Some(value) => value,
                    None => unsafe { trapezoid.get_lower_right_leaf_index().unwrap_unchecked() },
                }
            } else {
                match trapezoid.get_lower_right_leaf_index() {
                    Some(value) => value,
                    None => unsafe { trapezoid.get_upper_right_leaf_index().unwrap_unchecked() },
                }
            };
            result.push(leaf_index);
            trapezoid = Self::get_trapezoid(leaf_index, nodes);
        }
        result
    }

    #[inline]
    fn replace_node(original_index: usize, replacement_index: usize, nodes: &mut Vec<Node<Point>>) {
        debug_assert!(nodes.len() > 1);
        debug_assert_eq!(replacement_index, nodes.len() - 1);
        nodes[original_index] = unsafe { nodes.pop().unwrap_unchecked() };
    }
}
