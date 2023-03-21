use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Polygon;
use crate::traits::Multipolygonal;

use super::types::Multipolygon;

impl<Digit, const SHIFT: usize> Multipolygonal for Multipolygon<Fraction<BigInt<Digit, SHIFT>>>
where
    BigInt<Digit, SHIFT>: Clone,
{
    type Polygon = self::Polygon<Fraction<BigInt<Digit, SHIFT>>>;

    fn polygons(&self) -> Vec<Self::Polygon> {
        self.polygons.clone()
    }

    fn polygons_count(&self) -> usize {
        self.polygons.len()
    }
}
