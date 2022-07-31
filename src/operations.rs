use rithm::traits::{
    AdditiveGroup, BitLength, DivisivePartialMagma, IsPowerOfTwo, MultiplicativeMonoid, Sign,
    Signed, SubtractiveMagma,
};

use crate::constants::MIN_CONTOUR_VERTICES_COUNT;
use crate::locatable::Location;
use crate::oriented::Orientation;
use crate::relatable::Relation;
use crate::traits;

pub(crate) fn cross_multiply<
    Scalar: AdditiveGroup + MultiplicativeMonoid,
    Point: traits::Point<Coordinate = Scalar>,
>(
    first_start: &Point,
    first_end: &Point,
    second_start: &Point,
    second_end: &Point,
) -> Scalar {
    (first_end.x() - first_start.x()) * (second_end.y() - second_start.y())
        - (first_end.y() - first_start.y()) * (second_end.x() - second_start.x())
}

pub(crate) fn orient<
    Scalar: AdditiveGroup + MultiplicativeMonoid + Signed,
    Point: traits::Point<Coordinate = Scalar>,
>(
    vertex: &Point,
    first_ray_point: &Point,
    second_ray_point: &Point,
) -> Orientation {
    match cross_multiply::<Scalar, Point>(vertex, first_ray_point, vertex, second_ray_point).sign()
    {
        Sign::Negative => Orientation::Clockwise,
        Sign::Positive => Orientation::Counterclockwise,
        Sign::Zero => Orientation::Collinear,
    }
}

pub(crate) fn relate_segments<
    Scalar: AdditiveGroup + MultiplicativeMonoid + Signed,
    Point: PartialOrd + traits::Point<Coordinate = Scalar>,
>(
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
    let test_start_orientation = orient(goal_end, goal_start, test_start);
    let test_end_orientation = orient(goal_end, goal_start, test_end);
    if test_start_orientation != Orientation::Collinear
        && test_end_orientation != Orientation::Collinear
    {
        if test_start_orientation == test_end_orientation {
            Relation::Disjoint
        } else {
            let goal_start_orientation = orient(test_start, test_end, goal_start);
            let goal_end_orientation = orient(test_start, test_end, goal_end);
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

pub(crate) fn intersect_crossing_segments<
    Scalar: AdditiveGroup + Clone + DivisivePartialMagma + MultiplicativeMonoid,
    Point: From<(Scalar, Scalar)> + traits::Point<Coordinate = Scalar>,
>(
    first_start: &Point,
    first_end: &Point,
    second_start: &Point,
    second_end: &Point,
) -> Point {
    let scale = cross_multiply(first_start, second_start, second_start, second_end)
        / cross_multiply(first_start, first_end, second_start, second_end);
    Point::from((
        first_start.x() + (first_end.x() - first_start.x()) * scale.clone(),
        first_start.y() + (first_end.y() - first_start.y()) * scale,
    ))
}

pub(crate) fn locate_point_in_point_point_point_circle<
    Scalar: AdditiveGroup + Clone + MultiplicativeMonoid + Signed,
    Point: traits::Point<Coordinate = Scalar>,
>(
    point: &Point,
    first: &Point,
    second: &Point,
    third: &Point,
) -> Location {
    let (first_dx, first_dy) = (first.x() - point.x(), first.y() - point.y());
    let (second_dx, second_dy) = (second.x() - point.x(), second.y() - point.y());
    let (third_dx, third_dy) = (third.x() - point.x(), third.y() - point.y());
    match ((first_dx.clone() * first_dx.clone() + first_dy.clone() * first_dy.clone())
        * (second_dx.clone() * third_dy.clone() - second_dy.clone() * third_dx.clone())
        - (second_dx.clone() * second_dx.clone() + second_dy.clone() * second_dy.clone())
            * (first_dx.clone() * third_dy.clone() - first_dy.clone() * third_dx.clone())
        + (third_dx.clone() * third_dx + third_dy.clone() * third_dy)
            * (first_dx * second_dy - first_dy * second_dx))
        .sign()
    {
        Sign::Negative => Location::Exterior,
        Sign::Positive => Location::Interior,
        Sign::Zero => Location::Boundary,
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

pub(crate) fn shrink_collinear_vertices<
    'a,
    Scalar: AdditiveGroup + MultiplicativeMonoid + Signed,
    Point: traits::Point<Coordinate = Scalar>,
>(
    vertices: &[&'a Point],
) -> Vec<&'a Point> {
    debug_assert!(vertices.len() >= MIN_CONTOUR_VERTICES_COUNT);
    let mut result = Vec::with_capacity(vertices.len());
    result.push(vertices[0]);
    for index in 1..vertices.len() - 1 {
        if !matches!(
            orient(
                result[result.len() - 1],
                vertices[index],
                vertices[index + 1]
            ),
            Orientation::Collinear
        ) {
            result.push(vertices[index])
        }
    }
    if !matches!(
        orient(
            result[result.len() - 1],
            vertices[vertices.len() - 1],
            result[0]
        ),
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
    Value: SubtractiveMagma + From<bool>,
>(
    number: Number,
) -> Value {
    number.bit_length() - <Number as BitLength>::Output::from(number.is_power_of_two())
}
