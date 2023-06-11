use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::{Contour, Segment};
use crate::traits::{Multisegmental, Segmental};

use super::types::Polygon;

impl<Digit, const SHIFT: usize> Multisegmental for Polygon<Fraction<BigInt<Digit, SHIFT>>>
where
    Contour<Fraction<BigInt<Digit, SHIFT>>>:
        Multisegmental<Segment = Segment<Fraction<BigInt<Digit, SHIFT>>>>,
    Segment<Fraction<BigInt<Digit, SHIFT>>>: Segmental,
{
    type Segment = Segment<Fraction<BigInt<Digit, SHIFT>>>;

    fn segments(&self) -> Vec<Self::Segment> {
        let mut result = Vec::<Self::Segment>::with_capacity(self.segments_count());
        result.append(&mut self.border.segments());
        for hole in &self.holes {
            result.append(&mut hole.segments());
        }
        result
    }

    fn segments_count(&self) -> usize {
        self.border.segments_count()
            + self
                .holes
                .iter()
                .map(Multisegmental::segments_count)
                .sum::<usize>()
    }
}
