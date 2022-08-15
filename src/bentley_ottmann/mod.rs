pub(crate) use base::{
    is_contour_valid, is_multisegment_valid, to_unique_non_crossing_or_overlapping_segments,
};

mod base;
mod event;
mod events_queue_key;
mod events_registry;
mod sweep;
mod sweep_line_key;
pub(crate) mod traits;
