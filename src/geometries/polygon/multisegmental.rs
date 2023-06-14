use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::{Contour, Segment};
use crate::traits::{Multisegmental, Segmental};

use super::types::Polygon;

impl<'a, Digit, const SHIFT: usize> Multisegmental for &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>
where
    &'a Contour<Fraction<BigInt<Digit, SHIFT>>>:
        Multisegmental<Segment = Segment<Fraction<BigInt<Digit, SHIFT>>>>,
    Segment<Fraction<BigInt<Digit, SHIFT>>>: Segmental,
{
    type Segment = Segment<Fraction<BigInt<Digit, SHIFT>>>;
    type Segments = std::vec::IntoIter<Self::Segment>;

    fn segments(self) -> Self::Segments {
        let mut result = Vec::<Self::Segment>::with_capacity(self.segments_count());
        result.extend(self.border.segments());
        for hole in &self.holes {
            result.extend(hole.segments());
        }
        result.into_iter()
    }

    fn segments_count(self) -> usize {
        self.border.segments_count()
            + self
                .holes
                .iter()
                .map(Multisegmental::segments_count)
                .sum::<usize>()
    }
}
