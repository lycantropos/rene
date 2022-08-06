use std::iter::Map;
use std::ops::Range;

use traiter::numbers::{DivRem, Parity};

use crate::locatable::Location;
use crate::operations::{ceil_log2, LocatePointInPointPointPointCircle, Orient};
use crate::oriented::Orientation;

use super::operations::DelaunayTriangulatable;
use super::quad_edge::{to_opposite_edge, to_rotated_edge, QuadEdge, UNDEFINED_QUAD_EDGE};

#[derive(Clone)]
pub(super) struct Mesh<Endpoint> {
    endpoints: Vec<Endpoint>,
    left_from_start: Vec<QuadEdge>,
    starts_indices: Vec<usize>,
}

impl<Endpoint> From<Vec<Endpoint>> for Mesh<Endpoint> {
    fn from(endpoints: Vec<Endpoint>) -> Self {
        let endpoints_count = endpoints.len();
        Self {
            endpoints,
            left_from_start: Vec::with_capacity(4 * endpoints_count),
            starts_indices: Vec::with_capacity(2 * endpoints_count),
        }
    }
}

impl<Endpoint> Mesh<Endpoint> {
    pub(super) fn get_endpoints(&self) -> &[Endpoint] {
        &self.endpoints
    }

    pub(super) fn get_start(&self, edge: QuadEdge) -> &Endpoint {
        &self.endpoints[self.to_start_index(edge)]
    }

    pub(super) fn get_end(&self, edge: QuadEdge) -> &Endpoint {
        &self.endpoints[self.to_start_index(to_opposite_edge(edge))]
    }

    pub(super) fn is_empty(&self) -> bool {
        self.left_from_start.is_empty()
    }

    pub(super) fn to_edges(&self) -> Map<Range<usize>, fn(usize) -> QuadEdge> {
        (0..self.left_from_start.len() / 2).map(|index| index * 2)
    }

    pub(super) fn to_left_from_start(&self, edge: QuadEdge) -> QuadEdge {
        self.left_from_start[edge]
    }

    pub(super) fn to_left_from_end(&self, edge: QuadEdge) -> QuadEdge {
        to_rotated_edge(self.to_left_from_start(to_opposite_edge(to_rotated_edge(edge))))
    }

    pub(super) fn to_right_from_end(&self, edge: QuadEdge) -> QuadEdge {
        self.to_left_from_start(to_opposite_edge(edge))
    }

    pub(super) fn to_right_from_start(&self, edge: QuadEdge) -> QuadEdge {
        to_rotated_edge(self.to_left_from_start(to_rotated_edge(edge)))
    }

    fn to_end_index(&self, edge: QuadEdge) -> usize {
        self.to_start_index(to_opposite_edge(edge))
    }

    fn to_start_index(&self, edge: QuadEdge) -> usize {
        debug_assert!(edge.is_even());
        self.starts_indices[edge / 2]
    }
}

impl<Endpoint> Mesh<Endpoint> {
    pub(super) fn connect_edges(&mut self, first: QuadEdge, second: QuadEdge) -> QuadEdge {
        let result = self.create_edge(self.to_end_index(first), self.to_start_index(second));
        self.splice_edges(result, self.to_left_from_end(first));
        self.splice_edges(to_opposite_edge(result), second);
        result
    }

    pub(super) fn create_edge(&mut self, start_index: usize, end_index: usize) -> QuadEdge {
        self.starts_indices.push(start_index);
        self.starts_indices.push(end_index);
        let edge = self.left_from_start.len();
        let rotated_edge = edge + 1;
        let opposite_edge = edge + 2;
        let triple_rotated_edge = edge + 3;
        self.left_from_start.push(edge);
        self.left_from_start.push(triple_rotated_edge);
        self.left_from_start.push(opposite_edge);
        self.left_from_start.push(rotated_edge);
        edge
    }

    pub(super) fn delete_edge(&mut self, edge: QuadEdge) {
        self.splice_edges(edge, self.to_right_from_start(edge));
        let opposite_edge = to_opposite_edge(edge);
        self.splice_edges(opposite_edge, self.to_right_from_start(opposite_edge));
    }

    pub(super) fn splice_edges(&mut self, first: QuadEdge, second: QuadEdge) {
        let alpha = to_rotated_edge(self.to_left_from_start(first));
        let beta = to_rotated_edge(self.to_left_from_start(second));
        (self.left_from_start[first], self.left_from_start[second]) = (
            self.to_left_from_start(second),
            self.to_left_from_start(first),
        );
        (self.left_from_start[alpha], self.left_from_start[beta]) = (
            self.to_left_from_start(beta),
            self.to_left_from_start(alpha),
        );
    }
}

