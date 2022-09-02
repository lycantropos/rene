pub(crate) use self::event::Event;
pub(crate) use self::operation::{Operation, ReduceEvents};
pub(crate) use self::operation_kind::{DIFFERENCE, INTERSECTION, SYMMETRIC_DIFFERENCE, UNION};

mod constants;
mod event;
mod events_queue_key;
mod operation;
mod operation_kind;
mod sweep_line_key;
mod types;
