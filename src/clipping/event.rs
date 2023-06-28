use traiter::numbers::Parity;

pub(crate) type Event = usize;
pub(super) const UNDEFINED_EVENT: Event = usize::MAX;

pub(super) fn is_left_event(event: Event) -> bool {
    event.is_even()
}

pub(crate) fn is_right_event(event: Event) -> bool {
    event.is_odd()
}

pub(super) fn left_event_to_position(event: Event) -> usize {
    debug_assert!(is_left_event(event));
    event / 2
}

pub(super) fn segment_id_to_left_event(segment_id: usize) -> Event {
    segment_id * 2
}

pub(super) fn segment_id_to_right_event(segment_id: usize) -> Event {
    segment_id * 2 + 1
}
