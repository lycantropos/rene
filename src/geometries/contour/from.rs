use crate::geometries::Point;

use super::types::Contour;

impl<Scalar: Clone, const N: usize> From<[Point<Scalar>; N]> for Contour<Scalar> {
    fn from(vertices: [Point<Scalar>; N]) -> Self {
        Self::new(vertices.to_vec())
    }
}
