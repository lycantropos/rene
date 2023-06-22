use crate::bounded;
use crate::bounded::Bounded;
use crate::geometries::Segment;
use crate::operations::{merge_bounds, to_sorted_pair};
use crate::traits::{Elemental, Segmental, SegmentalCoordinate};

use super::types::Multisegment;

impl<Point, Scalar: Clone + Ord> Bounded<Scalar> for &Multisegment<Scalar>
where
    for<'a> &'a Point: Elemental<Coordinate = &'a Scalar>,
    for<'a> &'a Segment<Scalar>: Segmental<Endpoint = &'a Point>,
{
    fn to_bounding_box(self) -> bounded::Box<Scalar> {
        let (min_x, max_x, min_y, max_y) =
            merge_bounds(self.segments.iter().map(segmental_to_bounds));
        bounded::Box::new(min_x.clone(), max_x.clone(), min_y.clone(), max_y.clone())
    }

    fn to_max_x(self) -> Scalar {
        unsafe {
            self.segments
                .iter()
                .map(|segment| {
                    let (start, end) = segment.endpoints();
                    start.x().max(end.x())
                })
                .max()
                .unwrap_unchecked()
                .clone()
        }
    }

    fn to_max_y(self) -> Scalar {
        unsafe {
            self.segments
                .iter()
                .map(|segment| {
                    let (start, end) = segment.endpoints();
                    start.y().max(end.y())
                })
                .max()
                .unwrap_unchecked()
                .clone()
        }
    }

    fn to_min_x(self) -> Scalar {
        unsafe {
            self.segments
                .iter()
                .map(|segment| {
                    let (start, end) = segment.endpoints();
                    start.x().min(end.x())
                })
                .min()
                .unwrap_unchecked()
                .clone()
        }
    }

    fn to_min_y(self) -> Scalar {
        unsafe {
            self.segments
                .iter()
                .map(|segment| {
                    let (start, end) = segment.endpoints();
                    start.y().min(end.y())
                })
                .min()
                .unwrap_unchecked()
                .clone()
        }
    }
}

impl<'a, Point, Scalar: Ord> Bounded<Scalar> for Multisegment<Scalar>
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

fn segmental_to_bounds<Segment: Segmental>(
    segment: Segment,
) -> (
    SegmentalCoordinate<Segment>,
    SegmentalCoordinate<Segment>,
    SegmentalCoordinate<Segment>,
    SegmentalCoordinate<Segment>,
)
where
    SegmentalCoordinate<Segment>: PartialOrd,
{
    let (start, end) = segment.endpoints();
    let (start_x, start_y) = start.coordinates();
    let (end_x, end_y) = end.coordinates();
    let (min_x, max_x) = to_sorted_pair((start_x, end_x));
    let (min_y, max_y) = to_sorted_pair((start_y, end_y));
    (min_x, max_x, min_y, max_y)
}
