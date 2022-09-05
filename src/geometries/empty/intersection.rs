use crate::geometries::{Multipolygon, Polygon};
use crate::traits::Intersection;

use super::types::Empty;

impl Intersection for Empty {
    type Output = Self;

    fn intersection(self, _other: Self) -> Self::Output {
        self
    }
}

impl Intersection<&Self> for Empty {
    type Output = Self;

    fn intersection(self, _other: &Self) -> Self::Output {
        self
    }
}

impl Intersection<Empty> for &Empty {
    type Output = Empty;

    fn intersection(self, _other: Empty) -> Self::Output {
        *self
    }
}

impl Intersection for &Empty {
    type Output = Empty;

    fn intersection(self, _other: Self) -> Self::Output {
        *self
    }
}

impl<Scalar> Intersection<Polygon<Scalar>> for Empty {
    type Output = Self;

    fn intersection(self, _other: Polygon<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Intersection<&Polygon<Scalar>> for Empty {
    type Output = Self;

    fn intersection(self, _other: &Polygon<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Intersection<Polygon<Scalar>> for &Empty {
    type Output = Empty;

    fn intersection(self, _other: Polygon<Scalar>) -> Self::Output {
        *self
    }
}

impl<Scalar> Intersection<&Polygon<Scalar>> for &Empty {
    type Output = Empty;

    fn intersection(self, _other: &Polygon<Scalar>) -> Self::Output {
        *self
    }
}

impl<Scalar> Intersection<Multipolygon<Scalar>> for Empty {
    type Output = Self;

    fn intersection(self, _other: Multipolygon<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Intersection<&Multipolygon<Scalar>> for Empty {
    type Output = Self;

    fn intersection(self, _other: &Multipolygon<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Intersection<Multipolygon<Scalar>> for &Empty {
    type Output = Empty;

    fn intersection(self, _other: Multipolygon<Scalar>) -> Self::Output {
        *self
    }
}

impl<Scalar> Intersection<&Multipolygon<Scalar>> for &Empty {
    type Output = Empty;

    fn intersection(self, _other: &Multipolygon<Scalar>) -> Self::Output {
        *self
    }
}
