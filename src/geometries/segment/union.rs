use crate::bounded::{Bounded, Box};
use crate::geometries::{Contour, Empty, Multisegment, Point};
use crate::operations::{
    do_boxes_have_no_common_continuum, to_sorted_pair,
    IntersectCrossingSegments, Orient,
};
use crate::oriented::Orientation;
use crate::relatable::Relatable;
use crate::traits::{
    Iterable, Lengthsome, Multisegmental, Segmental, Sequence, Union,
};

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
            return vec![self.clone()];
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

impl<Scalar: PartialEq> Union<&Contour<Scalar>> for &Segment<Scalar>
where
    Point<Scalar>: Clone + Ord,
    Segment<Scalar>: Clone,
    for<'a, 'b> &'a Box<&'b Scalar>: Relatable,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Point<Scalar>:
        IntersectCrossingSegments<Output = Point<Scalar>> + Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn union(self, other: &Contour<Scalar>) -> Self::Output {
        let (bounding_box, other_bounding_box) =
            (self.to_bounding_box(), other.to_bounding_box());
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            let mut result = Vec::with_capacity(other.segments().len() + 1);
            result.push(self.clone());
            result.extend(other.segments().iter().cloned());
            return result;
        }
        unite_segment_with_segments(self, other.segments(), &bounding_box)
    }
}

impl<Scalar: PartialEq> Union<&Multisegment<Scalar>> for &Segment<Scalar>
where
    Point<Scalar>: Clone + Ord,
    Segment<Scalar>: Clone,
    for<'a, 'b> &'a Box<&'b Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Point<Scalar>:
        IntersectCrossingSegments<Output = Point<Scalar>> + Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn union(self, other: &Multisegment<Scalar>) -> Self::Output {
        let (bounding_box, other_bounding_box) =
            (self.to_bounding_box(), other.to_bounding_box());
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            let mut result = Vec::with_capacity(other.segments().len() + 1);
            result.push(self.clone());
            result.extend(other.segments().iter().cloned());
            return result;
        }
        unite_segment_with_segments(self, other.segments(), &bounding_box)
    }
}

fn unite_segment_with_segments<
    Scalar: PartialEq,
    Segments: Sequence<IndexItem = Segment<Scalar>>,
>(
    segment: &Segment<Scalar>,
    other_segments: Segments,
    bounding_box: &Box<&Scalar>,
) -> Vec<Segment<Scalar>>
where
    Point<Scalar>: Clone + Ord,
    Segment<Scalar>: Clone,
    for<'a, 'b> &'a Box<&'b Scalar>: Relatable,
    for<'a> &'a Point<Scalar>:
        IntersectCrossingSegments<Output = Point<Scalar>> + Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    let mut result = Vec::with_capacity(other_segments.len());
    let mut break_points = vec![];
    let (start, end) = to_sorted_pair(segment.endpoints());
    for (index, other_segment) in other_segments.iter().enumerate() {
        if other_segment.to_bounding_box().disjoint_with(bounding_box) {
            result.push(other_segment.clone());
            continue;
        }
        let (other_start, other_end) =
            to_sorted_pair(other_segment.endpoints());
        if start == other_start && end == other_end {
            result.extend(other_segments.iter().skip(index + 1).cloned());
            break;
        }
        let other_start_orientation = end.orient(start, other_start);
        let other_end_orientation = end.orient(start, other_end);
        if other_start_orientation == other_end_orientation {
            if other_start_orientation == Orientation::Collinear {
                if start == other_start {
                    if end < other_end {
                        result.push(Segment::new(
                            end.clone(),
                            other_end.clone(),
                        ));
                    }
                    continue;
                } else if end == other_end {
                    if other_start < start {
                        result.push(Segment::new(
                            other_start.clone(),
                            start.clone(),
                        ));
                    }
                    continue;
                } else if start < other_start && other_start < end {
                    if end < other_end {
                        result.push(Segment::new(
                            end.clone(),
                            other_end.clone(),
                        ));
                    }
                    continue;
                } else if other_start < start && start < other_end {
                    result.push(Segment::new(
                        other_start.clone(),
                        start.clone(),
                    ));
                    if end < other_end {
                        result.push(Segment::new(
                            end.clone(),
                            other_end.clone(),
                        ));
                    }
                    continue;
                }
            }
        } else if other_start_orientation == Orientation::Collinear {
            if start < other_start && other_start < end {
                break_points.push(other_start.clone());
            }
        } else if other_end_orientation == Orientation::Collinear {
            if start < other_end && other_end < end {
                break_points.push(other_end.clone());
            }
        } else {
            let start_orientation = other_start.orient(other_end, start);
            let end_orientation = other_start.orient(other_end, end);
            if start_orientation == Orientation::Collinear {
                if other_start < start && start < other_end {
                    result.push(Segment::new(
                        other_start.clone(),
                        start.clone(),
                    ));
                    result
                        .push(Segment::new(start.clone(), other_end.clone()));
                    continue;
                }
            } else if end_orientation == Orientation::Collinear {
                if other_start < end && end < other_end {
                    result
                        .push(Segment::new(other_start.clone(), end.clone()));
                    result.push(Segment::new(end.clone(), other_end.clone()));
                    continue;
                }
            } else if start_orientation != end_orientation {
                let cross_point =
                    IntersectCrossingSegments::intersect_crossing_segments(
                        other_start,
                        other_end,
                        start,
                        end,
                    );
                break_points.push(cross_point.clone());
                result.push(Segment::new(
                    other_start.clone(),
                    cross_point.clone(),
                ));
                result.push(Segment::new(cross_point, other_end.clone()));
                continue;
            }
        }
        result.push(other_segment.clone());
    }
    if !break_points.is_empty() {
        break_points.sort();
        break_points.dedup();
        let mut start = start.clone();
        for end in break_points {
            result.push(Segment::new(start, end.clone()));
            start = end;
        }
        result.push(Segment::new(start, end.clone()));
    } else {
        result.push(segment.clone());
    }
    result
}
