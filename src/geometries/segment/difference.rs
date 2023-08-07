use crate::bounded::{Bounded, Box};
use crate::clipping::linear::Operation;
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{Event, DIFFERENCE};
use crate::geometries::{Contour, Empty, Multisegment, Point};
use crate::operations::{
    do_boxes_have_no_common_continuum, flags_to_true_indices,
    to_boxes_have_common_continuum, to_sorted_pair, IntersectCrossingSegments,
    Orient,
};
use crate::oriented::Orientation;
use crate::relatable::Relatable;
use crate::sweeping::traits::EventsContainer;
use crate::traits::{Difference, Elemental, Iterable, Multisegmental};

use super::types::Segment;

impl<Scalar> Difference<Empty> for Segment<Scalar> {
    type Output = Self;

    fn difference(self, _other: Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<&Empty> for Segment<Scalar> {
    type Output = Self;

    fn difference(self, _other: &Empty) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<Empty> for &Segment<Scalar>
where
    Segment<Scalar>: Clone,
{
    type Output = Segment<Scalar>;

    fn difference(self, _other: Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> Difference<&Empty> for &Segment<Scalar>
where
    Segment<Scalar>: Clone,
{
    type Output = Segment<Scalar>;

    fn difference(self, _other: &Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar: Ord> Difference for &Segment<Scalar>
where
    Segment<Scalar>: Clone,
    Point<Scalar>: Clone + PartialOrd,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>:
        IntersectCrossingSegments<Output = Point<Scalar>> + Orient,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn difference(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            return vec![self.clone()];
        }

        let (start, end) = to_sorted_pair((&self.start, &self.end));
        let (other_start, other_end) =
            to_sorted_pair((&other.start, &other.end));
        let starts_equal = other_start == start;
        let ends_equal = other_end == end;
        if starts_equal && ends_equal {
            return vec![];
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
                    Segment::new(cross_point, end.clone()),
                ];
            }
        } else if other_start_orientation == Orientation::Collinear
            && other_end_orientation == Orientation::Collinear
            && other_start < end
            && start < other_end
        {
            if starts_equal {
                return if other_end < end {
                    vec![Segment::new(other_end.clone(), end.clone())]
                } else {
                    vec![]
                };
            } else if ends_equal {
                return if other_start < start {
                    vec![]
                } else {
                    vec![Segment::new(other_start.clone(), start.clone())]
                };
            } else if start < other_start {
                return if other_end < end {
                    vec![
                        Segment::new(start.clone(), other_start.clone()),
                        Segment::new(other_end.clone(), end.clone()),
                    ]
                } else {
                    vec![Segment::new(start.clone(), other_start.clone())]
                };
            } else if other_start < start {
                return if end < other_end {
                    vec![]
                } else {
                    vec![Segment::new(other_end.clone(), end.clone())]
                };
            }
        }
        vec![self.clone()]
    }
}

impl<Scalar: Ord> Difference<&Contour<Scalar>> for &Segment<Scalar>
where
    Operation<Point<Scalar>, DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a Segment<Scalar>, &'a [&'a Segment<Scalar>])>,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Contour<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn difference(self, other: &Contour<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            return vec![self.clone()];
        }
        let other_segments = other.segments();
        let other_boxes_have_common_continuum = to_boxes_have_common_continuum(
            &other_segments
                .iter()
                .map(Bounded::to_bounding_box)
                .collect::<Vec<_>>(),
            &bounding_box,
        );
        let other_common_continuum_segments_ids =
            flags_to_true_indices(&other_boxes_have_common_continuum);
        if other_common_continuum_segments_ids.is_empty() {
            return vec![self.clone()];
        }
        let mut operation = Operation::<Point<_>, DIFFERENCE>::from((
            self,
            &other_common_continuum_segments_ids
                .into_iter()
                .map(|index| &other_segments[index])
                .collect::<Vec<_>>(),
        ));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        let max_x = *bounding_box.get_max_x();
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(max_x) {
                break;
            }
            events.push(event);
        }
        operation.reduce_events(events)
    }
}

impl<Scalar: Ord> Difference<&Multisegment<Scalar>> for &Segment<Scalar>
where
    Operation<Point<Scalar>, DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Segment<Scalar>>>
        + for<'a> From<(&'a Segment<Scalar>, &'a [&'a Segment<Scalar>])>,
    Segment<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multisegment<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Segment<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Segment<Scalar>>;

    fn difference(self, other: &Multisegment<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            return vec![self.clone()];
        }
        let other_segments = other.segments();
        let other_boxes_have_common_continuum = to_boxes_have_common_continuum(
            &other_segments
                .iter()
                .map(Bounded::to_bounding_box)
                .collect::<Vec<_>>(),
            &bounding_box,
        );
        let other_common_continuum_segments_ids =
            flags_to_true_indices(&other_boxes_have_common_continuum);
        if other_common_continuum_segments_ids.is_empty() {
            return vec![self.clone()];
        }
        let mut operation = Operation::<Point<_>, DIFFERENCE>::from((
            self,
            &other_common_continuum_segments_ids
                .into_iter()
                .map(|index| &other_segments[index])
                .collect::<Vec<_>>(),
        ));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        let max_x = *bounding_box.get_max_x();
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(max_x) {
                break;
            }
            events.push(event);
        }
        operation.reduce_events(events)
    }
}
