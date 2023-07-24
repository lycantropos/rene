use crate::geometries::Segment;
use crate::traits::Multisegmental;

use super::types::Contour;

impl<'a, Scalar> Multisegmental for &'a Contour<Scalar> {
    type Segment = &'a Segment<Scalar>;
    type Segments = std::slice::Iter<'a, Segment<Scalar>>;

    fn segments(self) -> Self::Segments {
        self.segments.iter()
    }

    fn segments_count(self) -> usize {
        self.segments.len()
    }
}

impl<Scalar> Multisegmental for Contour<Scalar> {
    type Segment = Segment<Scalar>;
    type Segments = std::vec::IntoIter<Segment<Scalar>>;

    fn segments(self) -> Self::Segments {
        self.segments.into_iter()
    }

    fn segments_count(self) -> usize {
        self.segments.len()
    }
}
