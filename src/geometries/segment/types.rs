use crate::geometries::Point;

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
