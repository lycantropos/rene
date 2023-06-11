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

impl<Digit, const SHIFT: usize> traits::Segmental for Segment<Fraction<BigInt<Digit, SHIFT>>>
where
    BigInt<Digit, SHIFT>: Clone,
{
    type Endpoint = Point<Fraction<BigInt<Digit, SHIFT>>>;

    fn start(&self) -> Self::Endpoint {
        self.start.clone()
    }

    fn end(&self) -> Self::Endpoint {
        self.end.clone()
    }
}
