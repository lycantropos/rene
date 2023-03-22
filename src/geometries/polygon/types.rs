use crate::geometries::Contour;

#[derive(Clone)]
pub struct Polygon<Scalar> {
    pub(super) border: Contour<Scalar>,
    pub(super) holes: Vec<Contour<Scalar>>,
}

impl<Scalar> Polygon<Scalar> {
    #[must_use]
    pub fn new(border: Contour<Scalar>, holes: Vec<Contour<Scalar>>) -> Self {
        Self { border, holes }
    }
}
