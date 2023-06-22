use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::{Point, Segment};
use crate::traits::{Multisegmental, Segmental};

use super::types::Contour;

impl<'a, Digit, const SHIFT: usize> Multisegmental for &'a Contour<Fraction<BigInt<Digit, SHIFT>>>
where
    BigInt<Digit, SHIFT>: Clone,
    Segment<Fraction<BigInt<Digit, SHIFT>>>:
        Segmental<Endpoint = Point<Fraction<BigInt<Digit, SHIFT>>>>,
{
    type Segment = &'a Segment<Fraction<BigInt<Digit, SHIFT>>>;
    type Segments = std::slice::Iter<'a, Segment<Fraction<BigInt<Digit, SHIFT>>>>;

    fn segments(self) -> Self::Segments {
        self.segments.iter()
    }

    fn segments_count(self) -> usize {
        self.vertices.len()
    }
}
