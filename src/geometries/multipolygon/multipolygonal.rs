use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Polygon;
use crate::traits::Multipolygonal;

use super::types::Multipolygon;

impl<'a, Digit, const SHIFT: usize> Multipolygonal
    for &'a Multipolygon<Fraction<BigInt<Digit, SHIFT>>>
{
    type Polygon = &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>;
    type Polygons =
        std::slice::Iter<'a, Polygon<Fraction<BigInt<Digit, SHIFT>>>>;

    fn polygons(self) -> Self::Polygons {
        self.polygons.iter()
    }

    fn polygons_count(self) -> usize {
        self.polygons.len()
    }
}

impl<Digit, const SHIFT: usize> Multipolygonal
    for Multipolygon<Fraction<BigInt<Digit, SHIFT>>>
{
    type Polygon = Polygon<Fraction<BigInt<Digit, SHIFT>>>;
    type Polygons =
        std::vec::IntoIter<Polygon<Fraction<BigInt<Digit, SHIFT>>>>;

    fn polygons(self) -> Self::Polygons {
        self.polygons.into_iter()
    }

    fn polygons_count(self) -> usize {
        self.polygons.len()
    }
}
