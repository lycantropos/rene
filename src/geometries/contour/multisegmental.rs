use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Segment;
use crate::traits::Multisegmental;

use super::types::Contour;

impl<'a, Digit, const SHIFT: usize> Multisegmental
    for &'a Contour<Fraction<BigInt<Digit, SHIFT>>>
{
    type Segment = &'a Segment<Fraction<BigInt<Digit, SHIFT>>>;
    type Segments =
        std::slice::Iter<'a, Segment<Fraction<BigInt<Digit, SHIFT>>>>;

    fn segments(self) -> Self::Segments {
        self.segments.iter()
    }

    fn segments_count(self) -> usize {
        self.segments.len()
    }
}

impl<Digit, const SHIFT: usize> Multisegmental
    for Contour<Fraction<BigInt<Digit, SHIFT>>>
{
    type Segment = Segment<Fraction<BigInt<Digit, SHIFT>>>;
    type Segments =
        std::vec::IntoIter<Segment<Fraction<BigInt<Digit, SHIFT>>>>;

    fn segments(self) -> Self::Segments {
        self.segments.into_iter()
    }

    fn segments_count(self) -> usize {
        self.segments.len()
    }
}
