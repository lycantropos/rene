use std::hash::Hash;

use super::types::Multisegment;

impl<Scalar: Eq + Hash + PartialOrd> Eq for Multisegment<Scalar> {}
