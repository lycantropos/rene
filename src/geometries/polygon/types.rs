use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

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

impl<Digit, const SEPARATOR: char, const SHIFT: usize> traits::Polygon
    for Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
{
    type Point = self::Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;
    type Segment = self::Segment<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;
    type Contour = self::Contour<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn border(&self) -> Self::Contour {
        self.border.clone()
    }

    fn holes(&self) -> Vec<Self::Contour> {
        self.holes.clone()
    }
}
