use crate::bounded;
use crate::bounded::Bounded;
use crate::geometries::{Point, Segment};
use crate::operations::{merge_bounds, segmental_to_bounds};
use crate::traits::{Elemental, Segmental};

use super::types::Multisegment;

impl<'a, Scalar: Ord> Bounded<&'a Scalar> for &'a Multisegment<Scalar>
where
    for<'b> &'b Point<Scalar>: Elemental<Coordinate = &'b Scalar>,
    for<'b> &'b Segment<Scalar>: Segmental<Endpoint = &'b Point<Scalar>>,
{
    fn to_bounding_box(self) -> bounded::Box<&'a Scalar> {
        let (min_x, max_x, min_y, max_y) =
            merge_bounds(self.segments.iter().map(segmental_to_bounds));
        bounded::Box::new(min_x, max_x, min_y, max_y)
    }

    fn to_max_x(self) -> &'a Scalar {
        unsafe {
            self.segments
                .iter()
                .map(|segment| {
                    let (start, end) = segment.endpoints();
                    start.x().max(end.x())
                })
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_max_y(self) -> &'a Scalar {
        unsafe {
            self.segments
                .iter()
                .map(|segment| {
                    let (start, end) = segment.endpoints();
                    start.y().max(end.y())
                })
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_min_x(self) -> &'a Scalar {
        unsafe {
            self.segments
                .iter()
                .map(|segment| {
                    let (start, end) = segment.endpoints();
                    start.x().min(end.x())
                })
                .min()
                .unwrap_unchecked()
        }
    }

    fn to_min_y(self) -> &'a Scalar {
        unsafe {
            self.segments
                .iter()
                .map(|segment| {
                    let (start, end) = segment.endpoints();
                    start.y().min(end.y())
                })
                .min()
                .unwrap_unchecked()
        }
    }
}

impl<Point, Scalar: Ord> Bounded<Scalar> for Multisegment<Scalar>
where
    Point: Elemental<Coordinate = Scalar>,
    Segment<Scalar>: Segmental<Endpoint = Point>,
{
    fn to_bounding_box(self) -> bounded::Box<Scalar> {
        let (min_x, max_x, min_y, max_y) =
            merge_bounds(self.segments.into_iter().map(segmental_to_bounds));
        bounded::Box::new(min_x, max_x, min_y, max_y)
    }

    fn to_max_x(self) -> Scalar {
        unsafe {
            self.segments
                .into_iter()
                .map(|segment| {
                    let (start, end) = segment.endpoints();
                    start.x().max(end.x())
                })
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_max_y(self) -> Scalar {
        unsafe {
            self.segments
                .into_iter()
                .map(|segment| {
                    let (start, end) = segment.endpoints();
                    start.y().max(end.y())
                })
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_min_x(self) -> Scalar {
        unsafe {
            self.segments
                .into_iter()
                .map(|segment| {
                    let (start, end) = segment.endpoints();
                    start.x().min(end.x())
                })
                .min()
                .unwrap_unchecked()
        }
    }

    fn to_min_y(self) -> Scalar {
        unsafe {
            self.segments
                .into_iter()
                .map(|segment| {
                    let (start, end) = segment.endpoints();
                    start.y().min(end.y())
                })
                .min()
                .unwrap_unchecked()
        }
    }
}
