use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::{Polygon, Segment};
use crate::traits::{Multisegmental, Segmental};

use super::types::Multipolygon;

impl<Digit, const SHIFT: usize> Multisegmental for Multipolygon<Fraction<BigInt<Digit, SHIFT>>>
where
    Polygon<Fraction<BigInt<Digit, SHIFT>>>:
        Multisegmental<Segment = Segment<Fraction<BigInt<Digit, SHIFT>>>>,
    Segment<Fraction<BigInt<Digit, SHIFT>>>: Segmental,
{
    type Segment = Segment<Fraction<BigInt<Digit, SHIFT>>>;

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
            .map(Multisegmental::segments_count)
            .sum::<usize>()
    }
}
