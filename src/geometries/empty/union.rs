use crate::geometries::{Contour, Multipolygon, Multisegment, Polygon, Segment};
use crate::traits::Union;

use super::types::Empty;

impl<Scalar> Union<Contour<Scalar>> for Empty {
    type Output = Contour<Scalar>;

    fn union(self, other: Contour<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> Union<&Contour<Scalar>> for Empty
where
    Contour<Scalar>: Clone,
{
    type Output = Contour<Scalar>;

    fn union(self, other: &Contour<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> Union<Contour<Scalar>> for &Empty {
    type Output = Contour<Scalar>;

    fn union(self, other: Contour<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> Union<&Contour<Scalar>> for &Empty
where
    Contour<Scalar>: Clone,
{
    type Output = Contour<Scalar>;

    fn union(self, other: &Contour<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl Union for Empty {
    type Output = Self;

    fn union(self, other: Self) -> Self::Output {
        other
    }
}

impl Union<&Self> for Empty {
    type Output = Self;

    fn union(self, other: &Self) -> Self::Output {
        *other
    }
}

impl Union<Empty> for &Empty {
    type Output = Empty;

    fn union(self, other: Empty) -> Self::Output {
        other
    }
}

impl Union for &Empty {
    type Output = Empty;

    fn union(self, other: Self) -> Self::Output {
        *other
    }
}

impl<Scalar> Union<Multipolygon<Scalar>> for Empty {
    type Output = Multipolygon<Scalar>;

    fn union(self, other: Multipolygon<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> Union<&Multipolygon<Scalar>> for Empty
where
    Multipolygon<Scalar>: Clone,
{
    type Output = Multipolygon<Scalar>;

    fn union(self, other: &Multipolygon<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> Union<Multipolygon<Scalar>> for &Empty {
    type Output = Multipolygon<Scalar>;

    fn union(self, other: Multipolygon<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> Union<&Multipolygon<Scalar>> for &Empty
where
    Multipolygon<Scalar>: Clone,
{
    type Output = Multipolygon<Scalar>;

    fn union(self, other: &Multipolygon<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> Union<Multisegment<Scalar>> for Empty {
    type Output = Multisegment<Scalar>;

    fn union(self, other: Multisegment<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> Union<&Multisegment<Scalar>> for Empty
where
    Multisegment<Scalar>: Clone,
{
    type Output = Multisegment<Scalar>;

    fn union(self, other: &Multisegment<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> Union<Multisegment<Scalar>> for &Empty {
    type Output = Multisegment<Scalar>;

    fn union(self, other: Multisegment<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> Union<&Multisegment<Scalar>> for &Empty
where
    Multisegment<Scalar>: Clone,
{
    type Output = Multisegment<Scalar>;

    fn union(self, other: &Multisegment<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> Union<Polygon<Scalar>> for Empty {
    type Output = Polygon<Scalar>;

    fn union(self, other: Polygon<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> Union<&Polygon<Scalar>> for Empty
where
    Polygon<Scalar>: Clone,
{
    type Output = Polygon<Scalar>;

    fn union(self, other: &Polygon<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> Union<Polygon<Scalar>> for &Empty {
    type Output = Polygon<Scalar>;

    fn union(self, other: Polygon<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> Union<&Polygon<Scalar>> for &Empty
where
    Polygon<Scalar>: Clone,
{
    type Output = Polygon<Scalar>;

    fn union(self, other: &Polygon<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> Union<Segment<Scalar>> for Empty {
    type Output = Segment<Scalar>;

    fn union(self, other: Segment<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> Union<&Segment<Scalar>> for Empty
where
    Segment<Scalar>: Clone,
{
    type Output = Segment<Scalar>;

    fn union(self, other: &Segment<Scalar>) -> Self::Output {
        other.clone()
    }
}

impl<Scalar> Union<Segment<Scalar>> for &Empty {
    type Output = Segment<Scalar>;

    fn union(self, other: Segment<Scalar>) -> Self::Output {
        other
    }
}

impl<Scalar> Union<&Segment<Scalar>> for &Empty
where
    Segment<Scalar>: Clone,
{
    type Output = Segment<Scalar>;

    fn union(self, other: &Segment<Scalar>) -> Self::Output {
        other.clone()
    }
}
