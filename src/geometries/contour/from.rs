use crate::geometries::Point;

use super::types::Contour;

impl<Scalar, const N: usize> From<[Point<Scalar>; N]> for Contour<Scalar>
where
    Point<Scalar>: Clone,
{
    fn from(vertices: [Point<Scalar>; N]) -> Self {
        Self::new(vertices.to_vec())
    }
}

impl<Scalar> From<&[Point<Scalar>]> for Contour<Scalar>
where
    Point<Scalar>: Clone,
{
    fn from(vertices: &[Point<Scalar>]) -> Self {
        Self::new(vertices.to_vec())
    }
}

impl<Scalar> From<Vec<Point<Scalar>>> for Contour<Scalar>
where
    Point<Scalar>: Clone,
{
    fn from(vertices: Vec<Point<Scalar>>) -> Self {
        Self::new(vertices)
    }
}
