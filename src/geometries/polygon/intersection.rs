use crate::bounded::{Bounded, Box};
use crate::clipping::shaped::Operation;
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{Event, INTERSECTION};
use crate::geometries::{Empty, Multipolygon, Point};
use crate::operations::{
    do_boxes_have_no_common_area, to_boxes_ids_with_common_area,
};
use crate::relatable::Relatable;
use crate::sweeping::traits::EventsContainer;
use crate::traits::{
    Elemental, Intersection, Iterable, Multipolygonal2, Polygonal,
};

use super::types::Polygon;

impl<Scalar> Intersection<Empty> for Polygon<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for Polygon<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar> Intersection<Empty> for &Polygon<Scalar> {
    type Output = Empty;

    fn intersection(self, other: Empty) -> Self::Output {
        other
    }
}

impl<Scalar> Intersection<&Empty> for &Polygon<Scalar> {
    type Output = Empty;

    fn intersection(self, other: &Empty) -> Self::Output {
        *other
    }
}

impl<Scalar> Intersection for &Polygon<Scalar>
where
    Scalar: Ord,
    Operation<Point<Scalar>, INTERSECTION>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Polygon<Scalar>>>
        + for<'a> From<(&'a Polygon<Scalar>, &'a Polygon<Scalar>)>,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Point<Scalar>: Elemental<Coordinate = &'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Polygon<Scalar>>;

    fn intersection(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return vec![];
        }
        let min_max_x =
            bounding_box.get_max_x().min(other_bounding_box.get_max_x());
        let mut operation =
            Operation::<Point<_>, INTERSECTION>::from((self, other));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(min_max_x) {
                break;
            }
            events.push(event);
        }
        operation.reduce_events(events)
    }
}

impl<Scalar> Intersection<&Multipolygon<Scalar>> for &Polygon<Scalar>
where
    Scalar: Clone + Ord,
    Operation<Point<Scalar>, INTERSECTION>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Polygon<Scalar>>>
        + for<'a> From<(&'a Polygon<Scalar>, &'a [&'a Polygon<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multipolygon<Scalar>: Bounded<&'a Scalar>
        + Multipolygonal2<
            IndexPolygon = Polygon<Scalar>,
            IntoIteratorPolygon = &'a Polygon<Scalar>,
        >,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar> + Polygonal,
{
    type Output = Vec<Polygon<Scalar>>;

    fn intersection(self, other: &Multipolygon<Scalar>) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_area(&bounding_box, &other_bounding_box) {
            return vec![];
        }
        let other_polygons = other.polygons2();
        let other_bounding_boxes = other_polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let other_common_area_polygons_ids = to_boxes_ids_with_common_area(
            &other_bounding_boxes,
            &bounding_box,
        );
        if other_common_area_polygons_ids.is_empty() {
            return vec![];
        }
        let min_max_x = bounding_box.get_max_x().min(unsafe {
            other_common_area_polygons_ids
                .iter()
                .map(|&index| other_bounding_boxes[index].get_max_x())
                .max()
                .unwrap_unchecked()
        });
        let other_common_area_polygons = other_common_area_polygons_ids
            .into_iter()
            .map(|index| &other_polygons[index])
            .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, INTERSECTION>::from((
            self,
            &other_common_area_polygons,
        ));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(min_max_x) {
                break;
            }
            events.push(event);
        }
        operation.reduce_events(events)
    }
}
