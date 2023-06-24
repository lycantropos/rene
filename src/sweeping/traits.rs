pub(crate) trait EventsQueue {
    type Event;

    fn peek(&mut self) -> Option<Self::Event>;
    fn pop(&mut self) -> Option<Self::Event>;
    fn push(&mut self, event: Self::Event);
}

pub(crate) trait SweepLine {
    type Event;

    fn above(&self, event: Self::Event) -> Option<Self::Event>;
    fn below(&self, event: Self::Event) -> Option<Self::Event>;
    fn find(&self, event: Self::Event) -> Option<Self::Event>;
    fn insert(&mut self, event: Self::Event) -> bool;
    fn remove(&mut self, event: Self::Event) -> bool;
}
