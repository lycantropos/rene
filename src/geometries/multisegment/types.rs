use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::{Point, Segment};
use crate::traits;

#[derive(Clone)]
pub struct Multisegment<Scalar> {
    pub(super) segments: Vec<Segment<Scalar>>,
}

impl<Scalar: Clone> Multisegment<Scalar> {
    pub fn new(segments: Vec<Segment<Scalar>>) -> Self {
        Self { segments }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> traits::Multisegment
    for Multisegment<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
{
    type Point = self::Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;
    type Segment = self::Segment<<Self::Point as traits::Point>::Coordinate>;

    fn segments(&self) -> Vec<Self::Segment> {
        self.segments.clone()
    }
}
