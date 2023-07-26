use crate::geometries::Segment;
use crate::slice_sequence::SliceSequence;
use crate::traits::Multisegmental;

use super::types::Contour;

impl<'a, Scalar> Multisegmental for &'a Contour<Scalar> {
    type IndexSegment = Segment<Scalar>;
    type IntoIteratorSegment = &'a Segment<Scalar>;
    type Segments = SliceSequence<'a, Segment<Scalar>>;

    fn segments(self) -> Self::Segments {
        SliceSequence::new(&self.segments)
    }
}

impl<Scalar> Multisegmental for Contour<Scalar> {
    type IndexSegment = Segment<Scalar>;
    type IntoIteratorSegment = Segment<Scalar>;
    type Segments = Vec<Segment<Scalar>>;

    fn segments(self) -> Self::Segments {
        self.segments
    }
}
