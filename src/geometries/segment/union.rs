use crate::geometries::{Contour, Empty, Multisegment, Point};
use crate::operations::{to_sorted_pair, IntersectCrossingSegments, Orient};
use crate::oriented::Orientation;
use crate::traits::{Difference, Union};

use super::types::Segment;

impl<Scalar> Union<Empty> for Segment<Scalar> {
    type Output = Self;

    fn union(self, _other: Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Union<&Empty> for Segment<Scalar> {
    type Output = Self;

    fn union(self, _other: &Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Union<Empty> for &Segment<Scalar>
where
    Segment<Scalar>: Clone,
{
    type Output = Segment<Scalar>;

    fn union(self, _other: Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> Union<&Empty> for &Segment<Scalar>
where
    Segment<Scalar>: Clone,
{
    type Output = Segment<Scalar>;

    fn union(self, _other: &Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> Union for &Segment<Scalar>
where
    Segment<Scalar>: Clone,
    Point<Scalar>: Clone + Ord,
    for<'a> &'a Point<Scalar>:
        IntersectCrossingSegments<Output = Point<Scalar>> + Orient,
{
    type Output = Vec<Segment<Scalar>>;

    fn union(self, other: Self) -> Self::Output {
        let (start, end) = to_sorted_pair((&self.start, &self.end));
        let (other_start, other_end) =
            to_sorted_pair((&other.start, &other.end));
        if start == other_start && end == other_end {
            return vec![self.clone(), other.clone()];
        }
        let other_start_orientation = end.orient(start, other_start);
        let other_end_orientation = end.orient(start, other_end);
        if other_start_orientation != Orientation::Collinear
            && other_end_orientation != Orientation::Collinear
            && other_start_orientation != other_end_orientation
        {
            let start_orientation = other_start.orient(other_end, start);
            let end_orientation = other_start.orient(other_end, end);
            if start_orientation != Orientation::Collinear
                && end_orientation != Orientation::Collinear
                && start_orientation != end_orientation
            {
                let cross_point =
                    IntersectCrossingSegments::intersect_crossing_segments(
                        start,
                        end,
                        other_start,
                        other_end,
                    );
                return vec![
                    Segment::new(start.clone(), cross_point.clone()),
                    Segment::new(cross_point.clone(), end.clone()),
                    Segment::new(other_start.clone(), cross_point.clone()),
                    Segment::new(cross_point, other_end.clone()),
                ];
            }
        } else if other_start_orientation == Orientation::Collinear
            && other_end_orientation == Orientation::Collinear
            && other_start <= end
            && start <= other_end
        {
            return vec![Segment::new(
                start.min(other_start).clone(),
                end.max(other_end).clone(),
            )];
        }
        vec![self.clone(), other.clone()]
    }
}

impl<Scalar> Union<&Contour<Scalar>> for &Segment<Scalar>
where
    Segment<Scalar>: Clone,
    for<'a> &'a Segment<Scalar>:
        Difference<&'a Contour<Scalar>, Output = Vec<Segment<Scalar>>>,
{
    type Output = Vec<Segment<Scalar>>;

    fn union(self, other: &Contour<Scalar>) -> Self::Output {
        let mut result = self.difference(other);
        result.push(self.clone());
        result
    }
}

impl<Scalar> Union<&Multisegment<Scalar>> for &Segment<Scalar>
where
    Segment<Scalar>: Clone,
    for<'a> &'a Segment<Scalar>:
        Difference<&'a Multisegment<Scalar>, Output = Vec<Segment<Scalar>>>,
{
    type Output = Vec<Segment<Scalar>>;

    fn union(self, other: &Multisegment<Scalar>) -> Self::Output {
        let mut result = self.difference(other);
        result.push(self.clone());
        result
    }
}
