use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Segment;
use crate::traits::Multisegmental;

use super::types::Multisegment;

impl<'a, Digit, const SHIFT: usize> Multisegmental
    for &'a Multisegment<Fraction<BigInt<Digit, SHIFT>>>
where
    BigInt<Digit, SHIFT>: Clone,
{
    type Segment = &'a Segment<Fraction<BigInt<Digit, SHIFT>>>;
    type Segments = std::slice::Iter<'a, Segment<Fraction<BigInt<Digit, SHIFT>>>>;

    fn segments(self) -> Self::Segments {
        self.segments.iter()
    }

    fn segments_count(self) -> usize {
        self.segments.len()
    }
}
