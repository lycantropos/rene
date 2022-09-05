use std::ops::{Add, Div, Mul, Sub};

use rithm::big_int::BigInt;
use rithm::fraction::Fraction;
use traiter::numbers::{BitLength, IsPowerOfTwo, Sign, Signed, Unitary};

use crate::bounded;
use crate::constants::MIN_CONTOUR_VERTICES_COUNT;
use crate::locatable::Location;
use crate::oriented::Orientation;
use crate::relatable::{Relatable, Relation};
use crate::traits::Elemental;

pub(crate) fn boxes_ids_coupled_with_box<Scalar>(
    boxes: &[bounded::Box<Scalar>],
    target_box: &bounded::Box<Scalar>,
) -> Vec<usize>
where
    for<'a> &'a bounded::Box<Scalar>: Relatable,
{
    (0..boxes.len())
        .filter(|&index| {
            let box_ = &boxes[index];
            !box_.disjoint_with(&target_box) && !box_.touches(&target_box)
        })
        .collect::<Vec<_>>()
}

pub(crate) trait CrossMultiply {
    type Output;

    fn cross_multiply(
        first_start: &Self,
        first_end: &Self,
        second_start: &Self,
        second_end: &Self,
    ) -> Self::Output;
}

impl<
        Digit,
        const SEPARATOR: char,
        const SHIFT: usize,
        Point: Elemental<Coordinate = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    > CrossMultiply for Point
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
    <Self as Elemental>::Coordinate: Mul<Output = <Self as Elemental>::Coordinate>
        + Sub<Output = <Self as Elemental>::Coordinate>,
{
    type Output = <Self as Elemental>::Coordinate;

    fn cross_multiply(
        first_start: &Self,
        first_end: &Self,
        second_start: &Self,
        second_end: &Self,
    ) -> Self::Output {
        (first_end.x() - first_start.x()) * (second_end.y() - second_start.y())
            - (first_end.y() - first_start.y()) * (second_end.x() - second_start.x())
    }
}

pub(crate) trait Orient {
    fn orient(&self, first_ray_point: &Self, second_ray_point: &Self) -> Orientation;
}

impl<Point: CrossMultiply> Orient for Point
where
    <Self as CrossMultiply>::Output: Signed,
{
    fn orient(&self, first_ray_point: &Self, second_ray_point: &Self) -> Orientation {
        match Self::cross_multiply(self, first_ray_point, self, second_ray_point).sign() {
            Sign::Negative => Orientation::Clockwise,
            Sign::Positive => Orientation::Counterclockwise,
            Sign::Zero => Orientation::Collinear,
        }
    }
}

pub(crate) fn segment_in_segment<Point: Orient + PartialOrd>(
    first_start: &Point,
    first_end: &Point,
    second_start: &Point,
    second_end: &Point,
) -> Relation {
    let (first_start, first_end) = to_sorted_pair((first_start, first_end));
    let (second_start, second_end) = to_sorted_pair((second_start, second_end));
    let starts_equal = second_start == first_start;
    let ends_equal = second_end == first_end;
    if starts_equal && ends_equal {
        return Relation::Equal;
    }
    let second_start_orientation = first_end.orient(first_start, second_start);
    let second_end_orientation = first_end.orient(first_start, second_end);
    if second_start_orientation != Orientation::Collinear
        && second_end_orientation != Orientation::Collinear
    {
        if second_start_orientation == second_end_orientation {
            Relation::Disjoint
        } else {
            let first_start_orientation = second_start.orient(second_end, first_start);
            let first_end_orientation = second_start.orient(second_end, first_end);
            if first_start_orientation != Orientation::Collinear
                && first_end_orientation != Orientation::Collinear
            {
                if first_start_orientation == first_end_orientation {
                    Relation::Disjoint
                } else {
                    Relation::Cross
                }
            } else if first_start_orientation != Orientation::Collinear {
                if second_start < first_end && first_end < second_end {
                    Relation::Touch
                } else {
                    Relation::Disjoint
                }
            } else if second_start < first_start && first_start < second_end {
                Relation::Touch
            } else {
                Relation::Disjoint
            }
        }
    } else if second_start_orientation != Orientation::Collinear {
        if first_start <= second_end && second_end <= first_end {
            Relation::Touch
        } else {
            Relation::Disjoint
        }
    } else if second_end_orientation != Orientation::Collinear {
        if first_start <= second_start && second_start <= first_end {
            Relation::Touch
        } else {
            Relation::Disjoint
        }
    } else if starts_equal {
        if second_end < first_end {
            Relation::Composite
        } else {
            Relation::Component
        }
    } else if ends_equal {
        if second_start < first_start {
            Relation::Component
        } else {
            Relation::Composite
        }
    } else if second_start == first_end || second_end == first_start {
        Relation::Touch
    } else if first_start < second_start && second_start < first_end {
        if second_end < first_end {
            Relation::Composite
        } else {
            Relation::Overlap
        }
    } else if second_start < first_start && first_start < second_end {
        if first_end < second_end {
            Relation::Component
        } else {
            Relation::Overlap
        }
    } else {
        Relation::Disjoint
    }
}

pub(crate) trait IntersectCrossingSegments {
    fn intersect_crossing_segments(
        first_start: &Self,
        first_end: &Self,
        second_start: &Self,
        second_end: &Self,
    ) -> Self;
}

