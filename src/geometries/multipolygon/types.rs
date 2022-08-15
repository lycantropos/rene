use crate::geometries::Polygon;

#[derive(Clone)]
pub struct Multipolygon<Scalar> {
    pub(super) polygons: Vec<Polygon<Scalar>>,
}
