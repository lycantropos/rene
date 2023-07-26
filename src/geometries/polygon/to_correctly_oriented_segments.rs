use crate::geometries::{Contour, Point, Segment};
use crate::operations::{
    Orient, ToCorrectlyOrientedSegments, ToReversedSegments,
};
use crate::oriented::{Orientation, Oriented};
use crate::traits::{
    Contoural, Elemental, Iterable, Lengthsome, Multisegmental,
    MultivertexalIndexVertex, Segmental,
};

use super::types::Polygon;

impl<Scalar> ToCorrectlyOrientedSegments for &Polygon<Scalar>
where
    for<'a> &'a Contour<Scalar>: Contoural<IndexSegment = Segment<Scalar>>
        + Oriented
        + ToReversedSegments<Output = Vec<Segment<Scalar>>>,
    for<'a> &'a Segment<Scalar>: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Contour<Scalar>>: Elemental,
    for<'a> &'a Point<Scalar>: Orient,
    Point<Scalar>: Clone,
    Segment<Scalar>: Clone,
{
    type Output = std::vec::IntoIter<Segment<Scalar>>;

    fn to_correctly_oriented_segments(self) -> Self::Output {
        let mut result = Vec::<Segment<Scalar>>::with_capacity(
            (&self.border).segments().len()
                + self
                    .holes
                    .iter()
                    .map(|hole| hole.segments().len())
                    .sum::<usize>(),
        );
        if (&self.border).to_orientation() == Orientation::Counterclockwise {
            result.extend((&self.border).segments().iter().cloned());
        } else {
            result.append(&mut (&self.border).to_reversed_segments());
        }
        for hole in &self.holes {
            if hole.to_orientation() == Orientation::Clockwise {
                result.extend(hole.segments().iter().cloned());
            } else {
                result.append(&mut hole.to_reversed_segments());
            }
        }
        result.into_iter()
    }
}
