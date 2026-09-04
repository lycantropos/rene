#[cfg_attr(not(feature = "python_binding"), allow(dead_code, unused_imports))]
mod bentley_ottmann;
pub mod bounded;
mod clipping;
mod constants;
mod contracts;
pub mod geometries;
mod iteration;
pub mod locatable;
mod operations;
pub mod oriented;
#[cfg(feature = "python_binding")]
mod python_binding;
pub mod relatable;
mod relating;
#[cfg_attr(not(feature = "python_binding"), allow(dead_code, unused_imports))]
mod seidel;
mod slice_sequence;
mod sweeping;
pub mod traits;
#[cfg_attr(not(feature = "python_binding"), allow(dead_code, unused_imports))]
mod triangulation;
