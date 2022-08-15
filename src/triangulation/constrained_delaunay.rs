use std::cmp::Ordering;
use std::collections::VecDeque;

use crate::constants::MIN_CONTOUR_VERTICES_COUNT;
use crate::locatable::Location;
use crate::operations::{
    relate_segments, shrink_collinear_vertices, LocatePointInPointPointPointCircle, Orient,
};
use crate::oriented::Orientation;
use crate::relatable::Relation;
use crate::traits::{Contoural, Multivertexal, Polygonal};

use super::mesh::Mesh;
use super::operations::{BoundaryEndpoints, DelaunayTriangulatable};
use super::quad_edge::{to_opposite_edge, QuadEdge, UNDEFINED_QUAD_EDGE};

#[derive(Clone)]
pub(crate) struct ConstrainedDelaunayTriangulation<Endpoint> {
    left_side: QuadEdge,
    mesh: Mesh<Endpoint>,
    polygon_vertices_positions: Vec<Vec<PolygonVertexPosition>>,
    right_side: QuadEdge,
    triangular_holes_indices: Vec<usize>,
}

impl<Endpoint: Clone + Orient> BoundaryEndpoints<Endpoint>
    for ConstrainedDelaunayTriangulation<Endpoint>
{
    fn get_boundary_points(&self) -> Vec<&Endpoint> {
        debug_assert!(self.mesh.get_endpoints().len() >= MIN_CONTOUR_VERTICES_COUNT);
        let mut result = Vec::new();
        let start = self.left_side;
        let mut edge = start;
        loop {
            result.push(self.mesh.get_start(edge));
            let candidate = self.mesh.to_right_from_end(edge);
            if candidate == start {
                break;
            }
            edge = candidate;
        }
        shrink_collinear_vertices(&result)
    }
}

struct PolygonEndpoint<'a, Endpoint> {
    contour_index: usize,
    vertex_index: usize,
    point: &'a Endpoint,
}

impl<'a, Endpoint: PartialEq> PartialEq for PolygonEndpoint<'a, Endpoint> {
    fn eq(&self, other: &Self) -> bool {
        debug_assert!(self.point.eq(other.point));
        self.contour_index.eq(&other.contour_index) && self.vertex_index.eq(&other.vertex_index)
    }
}

impl<'a, Endpoint: PartialOrd> PartialOrd for PolygonEndpoint<'a, Endpoint> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.point.partial_cmp(other.point)? {
            Ordering::Equal => self.contour_index.cmp(&other.contour_index),
            value => value,
        })
    }
}

impl<'a, Endpoint: Eq> Eq for PolygonEndpoint<'a, Endpoint> {}

