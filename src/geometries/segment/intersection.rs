use crate::bounded::{Bounded, Box};
use crate::geometries::{Contour, Empty, Point};
use crate::operations::{
    do_boxes_have_no_common_continuum, intersect_segment_with_segments,
    intersect_segments_with_common_continuum_bounding_boxes, Orient,
};
use crate::relatable::Relatable;
use crate::traits::{Intersection, Multisegmental, Segmental};

use super::types::Segment;

impl<Scalar> Intersection<Empty> for Segment<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for Segment<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar> Intersection<Empty> for &Segment<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for &Segment<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar> Intersection for &Segment<Scalar>
where
    Scalar: PartialEq,
    Point<Scalar>: Clone + Ord,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Option<Segment<Scalar>>;

    fn intersection(self, other: Self) -> Self::Output {
        if do_boxes_have_no_common_continuum(
            &self.to_bounding_box(),
            &other.to_bounding_box(),
        ) {
            None
        } else {
            let (start, end) = self.endpoints();
            let (other_start, other_end) = other.endpoints();
            intersect_segments_with_common_continuum_bounding_boxes(
                start,
                end,
                other_start,
                other_end,
            )
            .map(|(start, end)| Segment::new(start.clone(), end.clone()))
        }
    }
}

impl<Scalar> Intersection<&Contour<Scalar>> for &Segment<Scalar>
where
    Scalar: PartialEq,
    Point<Scalar>: Clone + Ord,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn intersection(self, other: &Contour<Scalar>) -> Self::Output {
        intersect_segment_with_segments(self, other.segments().into_iter())
    }
}
