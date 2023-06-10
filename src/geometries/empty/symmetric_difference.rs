use crate::geometries::{Contour, Multipolygon, Multisegment, Polygon, Segment};
use crate::traits::SymmetricDifference;

use super::types::Empty;

impl<Scalar> SymmetricDifference<Contour<Scalar>> for Empty {
    type Output = Contour<Scalar>;

    fn symmetric_difference(self, other: Contour<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> SymmetricDifference<&Contour<Scalar>> for Empty
where
    Contour<Scalar>: Clone,
{
    type Output = Contour<Scalar>;

    fn symmetric_difference(self, other: &Contour<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> SymmetricDifference<Contour<Scalar>> for &Empty {
    type Output = Contour<Scalar>;

    fn symmetric_difference(self, other: Contour<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> SymmetricDifference<&Contour<Scalar>> for &Empty
where
    Contour<Scalar>: Clone,
{
    type Output = Contour<Scalar>;

    fn symmetric_difference(self, other: &Contour<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl SymmetricDifference for Empty {
    type Output = Self;

    fn symmetric_difference(self, other: Self) -> Self::Output {
        other
    }
}

impl SymmetricDifference<&Self> for Empty {
    type Output = Self;

    fn symmetric_difference(self, other: &Self) -> Self::Output {
        *other
    }
}

impl SymmetricDifference<Empty> for &Empty {
    type Output = Empty;

    fn symmetric_difference(self, other: Empty) -> Self::Output {
        other
    }
}

impl SymmetricDifference for &Empty {
    type Output = Empty;

    fn symmetric_difference(self, other: Self) -> Self::Output {
        *other
    }
}

impl<Scalar> SymmetricDifference<Multipolygon<Scalar>> for Empty {
    type Output = Multipolygon<Scalar>;

    fn symmetric_difference(self, other: Multipolygon<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> SymmetricDifference<&Multipolygon<Scalar>> for Empty
where
    Multipolygon<Scalar>: Clone,
{
    type Output = Multipolygon<Scalar>;

    fn symmetric_difference(self, other: &Multipolygon<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> SymmetricDifference<Multipolygon<Scalar>> for &Empty {
    type Output = Multipolygon<Scalar>;

    fn symmetric_difference(self, other: Multipolygon<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> SymmetricDifference<&Multipolygon<Scalar>> for &Empty
where
    Multipolygon<Scalar>: Clone,
{
    type Output = Multipolygon<Scalar>;

    fn symmetric_difference(self, other: &Multipolygon<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> SymmetricDifference<Multisegment<Scalar>> for Empty {
    type Output = Multisegment<Scalar>;

    fn symmetric_difference(self, other: Multisegment<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> SymmetricDifference<&Multisegment<Scalar>> for Empty
where
    Multisegment<Scalar>: Clone,
{
    type Output = Multisegment<Scalar>;

    fn symmetric_difference(self, other: &Multisegment<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> SymmetricDifference<Multisegment<Scalar>> for &Empty {
    type Output = Multisegment<Scalar>;

    fn symmetric_difference(self, other: Multisegment<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> SymmetricDifference<&Multisegment<Scalar>> for &Empty
where
    Multisegment<Scalar>: Clone,
{
    type Output = Multisegment<Scalar>;

    fn symmetric_difference(self, other: &Multisegment<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> SymmetricDifference<Polygon<Scalar>> for Empty {
    type Output = Polygon<Scalar>;

    fn symmetric_difference(self, other: Polygon<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> SymmetricDifference<&Polygon<Scalar>> for Empty
where
    Polygon<Scalar>: Clone,
{
    type Output = Polygon<Scalar>;

    fn symmetric_difference(self, other: &Polygon<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> SymmetricDifference<Polygon<Scalar>> for &Empty {
    type Output = Polygon<Scalar>;

    fn symmetric_difference(self, other: Polygon<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> SymmetricDifference<&Polygon<Scalar>> for &Empty
where
    Polygon<Scalar>: Clone,
{
    type Output = Polygon<Scalar>;

    fn symmetric_difference(self, other: &Polygon<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> SymmetricDifference<Segment<Scalar>> for Empty {
    type Output = Segment<Scalar>;

    fn symmetric_difference(self, other: Segment<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> SymmetricDifference<&Segment<Scalar>> for Empty
where
    Segment<Scalar>: Clone,
{
    type Output = Segment<Scalar>;

    fn symmetric_difference(self, other: &Segment<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> SymmetricDifference<Segment<Scalar>> for &Empty {
    type Output = Segment<Scalar>;

    fn symmetric_difference(self, other: Segment<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> SymmetricDifference<&Segment<Scalar>> for &Empty
where
    Segment<Scalar>: Clone,
{
    type Output = Segment<Scalar>;

    fn symmetric_difference(self, other: &Segment<Scalar>) -> Self::Output {
        other.clone()
    }
}
