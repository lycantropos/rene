use std::ops::{Add, Div, Mul, Sub};

use traiter::numbers::{BitLength, IsPowerOfTwo, One, Sign, Signed};

use crate::bounded;
use crate::constants::MIN_CONTOUR_VERTICES_COUNT;
use crate::locatable::Location;
use crate::oriented::Orientation;
use crate::relatable::Relatable;
use crate::traits::{
    Contoural, Elemental, Iterable, Lengthsome, Multisegmental,
    MultisegmentalIndexSegment, MultivertexalIndexVertex, Polygonal,
    PolygonalContour, PolygonalIndexHole, PolygonalIntoIteratorHole,
    Segmental, SegmentalCoordinate,
};

pub(crate) trait CrossMultiply {
    type Output;

    fn cross_multiply(
        first_start: Self,
        first_end: Self,
        second_start: Self,
        second_end: Self,
    ) -> Self::Output;
}

pub(crate) trait DotMultiply {
    type Output;

    fn dot_multiply(
        first_start: Self,
        first_end: Self,
        second_start: Self,
        second_end: Self,
    ) -> Self::Output;
}

pub(crate) trait IntersectCrossingSegments {
    type Output;

    fn intersect_crossing_segments(
        first_start: Self,
        first_end: Self,
        second_start: Self,
        second_end: Self,
    ) -> Self::Output;
}

impl<Scalar, Point: From<(Scalar, Scalar)>> IntersectCrossingSegments
    for &Point
where
    Scalar: Add<Output = Scalar>
        + Div<Output = Scalar>
        + Mul<Output = Scalar>
        + Sub<Output = Scalar>
        + for<'a> Mul<&'a Scalar, Output = Scalar>,
    for<'a> &'a Scalar: Add<Scalar, Output = Scalar> + Sub<Output = Scalar>,
    for<'a> &'a Point:
        CrossMultiply<Output = Scalar> + Elemental<Coordinate = &'a Scalar>,
{
    type Output = Point;

    fn intersect_crossing_segments(
        first_start: Self,
        first_end: Self,
        second_start: Self,
        second_end: Self,
    ) -> Self::Output {
        let scale = to_segments_intersection_scale(
            first_start,
            first_end,
            second_start,
            second_end,
        );
        Point::from((
            first_start.x() + (first_end.x() - first_start.x()) * &scale,
            first_start.y() + (first_end.y() - first_start.y()) * scale,
        ))
    }
}

pub(crate) trait LocatePointInPointPointPointCircle {
    fn locate_point_in_point_point_point_circle(
        self,
        first: Self,
        second: Self,
        third: Self,
    ) -> Location;
}

pub(crate) trait Orient {
    fn orient(
        self,
        first_ray_point: Self,
        second_ray_point: Self,
    ) -> Orientation;
}

impl<'a, Point> Orient for &'a Point
where
    &'a Point: CrossMultiply,
    <&'a Point as CrossMultiply>::Output: Signed,
{
    fn orient(
        self,
        first_ray_point: Self,
        second_ray_point: Self,
    ) -> Orientation {
        match CrossMultiply::cross_multiply(
            self,
            first_ray_point,
            self,
            second_ray_point,
        )
        .sign()
        {
            Sign::Negative => Orientation::Clockwise,
            Sign::Positive => Orientation::Counterclockwise,
            Sign::Zero => Orientation::Collinear,
        }
    }
}

pub(crate) trait ToCorrectlyOrientedSegments {
    type Output;

    fn to_correctly_oriented_segments(self) -> Self::Output;
}

pub(crate) trait ToReversedSegments {
    type Output;

    fn to_reversed_segments(self) -> Self::Output;
}

pub(crate) trait Square {
    type Output;

    fn square(self) -> Self::Output;
}

pub(crate) trait SquaredMetric<Other = Self> {
    type Output;

    fn squared_distance_to(self, other: Other) -> Self::Output;
}

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

pub(crate) fn do_boxes_have_common_area<'a, Scalar>(
    first: &'a bounded::Box<Scalar>,
    second: &'a bounded::Box<Scalar>,
) -> bool
where
    &'a bounded::Box<Scalar>: Relatable,
{
    !first.disjoint_with(second) && !first.touches(second)
}

pub(crate) fn do_boxes_have_common_continuum<'a, Scalar: PartialEq>(
    first: &'a bounded::Box<Scalar>,
    second: &'a bounded::Box<Scalar>,
) -> bool
where
    &'a bounded::Box<Scalar>: Relatable,
{
    !first.disjoint_with(second)
        && (!first.touches(second)
            || (first.get_min_y() != second.get_max_y()
                && second.get_min_y() != first.get_max_y())
            || (first.get_min_x() != second.get_max_x()
                && second.get_min_x() != first.get_max_x()))
}

