use std::ops::{Add, Sub};

use traiter::numbers::{One, Zeroable};

use crate::bounded;
use crate::bounded::Bounded;
use crate::operations::Orient;
use crate::oriented::{Orientation, Oriented};
use crate::traits::{
    Contoural2, Elemental, Iterable, Lengthsome, Multisegmental,
    Multisegmental2IndexSegment, Multivertexal2, Multivertexal2IndexVertex,
    Polygonal2, Polygonal2IndexHole, Segmental,
};

use super::edge::Edge;
use super::node::Node;
use super::trapezoid::Trapezoid;

#[derive(Clone)]
pub(crate) struct Trapezoidation<Point> {
    edges: Vec<Edge>,
    endpoints: Vec<Point>,
    nodes: Vec<Node>,
}

impl<Point> Trapezoidation<Point> {
    pub(super) fn get_root(&self) -> &Node {
        &self.nodes[0]
    }

    pub(super) fn get_edges(&self) -> &[Edge] {
        &self.edges
    }

    pub(super) fn get_endpoints(&self) -> &[Point] {
        &self.endpoints
    }

    pub(super) fn get_nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub(crate) fn height(&self) -> usize {
        self.get_root().height(self.get_nodes())
    }
}

impl<Point> Trapezoidation<Point> {
    pub(crate) fn from_multisegment<
        'a,
        Multisegment,
        Scalar,
        Segment,
        Shuffler: FnOnce(&mut Vec<Edge>),
    >(
        multisegment: &'a Multisegment,
        shuffler: Shuffler,
    ) -> Self
    where
        Point: Clone + From<(Scalar, Scalar)> + PartialOrd,
        Scalar: Clone + One,
        for<'b> &'b Multisegment:
            Bounded<&'b Scalar> + Multisegmental<Segment = &'b Segment>,
        for<'b> &'b Point: Orient,
        for<'b> &'b Scalar: Add<Scalar, Output = Scalar>
            + Sub<Scalar, Output = Scalar>
            + Sub<Output = Scalar>
            + Zeroable,
        for<'b> &'b Segment: Segmental<Endpoint = &'b Point>,
    {
        let mut edges =
            Vec::<Edge>::with_capacity(multisegment.segments_count());
        let mut endpoints =
            Vec::<Point>::with_capacity(2 * multisegment.segments_count());
        for segment in multisegment.segments() {
            let (start, end) = segment.endpoints();
            let start_index = endpoints.len();
            let end_index = endpoints.len() + 1;
            edges.push(if start < end {
                Edge {
                    left_point_index: start_index,
                    right_point_index: end_index,
                    interior_to_left: false,
                }
            } else {
                Edge {
                    left_point_index: end_index,
                    right_point_index: start_index,
                    interior_to_left: false,
                }
            });
            endpoints.push(start.clone());
            endpoints.push(end.clone());
        }
        shuffler(&mut edges);
        Self::from_box(multisegment.to_bounding_box(), edges, endpoints)
    }

