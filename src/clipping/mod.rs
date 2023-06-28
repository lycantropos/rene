pub(crate) use self::event::{is_right_event, Event};
pub(crate) use self::operation_kind::{DIFFERENCE, INTERSECTION, SYMMETRIC_DIFFERENCE, UNION};

mod constants;
mod event;
mod events_queue_key;
pub(crate) mod linear;
mod operation_kind;
pub(crate) mod shaped;
mod sweep_line_key;
pub(crate) mod traits;
mod types;
