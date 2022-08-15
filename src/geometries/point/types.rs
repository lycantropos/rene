use std::fmt;

#[derive(Clone, fmt::Debug)]
pub struct Point<Scalar> {
    pub(super) x: Scalar,
    pub(super) y: Scalar,
}

impl<Scalar> Point<Scalar> {
    pub fn new(x: Scalar, y: Scalar) -> Self {
        Self { x, y }
    }
}
