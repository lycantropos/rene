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
        MultisegmentalsSegments::new((&self.polygons[0]).segments(), self.polygons[1..].iter())
    }

    fn segments_count(self) -> usize {
        self.polygons
            .iter()
            .map(Multisegmental::segments_count)
            .sum::<usize>()
    }
}

impl<Digit, const SHIFT: usize> Multisegmental for Multipolygon<Fraction<BigInt<Digit, SHIFT>>>
where
    Polygon<Fraction<BigInt<Digit, SHIFT>>>: Multisegmental,
{
    type Segment = <Polygon<Fraction<BigInt<Digit, SHIFT>>> as Multisegmental>::Segment;
    type Segments = MultisegmentalsSegments<
        std::vec::IntoIter<Polygon<Fraction<BigInt<Digit, SHIFT>>>>,
        <Polygon<Fraction<BigInt<Digit, SHIFT>>> as Multisegmental>::Segments,
    >;

    fn segments(self) -> Self::Segments {
        let mut polygons = self.polygons.into_iter();
        MultisegmentalsSegments::new(
            unsafe { polygons.next().unwrap_unchecked() }.segments(),
            polygons,
        )
    }

    fn segments_count(self) -> usize {
        self.polygons
            .into_iter()
            .map(Multisegmental::segments_count)
            .sum::<usize>()
    }
}
