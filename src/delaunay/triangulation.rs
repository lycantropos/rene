use rithm::traits::{AdditiveGroup, DivRem, MultiplicativeMonoid, Signed};

use crate::locatable::Location;
use crate::operations::{ceil_log2, locate_point_in_point_point_point_circle, orient};
use crate::oriented::Orientation;
use crate::traits::Point;

use super::contracts::UNDEFINED_INDEX;
use super::quad_edge::{to_opposite_edge, QuadEdge};
use super::quad_edge_registry::QuadEdgeRegistry;

pub(crate) struct Triangulation<Scalar, Endpoint> {
    edges_registry: QuadEdgeRegistry<Scalar, Endpoint>,
    left_side: QuadEdge,
    right_side: QuadEdge,
}

impl<
        Scalar: AdditiveGroup + Clone + MultiplicativeMonoid + Signed,
        Endpoint: Clone + Ord + self::Point<Scalar>,
    > From<&[Endpoint]> for Triangulation<Scalar, Endpoint>
{
    fn from(points: &[Endpoint]) -> Self {
        let mut endpoints = points.iter().cloned().collect::<Vec<Endpoint>>();
        endpoints.sort();
        endpoints.dedup();
        let endpoints_count = endpoints.len();
        let mut edges_registry = QuadEdgeRegistry::from(endpoints);
        if endpoints_count < 2 {
            Self {
                edges_registry,
                left_side: UNDEFINED_INDEX,
                right_side: UNDEFINED_INDEX,
            }
        } else {
            let (segments_count, triangles_count) = to_base_cases(endpoints_count);
            let mut sub_triangulations_sides =
                Vec::<(QuadEdge, QuadEdge)>::with_capacity(segments_count + triangles_count);
            for index in 0..segments_count {
                let edge = edges_registry.create_edge(2 * index, 2 * index + 1);
                let opposite_edge = to_opposite_edge(edge);
                sub_triangulations_sides.push((edge, opposite_edge));
            }
            let offset = 2 * segments_count;
            for index in 0..triangles_count {
                sub_triangulations_sides.push(edges_registry.create_triangle(
                    offset + 3 * index,
                    offset + 3 * index + 1,
                    offset + 3 * index + 2,
                ));
            }
            for _ in 0..ceil_log2(sub_triangulations_sides.len()) {
                let merge_steps_count = sub_triangulations_sides.len() / 2;
                let mut next_sub_triangulations_sides = Vec::with_capacity(merge_steps_count);
                for step in 0..merge_steps_count {
                    next_sub_triangulations_sides.push(edges_registry.merge(
                        sub_triangulations_sides[2 * step],
                        sub_triangulations_sides[2 * step + 1],
                    ));
                }
                next_sub_triangulations_sides
                    .extend(&sub_triangulations_sides[2 * merge_steps_count..]);
                sub_triangulations_sides.clear();
                sub_triangulations_sides.append(&mut next_sub_triangulations_sides);
            }
            debug_assert_eq!(sub_triangulations_sides.len(), 1);
            let (left_side, right_side) = sub_triangulations_sides[0];
            Self {
                edges_registry,
                left_side,
                right_side,
            }
        }
    }
}

impl<Scalar, Endpoint> Triangulation<Scalar, Endpoint> {
    pub(crate) fn to_boundary_edges(&self) -> Vec<QuadEdge> {
        let start = self.left_side;
        let mut edge = start;
        let mut result = Vec::new();
        loop {
            result.push(edge);
            let candidate = self.edges_registry.to_right_from_end(edge);
            if candidate == start {
                break;
            }
            edge = candidate;
        }
        result
    }

    pub(crate) fn get_start(&self, edge: QuadEdge) -> &Endpoint {
        self.edges_registry.get_start(edge)
    }

    pub(crate) fn get_end(&self, edge: QuadEdge) -> &Endpoint {
        self.edges_registry.get_end(edge)
    }
}

fn to_base_cases(points_count: usize) -> (usize, usize) {
    let (triangles_count, rest_points) = points_count.div_rem(3);
    if rest_points == 0 {
        (0, triangles_count)
    } else if rest_points == 1 {
        (2, triangles_count - 1)
    } else {
        (1, triangles_count)
    }
}

