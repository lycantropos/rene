use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Polygon;
use crate::traits::Multipolygonal;

use super::types::Multipolygon;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Multipolygonal
    for Multipolygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
{
    type Polygon = self::Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn polygons(&self) -> Vec<Self::Polygon> {
        self.polygons.clone()
    }

    fn polygons_count(&self) -> usize {
        self.polygons.len()
    }
}
