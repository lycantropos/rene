use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::constants::MIN_CONTOUR_VERTICES_COUNT;
use crate::operations::orient;
use crate::oriented::Orientation;
use crate::traits;

pub(crate) fn are_contour_vertices_non_degenerate<
    Scalar: AdditiveGroup + MultiplicativeMonoid + Signed,
    Point: traits::Point<Coordinate = Scalar>,
>(
    vertices: &[Point],
) -> bool {
    for index in 1..vertices.len() - 1 {
        if matches!(
            orient(&vertices[index - 1], &vertices[index], &vertices[index + 1]),
            Orientation::Collinear
        ) {
            return false;
        }
    }
    vertices.len() <= MIN_CONTOUR_VERTICES_COUNT
        || !matches!(
            orient(
                &vertices[vertices.len() - 2],
                &vertices[vertices.len() - 1],
                &vertices[0]
            ),
            Orientation::Collinear
        ) && !matches!(
            orient(&vertices[vertices.len() - 1], &vertices[0], &vertices[1]),
            Orientation::Collinear
        )
}