impl<
        Scalar: AdditiveGroup + Clone + MultiplicativeMonoid + Signed,
        Endpoint: self::Point<Scalar> + PartialEq,
    > QuadEdgeRegistry<Scalar, Endpoint>
{
    fn build_base_edge(
        &mut self,
        mut first_right_side: QuadEdge,
        mut second_left_side: QuadEdge,
    ) -> (QuadEdge, QuadEdge, QuadEdge) {
        loop {
            if matches!(
                self.orient_point_to_edge(first_right_side, self.get_start(second_left_side)),
                Orientation::Counterclockwise
            ) {
                first_right_side = self.to_left_from_end(first_right_side);
            } else if matches!(
                self.orient_point_to_edge(second_left_side, self.get_start(first_right_side)),
                Orientation::Clockwise
            ) {
                second_left_side = self.to_right_from_end(second_left_side);
            } else {
                break;
            }
        }
        (
            first_right_side,
            self.connect_edges(to_opposite_edge(second_left_side), first_right_side),
            second_left_side,
        )
    }

    fn merge(
        &mut self,
        (first_left_side, first_right_side): (QuadEdge, QuadEdge),
        (second_left_side, second_right_side): (QuadEdge, QuadEdge),
    ) -> (QuadEdge, QuadEdge) {
        let (first_right_side, base_edge, second_left_side) =
            self.build_base_edge(first_right_side, second_left_side);
        self.rise_bubble(base_edge);
        let left_side = if self.get_start(first_left_side) == self.get_start(first_right_side) {
            to_opposite_edge(base_edge)
        } else {
            first_left_side
        };
        let right_side = if self.get_start(second_left_side) == self.get_start(second_right_side) {
            base_edge
        } else {
            second_right_side
        };
        (left_side, right_side)
    }

    fn rise_bubble(&mut self, mut base_edge: QuadEdge) {
        loop {
            let (maybe_left_candidate, maybe_right_candidate) = (
                self.to_left_candidate(base_edge),
                self.to_right_candidate(base_edge),
            );
            base_edge = match maybe_left_candidate {
                Some(left_candidate) => match maybe_right_candidate {
                    Some(right_candidate) => {
                        if matches!(
                            locate_point_in_point_point_point_circle(
                                self.get_end(right_candidate),
                                self.get_end(left_candidate),
                                self.get_end(base_edge),
                                self.get_start(base_edge),
                            ),
                            Location::Interior,
                        ) {
                            self.connect_edges(right_candidate, to_opposite_edge(base_edge))
                        } else {
                            self.connect_edges(
                                to_opposite_edge(base_edge),
                                to_opposite_edge(left_candidate),
                            )
                        }
                    }
                    None => self.connect_edges(
                        to_opposite_edge(base_edge),
                        to_opposite_edge(left_candidate),
                    ),
                },
                None => match maybe_right_candidate {
                    Some(right_candidate) => {
                        self.connect_edges(right_candidate, to_opposite_edge(base_edge))
                    }
                    None => break,
                },
            };
        }
    }

    fn to_left_candidate(&mut self, base_edge: QuadEdge) -> Option<QuadEdge> {
        let mut result = self.to_left_from_start(to_opposite_edge(base_edge));
        if !matches!(
            self.orient_point_to_edge(base_edge, self.get_end(result)),
            Orientation::Clockwise
        ) {
            None
        } else {
            while matches!(
                self.orient_point_to_edge(base_edge, self.get_end(self.to_left_from_start(result))),
                Orientation::Clockwise
            ) && matches!(
                locate_point_in_point_point_point_circle(
                    self.get_end(self.to_left_from_start(result)),
                    self.get_end(base_edge),
                    self.get_start(base_edge),
                    self.get_end(result)
                ),
                Location::Interior
            ) {
                let next_candidate = self.to_left_from_start(result);
                self.delete_edge(result);
                result = next_candidate;
            }
            Some(result)
        }
    }

    fn to_right_candidate(&mut self, base_edge: QuadEdge) -> Option<QuadEdge> {
        let mut result = self.to_right_from_start(base_edge);
        if !matches!(
            self.orient_point_to_edge(base_edge, self.get_end(result)),
            Orientation::Clockwise
        ) {
            None
        } else {
            while matches!(
                self.orient_point_to_edge(
                    base_edge,
                    self.get_end(self.to_right_from_start(result))
                ),
                Orientation::Clockwise
            ) && matches!(
                locate_point_in_point_point_point_circle(
                    self.get_end(self.to_right_from_start(result)),
                    self.get_end(base_edge),
                    self.get_start(base_edge),
                    self.get_end(result)
                ),
                Location::Interior
            ) {
                let next_candidate = self.to_right_from_start(result);
                self.delete_edge(result);
                result = next_candidate;
            }
            Some(result)
        }
    }
}

impl<Scalar: AdditiveGroup + MultiplicativeMonoid + Signed, Endpoint: self::Point<Scalar>>
    QuadEdgeRegistry<Scalar, Endpoint>
{
    pub(super) fn create_triangle(
        &mut self,
        left_point_index: usize,
        mid_point_index: usize,
        right_point_index: usize,
    ) -> (QuadEdge, QuadEdge) {
        let first_edge = self.create_edge(left_point_index, mid_point_index);
        let second_edge = self.create_edge(mid_point_index, right_point_index);
        self.splice_edges(to_opposite_edge(first_edge), second_edge);
        match self.orient_point_to_edge(first_edge, self.get_end(second_edge)) {
            Orientation::Clockwise => {
                let third_edge = self.connect_edges(second_edge, first_edge);
                (to_opposite_edge(third_edge), third_edge)
            }
            Orientation::Collinear => (first_edge, to_opposite_edge(second_edge)),
            Orientation::Counterclockwise => {
                self.connect_edges(second_edge, first_edge);
                (first_edge, to_opposite_edge(second_edge))
            }
        }
    }
}

impl<Scalar: AdditiveGroup + MultiplicativeMonoid + Signed, Endpoint: self::Point<Scalar>>
    QuadEdgeRegistry<Scalar, Endpoint>
{
    fn orient_point_to_edge(&self, edge: usize, point: &Endpoint) -> Orientation {
        orient(self.get_start(edge), self.get_end(edge), point)
    }
}
