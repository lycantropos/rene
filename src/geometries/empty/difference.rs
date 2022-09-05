use crate::geometries::{Multipolygon, Polygon};
use crate::traits::Difference;

use super::types::Empty;

impl Difference for Empty {
    type Output = Self;

    fn difference(self, _other: Self) -> Self::Output {
        self
    }
}

impl Difference<&Self> for Empty {
    type Output = Self;

    fn difference(self, _other: &Self) -> Self::Output {
        self
    }
}

impl Difference<Empty> for &Empty {
    type Output = Empty;

    fn difference(self, _other: Empty) -> Self::Output {
        *self
    }
}

impl Difference for &Empty {
    type Output = Empty;

    fn difference(self, _other: Self) -> Self::Output {
        *self
    }
}

impl<Scalar> Difference<Polygon<Scalar>> for Empty {
    type Output = Self;

    fn difference(self, _other: Polygon<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<&Polygon<Scalar>> for Empty {
    type Output = Self;

    fn difference(self, _other: &Polygon<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<Polygon<Scalar>> for &Empty {
    type Output = Empty;

    fn difference(self, _other: Polygon<Scalar>) -> Self::Output {
        *self
    }
}

impl<Scalar> Difference<&Polygon<Scalar>> for &Empty {
    type Output = Empty;

    fn difference(self, _other: &Polygon<Scalar>) -> Self::Output {
        *self
    }
}

impl<Scalar> Difference<Multipolygon<Scalar>> for Empty {
    type Output = Self;

    fn difference(self, _other: Multipolygon<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<&Multipolygon<Scalar>> for Empty {
    type Output = Self;

    fn difference(self, _other: &Multipolygon<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<Multipolygon<Scalar>> for &Empty {
    type Output = Empty;

    fn difference(self, _other: Multipolygon<Scalar>) -> Self::Output {
        *self
    }
}

impl<Scalar> Difference<&Multipolygon<Scalar>> for &Empty {
    type Output = Empty;

    fn difference(self, _other: &Multipolygon<Scalar>) -> Self::Output {
        *self
    }
}