pub(crate) fn do_boxes_have_no_common_area<'a, Scalar>(
    first: &'a bounded::Box<Scalar>,
    second: &'a bounded::Box<Scalar>,
) -> bool
where
    &'a bounded::Box<Scalar>: Relatable,
{
    first.disjoint_with(second) || first.touches(second)
}

pub(crate) fn do_boxes_have_no_common_continuum<'a, Scalar: PartialEq>(
    first: &'a bounded::Box<Scalar>,
    second: &'a bounded::Box<Scalar>,
) -> bool
where
    &'a bounded::Box<Scalar>: Relatable,
{
    first.disjoint_with(second)
        || (first.touches(second)
            && (first.get_min_y() == second.get_max_y()
                || second.get_min_y() == first.get_max_y())
            && (first.get_min_x() == second.get_max_x()
                || second.get_min_x() == first.get_max_x()))
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

pub(crate) fn is_point_in_segment<'a, Point: PartialEq>(
    point: &'a Point,
    start: &'a Point,
    end: &'a Point,
) -> bool
where
    &'a Point: Elemental + Orient,
    <&'a Point as Elemental>::Coordinate: PartialOrd,
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

pub(crate) fn locate_point_in_region<
    Border,
    Point: PartialEq,
    Scalar,
    Segment,
