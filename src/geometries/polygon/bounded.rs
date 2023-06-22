use crate::bounded;
use crate::bounded::Bounded;
use crate::geometries::Contour;

use super::types::Polygon;

impl<'a, Scalar> Bounded<&'a Scalar> for &'a Polygon<Scalar>
where
    &'a Contour<Scalar>: Bounded<&'a Scalar>,
{
    fn to_bounding_box(self) -> bounded::Box<&'a Scalar> {
        (&self.border).to_bounding_box()
    }

    fn to_max_x(self) -> &'a Scalar {
        (&self.border).to_max_x()
    }

    fn to_max_y(self) -> &'a Scalar {
        (&self.border).to_max_y()
    }

    fn to_min_x(self) -> &'a Scalar {
        (&self.border).to_min_x()
    }

    fn to_min_y(self) -> &'a Scalar {
        (&self.border).to_min_y()
    }
}

impl<Scalar> Bounded<Scalar> for Polygon<Scalar>
where
    Contour<Scalar>: Bounded<Scalar>,
{
    fn to_bounding_box(self) -> bounded::Box<Scalar> {
        self.border.to_bounding_box()
    }

    fn to_max_x(self) -> Scalar {
        self.border.to_max_x()
    }

    fn to_max_y(self) -> Scalar {
        self.border.to_max_y()
    }

    fn to_min_x(self) -> Scalar {
        self.border.to_min_x()
    }

    fn to_min_y(self) -> Scalar {
        self.border.to_min_y()
    }
}
