use std::ops::{Add, Div, Mul, Sub};

use rithm::big_int::BigInt;
use rithm::fraction::Fraction;
use traiter::numbers::{BitLength, IsPowerOfTwo, One, Sign, Signed};

use crate::bounded::Box;
use crate::constants::MIN_CONTOUR_VERTICES_COUNT;
use crate::locatable::Location;
use crate::oriented::Orientation;
use crate::relatable::Relatable;
use crate::traits::{Elemental, Multisegmental, MultisegmentalSegment, Segmental};

pub(crate) fn ceil_log2<
    Number: Copy + BitLength<Output = Value> + IsPowerOfTwo,
    Value: Sub<Output = Value> + One,
>(
    number: Number,
) -> Value {
    if number.is_power_of_two() {
        number.bit_length() - <Number as BitLength>::Output::one()
    } else {
        number.bit_length()
    }
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

impl<Digit, const SHIFT: usize, Point: Elemental<Coordinate = Fraction<BigInt<Digit, SHIFT>>>>
    CrossMultiply for Point
where
    BigInt<Digit, SHIFT>: Clone,
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

pub(crate) fn do_boxes_have_common_area<Scalar>(first: &Box<Scalar>, second: &Box<Scalar>) -> bool
where
    for<'a> &'a Box<Scalar>: Relatable,
{
    !first.disjoint_with(second) && !first.touches(second)
}

pub(crate) fn do_boxes_have_common_continuum<Scalar: PartialEq>(
    first: &Box<Scalar>,
    second: &Box<Scalar>,
) -> bool
where
    for<'a> &'a Box<Scalar>: Relatable,
{
    !first.disjoint_with(second)
        && (!first.touches(second)
            || (first.get_min_y() != second.get_max_y() && second.get_min_y() != first.get_max_y())
            || (first.get_min_x() != second.get_max_x() && second.get_min_x() != first.get_max_x()))
}

pub(crate) fn do_boxes_have_no_common_area<Scalar>(
    first: &Box<Scalar>,
    second: &Box<Scalar>,
) -> bool
where
    for<'a> &'a Box<Scalar>: Relatable,
{
    first.disjoint_with(second) || first.touches(second)
}

pub(crate) fn do_boxes_have_no_common_continuum<Scalar: PartialEq>(
    first: &Box<Scalar>,
    second: &Box<Scalar>,
) -> bool
where
    for<'a> &'a Box<Scalar>: Relatable,
{
    first.disjoint_with(second)
        || (first.touches(second)
            && (first.get_min_y() == second.get_max_y() || second.get_min_y() == first.get_max_y())
            && (first.get_min_x() != second.get_max_x() || second.get_min_x() == first.get_max_x()))
}

pub(crate) fn flags_to_false_indices(flags: &[bool]) -> Vec<usize> {
    flags
        .iter()
        .enumerate()
        .filter(|(_, &flag)| !flag)
        .map(|(index, _)| index)
        .collect::<Vec<_>>()
}

pub(crate) fn flags_to_true_indices(flags: &[bool]) -> Vec<usize> {
    flags
        .iter()
        .enumerate()
        .filter(|(_, &flag)| flag)
        .map(|(index, _)| index)
        .collect::<Vec<_>>()
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
        const SHIFT: usize,
        Point: CrossMultiply<Output = <Point as Elemental>::Coordinate>
            + From<(
                <Point as Elemental>::Coordinate,
                <Point as Elemental>::Coordinate,
            )> + Elemental<Coordinate = Fraction<BigInt<Digit, SHIFT>>>,
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

pub(crate) fn is_point_in_segment<Point: Elemental + Orient + PartialEq>(
    point: &Point,
    start: &Point,
    end: &Point,
) -> bool
where
    <Point as Elemental>::Coordinate: PartialOrd,
{
    start.eq(point)
        || end.eq(point)
        || ({
            let start_x = start.x();
            let end_x = end.x();
            let point_x = point.x();
            if start_x <= end_x {
                start_x <= point_x && point_x <= end_x
            } else {
                end_x < point_x && point_x < start_x
            }
        } && {
            let start_y = start.y();
            let end_y = end.y();
            let point_y = point.y();
            if start_y <= end_y {
                start_y <= point_y && point_y <= end_y
            } else {
                end_y < point_y && point_y < start_y
            }
        } && start.orient(end, point) == Orientation::Collinear)
}

pub(crate) trait LocatePointInPointPointPointCircle {
    fn locate_point_in_point_point_point_circle(
        &self,
        first: &Self,
        second: &Self,
        third: &Self,
    ) -> Location;
}

impl<Digit, const SHIFT: usize, Point: Elemental<Coordinate = Fraction<BigInt<Digit, SHIFT>>>>
    LocatePointInPointPointPointCircle for Point
where
    for<'a> &'a <Point as Elemental>::Coordinate:
        Mul<Output = <Point as Elemental>::Coordinate> + Signed,
    <Point as Elemental>::Coordinate: Add<Output = <Point as Elemental>::Coordinate>
        + Mul<Output = <Point as Elemental>::Coordinate>
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

pub(crate) fn locate_point_in_region<'a, Border, Point: Elemental + Orient + PartialEq>(
    border: &'a Border,
    point: &Point,
) -> Location
where
    &'a Border: Multisegmental,
    MultisegmentalSegment<&'a Border>: Segmental<Endpoint = Point>,
    <Point as Elemental>::Coordinate: PartialOrd,
{
    let mut result = false;
    let point_y = point.y();
    for edge in border.segments() {
        let (start, end) = edge.endpoints();
        if is_point_in_segment(point, &start, &end) {
            return Location::Boundary;
        }
        if (start.y() > point_y) != (end.y() > point_y)
            && ((end.y() > start.y())
                == (start.orient(&end, point) == Orientation::Counterclockwise))
        {
            result = !result;
        }
    }
    if result {
        Location::Interior
    } else {
        Location::Exterior
    }
}

pub(crate) fn merge_boxes<Scalar: Clone + PartialOrd>(boxes: &[Box<Scalar>]) -> Box<Scalar> {
    debug_assert!(!boxes.is_empty());
    let first_box = &boxes[0];
    let mut max_x = first_box.get_max_x();
    let mut max_y = first_box.get_max_y();
    let mut min_x = first_box.get_min_x();
    let mut min_y = first_box.get_min_y();
    for box_ in &boxes[1..] {
        if box_.get_max_x() > max_x {
            max_x = box_.get_max_x();
        }
        if box_.get_max_y() > max_y {
            max_y = box_.get_max_y();
        }
        if box_.get_min_x() < min_x {
            min_x = box_.get_min_x();
        }
        if box_.get_min_y() < min_y {
            min_y = box_.get_min_y();
        }
    }
    Box::new(min_x.clone(), max_x.clone(), min_y.clone(), max_y.clone())
}

pub(crate) trait Orient {
    fn orient(&self, first_ray_point: &Self, second_ray_point: &Self) -> Orientation;
}

impl<Point: CrossMultiply> Orient for Point
where
    for<'a> &'a <Self as CrossMultiply>::Output: Signed,
{
    fn orient(&self, first_ray_point: &Self, second_ray_point: &Self) -> Orientation {
        match Self::cross_multiply(self, first_ray_point, self, second_ray_point).sign() {
            Sign::Negative => Orientation::Clockwise,
            Sign::Positive => Orientation::Counterclockwise,
            Sign::Zero => Orientation::Collinear,
        }
    }
}

/// Based on "Ranking and unranking permutations in linear time"
/// by W. Myrvold, F. Ruskey
///
/// Time complexity: O(values.len())
/// Memory complexity: O(1)
///
/// More at: http://webhome.cs.uvic.ca/~ruskey/Publications/RankPerm/MyrvoldRuskey.pdf
pub(crate) fn permute<T>(values: &mut [T], mut seed: usize) {
    for step in (1..=values.len()).rev() {
        values.swap(step - 1, seed % step);
        seed /= step;
    }
}

pub(crate) fn point_vertex_line_divides_angle<Point: Orient>(
    point: &Point,
    vertex: &Point,
    first_ray_point: &Point,
    second_ray_point: &Point,
) -> bool {
    vertex.orient(first_ray_point, point) == vertex.orient(point, second_ray_point)
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
            result.push(vertices[index]);
        }
    }
    if result[result.len() - 1].orient(vertices[vertices.len() - 1], result[0])
        != Orientation::Collinear
    {
        result.push(vertices[vertices.len() - 1]);
    }
    result
}