impl<Endpoint: Clone + Orient + PartialOrd> Mesh<Endpoint> {
    pub(super) fn to_triangles_vertices(&self) -> Vec<[Endpoint; 3]> {
        let mut result = Vec::new();
        for edge in self.to_edges() {
            let first_vertex = self.get_start(edge);
            let second_vertex = self.get_end(edge);
            let third_vertex = self.get_end(self.to_left_from_start(edge));
            if first_vertex < second_vertex
                && first_vertex < third_vertex
                && third_vertex == self.get_end(self.to_right_from_start(to_opposite_edge(edge)))
                && matches!(
                    self.orient_point_to_edge(edge, self.get_end(self.to_left_from_start(edge))),
                    Orientation::Counterclockwise
                )
            {
                result.push([
                    first_vertex.clone(),
                    second_vertex.clone(),
                    third_vertex.clone(),
                ]);
            }
        }
        result
    }
}

impl<Endpoint: Orient + PartialEq + LocatePointInPointPointPointCircle> Mesh<Endpoint> {
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

    fn find_left_candidate(&mut self, base_edge: QuadEdge) -> Option<QuadEdge> {
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
                self.get_end(self.to_left_from_start(result))
                    .locate_point_in_point_point_point_circle(
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

    fn find_right_candidate(&mut self, base_edge: QuadEdge) -> Option<QuadEdge> {
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
                self.get_end(self.to_right_from_start(result))
                    .locate_point_in_point_point_point_circle(
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
                self.find_left_candidate(base_edge),
                self.find_right_candidate(base_edge),
            );
            base_edge = match maybe_left_candidate {
                Some(left_candidate) => match maybe_right_candidate {
                    Some(right_candidate) => {
                        if matches!(
                            self.get_end(right_candidate)
                                .locate_point_in_point_point_point_circle(
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
}

impl<Endpoint: Orient> Mesh<Endpoint> {
    pub(in crate::triangulation) fn create_triangle(
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

impl<Endpoint: Orient> Mesh<Endpoint> {
    fn orient_point_to_edge(&self, edge: usize, point: &Endpoint) -> Orientation {
        self.get_start(edge).orient(self.get_end(edge), point)
    }
}

impl<Endpoint: Clone + LocatePointInPointPointPointCircle + Ord + Orient> DelaunayTriangulatable
    for Mesh<Endpoint>
{
    fn delunay_triangulation(&mut self) -> (usize, usize) {
        let endpoints_count = self.get_endpoints().len();
        if endpoints_count < 2 {
            (UNDEFINED_QUAD_EDGE, UNDEFINED_QUAD_EDGE)
        } else {
            let (segments_count, triangles_count) = to_base_cases(endpoints_count);
            let mut sub_triangulations_sides =
                Vec::<(QuadEdge, QuadEdge)>::with_capacity(segments_count + triangles_count);
            for index in 0..segments_count {
                let edge = self.create_edge(2 * index, 2 * index + 1);
                let opposite_edge = to_opposite_edge(edge);
                sub_triangulations_sides.push((edge, opposite_edge));
            }
            let offset = 2 * segments_count;
            for index in 0..triangles_count {
                sub_triangulations_sides.push(self.create_triangle(
                    offset + 3 * index,
                    offset + 3 * index + 1,
                    offset + 3 * index + 2,
                ));
            }
            for _ in 0..ceil_log2(sub_triangulations_sides.len()) {
                let merge_steps_count = sub_triangulations_sides.len() / 2;
                let mut next_sub_triangulations_sides = Vec::with_capacity(merge_steps_count);
                for step in 0..merge_steps_count {
                    next_sub_triangulations_sides.push(self.merge(
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
            (left_side, right_side)
        }
    }
}

/// Searches solution of linear diophantine equation
///   `2 * segments_count + 3 * triangles_count == points_count`
/// where `points_count >= 2`
fn to_base_cases(points_count: usize) -> (usize, usize) {
    debug_assert!(points_count >= 2);
    let (triangles_count, rest_points) = points_count.div_rem(3);
    if rest_points == 0 {
        (0, triangles_count)
    } else if rest_points == 1 {
        (2, triangles_count - 1)
    } else {
        (1, triangles_count)
    }
}
