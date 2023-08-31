pub(crate) use self::event::Event;

pub(crate) mod contour;
mod event;
mod events_queue_key;
pub(crate) mod linear;
pub(crate) mod multisegment;
mod multisegmental;
pub(crate) mod segment;
mod sweep_line_key;
mod utils;
