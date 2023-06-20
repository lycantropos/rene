use crate::constants::MIN_CONTOUR_VERTICES_COUNT;
use crate::operations::Orient;
use crate::oriented::Orientation;

pub(crate) fn are_contour_vertices_non_degenerate<'a, Point>(vertices: &'a [Point]) -> bool
where
    &'a Point: Orient,
{
    vertices.len() >= MIN_CONTOUR_VERTICES_COUNT && {
        let mut first_vertex = &vertices[vertices.len() - 2];
        let mut second_vertex = &vertices[vertices.len() - 1];
        for third_vertex in vertices {
            if first_vertex.orient(second_vertex, third_vertex) == Orientation::Collinear {
                return false;
            }
            (first_vertex, second_vertex) = (second_vertex, third_vertex);
        }
        true
    }
}
