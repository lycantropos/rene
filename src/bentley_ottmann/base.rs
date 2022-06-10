use core::convert::From;

use rithm::traits::{AdditiveGroup, DivisivePartialMagma, MultiplicativeMonoid, Signed};

use crate::relatable::Relation;
use crate::traits::{Contour, Point, Segment};

use super::event::is_left_event;
use super::events_registry::EventsRegistry;
use super::sweep::Sweep;

pub(crate) fn is_contour_valid<
    Scalar: AdditiveGroup + Clone + DivisivePartialMagma + MultiplicativeMonoid + Ord + Signed,
    Endpoint: Clone + From<(Scalar, Scalar)> + Ord + self::Point<Scalar>,
    Segment: self::Segment<Scalar, Point = Endpoint>,
    Contour: self::Contour<Scalar, Point = Endpoint, Segment = Segment>,
>(
    contour: &Contour,
) -> bool {
    let segments = contour.segments();
    segments.len() >= 3 && {
        let mut sweep = Sweep::from(segments.as_slice());
        let intersection = unsafe { sweep.next().unwrap_unchecked() };
        matches!(intersection.relation, Relation::Touch) && {
            let mut segment_id = intersection.first_segment_id;
            let mut has_second_tangent = false;
            while let Some(intersection) = sweep.next() {
                if !matches!(intersection.relation, Relation::Touch) {
                    return false;
                } else if intersection.first_segment_id == segment_id {
                    if has_second_tangent {
                        return false;
                    }
                    has_second_tangent = true;
                } else {
                    debug_assert_ne!(intersection.second_segment_id, segment_id);
                    if !has_second_tangent {
                        return false;
                    }
                    segment_id = intersection.first_segment_id;
                    has_second_tangent = false;
                }
            }
            true
        }
    }
}

pub(crate) fn to_unique_non_crossing_or_overlapping_segments<
    Scalar: AdditiveGroup + Clone + DivisivePartialMagma + MultiplicativeMonoid + Ord + Signed,
    Endpoint: Clone + From<(Scalar, Scalar)> + Ord + self::Point<Scalar>,
    Segment: From<(Endpoint, Endpoint)> + self::Segment<Scalar, Point = Endpoint>,
>(
    segments: &[Segment],
) -> Vec<Segment> {
    let mut result = Vec::with_capacity(segments.len());
    let mut events_registry = EventsRegistry::<Scalar, Endpoint, true>::from(segments);
    while let Some(event) = events_registry.next() {
        if !is_left_event(event) {
            result.push(Segment::from((
                events_registry.get_event_start(event).clone(),
                events_registry.get_event_end(event).clone(),
            )))
        }
    }
    result
}
