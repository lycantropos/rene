use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::utils::MultisegmentalsSegments;
use crate::geometries::Polygon;
use crate::traits::Multisegmental;

use super::types::Multipolygon;

impl<'a, Digit, const SHIFT: usize> Multisegmental
    for &'a Multipolygon<Fraction<BigInt<Digit, SHIFT>>>
where
    for<'b> &'b Polygon<Fraction<BigInt<Digit, SHIFT>>>: Multisegmental,
{
    type Segment = <&'a Polygon<Fraction<BigInt<Digit, SHIFT>>> as Multisegmental>::Segment;
    type Segments = MultisegmentalsSegments<
        std::slice::Iter<'a, Polygon<Fraction<BigInt<Digit, SHIFT>>>>,
        <&'a Polygon<Fraction<BigInt<Digit, SHIFT>>> as Multisegmental>::Segments,
    >;

    fn segments(self) -> Self::Segments {
        MultisegmentalsSegments::new(self.polygons[0].segments(), self.polygons[1..].iter())
    }

    fn segments_count(self) -> usize {
        self.polygons
            .iter()
            .map(Multisegmental::segments_count)
            .sum::<usize>()
    }
}
