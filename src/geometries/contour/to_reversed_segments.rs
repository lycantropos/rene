use crate::geometries::{Point, Segment};
use crate::operations::ToReversedSegments;

use super::types::Contour;

impl<'a, Scalar> ToReversedSegments for &'a Contour<Scalar>
where
    Point<Scalar>: Clone,
{
    type Output = Vec<Segment<Scalar>>;

    fn to_reversed_segments(self) -> Self::Output {
        let mut segments =
            Vec::<Segment<Scalar>>::with_capacity(self.vertices.len());
        for index in 0..self.vertices.len() - 1 {
            segments.push(Segment::new(
                self.vertices[index + 1].clone(),
                self.vertices[index].clone(),
            ));
        }
        segments.push(Segment::new(
            self.vertices[0].clone(),
            self.vertices[self.vertices.len() - 1].clone(),
        ));
        segments
    }
}