impl<'a, Endpoint: Ord> Ord for PolygonEndpoint<'a, Endpoint> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.point.cmp(other.point) {
            Ordering::Equal => self.contour_index.cmp(&other.contour_index),
            value => value,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PolygonVertexPosition {
    contour_index: usize,
    vertex_index: usize,
}

impl<
        Endpoint: Clone + LocatePointInPointPointPointCircle + Ord + Orient + PartialOrd,
        Polygon: self::Polygonal,
    > From<&Polygon> for ConstrainedDelaunayTriangulation<Endpoint>
where
    Mesh<Endpoint>: DelaunayTriangulatable,
    <Polygon as Polygonal>::Contour: Contoural<Vertex = Endpoint>,
{
    fn from(polygon: &Polygon) -> Self {
        let mut contours_vertices = Vec::with_capacity(1 + polygon.holes_count());
        contours_vertices.push(polygon.border().vertices());
        for hole in polygon.holes() {
            contours_vertices.push(hole.vertices());
        }
        let mut polygon_endpoints = Vec::with_capacity(
            contours_vertices
                .iter()
                .map(|vertices| vertices.len())
                .sum(),
        );
        for (contour_index, contour_vertices) in contours_vertices.iter().enumerate() {
            polygon_endpoints.extend(contour_vertices.iter().enumerate().map(
                |(vertex_index, point)| PolygonEndpoint {
                    contour_index,
                    vertex_index,
                    point,
                },
            ))
        }
        polygon_endpoints.sort();
        let mut polygon_vertices_positions = Vec::with_capacity(polygon_endpoints.len());
        let mut polygon_vertices = Vec::<Endpoint>::with_capacity(polygon_endpoints.len());
        let mut endpoint = &polygon_endpoints[0];
        polygon_vertices.push(endpoint.point.clone());
        polygon_vertices_positions.push(vec![PolygonVertexPosition {
            contour_index: endpoint.contour_index,
            vertex_index: endpoint.vertex_index,
        }]);
        let mut vertex_index = 0usize;
        for next_endpoint in &polygon_endpoints[1..] {
            if next_endpoint.point.eq(endpoint.point) {
                polygon_vertices_positions[vertex_index].push(PolygonVertexPosition {
                    contour_index: next_endpoint.contour_index,
                    vertex_index: next_endpoint.vertex_index,
                });
            } else {
                vertex_index += 1;
                polygon_vertices.push(next_endpoint.point.clone());
                polygon_vertices_positions.push(vec![PolygonVertexPosition {
                    contour_index: next_endpoint.contour_index,
                    vertex_index: next_endpoint.vertex_index,
                }]);
                endpoint = next_endpoint;
            }
        }
        let mut mesh = Mesh::from(polygon_vertices);
        let (left_side, right_side) = mesh.delaunay_triangulation();
        let triangular_holes_indices = contours_vertices[1..]
            .iter()
            .enumerate()
            .filter(|(_, hole_vertices)| hole_vertices.len() == 3)
            .map(|(hole_index, _)| hole_index + 1)
            .collect();
        let mut result = Self {
            left_side,
            mesh,
            polygon_vertices_positions,
            right_side,
            triangular_holes_indices,
        };
        let contours_sizes = contours_vertices
            .iter()
            .map(|contour_vertices| contour_vertices.len())
            .collect::<Vec<usize>>();
        result.constrain(&contours_sizes, &contours_vertices);
        result.bound(&contours_sizes);
        result.cut(&contours_vertices);
        result
    }
}

impl<Endpoint> ConstrainedDelaunayTriangulation<Endpoint> {
    pub(crate) fn is_empty(&self) -> bool {
        let result = self.mesh.is_empty();
        debug_assert_eq!(self.left_side == UNDEFINED_QUAD_EDGE, result);
        debug_assert_eq!(self.right_side == UNDEFINED_QUAD_EDGE, result);
        result
    }

    fn delete_edge(&mut self, edge: QuadEdge) {
        if edge == self.right_side || to_opposite_edge(edge) == self.right_side {
            self.right_side = to_opposite_edge(self.mesh.to_right_from_end(self.right_side));
        }
        if edge == self.left_side || to_opposite_edge(edge) == self.left_side {
            self.left_side = self.mesh.to_left_from_start(self.left_side);
        }
        self.mesh.delete_edge(edge)
    }

    fn to_unique_boundary_edges(&self) -> Vec<QuadEdge> {
        debug_assert!(!self.is_empty());
        let mut result = Vec::new();
        let start = self.left_side;
        let mut edge = start;
        loop {
            result.push(edge);
            let candidate = self.mesh.to_right_from_end(edge);
            if candidate == start {
                break;
            }
            edge = candidate;
        }
        result
    }
}

impl<Endpoint: Orient + PartialOrd> ConstrainedDelaunayTriangulation<Endpoint> {
    pub(crate) fn to_triangles_vertices(
        &self,
    ) -> impl Iterator<Item = (&Endpoint, &Endpoint, &Endpoint)> + '_ {
        self.mesh
            .to_triangles_base_edges()
            .filter(move |&edge| {
                self.triangular_holes_indices.is_empty()
                    || !are_triangular_hole_vertices(
                        &self.polygon_vertices_positions[self.mesh.to_start_index(edge)],
                        &self.polygon_vertices_positions[self.mesh.to_end_index(edge)],
                        &self.polygon_vertices_positions
                            [self.mesh.to_end_index(self.mesh.to_left_from_start(edge))],
                        &self.triangular_holes_indices,
                    )
            })
            .map(move |edge| self.mesh.triangle_base_to_vertices(edge))
    }
}

