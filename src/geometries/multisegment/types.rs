use crate::geometries::Segment;

#[derive(Clone)]
pub struct Multisegment<Scalar> {
    pub(super) segments: Vec<Segment<Scalar>>,
}

impl<Scalar> Multisegment<Scalar> {
    pub fn new(segments: Vec<Segment<Scalar>>) -> Self {
        Self { segments }
    }
}
