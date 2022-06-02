use rithm::traits::Parity;

pub(super) type Event = usize;

pub(super) fn is_left_event(event: Event) -> bool {
    event.is_even()
}
