use std::fmt;

use crate::traits;

#[derive(Clone, fmt::Debug)]
pub struct Point<Scalar> {
    pub(super) x: Scalar,
    pub(super) y: Scalar,
}

impl<Scalar: Clone> traits::Point for Point<Scalar> {
    type Coordinate = Scalar;

    fn x(&self) -> Self::Coordinate {
        self.x.clone()
    }

    fn y(&self) -> Self::Coordinate {
        self.y.clone()
    }
}

impl<Scalar> Point<Scalar> {
    pub fn new(x: Scalar, y: Scalar) -> Self {
        Self { x, y }
    }
}
