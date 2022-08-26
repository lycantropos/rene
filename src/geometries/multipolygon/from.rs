use crate::geometries::Polygon;

use super::types::Multipolygon;

impl<Scalar: Clone, const N: usize> From<[Polygon<Scalar>; N]> for Multipolygon<Scalar> {
    fn from(polygons: [Polygon<Scalar>; N]) -> Self {
        Self::new(polygons.to_vec())
    }
}

impl<Scalar: Clone> From<&[Polygon<Scalar>]> for Multipolygon<Scalar> {
    fn from(polygons: &[Polygon<Scalar>]) -> Self {
        Self::new(polygons.to_vec())
    }
}

impl<Scalar> From<Vec<Polygon<Scalar>>> for Multipolygon<Scalar> {
    fn from(polygons: Vec<Polygon<Scalar>>) -> Self {
        Self::new(polygons)
    }
}
