use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Contour;
use crate::traits::Polygonal;

use super::types::Polygon;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Polygonal
    for Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
{
    type Contour = self::Contour<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn border(&self) -> Self::Contour {
        self.border.clone()
    }

    fn holes(&self) -> Vec<Self::Contour> {
        self.holes.clone()
    }

    fn holes_count(&self) -> usize {
        self.holes.len()
    }
}
