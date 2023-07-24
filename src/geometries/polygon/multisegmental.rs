use crate::geometries::utils::MultisegmentalsSegments;
use crate::geometries::{Contour, Point, Segment};
use crate::traits::{Multisegmental, Segmental};

use super::types::Polygon;

impl<'a, Scalar> Multisegmental for &'a Polygon<Scalar>
where
    for<'b> &'b Contour<Scalar>: Multisegmental<Segment = &'b Segment<Scalar>>,
    for<'b> &'b Segment<Scalar>: Segmental<Endpoint = &'b Point<Scalar>>,
{
    type Segment = <&'a Contour<Scalar> as Multisegmental>::Segment;
    type Segments = MultisegmentalsSegments<
        std::slice::Iter<'a, Contour<Scalar>>,
        <&'a Contour<Scalar> as Multisegmental>::Segments,
    >;

    fn segments(self) -> Self::Segments {
        MultisegmentalsSegments::new(
            (&self.border).segments(),
            self.holes.iter(),
        )
    }

    fn segments_count(self) -> usize {
        (&self.border).segments_count()
            + self
                .holes
                .iter()
                .map(Multisegmental::segments_count)
                .sum::<usize>()
    }
}

impl<Scalar> Multisegmental for Polygon<Scalar>
where
    Contour<Scalar>: Multisegmental<Segment = Segment<Scalar>>,
    Segment<Scalar>: Segmental<Endpoint = Point<Scalar>>,
{
    type Segment = <Contour<Scalar> as Multisegmental>::Segment;
    type Segments = MultisegmentalsSegments<
        std::vec::IntoIter<Contour<Scalar>>,
        <Contour<Scalar> as Multisegmental>::Segments,
    >;

    fn segments(self) -> Self::Segments {
        MultisegmentalsSegments::new(
            self.border.segments(),
            self.holes.into_iter(),
        )
    }

    fn segments_count(self) -> usize {
        self.border.segments_count()
            + self
                .holes
                .into_iter()
                .map(Multisegmental::segments_count)
                .sum::<usize>()
    }
}
