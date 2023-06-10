use crate::geometries::{Contour, Multipolygon, Multisegment, Polygon, Segment};
use crate::traits::Difference;

use super::types::Empty;

impl<Scalar> Difference<Contour<Scalar>> for Empty {
    type Output = Self;

    fn difference(self, _other: Contour<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<&Contour<Scalar>> for Empty {
    type Output = Self;

    fn difference(self, _other: &Contour<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<Contour<Scalar>> for &Empty {
    type Output = Empty;

    fn difference(self, _other: Contour<Scalar>) -> Self::Output {
        *self
    }
}

impl<Scalar> Difference<&Contour<Scalar>> for &Empty {
    type Output = Empty;

    fn difference(self, _other: &Contour<Scalar>) -> Self::Output {
        *self
    }
}

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

impl<Scalar> Difference<Multisegment<Scalar>> for Empty {
    type Output = Self;

    fn difference(self, _other: Multisegment<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<&Multisegment<Scalar>> for Empty {
    type Output = Self;

    fn difference(self, _other: &Multisegment<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<Multisegment<Scalar>> for &Empty {
    type Output = Empty;

    fn difference(self, _other: Multisegment<Scalar>) -> Self::Output {
        *self
    }
}

impl<Scalar> Difference<&Multisegment<Scalar>> for &Empty {
    type Output = Empty;

    fn difference(self, _other: &Multisegment<Scalar>) -> Self::Output {
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

impl<Scalar> Difference<Segment<Scalar>> for Empty {
    type Output = Self;

    fn difference(self, _other: Segment<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<&Segment<Scalar>> for Empty {
    type Output = Self;

    fn difference(self, _other: &Segment<Scalar>) -> Self::Output {
        self
    }
}

impl<Scalar> Difference<Segment<Scalar>> for &Empty {
    type Output = Empty;

    fn difference(self, _other: Segment<Scalar>) -> Self::Output {
        *self
    }
}

impl<Scalar> Difference<&Segment<Scalar>> for &Empty {
    type Output = Empty;

    fn difference(self, _other: &Segment<Scalar>) -> Self::Output {
        *self
    }
}