    pub(crate) fn from_polygon<
        Scalar,
        Contour,
        Polygon,
        Shuffler: FnOnce(&mut Vec<Edge>),
    >(
        polygon: &Polygon,
        shuffler: Shuffler,
    ) -> Self
    where
        Point: Clone + From<(Scalar, Scalar)> + PartialOrd,
        Scalar: Clone + One,
        for<'a> &'a Contour: Contoural2<IndexVertex = Point> + Oriented,
        for<'a> &'a Point: Elemental + Orient,
        for<'a> &'a Polygon: Bounded<&'a Scalar>
            + Polygonal2<Contour = &'a Contour, IntoIteratorHole = &'a Contour>,
        for<'a> &'a Scalar: Add<Scalar, Output = Scalar>
            + Sub<Scalar, Output = Scalar>
            + Sub<Output = Scalar>
            + Zeroable,
        for<'a, 'b> &'a Multisegmental2IndexSegment<&'b Contour>: Segmental,
        for<'a, 'b> &'a Polygonal2IndexHole<&'b Polygon>: Contoural2,
        for<'a, 'b, 'c> &'a Multisegmental2IndexSegment<&'b Polygonal2IndexHole<&'c Polygon>>:
            Segmental,
        for<'a, 'b, 'c> &'a Multivertexal2IndexVertex<&'b Polygonal2IndexHole<&'c Polygon>>:
            Elemental,
    {
        let (border, holes) = (polygon.border2(), polygon.holes2());
        let endpoints_count = border.vertices2().len()
            + holes
                .iter()
                .map(|hole| hole.vertices2().len())
                .sum::<usize>();
        let mut edges = Vec::<Edge>::with_capacity(endpoints_count);
        let mut endpoints = Vec::<Point>::with_capacity(endpoints_count);
        let is_border_correctly_oriented =
            border.to_orientation() == Orientation::Counterclockwise;
        Self::populate_from_points(
            border.vertices2().iter().cloned(),
            is_border_correctly_oriented,
            &mut edges,
            &mut endpoints,
        );
        for hole in holes {
            let is_hole_correctly_oriented =
                hole.to_orientation() == Orientation::Clockwise;
            Self::populate_from_points(
                hole.vertices2().iter().cloned(),
                is_hole_correctly_oriented,
                &mut edges,
                &mut endpoints,
            );
        }
        shuffler(&mut edges);
        Self::from_box(polygon.to_bounding_box(), edges, endpoints)
    }

    fn from_box<Scalar: Clone + One>(
        box_: bounded::Box<&Scalar>,
        mut edges: Vec<Edge>,
        mut endpoints: Vec<Point>,
    ) -> Self
    where
        Point: From<(Scalar, Scalar)> + PartialOrd,
        for<'a> &'a Point: Orient,
        for<'a> &'a Scalar: Add<Scalar, Output = Scalar>
            + Sub<Scalar, Output = Scalar>
            + Sub<Output = Scalar>
            + Zeroable,
    {
        debug_assert!(!edges.is_empty());
        let mut nodes = Vec::<Node>::new();
        let first_leaf_index = Self::leaf_from_box_with_edges(
            box_,
            &mut edges,
            &mut endpoints,
            &mut nodes,
        );
        debug_assert_eq!(first_leaf_index, 0usize);
        Self::add_edge_to_single_trapezoid(0usize, 0usize, &edges, &mut nodes);
        for edge_index in 1..edges.len() - 2 {
            Self::add_edge(edge_index, &edges, &endpoints, &mut nodes)
        }
        Self {
            edges,
            endpoints,
            nodes,
        }
    }

