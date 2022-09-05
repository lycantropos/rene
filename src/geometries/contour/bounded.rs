use crate::bounded::{Bounded, Box};
use crate::geometries::Point;
use crate::Elemental;

use super::types::Contour;

impl<Scalar: Ord> Bounded<Scalar> for Contour<Scalar>
where
    Point<Scalar>: Elemental<Coordinate = Scalar>,
{
    fn to_max_x(&self) -> Scalar {
        unsafe {
            self.vertices
                .iter()
                .map(Elemental::x)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_max_y(&self) -> Scalar {
        unsafe {
            self.vertices
                .iter()
                .map(Elemental::y)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_min_x(&self) -> Scalar {
        unsafe {
            self.vertices
                .iter()
                .map(Elemental::x)
                .min()
                .unwrap_unchecked()
        }
    }

    fn to_min_y(&self) -> Scalar {
        unsafe {
            self.vertices
                .iter()
                .map(Elemental::y)
                .min()
                .unwrap_unchecked()
        }
    }

    fn to_bounding_box(&self) -> Box<Scalar> {
        let mut min_x = self.vertices[0].x();
        let mut max_x = self.vertices[0].x();
        let mut min_y = self.vertices[0].y();
        let mut max_y = self.vertices[0].y();
        for vertex in &self.vertices[1..] {
            let x = vertex.x();
            if min_x.gt(&x) {
                min_x = x;
            } else if max_x.lt(&x) {
                max_x = x;
            }
            let y = vertex.y();
            if min_y.gt(&y) {
                min_y = y;
            } else if max_y.lt(&y) {
                max_y = y;
            }
        }
        Box::new(min_x, max_x, min_y, max_y)
    }
}
