use crate::bounded::Bounded;
use crate::operations::Orient;
use crate::relatable::{Relatable, Relation};
use crate::traits::{
    Elemental, Intersection, Polygonal, PolygonalContour, PolygonalCoordinate, PolygonalVertex,
};

use super::event::Event;
use super::events_queue_key::EventsQueueKey;
use super::operation::Operation;
use super::operation_kind::INTERSECTION;

impl<
        Polygon: Bounded<PolygonalCoordinate<Polygon>>
            + From<(PolygonalContour<Polygon>, Vec<PolygonalContour<Polygon>>)>
            + Polygonal,
    > Intersection for &Polygon
where
    for<'a> Operation<PolygonalVertex<Polygon>, INTERSECTION>:
        From<(&'a Polygon, &'a Polygon)> + Iterator<Item = Event>,
    PolygonalContour<Polygon>: From<Vec<PolygonalVertex<Polygon>>>,
    PolygonalCoordinate<Polygon>: Ord,
    PolygonalVertex<Polygon>: Clone + Orient + PartialEq,
    EventsQueueKey<PolygonalVertex<Polygon>>: Ord,
{
    type Output = Vec<Polygon>;

    fn intersection(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if matches!(
            bounding_box.relate_to(&other_bounding_box),
            Relation::Disjoint | Relation::Touch
        ) {
            return vec![];
        }
        let min_max_x = bounding_box.get_max_x().min(other_bounding_box.get_max_x());
        let mut operation =
            Operation::<PolygonalVertex<Polygon>, INTERSECTION>::from((self, other));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        while let Some(event) = operation.next() {
            if operation.get_event_start(event).x().gt(min_max_x) {
                break;
            }
            events.push(event)
        }
        operation.events_to_polygons(events)
    }
}
