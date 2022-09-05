use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::{Contour, Segment};
use crate::traits::{Multisegmental, Segmental};

use super::types::Polygon;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Multisegmental
    for Polygon<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    Contour<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>:
        Multisegmental<Segment = self::Segment<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>>,
    Segment<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>: Segmental,
{
    type Segment = self::Segment<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

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
                .map(|hole| hole.segments_count())
                .sum::<usize>()
    }
}
