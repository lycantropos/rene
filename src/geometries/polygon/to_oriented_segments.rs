use crate::geometries::{Contour, Point, Segment};
use crate::operations::{Orient, ToOrientedSegments, ToReversedSegments};
use crate::oriented::{Orientation, Oriented};
use crate::traits::{Contoural, Multisegmental};

use super::types::Polygon;

impl<'a, Scalar> ToOrientedSegments for &'a Polygon<Scalar>
where
    &'a Point<Scalar>: Orient,
    &'a Contour<Scalar>: Contoural<Segment = &'a Segment<Scalar>>
        + Oriented
        + ToReversedSegments<Output = Vec<Segment<Scalar>>>,
    Point<Scalar>: Clone,
    Segment<Scalar>: Clone,
{
    type Output = std::vec::IntoIter<Segment<Scalar>>;

    fn to_oriented_segments(self) -> Self::Output {
        let mut result = Vec::<Segment<Scalar>>::with_capacity(
            self.border.segments_count()
                + self
                    .holes
                    .iter()
                    .map(Multisegmental::segments_count)
                    .sum::<usize>(),
        );
        if self.border.to_orientation() == Orientation::Counterclockwise {
            result.extend(self.border.segments().cloned());
        } else {
            result.append(&mut self.border.to_reversed_segments());
        }
        for hole in &self.holes {
            if hole.to_orientation() == Orientation::Clockwise {
                result.extend(hole.segments().cloned());
            } else {
                result.append(&mut hole.to_reversed_segments());
            }
        }
        result.into_iter()
    }
}
