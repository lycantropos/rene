pub(crate) use base::to_unique_non_crossing_or_overlapping_segments;

mod base;
mod event;
mod events_queue_key;
mod events_registry;
mod sweep;
mod sweep_line_key;
mod traits;
