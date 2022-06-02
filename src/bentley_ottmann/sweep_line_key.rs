use std::cmp::Ordering;
use std::marker::PhantomData;

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::operations::orient;
use crate::oriented::Orientation;
use crate::traits::Point;

pub(super) struct SweepLineKey<Scalar, Endpoint> {
    pub(super) event_index: usize,
    pub(super) endpoints: *const Vec<Endpoint>,
    pub(super) opposites: *const Vec<usize>,
    _phantom: PhantomData<fn() -> Scalar>,
}

impl<Scalar, Endpoint> SweepLineKey<Scalar, Endpoint> {
    pub(super) fn new(
        event_index: usize,
        endpoints: &Vec<Endpoint>,
        opposites: &Vec<usize>,
    ) -> Self {
        Self {
            event_index,
            endpoints,
            opposites,
            _phantom: PhantomData,
        }
    }
}

impl<Scalar, Endpoint> SweepLineKey<Scalar, Endpoint> {
    pub(super) fn endpoints(&self) -> &Vec<Endpoint> {
        unsafe { &(*self.endpoints) }
    }

    pub(super) fn opposites(&self) -> &Vec<usize> {
        unsafe { &(*self.opposites) }
    }
}

impl<Scalar, Endpoint: PartialEq> PartialEq for SweepLineKey<Scalar, Endpoint> {
    fn eq(&self, other: &Self) -> bool {
        self.event_index == other.event_index
    }
}

impl<Scalar, Endpoint: Eq> Eq for SweepLineKey<Scalar, Endpoint> {}

impl<
        Scalar: AdditiveGroup + MultiplicativeMonoid + Ord + Signed,
        Endpoint: PartialEq + Point<Scalar>,
    > PartialOrd for SweepLineKey<Scalar, Endpoint>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_sweep_line_keys(
            self.event_index,
            other.event_index,
            self.endpoints(),
            self.opposites(),
        ))
    }
}

impl<Scalar: AdditiveGroup + MultiplicativeMonoid + Ord + Signed, Endpoint: Eq + Point<Scalar>> Ord
    for SweepLineKey<Scalar, Endpoint>
{
    fn cmp(&self, other: &Self) -> Ordering {
        compare_sweep_line_keys(
            self.event_index,
            other.event_index,
            self.endpoints(),
            self.opposites(),
        )
    }
}

fn compare_sweep_line_keys<
    Scalar: AdditiveGroup + MultiplicativeMonoid + Ord + Signed,
    Endpoint: PartialEq + Point<Scalar>,
>(
    left_event_index: usize,
    right_event_index: usize,
    endpoints: &Vec<Endpoint>,
    opposites: &Vec<usize>,
) -> Ordering {
    compare_segments_position(
        &endpoints[left_event_index],
        &endpoints[opposites[left_event_index]],
        &endpoints[right_event_index],
        &endpoints[opposites[right_event_index]],
    )
}

fn compare_segments_position<
    Scalar: AdditiveGroup + MultiplicativeMonoid + Ord + Signed,
    Endpoint: PartialEq + Point<Scalar>,
>(
    left_start: &Endpoint,
    left_end: &Endpoint,
    right_start: &Endpoint,
    right_end: &Endpoint,
) -> Ordering {
    let other_start_orientation = orient(left_start, left_end, right_start);
    let other_end_orientation = orient(left_start, left_end, right_end);
    if other_start_orientation == other_end_orientation {
        match other_start_orientation {
            Orientation::Collinear => match left_start.y().cmp(&right_start.y()) {
                Ordering::Equal => match left_start.x().cmp(&right_start.x()) {
                    Ordering::Equal => match left_end.y().cmp(&right_end.y()) {
                        Ordering::Equal => left_end.x().cmp(&right_end.x()),
                        value => value,
                    },
                    value => value,
                },
                value => value,
            },
            Orientation::Clockwise => Ordering::Greater,
            Orientation::Counterclockwise => Ordering::Less,
        }
    } else {
        let start_orientation = orient(right_start, right_end, left_start);
        let end_orientation = orient(right_start, right_end, left_end);
        if start_orientation == end_orientation {
            match start_orientation {
                Orientation::Clockwise => Ordering::Less,
                _ => Ordering::Greater,
            }
        } else if other_start_orientation == Orientation::Collinear {
            match other_end_orientation {
                Orientation::Counterclockwise => Ordering::Less,
                _ => Ordering::Greater,
            }
        } else if start_orientation == Orientation::Collinear {
            match end_orientation {
                Orientation::Clockwise => Ordering::Less,
                _ => Ordering::Greater,
            }
        } else if end_orientation == Orientation::Collinear {
            match start_orientation {
                Orientation::Clockwise => Ordering::Less,
                _ => Ordering::Greater,
            }
        } else {
            match other_start_orientation {
                Orientation::Counterclockwise => Ordering::Less,
                _ => Ordering::Greater,
            }
        }
    }
}
