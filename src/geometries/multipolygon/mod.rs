use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::{Contour, Point, Polygon, Segment};
use crate::traits;

#[derive(Clone)]
struct Multipolygon<Scalar> {
    polygons: Vec<Polygon<Scalar>>,
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> traits::Multipolygon
    for Multipolygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
{
    type Point = self::Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;
    type Segment = self::Segment<<Self::Point as traits::Point>::Coordinate>;
    type Contour = self::Contour<<Self::Point as traits::Point>::Coordinate>;
    type Polygon = self::Polygon<<Self::Point as traits::Point>::Coordinate>;

    fn polygons(&self) -> Vec<Self::Polygon> {
        self.polygons.clone()
    }

    fn polygons_count(&self) -> usize {
        self.polygons.len()
    }
}