>(
    border: &Border,
    point: &Point,
) -> Location
where
    Scalar: PartialOrd,
    for<'a> &'a Border: Multisegmental<IndexSegment = Segment>,
    for<'a> &'a Point: Elemental<Coordinate = &'a Scalar> + Orient,
    for<'a> &'a Segment: Segmental<Endpoint = &'a Point>,
{
    let mut result = false;
    let point_y = point.y();
    for edge in border.segments().iter() {
        let (start, end) = edge.endpoints();
        if is_point_in_segment(point, start, end) {
            return Location::Boundary;
        }
        let start_y = start.y();
        let end_y = end.y();
        if (start_y.gt(point_y)) != (end_y.gt(point_y))
            && ((end_y.gt(start_y))
                == (start.orient(end, point) == Orientation::Counterclockwise))
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

pub(crate) fn merge_bounds<
    Scalar: PartialOrd,
    Iterator: std::iter::Iterator<Item = (Scalar, Scalar, Scalar, Scalar)>,
>(
    mut bounds: Iterator,
) -> (Scalar, Scalar, Scalar, Scalar) {
    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        unsafe { bounds.next().unwrap_unchecked() };
    for (segment_min_x, segment_max_x, segment_min_y, segment_max_y) in bounds
    {
        if min_x.gt(&segment_min_x) {
            min_x = segment_min_x;
        }
        if max_x.lt(&segment_max_x) {
            max_x = segment_max_x;
        }
        if min_y.gt(&segment_min_y) {
            min_y = segment_min_y;
        }
        if max_y.lt(&segment_max_y) {
            max_y = segment_max_y;
        }
    }
    (min_x, max_x, min_y, max_y)
}

pub(crate) fn coordinates_iterator_to_bounds<
    Iterator: std::iter::Iterator<Item = (Scalar, Scalar)>,
    Scalar: PartialOrd,
>(
    mut coordinates: Iterator,
) -> (Scalar, Scalar, Scalar, Scalar) {
    let (first_x, first_y) = unsafe { coordinates.next().unwrap_unchecked() };
    let (second_x, second_y) =
        unsafe { coordinates.next().unwrap_unchecked() };
    let (mut min_x, mut max_x) = to_sorted_pair((first_x, second_x));
    let (mut min_y, mut max_y) = to_sorted_pair((first_y, second_y));
    for (x, y) in coordinates {
        if min_x.gt(&x) {
            min_x = x;
        } else if max_x.lt(&x) {
            max_x = x;
        }
        if min_y.gt(&y) {
            min_y = y;
        } else if max_y.lt(&y) {
            max_y = y;
        }
    }
    (min_x, max_x, min_y, max_y)
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

pub(crate) fn point_vertex_line_divides_angle<'a, Point>(
    point: &'a Point,
    vertex: &'a Point,
    first_ray_point: &'a Point,
    second_ray_point: &'a Point,
) -> bool
where
    &'a Point: Orient,
{
    vertex.orient(first_ray_point, point)
        == vertex.orient(point, second_ray_point)
}

pub(crate) fn segmental_to_bounds<Segment: Segmental>(
    segment: Segment,
) -> (
    SegmentalCoordinate<Segment>,
    SegmentalCoordinate<Segment>,
    SegmentalCoordinate<Segment>,
    SegmentalCoordinate<Segment>,
)
where
    SegmentalCoordinate<Segment>: PartialOrd,
{
    let (start, end) = segment.endpoints();
    let (start_x, start_y) = start.coordinates();
    let (end_x, end_y) = end.coordinates();
    let (min_x, max_x) = to_sorted_pair((start_x, end_x));
    let (min_y, max_y) = to_sorted_pair((start_y, end_y));
    (min_x, max_x, min_y, max_y)
}

pub(crate) fn shrink_collinear_vertices<'a, Point>(
    vertices: &[&'a Point],
) -> Vec<&'a Point>
where
    for<'b> &'b Point: Orient,
{
    debug_assert!(vertices.len() >= MIN_CONTOUR_VERTICES_COUNT);
    let mut result = Vec::with_capacity(vertices.len());
    result.push(vertices[0]);
    for index in 1..vertices.len() - 1 {
        if result[result.len() - 1]
            .orient(vertices[index], vertices[index + 1])
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

pub(crate) fn to_boxes_have_common_area<Scalar>(
    boxes: &[bounded::Box<Scalar>],
    target_box: &bounded::Box<Scalar>,
) -> Vec<bool>
where
    for<'a> &'a bounded::Box<Scalar>: Relatable,
{
    boxes
        .iter()
        .map(|box_| do_boxes_have_common_area(box_, target_box))
        .collect::<Vec<_>>()
}

pub(crate) fn to_boxes_have_common_continuum<Scalar: PartialEq>(
    boxes: &[bounded::Box<Scalar>],
    target_box: &bounded::Box<Scalar>,
) -> Vec<bool>
where
    for<'a> &'a bounded::Box<Scalar>: Relatable,
{
    boxes
        .iter()
        .map(|box_| do_boxes_have_common_continuum(box_, target_box))
        .collect::<Vec<_>>()
}

pub(crate) fn to_arg_min<Value: Ord>(values: &[Value]) -> Option<usize> {
    (0..values.len()).min_by_key(|index| &values[*index])
}

pub(crate) fn to_boxes_ids_with_common_area<Scalar>(
    boxes: &[bounded::Box<Scalar>],
    target_box: &bounded::Box<Scalar>,
) -> Vec<usize>
where
    for<'a> &'a bounded::Box<Scalar>: Relatable,
{
    (0..boxes.len())
        .filter(|&index| do_boxes_have_common_area(&boxes[index], target_box))
        .collect::<Vec<_>>()
}

pub(crate) fn to_boxes_ids_with_common_continuum<Scalar: PartialEq>(
    boxes: &[bounded::Box<Scalar>],
    target_box: &bounded::Box<Scalar>,
) -> Vec<usize>
where
    for<'a> &'a bounded::Box<Scalar>: Relatable,
{
    (0..boxes.len())
        .filter(|&index| {
            do_boxes_have_common_continuum(&boxes[index], target_box)
        })
        .collect::<Vec<_>>()
}

pub(crate) fn to_boxes_ids_with_intersection<Scalar: PartialEq>(
    boxes: &[bounded::Box<Scalar>],
    target_box: &bounded::Box<Scalar>,
) -> Vec<usize>
where
    for<'a> &'a bounded::Box<Scalar>: Relatable,
{
    (0..boxes.len())
        .filter(|&index| !boxes[index].disjoint_with(target_box))
        .collect::<Vec<_>>()
}

pub(crate) trait SegmentsCountable {
    fn segments_count(self) -> usize;
}

impl<Polygon: Polygonal> SegmentsCountable for Polygon
where
    for<'a> &'a MultisegmentalIndexSegment<PolygonalContour<Polygon>>:
        Segmental,
    for<'a> &'a MultivertexalIndexVertex<PolygonalContour<Polygon>>: Elemental,
    for<'a> &'a PolygonalIndexHole<Polygon>: Contoural,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b PolygonalIndexHole<Polygon>>:
        Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b PolygonalIndexHole<Polygon>>:
        Elemental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<PolygonalIntoIteratorHole<Polygon>>:
        Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<PolygonalIntoIteratorHole<Polygon>>:
        Elemental,
    Polygon: Polygonal,
    PolygonalContour<Polygon>: Contoural,
{
    fn segments_count(self) -> usize {
        let (border, holes) = self.components();
        border.segments().len()
            + holes
                .iter()
                .map(|hole| hole.segments().len())
                .sum::<usize>()
    }
}

pub(crate) fn to_segments_intersection_scale<Point, Scalar>(
    first_start: &Point,
    first_end: &Point,
    second_start: &Point,
    second_end: &Point,
) -> Scalar
where
    for<'a> &'a Point:
        CrossMultiply<Output = Scalar> + Elemental<Coordinate = &'a Scalar>,
    Scalar: Div<Output = Scalar>,
{
    CrossMultiply::cross_multiply(
        first_start,
        second_start,
        second_start,
        second_end,
    ) / CrossMultiply::cross_multiply(
        first_start,
        first_end,
        second_start,
        second_end,
    )
}

pub(crate) fn to_sorted_pair<Value: PartialOrd>(
    (left, right): (Value, Value),
) -> (Value, Value) {
    if left < right {
        (left, right)
    } else {
        (right, left)
    }
}
