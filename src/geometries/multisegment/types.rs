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

impl<Scalar: Clone> traits::Multisegment<Scalar> for Multisegment<Scalar> {
    type Point = self::Point<Scalar>;
    type Segment = self::Segment<Scalar>;

    fn segments(&self) -> Vec<Self::Segment> {
        self.segments.clone()
    }
}
