use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::{Polygon, Segment};
use crate::traits::{Multisegmental, Segmental};

use super::types::Multipolygon;

impl<'a, Digit, const SHIFT: usize> Multisegmental
    for &'a Multipolygon<Fraction<BigInt<Digit, SHIFT>>>
where
    &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>:
        Multisegmental<Segment = Segment<Fraction<BigInt<Digit, SHIFT>>>>,
    Segment<Fraction<BigInt<Digit, SHIFT>>>: Segmental,
{
    type Segment = Segment<Fraction<BigInt<Digit, SHIFT>>>;
    type Segments = std::vec::IntoIter<Self::Segment>;

    fn segments(self) -> Self::Segments {
        let mut result = Vec::<Self::Segment>::with_capacity(self.segments_count());
        for polygon in &self.polygons {
            result.extend(polygon.segments());
        }
        result.into_iter()
    }

    fn segments_count(self) -> usize {
        self.polygons
            .iter()
            .map(Multisegmental::segments_count)
            .sum::<usize>()
    }
}
