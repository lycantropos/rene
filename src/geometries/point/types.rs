use std::fmt;

use crate::traits;

#[derive(Clone, fmt::Debug)]
pub struct Point<Scalar> {
    pub(in crate::geometries) x: Scalar,
    pub(in crate::geometries) y: Scalar,
}

impl<Scalar: Clone> traits::Point<Scalar> for Point<Scalar> {
    fn x(&self) -> Scalar {
        self.x.clone()
    }

    fn y(&self) -> Scalar {
        self.y.clone()
    }
}

impl<Scalar> Point<Scalar> {
    pub fn new(x: Scalar, y: Scalar) -> Self {
        Self { x, y }
    }
}
