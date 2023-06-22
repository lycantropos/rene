use crate::bounded;
use crate::bounded::Bounded;
use crate::geometries::Point;
use crate::operations::coordinates_iterator_to_bounds;
use crate::traits::Elemental;

use super::types::Contour;

impl<'a, Scalar: Ord> Bounded<&'a Scalar> for &'a Contour<Scalar>
where
    &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>,
{
    fn to_bounding_box(self) -> bounded::Box<&'a Scalar> {
        let (min_x, max_x, min_y, max_y) =
            coordinates_iterator_to_bounds(self.vertices.iter().map(Elemental::coordinates));
        bounded::Box::new(min_x, max_x, min_y, max_y)
    }

    fn to_max_x(self) -> &'a Scalar {
        unsafe {
            self.vertices
                .iter()
                .map(Elemental::x)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_max_y(self) -> &'a Scalar {
        unsafe {
            self.vertices
                .iter()
                .map(Elemental::y)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_min_x(self) -> &'a Scalar {
        unsafe {
            self.vertices
                .iter()
                .map(Elemental::x)
                .min()
                .unwrap_unchecked()
        }
    }

    fn to_min_y(self) -> &'a Scalar {
        unsafe {
            self.vertices
                .iter()
                .map(Elemental::y)
                .min()
                .unwrap_unchecked()
        }
    }
}

impl<Scalar: Ord> Bounded<Scalar> for Contour<Scalar>
where
    Point<Scalar>: Elemental<Coordinate = Scalar>,
{
    fn to_bounding_box(self) -> bounded::Box<Scalar> {
        let (min_x, max_x, min_y, max_y) =
            coordinates_iterator_to_bounds(self.vertices.into_iter().map(Elemental::coordinates));
        bounded::Box::new(min_x, max_x, min_y, max_y)
    }

    fn to_max_x(self) -> Scalar {
        unsafe {
            self.vertices
                .into_iter()
                .map(Elemental::x)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_max_y(self) -> Scalar {
        unsafe {
            self.vertices
                .into_iter()
                .map(Elemental::y)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_min_x(self) -> Scalar {
        unsafe {
            self.vertices
                .into_iter()
                .map(Elemental::x)
                .min()
                .unwrap_unchecked()
        }
    }

    fn to_min_y(self) -> Scalar {
        unsafe {
            self.vertices
                .into_iter()
                .map(Elemental::y)
                .min()
                .unwrap_unchecked()
        }
    }
}
