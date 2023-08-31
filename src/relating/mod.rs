pub(crate) use self::event::Event;

mod event;
mod events_queue_key;
pub(crate) mod linear;
pub(crate) mod multisegment;
pub(crate) mod segment;
mod sweep_line_key;
mod utils;
