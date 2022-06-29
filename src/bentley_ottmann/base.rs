use core::convert::From;

use rithm::traits::{AdditiveGroup, DivisivePartialMagma, MultiplicativeMonoid, Signed};

use crate::constants::{MIN_CONTOUR_VERTICES_COUNT, MIN_MULTISEGMENT_SEGMENTS_COUNT};
use crate::contracts::are_contour_vertices_non_degenerate;
use crate::relatable::Relation;
use crate::traits::{Contour, Multisegment, Point, Segment};

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
    are_contour_vertices_non_degenerate(&contour.vertices()) && {
        let segments = contour.segments();
        segments.len() >= MIN_CONTOUR_VERTICES_COUNT
            && segments
                .iter()
                .all(|segment| segment.start() != segment.end())
            && {
                let mut sweep = Sweep::from(segments.as_slice());
                let mut neighbour_segments_touches_count = 0usize;
                while let Some(intersection) = sweep.next() {
                    debug_assert_eq!(
                        intersection.start == intersection.end,
                        matches!(intersection.relation, Relation::Touch | Relation::Cross)
                    );
                    let touches_at_vertices = (matches!(intersection.relation, Relation::Touch)
                        && (intersection
                            .start
                            .eq(sweep.get_segment_start(intersection.first_segment_id))
                            || intersection
                                .start
                                .eq(sweep.get_segment_end(intersection.first_segment_id)))
                        && (intersection
                            .start
                            .eq(sweep.get_segment_start(intersection.second_segment_id))
                            || intersection
                                .start
                                .eq(sweep.get_segment_end(intersection.second_segment_id))));
                    let neighbour_segments_intersection = intersection
                        .first_segment_id
                        .abs_diff(intersection.second_segment_id)
                        == 1
                        || (intersection.first_segment_id == segments.len() - 1
                            && intersection.second_segment_id == 0)
                        || (intersection.second_segment_id == segments.len() - 1
                            && intersection.first_segment_id == 0);
                    if !(touches_at_vertices && neighbour_segments_intersection) {
                        return false;
                    }
                    neighbour_segments_touches_count += 1;
                }
                neighbour_segments_touches_count == segments.len()
            }
    }
}

pub(crate) fn is_multisegment_valid<
    Scalar: AdditiveGroup + Clone + DivisivePartialMagma + MultiplicativeMonoid + Ord + Signed,
    Endpoint: Clone + From<(Scalar, Scalar)> + Ord + self::Point<Scalar>,
    Segment: self::Segment<Scalar, Point = Endpoint>,
    Multisegment: self::Multisegment<Scalar, Point = Endpoint, Segment = Segment>,
>(
    multisegment: &Multisegment,
) -> bool {
    let segments = multisegment.segments();
    segments.len() > MIN_MULTISEGMENT_SEGMENTS_COUNT
        && segments
            .iter()
            .all(|segment| segment.start() != segment.end())
        && Sweep::from(segments.as_slice())
            .all(|intersection| matches!(intersection.relation, Relation::Touch))
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
