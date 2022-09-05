use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::{Polygon, Segment};
use crate::traits::{Multisegmental, Segmental};

use super::types::Multipolygon;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Multisegmental
    for Multipolygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>:
        Multisegmental<Segment = self::Segment<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>,
    Segment<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>: Segmental,
{
    type Segment = self::Segment<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn segments(&self) -> Vec<Self::Segment> {
        let mut result = Vec::<Self::Segment>::with_capacity(self.segments_count());
        for polygon in &self.polygons {
            result.append(&mut polygon.segments());
        }
        result
    }

    fn segments_count(&self) -> usize {
        self.polygons
            .iter()
            .map(|polygon| polygon.segments_count())
            .sum::<usize>()
    }
}
