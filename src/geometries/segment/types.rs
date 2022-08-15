use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Point;
use crate::traits;

#[derive(Clone)]
pub struct Segment<Scalar> {
    pub(super) start: Point<Scalar>,
    pub(super) end: Point<Scalar>,
}

impl<Scalar> Segment<Scalar> {
    pub fn new(start: Point<Scalar>, end: Point<Scalar>) -> Self {
        Self { start, end }
    }
}

impl<Digit, const SEPARATOR: char, const SHIFT: usize> traits::Segmental
    for Segment<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    BigInt<Digit, SEPARATOR, SHIFT>: Clone,
{
    type Endpoint = self::Point<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>;

    fn start(&self) -> Self::Endpoint {
        self.start.clone()
    }

    fn end(&self) -> Self::Endpoint {
        self.end.clone()
    }
}
