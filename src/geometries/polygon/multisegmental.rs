use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::utils::MultisegmentalsSegments;
use crate::geometries::{Contour, Point, Segment};
use crate::traits::{Multisegmental, Segmental};

use super::types::Polygon;

impl<'a, Digit, const SHIFT: usize> Multisegmental for &'a Polygon<Fraction<BigInt<Digit, SHIFT>>>
where
    for<'b> &'b Contour<Fraction<BigInt<Digit, SHIFT>>>:
        Multisegmental<Segment = &'b Segment<Fraction<BigInt<Digit, SHIFT>>>>,
    for<'b> &'b Segment<Fraction<BigInt<Digit, SHIFT>>>:
        Segmental<Endpoint = &'b Point<Fraction<BigInt<Digit, SHIFT>>>>,
{
    type Segment = <&'a Contour<Fraction<BigInt<Digit, SHIFT>>> as Multisegmental>::Segment;
    type Segments = MultisegmentalsSegments<
        std::slice::Iter<'a, Contour<Fraction<BigInt<Digit, SHIFT>>>>,
        <&'a Contour<Fraction<BigInt<Digit, SHIFT>>> as Multisegmental>::Segments,
    >;

    fn segments(self) -> Self::Segments {
        MultisegmentalsSegments::new(self.border.segments(), self.holes.iter())
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
