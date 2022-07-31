use crate::constants::MIN_CONTOUR_VERTICES_COUNT;
use crate::operations::Orient;
use crate::oriented::Orientation;

pub(crate) fn are_contour_vertices_non_degenerate<Point: Orient>(vertices: &[Point]) -> bool {
    for index in 1..vertices.len() - 1 {
        if matches!(
            vertices[index - 1].orient(&vertices[index], &vertices[index + 1]),
            Orientation::Collinear
        ) {
            return false;
        }
    }
    vertices.len() <= MIN_CONTOUR_VERTICES_COUNT
        || !matches!(
            vertices[vertices.len() - 2].orient(&vertices[vertices.len() - 1], &vertices[0]),
            Orientation::Collinear
        ) && !matches!(
            vertices[vertices.len() - 1].orient(&vertices[0], &vertices[1]),
            Orientation::Collinear
        )
}
