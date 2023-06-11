use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Segment;
use crate::traits::Multisegmental;

use super::types::Multisegment;

impl<Digit, const SHIFT: usize> Multisegmental for Multisegment<Fraction<BigInt<Digit, SHIFT>>>
where
    BigInt<Digit, SHIFT>: Clone,
{
    type Segment = Segment<Fraction<BigInt<Digit, SHIFT>>>;

    fn segments(&self) -> Vec<Self::Segment> {
        self.segments.clone()
    }

    fn segments_count(&self) -> usize {
        self.segments.len()
    }
}
