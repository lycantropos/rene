use crate::geometries::Polygon;

#[derive(Clone)]
pub struct Multipolygon<Scalar> {
    pub(super) polygons: Vec<Polygon<Scalar>>,
}

impl<Scalar> Multipolygon<Scalar> {
    #[must_use]
    pub fn new(polygons: Vec<Polygon<Scalar>>) -> Self {
        Self { polygons }
    }
}
