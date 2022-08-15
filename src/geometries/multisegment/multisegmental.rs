use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Segment;
use crate::traits::Multisegmental;

use super::types::Multisegment;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Multisegmental
    for Multisegment<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
{
    type Segment = self::Segment<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn segments(&self) -> Vec<Self::Segment> {
        self.segments.clone()
    }

    fn segments_count(&self) -> usize {
        self.segments.len()
    }
}
