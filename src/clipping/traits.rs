use super::event::Event;

pub(crate) trait ReduceEvents {
    type Output;

    fn reduce_events(&self, events: Vec<Event>) -> Self::Output;
}