impl<Endpoint: LocatePointInPointPointPointCircle + Orient + PartialOrd>
    ConstrainedDelaunayTriangulation<Endpoint>
{
    fn bound(&mut self, contours_sizes: &[usize]) {
        let mut extraneous_mouths = self
            .to_unique_boundary_edges()
            .into_iter()
            .filter(|edge| {
                !is_polygon_edge(
                    &self.mesh,
                    *edge,
                    contours_sizes,
                    &self.polygon_vertices_positions,
                )
            })
            .collect::<Vec<QuadEdge>>();
        while let Some(mouth) = extraneous_mouths.pop() {
            let (first_candidate, second_candidate) = mouth_edge_to_incidents(&self.mesh, mouth);
            self.delete_edge(mouth);
            if !is_polygon_edge(
                &self.mesh,
                first_candidate,
                contours_sizes,
                &self.polygon_vertices_positions,
            ) {
                extraneous_mouths.push(first_candidate);
            }
            if !is_polygon_edge(
                &self.mesh,
                second_candidate,
                contours_sizes,
                &self.polygon_vertices_positions,
            ) {
                extraneous_mouths.push(second_candidate);
            }
        }
    }

    fn constrain(&mut self, contours_sizes: &[usize], contours_vertices: &[Vec<Endpoint>]) {
        let mut contours_constraints_flags = to_contours_constraints_flags(
            &self.mesh,
            contours_sizes,
            &self.polygon_vertices_positions,
        );
        for edge in self.mesh.to_edges() {
            let start_index = self.mesh.to_start_index(edge);
            for &PolygonVertexPosition {
                contour_index,
                vertex_index,
            } in &self.polygon_vertices_positions[start_index]
            {
                let next_vertex_index = (vertex_index + 1) % contours_sizes[contour_index];
                let constraint_index = to_constraint_index(vertex_index, next_vertex_index);
                if !contours_constraints_flags[contour_index][constraint_index] {
                    let vertex_point = &contours_vertices[contour_index][vertex_index];
                    let next_vertex_point = &contours_vertices[contour_index][next_vertex_index];
                    let angle_base_edge =
                        to_angle_containing_constraint_base(&self.mesh, edge, next_vertex_point);
                    let crossings = detect_crossings(
                        &self.mesh,
                        angle_base_edge,
                        vertex_point,
                        next_vertex_point,
                    );
                    if !crossings.is_empty() {
                        set_constraint(&mut self.mesh, vertex_point, next_vertex_point, crossings);
                    }
                    contours_constraints_flags[contour_index][constraint_index] = true;
                }
            }
        }
    }

    fn cut(&mut self, contours_vertices: &[Vec<Endpoint>]) {
        for edge in self.mesh.to_unique_edges() {
            if is_edge_inside_hole(
                &self.mesh,
                edge,
                contours_vertices,
                &self.polygon_vertices_positions,
            ) {
                self.delete_edge(edge)
            }
        }
    }
}

fn angle_contains_point<Point: Orient>(
    vertex: &Point,
    first_ray_point: &Point,
    second_ray_point: &Point,
    angle_orientation: Orientation,
    point: &Point,
) -> bool {
    debug_assert!(angle_orientation != Orientation::Collinear);
    let first_half_orientation = vertex.orient(first_ray_point, point);
    let second_half_orientation = second_ray_point.orient(vertex, point);
    (first_half_orientation == Orientation::Collinear
        || first_half_orientation == angle_orientation)
        && (second_half_orientation == Orientation::Collinear
            || second_half_orientation == angle_orientation)
}

fn are_polygon_edge_indices(
    first_vertex_index: usize,
    second_vertex_index: usize,
    contour_size: usize,
) -> bool {
    first_vertex_index.abs_diff(second_vertex_index) == 1
        || (first_vertex_index == 0 && second_vertex_index == contour_size - 1)
        || (first_vertex_index == contour_size - 1 && second_vertex_index == 0)
}

fn are_triangular_hole_vertices(
    first_positions: &[PolygonVertexPosition],
    second_positions: &[PolygonVertexPosition],
    third_positions: &[PolygonVertexPosition],
    triangular_holes_indices: &[usize],
) -> bool {
    first_positions
        .iter()
        .map(|position| position.contour_index)
        .filter(|contour_index| {
            second_positions
                .iter()
                .find(|position| position.contour_index.eq(contour_index))
                .is_some()
                && third_positions
                    .iter()
                    .find(|position| position.contour_index.eq(contour_index))
                    .is_some()
        })
        .any(|contour_index| triangular_holes_indices.contains(&contour_index))
}

