use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Contour;
use crate::traits::Polygonal;

use super::types::Polygon;

impl<Digit, const SHIFT: usize> Polygonal for Polygon<Fraction<BigInt<Digit, SHIFT>>>
where
    BigInt<Digit, SHIFT>: Clone,
{
    type Contour = Contour<Fraction<BigInt<Digit, SHIFT>>>;

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
