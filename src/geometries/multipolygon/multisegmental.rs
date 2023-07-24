use crate::geometries::utils::MultisegmentalsSegments;
use crate::geometries::Polygon;
use crate::traits::Multisegmental;

use super::types::Multipolygon;

impl<'a, Scalar> Multisegmental for &'a Multipolygon<Scalar>
where
    for<'b> &'b Polygon<Scalar>: Multisegmental,
{
    type Segment = <&'a Polygon<Scalar> as Multisegmental>::Segment;
    type Segments = MultisegmentalsSegments<
        std::slice::Iter<'a, Polygon<Scalar>>,
        <&'a Polygon<Scalar> as Multisegmental>::Segments,
    >;

    fn segments(self) -> Self::Segments {
        MultisegmentalsSegments::new(
            (&self.polygons[0]).segments(),
            self.polygons[1..].iter(),
        )
    }

    fn segments_count(self) -> usize {
        self.polygons
            .iter()
            .map(Multisegmental::segments_count)
            .sum::<usize>()
    }
}

impl<Scalar> Multisegmental for Multipolygon<Scalar>
where
    Polygon<Scalar>: Multisegmental,
{
    type Segment = <Polygon<Scalar> as Multisegmental>::Segment;
    type Segments = MultisegmentalsSegments<
        std::vec::IntoIter<Polygon<Scalar>>,
        <Polygon<Scalar> as Multisegmental>::Segments,
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