fn detect_crossings<Endpoint: Orient + PartialEq + PartialOrd>(
    mesh: &Mesh<Endpoint>,
    base_edge: QuadEdge,
    constraint_start: &Endpoint,
    constraint_end: &Endpoint,
) -> Vec<QuadEdge> {
    let mut candidate = mesh.to_left_from_end(base_edge);
    let mut result = Vec::new();
    while mesh.get_start(candidate).ne(constraint_end) {
        let last_crossing = candidate;
        debug_assert_eq!(
            relate_segments(
                mesh.get_start(last_crossing),
                mesh.get_end(last_crossing),
                constraint_start,
                constraint_end
            ),
            Relation::Cross
        );
        result.push(last_crossing);
        candidate = mesh.to_right_from_start(last_crossing);
        if mesh.orient_point_to_edge(candidate, constraint_end) != Orientation::Clockwise
            || constraint_start.orient(constraint_end, mesh.get_end(candidate))
                == Orientation::Clockwise
        {
            candidate = to_opposite_edge(mesh.to_right_from_end(last_crossing))
        }
    }
    result
}

fn edge_should_be_swapped<Endpoint: LocatePointInPointPointPointCircle + Orient>(
    mesh: &Mesh<Endpoint>,
    edge: QuadEdge,
) -> bool {
    is_convex_quadrilateral_diagonal(mesh, edge)
        && ((mesh
            .get_end(mesh.to_right_from_start(edge))
            .locate_point_in_point_point_point_circle(
                mesh.get_start(edge),
                mesh.get_end(edge),
                mesh.get_end(mesh.to_left_from_start(edge)),
            )
            == Location::Interior)
            || (mesh
                .get_end(mesh.to_left_from_start(edge))
                .locate_point_in_point_point_point_circle(
                    mesh.get_end(edge),
                    mesh.get_start(edge),
                    mesh.get_end(mesh.to_right_from_start(edge)),
                )
                == Location::Interior))
}

fn intersect_polygon_vertices_positions_slices<const WITH_BORDER: bool>(
    left: &[PolygonVertexPosition],
    right: &[PolygonVertexPosition],
) -> Vec<(PolygonVertexPosition, PolygonVertexPosition)> {
    if left.len() < right.len() {
        intersect_polygon_vertices_positions_slices_impl::<true, WITH_BORDER>(left, right)
    } else {
        intersect_polygon_vertices_positions_slices_impl::<false, WITH_BORDER>(right, left)
    }
}

fn intersect_polygon_vertices_positions_slices_impl<
    const REVERSE: bool,
    const WITH_BORDER: bool,
>(
    shorter: &[PolygonVertexPosition],
    longer: &[PolygonVertexPosition],
) -> Vec<(PolygonVertexPosition, PolygonVertexPosition)> {
    debug_assert!(shorter.len() <= longer.len());
    let mut result = Vec::with_capacity(shorter.len());
    for shorter_position in shorter {
        if WITH_BORDER || shorter_position.contour_index != 0 {
            match longer
                .iter()
                .find(|&candidate| candidate.contour_index == shorter_position.contour_index)
            {
                Some(longer_position) => result.push(if REVERSE {
                    (*shorter_position, *longer_position)
                } else {
                    (*longer_position, *shorter_position)
                }),
                _ => {}
            }
        }
    }
    result
}

