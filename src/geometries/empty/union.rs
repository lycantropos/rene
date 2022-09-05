use crate::geometries::{Multipolygon, Polygon};
use crate::traits::Union;

use super::types::Empty;

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
