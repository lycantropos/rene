use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Contour;
use crate::traits::Polygonal;

use super::types::Polygon;

impl<'a, Digit, const SHIFT: usize> Polygonal
    for &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>
{
    type Contour = &'a Contour<Fraction<BigInt<Digit, SHIFT>>>;
    type Holes = std::slice::Iter<'a, Contour<Fraction<BigInt<Digit, SHIFT>>>>;

    fn border(self) -> Self::Contour {
        &self.border
    }

    fn holes(self) -> Self::Holes {
        self.holes.iter()
    }

    fn holes_count(self) -> usize {
        self.holes.len()
    }
}

impl<Digit, const SHIFT: usize> Polygonal
    for Polygon<Fraction<BigInt<Digit, SHIFT>>>
{
    type Contour = Contour<Fraction<BigInt<Digit, SHIFT>>>;
    type Holes = std::vec::IntoIter<Contour<Fraction<BigInt<Digit, SHIFT>>>>;

    fn border(self) -> Self::Contour {
        self.border
    }

    fn holes(self) -> Self::Holes {
        self.holes.into_iter()
    }

    fn holes_count(self) -> usize {
        self.holes.len()
    }
}
