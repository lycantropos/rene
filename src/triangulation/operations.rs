use crate::triangulation::QuadEdge;

pub(crate) trait BoundaryEndpoints<Endpoint> {
    fn get_boundary_endpoints(&self) -> Vec<&Endpoint>;
}

pub(super) trait DelaunayTriangulatable {
    fn delaunay_triangulation(&mut self) -> (QuadEdge, QuadEdge);
}