fn is_edge_inside_hole<Endpoint: Orient>(
    mesh: &Mesh<Endpoint>,
    edge: QuadEdge,
    contours_vertices: &[Vec<Endpoint>],
    polygon_vertices_positions: &[Vec<PolygonVertexPosition>],
) -> bool {
    let (start_index, end_index) = (mesh.to_start_index(edge), mesh.to_end_index(edge));
    debug_assert_ne!(start_index, end_index);
    let (start, end) = {
        let endpoints = mesh.get_endpoints();
        (&endpoints[start_index], &endpoints[end_index])
    };
    let edge_holes_positions_pairs = intersect_polygon_vertices_positions_slices::<false>(
        &polygon_vertices_positions[start_index],
        &polygon_vertices_positions[end_index],
    );
    debug_assert!(edge_holes_positions_pairs.len() <= 1);
    if let Some((start_hole_position, end_hole_position)) = edge_holes_positions_pairs.first() {
        debug_assert_eq!(
            start_hole_position.contour_index,
            end_hole_position.contour_index
        );
        debug_assert_ne!(
            start_hole_position.vertex_index,
            end_hole_position.vertex_index
        );
        let hole_vertices = &contours_vertices[start_hole_position.contour_index];
        let hole_size = hole_vertices.len();
        let start_vertex_index = start_hole_position.vertex_index;
        let end_vertex_index = end_hole_position.vertex_index;
        if are_polygon_edge_indices(start_vertex_index, end_vertex_index, hole_size) {
            return false;
        }
        let prior_to_start_point = &hole_vertices[if start_vertex_index == 0 {
            hole_size - 1
        } else {
            start_vertex_index - 1
        }];
        let prior_to_end_point = &hole_vertices[if end_vertex_index == 0 {
            hole_size - 1
        } else {
            end_vertex_index - 1
        }];
        let next_to_start_point = &hole_vertices[(start_vertex_index + 1) % hole_size];
        let next_to_end_point = &hole_vertices[(end_vertex_index + 1) % hole_size];
        let start_angle_orientation = start.orient(prior_to_start_point, next_to_start_point);
        let end_angle_orientation = end.orient(prior_to_end_point, next_to_end_point);
        if ((end_angle_orientation == Orientation::Counterclockwise)
            == angle_contains_point(
                end,
                prior_to_end_point,
                next_to_end_point,
                end_angle_orientation,
                start,
            ))
            && ((start_angle_orientation == Orientation::Counterclockwise)
                == angle_contains_point(
                    start,
                    prior_to_start_point,
                    next_to_start_point,
                    start_angle_orientation,
                    end,
                ))
        {
            return true;
        }
    }
    false
}

fn is_convex_quadrilateral_diagonal<Endpoint: Orient>(
    mesh: &Mesh<Endpoint>,
    edge: QuadEdge,
) -> bool {
    mesh.orient_point_to_edge(mesh.to_left_from_end(edge), mesh.get_start(edge))
        == Orientation::Counterclockwise
        && mesh.orient_point_to_edge(mesh.to_right_from_start(edge), mesh.get_end(edge))
            == Orientation::Counterclockwise
        && mesh.orient_point_to_edge(
            to_opposite_edge(mesh.to_right_from_end(edge)),
            mesh.get_end(mesh.to_left_from_start(edge)),
        ) == Orientation::Counterclockwise
        && mesh.orient_point_to_edge(
            to_opposite_edge(mesh.to_left_from_start(edge)),
            mesh.get_end(mesh.to_right_from_start(edge)),
        ) == Orientation::Counterclockwise
}

fn is_polygon_edge<Endpoint>(
    mesh: &Mesh<Endpoint>,
    edge: QuadEdge,
    contours_sizes: &[usize],
    polygon_vertices_positions: &[Vec<PolygonVertexPosition>],
) -> bool {
    intersect_polygon_vertices_positions_slices::<true>(
        &polygon_vertices_positions[mesh.to_start_index(edge)],
        &polygon_vertices_positions[mesh.to_end_index(edge)],
    )
    .into_iter()
    .all(|(start_position, end_position)| {
        are_polygon_edge_indices(
            start_position.vertex_index,
            end_position.vertex_index,
            contours_sizes[start_position.contour_index],
        )
    })
}

fn mouth_edge_to_incidents<Endpoint>(
    mesh: &Mesh<Endpoint>,
    edge: QuadEdge,
) -> (QuadEdge, QuadEdge) {
    let left_from_start = mesh.to_left_from_start(edge);
    (left_from_start, mesh.to_right_from_end(left_from_start))
}