pub(crate) fn to_boxes_have_common_area_with_box<Scalar>(
    boxes: &[Box<Scalar>],
    target_box: &Box<Scalar>,
) -> Vec<bool>
where
    for<'a> &'a Box<Scalar>: Relatable,
{
    boxes
        .iter()
        .map(|box_| do_boxes_have_common_area(box_, target_box))
        .collect::<Vec<_>>()
}

pub(crate) fn to_boxes_have_common_continuum_with_box<Scalar: PartialEq>(
    boxes: &[Box<Scalar>],
    target_box: &Box<Scalar>,
) -> Vec<bool>
where
    for<'a> &'a Box<Scalar>: Relatable,
{
    boxes
        .iter()
        .map(|box_| do_boxes_have_common_continuum(box_, target_box))
        .collect::<Vec<_>>()
}

pub(crate) fn to_arg_min<Value: Ord>(values: &[Value]) -> Option<usize> {
    (0..values.len()).min_by_key(|index| &values[*index])
}

pub(crate) fn to_boxes_ids_with_common_area_with_box<Scalar>(
    boxes: &[Box<Scalar>],
    target_box: &Box<Scalar>,
) -> Vec<usize>
where
    for<'a> &'a Box<Scalar>: Relatable,
{
    (0..boxes.len())
        .filter(|&index| do_boxes_have_common_area(&boxes[index], target_box))
        .collect::<Vec<_>>()
}

#[inline]
pub(crate) fn to_sorted_pair<Value: PartialOrd>((left, right): (Value, Value)) -> (Value, Value) {
    if left < right {
        (left, right)
    } else {
        (right, left)
    }
}
