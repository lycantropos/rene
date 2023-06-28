pub(crate) use self::event::Event;
pub(crate) use self::operation_kind::{DIFFERENCE, INTERSECTION, SYMMETRIC_DIFFERENCE, UNION};

mod constants;
mod event;
mod events_queue_key;
mod operation_kind;
pub(crate) mod shaped;
mod sweep_line_key;
pub(crate) mod traits;
mod types;