    fn leaf_from_box_with_edges<Scalar: Clone + One>(
        box_: bounded::Box<&Scalar>,
        edges: &mut Vec<Edge>,
        endpoints: &mut Vec<Point>,
        nodes: &mut Vec<Node>,
    ) -> usize
    where
        Point: From<(Scalar, Scalar)> + PartialOrd,
        for<'a> &'a Point: Orient,
        for<'a> &'a Scalar: Add<Scalar, Output = Scalar>
            + Sub<Scalar, Output = Scalar>
            + Sub<Output = Scalar>
            + Zeroable,
    {
        let (min_x, min_y, max_x, max_y) = (
            *box_.get_min_x(),
            *box_.get_min_y(),
            *box_.get_max_x(),
            *box_.get_max_y(),
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
        let above_edge_left_point_index = endpoints.len();
        endpoints.push(Point::from((min_x.clone(), max_y.clone())));
        let above_edge_right_point_index = endpoints.len();
        endpoints.push(Point::from((max_x.clone(), max_y)));
        let above_edge_index = edges.len();
        edges.push(Edge {
            left_point_index: above_edge_left_point_index,
            right_point_index: above_edge_right_point_index,
            interior_to_left: true,
        });
        let below_edge_left_point_index = endpoints.len();
        endpoints.push(Point::from((min_x, min_y.clone())));
        let below_edge_right_point_index = endpoints.len();
        endpoints.push(Point::from((max_x, min_y)));
        let below_edge_index = edges.len();
        edges.push(Edge {
            left_point_index: below_edge_left_point_index,
            right_point_index: below_edge_right_point_index,
            interior_to_left: false,
        });
        Node::new_leaf(
            below_edge_left_point_index,
            below_edge_right_point_index,
            below_edge_index,
            above_edge_index,
            edges,
            nodes,
        )
    }

    fn populate_from_points<'a, PointsIterator>(
        points: PointsIterator,
        is_contour_correctly_oriented: bool,
        edges: &mut Vec<Edge>,
        endpoints: &mut Vec<Point>,
    ) where
        Point: 'a + Clone + PartialOrd,
        PointsIterator: Iterator<Item = Point>,
    {
        let first_start_index = endpoints.len();
        let mut start_index = endpoints.len();
        endpoints.extend(points);
        let mut start = &endpoints[start_index];
        for (end_offset, end) in
            endpoints[first_start_index + 1..].iter().enumerate()
        {
            let end_index = first_start_index + 1 + end_offset;
            edges.push(if start < end {
                Edge {
                    left_point_index: start_index,
                    right_point_index: end_index,
                    interior_to_left: is_contour_correctly_oriented,
                }
            } else {
                Edge {
                    left_point_index: end_index,
                    right_point_index: start_index,
                    interior_to_left: !is_contour_correctly_oriented,
                }
            });
            (start, start_index) = (end, end_index);
        }
        let last_end_index = endpoints.len() - 1;
        edges.push(
            if endpoints[first_start_index] < endpoints[last_end_index] {
                Edge {
                    left_point_index: first_start_index,
                    right_point_index: last_end_index,
                    interior_to_left: is_contour_correctly_oriented,
                }
            } else {
                Edge {
                    left_point_index: last_end_index,
                    right_point_index: first_start_index,
                    interior_to_left: !is_contour_correctly_oriented,
                }
            },
        );
    }
}

