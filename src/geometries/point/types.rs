use std::fmt;

use crate::traits;

#[derive(Clone, fmt::Debug)]
pub struct Point<Scalar>(
    pub(in crate::geometries) Scalar,
    pub(in crate::geometries) Scalar,
);

impl<Scalar: Clone> traits::Point<Scalar> for Point<Scalar> {
    fn x(&self) -> Scalar {
        self.0.clone()
    }

    fn y(&self) -> Scalar {
        self.1.clone()
    }
}

impl<Scalar> Point<Scalar> {
    pub fn new(x: Scalar, y: Scalar) -> Self {
        Self(x, y)
    }
}
