use crate::triangulation::QuadEdge;

pub(crate) trait BoundaryEndpoints<Endpoint> {
    fn to_boundary_points(&self) -> Vec<Endpoint>;
}

pub(super) trait DelaunayTriangulatable {
    fn delunay_triangulation(&mut self) -> (QuadEdge, QuadEdge);
}
