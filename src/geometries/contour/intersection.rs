use crate::bounded::{Bounded, Box};
use crate::clipping::linear::{
    intersect_segment_with_segments, intersect_segments, Operation,
};
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{Event, INTERSECTION};
use crate::geometries::{Empty, Multisegment, Point, Segment};
use crate::operations::{do_boxes_have_no_common_continuum, Orient};
use crate::relatable::Relatable;
use crate::traits::{Intersection, Multisegmental};

use super::types::Contour;

impl<Scalar> Intersection<Empty> for Contour<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for Contour<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar> Intersection<Empty> for &Contour<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for &Contour<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar: Ord> Intersection for &Contour<Scalar>
where
    Operation<Point<Scalar>, INTERSECTION>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn intersection(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            return vec![];
        }
        intersect_segments(
            self.segments(),
            other.segments(),
            bounding_box,
            other_bounding_box,
        )
    }
}

impl<Scalar> Intersection<&Segment<Scalar>> for &Contour<Scalar>
where
    Scalar: PartialEq,
    Point<Scalar>: Clone + Ord,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn intersection(self, other: &Segment<Scalar>) -> Self::Output {
        intersect_segment_with_segments(other, self.segments.iter())
    }
}

impl<Scalar: Ord> Intersection<&Multisegment<Scalar>> for &Contour<Scalar>
where
    Operation<Point<Scalar>, INTERSECTION>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a [&'a Segment<Scalar>], &'a [&'a Segment<Scalar>])>,
    Point<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn intersection(self, other: &Multisegment<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            return vec![];
        }
        intersect_segments(
            self.segments(),
            other.segments(),
            bounding_box,
            other_bounding_box,
        )
    }
}