impl<Point: PartialOrd> Trapezoidation<Point>
where
    for<'a> &'a Point: Orient,
{
    fn add_edge(
        edge_index: usize,
        edges: &[Edge],
        endpoints: &[Point],
        nodes: &mut Vec<Node>,
    ) {
        let trapezoids_leaves_indices =
            Self::find_intersecting_trapezoids_leaves_indices(
                edge_index, edges, endpoints, nodes,
            );
        debug_assert!(!trapezoids_leaves_indices.is_empty());
        if let [trapezoid_leaf_index] = trapezoids_leaves_indices.as_slice() {
            Self::add_edge_to_single_trapezoid(
                edge_index,
                *trapezoid_leaf_index,
                edges,
                nodes,
            );
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
                unreachable!(
                    "Edge intersects either single or multiple trapezoids."
                )
            };
            let (mut prev_above_leaf_index, mut prev_below_leaf_index) =
                Self::add_edge_to_first_trapezoid(
                    edge_index,
                    first_trapezoid_leaf_index,
                    edges,
                    nodes,
                );
            for &middle_trapezoid_leaf_index in
                middle_trapezoids_leaves_indices
            {
                (prev_above_leaf_index, prev_below_leaf_index) =
                    Self::add_edge_to_middle_trapezoid(
                        edge_index,
                        middle_trapezoid_leaf_index,
                        prev_above_leaf_index,
                        prev_below_leaf_index,
                        edges,
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
        edges: &[Edge],
        nodes: &mut Vec<Node>,
    ) -> (usize, usize) {
        let edge = &edges[edge_index];
        let (above_leaf_index, below_leaf_index) = (
            Node::new_leaf(
                edge.left_point_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .right_point_index,
                edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .above_edge_index,
                edges,
                nodes,
            ),
            Node::new_leaf(
                edge.left_point_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .right_point_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .below_edge_index,
                edge_index,
                edges,
                nodes,
            ),
        );
        let mut replacement_node_index = Node::new_y_node(
            edge_index,
            below_leaf_index,
            above_leaf_index,
            nodes,
        );
        if edge.left_point_index
            == Self::get_trapezoid(trapezoid_leaf_index, nodes)
                .left_point_index
        {
            Self::maybe_set_as_upper_left(
                above_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_upper_left_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_lower_left(
                below_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_lower_left_leaf_index(),
                nodes,
            );
        } else {
            let left_leaf_index = Node::new_leaf(
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .left_point_index,
                edge.left_point_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .below_edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .above_edge_index,
                edges,
                nodes,
            );
            Self::maybe_set_as_lower_left(
                left_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_lower_left_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_upper_left(
                left_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_upper_left_leaf_index(),
                nodes,
            );
            Self::set_as_lower_right(left_leaf_index, below_leaf_index, nodes);
            Self::set_as_upper_right(left_leaf_index, above_leaf_index, nodes);
            replacement_node_index = Node::new_x_node(
                edge.left_point_index,
                left_leaf_index,
                replacement_node_index,
                nodes,
            );
        }
        Self::maybe_set_as_upper_right(
            above_leaf_index,
            Self::get_trapezoid(trapezoid_leaf_index, nodes)
                .get_upper_right_leaf_index(),
            nodes,
        );
        Self::maybe_set_as_lower_right(
            below_leaf_index,
            Self::get_trapezoid(trapezoid_leaf_index, nodes)
                .get_lower_right_leaf_index(),
            nodes,
        );
        Self::replace_node(
            trapezoid_leaf_index,
            replacement_node_index,
            nodes,
        );
        (above_leaf_index, below_leaf_index)
    }

    fn add_edge_to_middle_trapezoid(
        edge_index: usize,
        trapezoid_leaf_index: usize,
        prev_above_leaf_index: usize,
        prev_below_leaf_index: usize,
        edges: &[Edge],
        nodes: &mut Vec<Node>,
    ) -> (usize, usize) {
        let above_leaf_index =
            if Self::get_trapezoid(prev_above_leaf_index, nodes)
                .above_edge_index
                == Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .above_edge_index
            {
                Self::get_trapezoid_mut(prev_above_leaf_index, nodes)
                    .right_point_index =
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .right_point_index;
                prev_above_leaf_index
            } else {
                let above_leaf_index = Node::new_leaf(
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .left_point_index,
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .right_point_index,
                    edge_index,
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .above_edge_index,
                    edges,
                    nodes,
                );
                Self::maybe_set_as_upper_left(
                    above_leaf_index,
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .get_upper_left_leaf_index(),
                    nodes,
                );
                Self::set_as_lower_left(
                    above_leaf_index,
                    prev_above_leaf_index,
                    nodes,
                );
                above_leaf_index
            };
        let below_leaf_index =
            if Self::get_trapezoid(prev_below_leaf_index, nodes)
                .below_edge_index
                == Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .below_edge_index
            {
                Self::get_trapezoid_mut(prev_below_leaf_index, nodes)
                    .right_point_index =
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .right_point_index;
                prev_below_leaf_index
            } else {
                let below_leaf_index = Node::new_leaf(
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .left_point_index,
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .right_point_index,
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .below_edge_index,
                    edge_index,
                    edges,
                    nodes,
                );
                Self::set_as_upper_left(
                    below_leaf_index,
                    prev_below_leaf_index,
                    nodes,
                );
                Self::maybe_set_as_lower_left(
                    below_leaf_index,
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .get_lower_left_leaf_index(),
                    nodes,
                );
                below_leaf_index
            };
        Self::maybe_set_as_upper_right(
            above_leaf_index,
            Self::get_trapezoid(trapezoid_leaf_index, nodes)
                .get_upper_right_leaf_index(),
            nodes,
        );
        Self::maybe_set_as_lower_right(
            below_leaf_index,
            Self::get_trapezoid(trapezoid_leaf_index, nodes)
                .get_lower_right_leaf_index(),
            nodes,
        );
        {
            let replacement_node_index = Node::new_y_node(
                edge_index,
                below_leaf_index,
                above_leaf_index,
                nodes,
            );
            Self::replace_node(
                trapezoid_leaf_index,
                replacement_node_index,
                nodes,
            )
        };
        (above_leaf_index, below_leaf_index)
    }

    fn add_edge_to_last_trapezoid(
        edge_index: usize,
        trapezoid_leaf_index: usize,
        prev_above_leaf_index: usize,
        prev_below_leaf_index: usize,
        edges: &[Edge],
        nodes: &mut Vec<Node>,
    ) {
        let edge = &edges[edge_index];
        let above_leaf_index =
            if Self::get_trapezoid(prev_above_leaf_index, nodes)
                .above_edge_index
                == Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .above_edge_index
            {
                Self::get_trapezoid_mut(prev_above_leaf_index, nodes)
                    .right_point_index = edge.right_point_index;
                prev_above_leaf_index
            } else {
                let above_leaf_index = Node::new_leaf(
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .left_point_index,
                    edge.right_point_index,
                    edge_index,
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .above_edge_index,
                    edges,
                    nodes,
                );
                Self::maybe_set_as_upper_left(
                    above_leaf_index,
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .get_upper_left_leaf_index(),
                    nodes,
                );
                Self::set_as_lower_left(
                    above_leaf_index,
                    prev_above_leaf_index,
                    nodes,
                );
                above_leaf_index
            };
        let below_leaf_index =
            if Self::get_trapezoid(prev_below_leaf_index, nodes)
                .below_edge_index
                == Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .below_edge_index
            {
                Self::get_trapezoid_mut(prev_below_leaf_index, nodes)
                    .right_point_index = edge.right_point_index;
                prev_below_leaf_index
            } else {
                let below_leaf_index = Node::new_leaf(
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .left_point_index,
                    edge.right_point_index,
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .below_edge_index,
                    edge_index,
                    edges,
                    nodes,
                );
                Self::maybe_set_as_lower_left(
                    below_leaf_index,
                    Self::get_trapezoid(trapezoid_leaf_index, nodes)
                        .get_lower_left_leaf_index(),
                    nodes,
                );
                Self::set_as_upper_left(
                    below_leaf_index,
                    prev_below_leaf_index,
                    nodes,
                );
                below_leaf_index
            };
        let mut replacement_node_index = Node::new_y_node(
            edge_index,
            below_leaf_index,
            above_leaf_index,
            nodes,
        );
        if edge.right_point_index
            == Self::get_trapezoid(trapezoid_leaf_index, nodes)
                .right_point_index
        {
            Self::maybe_set_as_upper_right(
                above_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_upper_right_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_lower_right(
                below_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_lower_right_leaf_index(),
                nodes,
            );
        } else {
            let right_leaf_index = Node::new_leaf(
                edge.right_point_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .right_point_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .below_edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .above_edge_index,
                edges,
                nodes,
            );
            Self::maybe_set_as_lower_right(
                right_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_lower_right_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_upper_right(
                right_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_upper_right_leaf_index(),
                nodes,
            );
            Self::set_as_lower_left(right_leaf_index, below_leaf_index, nodes);
            Self::set_as_upper_left(right_leaf_index, above_leaf_index, nodes);
            replacement_node_index = Node::new_x_node(
                edge.right_point_index,
                replacement_node_index,
                right_leaf_index,
                nodes,
            );
        }
        Self::replace_node(
            trapezoid_leaf_index,
            replacement_node_index,
            nodes,
        );
    }

    fn add_edge_to_single_trapezoid(
        edge_index: usize,
        trapezoid_leaf_index: usize,
        edges: &[Edge],
        nodes: &mut Vec<Node>,
    ) where
        Point: PartialOrd,
    {
        let edge = &edges[edge_index];
        let (above_leaf_index, below_leaf_index) = (
            Node::new_leaf(
                edge.left_point_index,
                edge.right_point_index,
                edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .above_edge_index,
                edges,
                nodes,
            ),
            Node::new_leaf(
                edge.left_point_index,
                edge.right_point_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .below_edge_index,
                edge_index,
                edges,
                nodes,
            ),
        );
        let mut replacement_node_index = Node::new_y_node(
            edge_index,
            below_leaf_index,
            above_leaf_index,
            nodes,
        );
        if edge.right_point_index
            == Self::get_trapezoid(trapezoid_leaf_index, nodes)
                .right_point_index
        {
            Self::maybe_set_as_upper_right(
                above_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_upper_right_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_lower_right(
                below_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_lower_right_leaf_index(),
                nodes,
            );
        } else {
            let right_leaf_index = Node::new_leaf(
                edge.right_point_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .right_point_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .below_edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .above_edge_index,
                edges,
                nodes,
            );
            Self::maybe_set_as_lower_right(
                right_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_lower_right_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_upper_right(
                right_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_upper_right_leaf_index(),
                nodes,
            );
            Self::set_as_lower_left(right_leaf_index, below_leaf_index, nodes);
            Self::set_as_upper_left(right_leaf_index, above_leaf_index, nodes);
            replacement_node_index = Node::new_x_node(
                edge.right_point_index,
                replacement_node_index,
                right_leaf_index,
                nodes,
            );
        }
        if edge.left_point_index
            == Self::get_trapezoid(trapezoid_leaf_index, nodes)
                .left_point_index
        {
            Self::maybe_set_as_upper_left(
                above_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_upper_left_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_lower_left(
                below_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_lower_left_leaf_index(),
                nodes,
            );
        } else {
            let left_leaf_index = Node::new_leaf(
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .left_point_index,
                edge.left_point_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .below_edge_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .above_edge_index,
                edges,
                nodes,
            );
            Self::maybe_set_as_lower_left(
                left_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_lower_left_leaf_index(),
                nodes,
            );
            Self::maybe_set_as_upper_left(
                left_leaf_index,
                Self::get_trapezoid(trapezoid_leaf_index, nodes)
                    .get_upper_left_leaf_index(),
                nodes,
            );
            Self::set_as_lower_right(left_leaf_index, below_leaf_index, nodes);
            Self::set_as_upper_right(left_leaf_index, above_leaf_index, nodes);
            replacement_node_index = Node::new_x_node(
                edge.left_point_index,
                left_leaf_index,
                replacement_node_index,
                nodes,
            );
        }
        Self::replace_node(
            trapezoid_leaf_index,
            replacement_node_index,
            nodes,
        );
    }

    #[inline]
    fn maybe_set_as_lower_left(
        leaf_index: usize,
        lower_left_leaf_index: Option<usize>,
        nodes: &mut [Node],
    ) {
        match lower_left_leaf_index {
            Some(lower_left_leaf_index) => Self::set_as_lower_left(
                leaf_index,
                lower_left_leaf_index,
                nodes,
            ),
            None => {
                Self::get_trapezoid_mut(leaf_index, nodes).reset_lower_left()
            }
        }
    }

    #[inline]
    fn maybe_set_as_lower_right(
        leaf_index: usize,
        lower_right_leaf_index: Option<usize>,
        nodes: &mut [Node],
    ) {
        match lower_right_leaf_index {
            Some(lower_right_leaf_index) => Self::set_as_lower_right(
                leaf_index,
                lower_right_leaf_index,
                nodes,
            ),
            None => {
                Self::get_trapezoid_mut(leaf_index, nodes).reset_lower_right()
            }
        }
    }

    #[inline]
    fn maybe_set_as_upper_left(
        leaf_index: usize,
        upper_left_leaf_index: Option<usize>,
        nodes: &mut [Node],
    ) {
        match upper_left_leaf_index {
            Some(upper_left_leaf_index) => Self::set_as_upper_left(
                leaf_index,
                upper_left_leaf_index,
                nodes,
            ),
            None => {
                Self::get_trapezoid_mut(leaf_index, nodes).reset_upper_left()
            }
        }
    }

    #[inline]
    fn maybe_set_as_upper_right(
        leaf_index: usize,
        upper_right_leaf_index: Option<usize>,
        nodes: &mut [Node],
    ) {
        match upper_right_leaf_index {
            Some(upper_right_leaf_index) => Self::set_as_upper_right(
                leaf_index,
                upper_right_leaf_index,
                nodes,
            ),
            None => {
                Self::get_trapezoid_mut(leaf_index, nodes).reset_upper_right()
            }
        }
    }

    #[inline]
    fn set_as_lower_left(
        leaf_index: usize,
        lower_left_leaf_index: usize,
        nodes: &mut [Node],
    ) {
        unsafe {
            &mut (*(Self::get_trapezoid_mut(leaf_index, nodes)
                as *mut Trapezoid))
        }
        .set_as_lower_left(Self::get_trapezoid_mut(
            lower_left_leaf_index,
            nodes,
        ));
    }

    #[inline]
    fn set_as_lower_right(
        leaf_index: usize,
        lower_right_index: usize,
        nodes: &mut [Node],
    ) {
        unsafe {
            &mut (*(Self::get_trapezoid_mut(leaf_index, nodes)
                as *mut Trapezoid))
        }
        .set_as_lower_right(Self::get_trapezoid_mut(lower_right_index, nodes));
    }

    #[inline]
    fn set_as_upper_left(
        leaf_index: usize,
        upper_left_leaf_index: usize,
        nodes: &mut [Node],
    ) {
        unsafe {
            &mut (*(Self::get_trapezoid_mut(leaf_index, nodes)
                as *mut Trapezoid))
        }
        .set_as_upper_left(Self::get_trapezoid_mut(
            upper_left_leaf_index,
            nodes,
        ));
    }

    #[inline]
    fn set_as_upper_right(
        leaf_index: usize,
        upper_right_index: usize,
        nodes: &mut [Node],
    ) {
        unsafe {
            &mut (*(Self::get_trapezoid_mut(leaf_index, nodes)
                as *mut Trapezoid))
        }
        .set_as_upper_right(Self::get_trapezoid_mut(upper_right_index, nodes));
    }

    #[inline]
    fn get_trapezoid(leaf_index: usize, nodes: &[Node]) -> &Trapezoid {
        nodes[leaf_index].get_trapezoid()
    }

    #[inline]
    fn get_trapezoid_mut(
        leaf_index: usize,
        nodes: &mut [Node],
    ) -> &mut Trapezoid {
        nodes[leaf_index].get_trapezoid_mut()
    }

    fn find_intersecting_trapezoids_leaves_indices(
        edge_index: usize,
        edges: &[Edge],
        endpoints: &[Point],
        nodes: &[Node],
    ) -> Vec<usize> {
        let edge = &edges[edge_index];
        let mut trapezoid = nodes[0]
            .search_intersecting_trapezoid(edge, edges, endpoints, nodes);
        let mut result = vec![trapezoid.get_leaf_index()];
        while endpoints[trapezoid.right_point_index]
            .lt(&endpoints[edge.right_point_index])
        {
            let leaf_index = if edge.orientation_of(
                &endpoints[trapezoid.right_point_index],
                endpoints,
            ) == Orientation::Clockwise
            {
                match trapezoid.get_upper_right_leaf_index() {
                    Some(value) => value,
                    None => unsafe {
                        trapezoid
                            .get_lower_right_leaf_index()
                            .unwrap_unchecked()
                    },
                }
            } else {
                match trapezoid.get_lower_right_leaf_index() {
                    Some(value) => value,
                    None => unsafe {
                        trapezoid
                            .get_upper_right_leaf_index()
                            .unwrap_unchecked()
                    },
                }
            };
            result.push(leaf_index);
            trapezoid = Self::get_trapezoid(leaf_index, nodes);
        }
        result
    }

    #[inline]
    fn replace_node(
        original_index: usize,
        replacement_index: usize,
        nodes: &mut Vec<Node>,
    ) {
        debug_assert!(nodes.len() > 1);
        debug_assert_eq!(replacement_index, nodes.len() - 1);
        nodes[original_index] = unsafe { nodes.pop().unwrap_unchecked() };
    }
}