fn resolve_crossings<Endpoint: Orient + PartialOrd>(
    mesh: &mut Mesh<Endpoint>,
    crossings: Vec<QuadEdge>,
    constraint_start: &Endpoint,
    constraint_end: &Endpoint,
) -> Vec<QuadEdge> {
    let mut result = Vec::with_capacity(crossings.len());
    let mut crossings_queue = VecDeque::from(crossings);
    while let Some(edge) = crossings_queue.pop_back() {
        if is_convex_quadrilateral_diagonal(mesh, edge) {
            mesh.swap_diagonal(edge);
            match relate_segments(
                mesh.get_start(edge),
                mesh.get_end(edge),
                constraint_start,
                constraint_end,
            ) {
                Relation::Cross => crossings_queue.push_front(edge),
                Relation::Equal => {}
                _ => result.push(edge),
            }
        } else {
            crossings_queue.push_front(edge)
        }
    }
    return result;
}

fn set_constraint<'a, Endpoint: LocatePointInPointPointPointCircle + Orient + PartialOrd>(
    mesh: &mut Mesh<Endpoint>,
    constraint_start: &Endpoint,
    constraint_end: &Endpoint,
    crossings: Vec<QuadEdge>,
) {
    let new_edges = resolve_crossings(mesh, crossings, constraint_start, constraint_end);
    set_criterion(mesh, new_edges);
}

fn set_criterion<Endpoint: LocatePointInPointPointPointCircle + Orient>(
    mesh: &mut Mesh<Endpoint>,
    mut candidates: Vec<QuadEdge>,
) {
    loop {
        let mut next_target_edges = Vec::with_capacity(candidates.capacity());
        let mut edges_to_swap = Vec::with_capacity(candidates.capacity());
        for edge in candidates {
            if edge_should_be_swapped(mesh, edge) {
                edges_to_swap.push(edge);
            } else {
                next_target_edges.push(edge)
            }
        }
        if edges_to_swap.is_empty() {
            break;
        }
        for edge in edges_to_swap {
            mesh.swap_diagonal(edge)
        }
        candidates = next_target_edges
    }
}

fn to_angle_containing_constraint_base<Endpoint: Orient + PartialEq>(
    mesh: &Mesh<Endpoint>,
    mut edge: QuadEdge,
    constraint_end: &Endpoint,
) -> QuadEdge {
    if mesh.get_end(edge).ne(constraint_end) {
        let mut orientation = mesh.orient_point_to_edge(edge, constraint_end);
        if orientation == Orientation::Counterclockwise {
            loop {
                let candidate = mesh.to_left_from_start(edge);
                orientation = mesh.orient_point_to_edge(candidate, constraint_end);
                if orientation == Orientation::Clockwise {
                    break;
                }
                edge = candidate
            }
        } else {
            loop {
                edge = mesh.to_right_from_start(edge);
                orientation = mesh.orient_point_to_edge(edge, constraint_end);
                if orientation != Orientation::Clockwise {
                    break;
                }
            }
        }
    }
    edge
}

fn to_constraint_index(first_vertex_index: usize, second_vertex_index: usize) -> usize {
    if first_vertex_index.abs_diff(second_vertex_index) == 1 {
        first_vertex_index.max(second_vertex_index)
    } else {
        0
    }
}

fn to_contours_constraints_flags<Endpoint>(
    mesh: &Mesh<Endpoint>,
    contours_sizes: &[usize],
    polygon_vertices_positions: &[Vec<PolygonVertexPosition>],
) -> Vec<Vec<bool>> {
    let mut are_constraints_satisfied: Vec<Vec<bool>> = contours_sizes
        .iter()
        .map(|contour_size| vec![false; *contour_size])
        .collect();
    for edge in mesh.iter_unique_edges() {
        for (start_position, end_position) in intersect_polygon_vertices_positions_slices::<true>(
            &polygon_vertices_positions[mesh.to_start_index(edge)],
            &polygon_vertices_positions[mesh.to_end_index(edge)],
        ) {
            debug_assert_eq!(start_position.contour_index, end_position.contour_index);
            let contour_index = start_position.contour_index;
            let start_index = start_position.vertex_index;
            let end_index = end_position.vertex_index;
            if are_polygon_edge_indices(start_index, end_index, contours_sizes[contour_index]) {
                are_constraints_satisfied[contour_index]
                    [to_constraint_index(start_index, end_index)] = true;
            }
        }
    }
    are_constraints_satisfied
}
