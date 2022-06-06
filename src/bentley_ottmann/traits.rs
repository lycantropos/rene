use super::event::Event;

pub(crate) trait EventsQueue {
    fn pop(&mut self) -> Option<Event>;
    fn push(&mut self, event: Event);
}

pub(crate) trait SweepLine {
    fn above(&self, event: Event) -> Option<Event>;
    fn below(&self, event: Event) -> Option<Event>;
    fn find(&self, event: Event) -> Option<Event>;
    fn insert(&mut self, event: Event) -> bool;
    fn remove(&mut self, event: Event) -> bool;
}
