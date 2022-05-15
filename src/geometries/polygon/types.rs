use crate::geometries::{Contour, Point, Segment};
use crate::traits;

#[derive(Clone)]
pub struct Polygon<Scalar> {
    pub(in crate::geometries) border: Contour<Scalar>,
    pub(in crate::geometries) holes: Vec<Contour<Scalar>>,
}

impl<Scalar: Clone> traits::Polygon<Scalar> for Polygon<Scalar> {
    type Point = self::Point<Scalar>;
    type Segment = self::Segment<Scalar>;
    type Contour = self::Contour<Scalar>;

    fn border(&self) -> Self::Contour {
        self.border.clone()
    }

    fn holes(&self) -> Vec<Self::Contour> {
        self.holes.clone()
    }
}
