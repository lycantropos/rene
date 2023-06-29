use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::bounded::{Bounded, Box};
use crate::geometries::{Empty, Point};
use crate::operations::{
    do_boxes_have_no_common_continuum,
    intersect_segments_with_common_continuum_bounding_boxes, Orient,
};
use crate::relatable::Relatable;
use crate::traits::{Intersection, Segmental};

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

impl<Digit, const SHIFT: usize> Intersection
    for &Segment<Fraction<BigInt<Digit, SHIFT>>>
where
    Fraction<BigInt<Digit, SHIFT>>: PartialEq,
    Point<Fraction<BigInt<Digit, SHIFT>>>: Clone + Ord,
    for<'a> &'a Box<&'a Fraction<BigInt<Digit, SHIFT>>>: Relatable,
    for<'a> &'a Point<Fraction<BigInt<Digit, SHIFT>>>: Orient,
    for<'a> &'a Segment<Fraction<BigInt<Digit, SHIFT>>>:
        Bounded<&'a Fraction<BigInt<Digit, SHIFT>>>,
{
    type Output = Option<Segment<Fraction<BigInt<Digit, SHIFT>>>>;

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