impl<
        Digit,
        const SEPARATOR: char,
        const SHIFT: usize,
        Point: CrossMultiply<Output = <Point as Elemental>::Coordinate>
            + From<(
                <Point as Elemental>::Coordinate,
                <Point as Elemental>::Coordinate,
            )> + Elemental<Coordinate = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    > IntersectCrossingSegments for Point
where
    for<'a> <Point as Elemental>::Coordinate: Add<Output = <Point as Elemental>::Coordinate>
        + Div<Output = <Point as Elemental>::Coordinate>
        + Mul<&'a <Point as Elemental>::Coordinate, Output = <Point as Elemental>::Coordinate>
        + Mul<Output = <Point as Elemental>::Coordinate>
        + Sub<Output = <Point as Elemental>::Coordinate>,
{
    fn intersect_crossing_segments(
        first_start: &Self,
        first_end: &Self,
        second_start: &Self,
        second_end: &Self,
    ) -> Self {
        let scale = Self::cross_multiply(first_start, second_start, second_start, second_end)
            / Self::cross_multiply(first_start, first_end, second_start, second_end);
        Point::from((
            first_start.x() + (first_end.x() - first_start.x()) * &scale,
            first_start.y() + (first_end.y() - first_start.y()) * scale,
        ))
    }
}

pub(crate) trait LocatePointInPointPointPointCircle {
    fn locate_point_in_point_point_point_circle(
        &self,
        first: &Self,
        second: &Self,
        third: &Self,
    ) -> Location;
}

impl<
        Digit,
        const SEPARATOR: char,
        const SHIFT: usize,
        Point: Elemental<Coordinate = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    > LocatePointInPointPointPointCircle for Point
where
    for<'a> &'a <Point as Elemental>::Coordinate: Mul<Output = <Point as Elemental>::Coordinate>,
    <Point as Elemental>::Coordinate: Add<Output = <Point as Elemental>::Coordinate>
        + Mul<Output = <Point as Elemental>::Coordinate>
        + Signed
        + Sub<Output = <Point as Elemental>::Coordinate>,
{
    fn locate_point_in_point_point_point_circle(
        &self,
        first: &Self,
        second: &Self,
        third: &Self,
    ) -> Location {
        let (first_dx, first_dy) = (first.x() - self.x(), first.y() - self.y());
        let (second_dx, second_dy) = (second.x() - self.x(), second.y() - self.y());
        let (third_dx, third_dy) = (third.x() - self.x(), third.y() - self.y());
        match ((&first_dx * &first_dx + &first_dy * &first_dy)
            * (&second_dx * &third_dy - &second_dy * &third_dx)
            - (&second_dx * &second_dx + &second_dy * &second_dy)
                * (&first_dx * &third_dy - &first_dy * &third_dx)
            + (&third_dx * &third_dx + &third_dy * &third_dy)
                * (first_dx * second_dy - first_dy * second_dx))
            .sign()
        {
            Sign::Negative => Location::Exterior,
            Sign::Positive => Location::Interior,
            Sign::Zero => Location::Boundary,
        }
    }
}

pub(crate) fn merge_boxes<Scalar: Clone + PartialOrd>(
    boxes: &[bounded::Box<Scalar>],
) -> bounded::Box<Scalar> {
    debug_assert!(!boxes.is_empty());
    let first_box = &boxes[0];
    let mut max_x = first_box.get_max_x();
    let mut max_y = first_box.get_max_y();
    let mut min_x = first_box.get_min_x();
    let mut min_y = first_box.get_min_y();
    for box_ in &boxes[1..] {
        if box_.get_max_x() < max_x {
            max_x = box_.get_max_x();
        }
        if box_.get_max_y() < max_y {
            max_y = box_.get_max_y();
        }
        if box_.get_min_x() < min_x {
            min_x = box_.get_min_x();
        }
        if box_.get_min_y() < min_y {
            min_y = box_.get_min_y();
        }
    }
    bounded::Box::new(min_x.clone(), max_x.clone(), min_y.clone(), max_y.clone())
}

pub(crate) fn to_arg_min<Value: Ord>(values: &[Value]) -> Option<usize> {
    (0..values.len()).min_by_key(|index| &values[*index])
}

pub(crate) fn to_sorted_pair<Value: PartialOrd>((left, right): (Value, Value)) -> (Value, Value) {
    if left < right {
        (left, right)
    } else {
        (right, left)
    }
}

pub(crate) fn shrink_collinear_vertices<'a, Point: Orient>(
    vertices: &[&'a Point],
) -> Vec<&'a Point> {
    debug_assert!(vertices.len() >= MIN_CONTOUR_VERTICES_COUNT);
    let mut result = Vec::with_capacity(vertices.len());
    result.push(vertices[0]);
    for index in 1..vertices.len() - 1 {
        if result[result.len() - 1].orient(vertices[index], vertices[index + 1])
            != Orientation::Collinear
        {
            result.push(vertices[index])
        }
    }
    if result[result.len() - 1].orient(vertices[vertices.len() - 1], result[0])
        != Orientation::Collinear
    {
        result.push(vertices[vertices.len() - 1])
    }
    result
}

pub(crate) fn ceil_log2<
    Number: Copy + BitLength<Output = Value> + IsPowerOfTwo,
    Value: Sub<Output = Value> + Unitary,
>(
    number: Number,
) -> Value {
    if number.is_power_of_two() {
        number.bit_length() - <Number as BitLength>::Output::one()
    } else {
        number.bit_length()
    }
}
