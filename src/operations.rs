use std::ops::{Add, Div, Mul, Sub};

use rithm::big_int::BigInt;
use rithm::fraction::Fraction;
use traiter::numbers::{BitLength, IsPowerOfTwo, Sign, Signed, Unitary};

use crate::constants::MIN_CONTOUR_VERTICES_COUNT;
use crate::locatable::Location;
use crate::oriented::Orientation;
use crate::relatable::Relation;
use crate::traits;

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
        Point: traits::Point<Coordinate = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    > CrossMultiply for Point
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
    <Self as traits::Point>::Coordinate: Mul<Output = <Self as traits::Point>::Coordinate>
        + Sub<Output = <Self as traits::Point>::Coordinate>,
{
    type Output = <Self as traits::Point>::Coordinate;

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

pub(crate) fn relate_segments<Point: Orient + PartialOrd>(
    goal_start: &Point,
    goal_end: &Point,
    test_start: &Point,
    test_end: &Point,
) -> Relation {
    let (goal_start, goal_end) = to_sorted_pair((goal_start, goal_end));
    let (test_start, test_end) = to_sorted_pair((test_start, test_end));
    let starts_equal = test_start == goal_start;
    let ends_equal = test_end == goal_end;
    if starts_equal && ends_equal {
        return Relation::Equal;
    }
    let test_start_orientation = goal_end.orient(goal_start, test_start);
    let test_end_orientation = goal_end.orient(goal_start, test_end);
    if test_start_orientation != Orientation::Collinear
        && test_end_orientation != Orientation::Collinear
    {
        if test_start_orientation == test_end_orientation {
            Relation::Disjoint
        } else {
            let goal_start_orientation = test_start.orient(test_end, goal_start);
            let goal_end_orientation = test_start.orient(test_end, goal_end);
            if goal_start_orientation != Orientation::Collinear
                && goal_end_orientation != Orientation::Collinear
            {
                if goal_start_orientation == goal_end_orientation {
                    Relation::Disjoint
                } else {
                    Relation::Cross
                }
            } else if goal_start_orientation != Orientation::Collinear {
                if test_start < goal_end && goal_end < test_end {
                    Relation::Touch
                } else {
                    Relation::Disjoint
                }
            } else if test_start < goal_start && goal_start < test_end {
                Relation::Touch
            } else {
                Relation::Disjoint
            }
        }
    } else if test_start_orientation != Orientation::Collinear {
        if goal_start <= test_end && test_end <= goal_end {
            Relation::Touch
        } else {
            Relation::Disjoint
        }
    } else if test_end_orientation != Orientation::Collinear {
        if goal_start <= test_start && test_start <= goal_end {
            Relation::Touch
        } else {
            Relation::Disjoint
        }
    } else if starts_equal {
        if test_end < goal_end {
            Relation::Composite
        } else {
            Relation::Component
        }
    } else if ends_equal {
        if test_start < goal_start {
            Relation::Component
        } else {
            Relation::Composite
        }
    } else if test_start == goal_end || test_end == goal_start {
        Relation::Touch
    } else if goal_start < test_start && test_start < goal_end {
        if test_end < goal_end {
            Relation::Composite
        } else {
            Relation::Overlap
        }
    } else if test_start < goal_start && goal_start < test_end {
        if goal_end < test_end {
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
        Point: CrossMultiply<Output = <Point as traits::Point>::Coordinate>
            + From<(
                <Point as traits::Point>::Coordinate,
                <Point as traits::Point>::Coordinate,
            )> + traits::Point<Coordinate = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    > IntersectCrossingSegments for Point
where
    for<'a> <Point as traits::Point>::Coordinate: Add<Output = <Point as traits::Point>::Coordinate>
        + Div<Output = <Point as traits::Point>::Coordinate>
        + Mul<&'a <Point as traits::Point>::Coordinate, Output = <Point as traits::Point>::Coordinate>
        + Mul<Output = <Point as traits::Point>::Coordinate>
        + Sub<Output = <Point as traits::Point>::Coordinate>,
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
        Point: traits::Point<Coordinate = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>,
    > LocatePointInPointPointPointCircle for Point
where
    for<'a> &'a <Point as traits::Point>::Coordinate:
        Mul<Output = <Point as traits::Point>::Coordinate>,
    <Point as traits::Point>::Coordinate: Add<Output = <Point as traits::Point>::Coordinate>
        + Mul<Output = <Point as traits::Point>::Coordinate>
        + Signed
        + Sub<Output = <Point as traits::Point>::Coordinate>,
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
        if !matches!(
            result[result.len() - 1].orient(vertices[index], vertices[index + 1]),
            Orientation::Collinear
        ) {
            result.push(vertices[index])
        }
    }
    if !matches!(
        result[result.len() - 1].orient(vertices[vertices.len() - 1], result[0]),
        Orientation::Collinear
    ) {
        result.push(vertices[vertices.len() - 1])
    } else if result.len() > 2 {
        result[0] = unsafe { result.pop().unwrap_unchecked() }
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
