pub(crate) use base::{is_contour_valid, is_multisegment_valid};

mod base;
mod event;
mod events_queue_key;
mod events_registry;
mod sweep;
mod sweep_line_key;
