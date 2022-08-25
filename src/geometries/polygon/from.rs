use crate::geometries::Contour;

use super::types::Polygon;

impl<Scalar> From<(Contour<Scalar>, Vec<Contour<Scalar>>)> for Polygon<Scalar> {
    fn from((border, holes): (Contour<Scalar>, Vec<Contour<Scalar>>)) -> Self {
        Self::new(border, holes)
    }
}
