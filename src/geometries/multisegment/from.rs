use crate::geometries::Segment;

use super::types::Multisegment;

impl<Scalar, const N: usize> From<[Segment<Scalar>; N]> for Multisegment<Scalar>
where
    Segment<Scalar>: Clone,
{
    fn from(segments: [Segment<Scalar>; N]) -> Self {
        Self::new(segments.to_vec())
    }
}

impl<Scalar> From<&[Segment<Scalar>]> for Multisegment<Scalar>
where
    Segment<Scalar>: Clone,
{
    fn from(segments: &[Segment<Scalar>]) -> Self {
        Self::new(segments.to_vec())
    }
}

impl<Scalar> From<Vec<Segment<Scalar>>> for Multisegment<Scalar> {
    fn from(segments: Vec<Segment<Scalar>>) -> Self {
        Self::new(segments)
    }
}
