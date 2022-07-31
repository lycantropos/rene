use std::fmt;

use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::traits;

#[derive(Clone, fmt::Debug)]
pub struct Point<Scalar> {
    pub(super) x: Scalar,
    pub(super) y: Scalar,
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> traits::Point
    for Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
{
    type Coordinate = Fraction<BigInt<Digit, SEPARATOR, SHIFT>>;

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
