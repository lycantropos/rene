use crate::geometries::Polygon;

#[derive(Clone)]
pub struct Multipolygon<Scalar> {
    pub(super) polygons: Vec<Polygon<Scalar>>,
}

impl<Scalar> Multipolygon<Scalar> {
    pub fn new(polygons: Vec<Polygon<Scalar>>) -> Self {
        Self { polygons }
    }
}
