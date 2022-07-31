use super::types::Polygon;

impl<Scalar: Eq> Eq for Polygon<Scalar> where Self: PartialEq {}
