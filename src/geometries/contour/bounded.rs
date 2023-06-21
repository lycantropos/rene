use crate::bounded::{Bounded, Box};
use crate::geometries::Point;
use crate::Elemental;

use super::types::Contour;

impl<Scalar: Clone + Ord> Bounded<Scalar> for Contour<Scalar>
where
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>,
{
    fn to_max_x(&self) -> Scalar {
        unsafe {
            self.vertices
                .iter()
                .map(Elemental::x)
                .max()
                .unwrap_unchecked()
                .clone()
        }
    }

    fn to_max_y(&self) -> Scalar {
        unsafe {
            self.vertices
                .iter()
                .map(Elemental::y)
                .max()
                .unwrap_unchecked()
                .clone()
        }
    }

    fn to_min_x(&self) -> Scalar {
        unsafe {
            self.vertices
                .iter()
                .map(Elemental::x)
                .min()
                .unwrap_unchecked()
                .clone()
        }
    }

    fn to_min_y(&self) -> Scalar {
        unsafe {
            self.vertices
                .iter()
                .map(Elemental::y)
                .min()
                .unwrap_unchecked()
                .clone()
        }
    }

    fn to_bounding_box(&self) -> Box<Scalar> {
        let (mut min_x, mut min_y) = self.vertices[0].coordinates();
        let (mut max_x, mut max_y) = self.vertices[0].coordinates();
        for (x, y) in self.vertices[1..].iter().map(Elemental::coordinates) {
            if min_x.gt(&x) {
                min_x = x;
            } else if max_x.lt(&x) {
                max_x = x;
            }
            if min_y.gt(&y) {
                min_y = y;
            } else if max_y.lt(&y) {
                max_y = y;
            }
        }
        Box::new(min_x.clone(), max_x.clone(), min_y.clone(), max_y.clone())
    }
}
