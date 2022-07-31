use crate::geometries::{Contour, Point, Segment};
use crate::traits;

#[derive(Clone)]
pub struct Polygon<Scalar> {
    pub(in crate::geometries) border: Contour<Scalar>,
    pub(in crate::geometries) holes: Vec<Contour<Scalar>>,
}

impl<Scalar> Polygon<Scalar> {
    pub fn new(border: Contour<Scalar>, holes: Vec<Contour<Scalar>>) -> Self {
        Self { border, holes }
    }
}

impl<Scalar: Clone> traits::Polygon for Polygon<Scalar> {
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
