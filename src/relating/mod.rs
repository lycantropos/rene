pub(crate) use self::event::Event;

pub(crate) mod contour;
mod event;
mod events_queue_key;
pub(crate) mod linear;
pub(crate) mod mixed;
pub(crate) mod multipolygon;
pub(crate) mod multisegment;
mod multisegmental;
pub(crate) mod polygon;
pub(crate) mod segment;
pub(crate) mod segment_endpoints;
pub(crate) mod shaped;
mod sweep_line_key;
mod utils;
