use crate::geometries::Point;
use crate::traits;

#[derive(Clone)]
pub struct Segment<Scalar>(
    pub(in crate::geometries) Point<Scalar>,
    pub(in crate::geometries) Point<Scalar>,
);

impl<Scalar> Segment<Scalar> {
    pub fn new(start: Point<Scalar>, end: Point<Scalar>) -> Self {
        Self(start, end)
    }
}

impl<Scalar: Clone> traits::Segment<Scalar> for Segment<Scalar> {
    type Point = self::Point<Scalar>;

    fn start(&self) -> Self::Point {
        self.0.clone()
    }

    fn end(&self) -> Self::Point {
        self.1.clone()
    }
}
