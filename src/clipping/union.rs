use crate::bounded::Bounded;
use crate::operations::Orient;
use crate::relatable::Relatable;
use crate::traits::{Polygonal, PolygonalContour, PolygonalCoordinate, PolygonalVertex, Union};

use super::event::Event;
use super::events_queue_key::EventsQueueKey;
use super::operation::Operation;
use super::operation_kind::UNION;

impl<
        Polygon: Bounded<PolygonalCoordinate<Polygon>>
            + Clone
            + From<(PolygonalContour<Polygon>, Vec<PolygonalContour<Polygon>>)>
            + Polygonal,
    > Union for &Polygon
where
    for<'a> Operation<PolygonalVertex<Polygon>, UNION>:
        From<(&'a Polygon, &'a Polygon)> + Iterator<Item = Event>,
    PolygonalContour<Polygon>: From<Vec<PolygonalVertex<Polygon>>>,
    PolygonalCoordinate<Polygon>: Ord,
    PolygonalVertex<Polygon>: Clone + Orient + PartialEq,
    EventsQueueKey<PolygonalVertex<Polygon>>: Ord,
{
    type Output = Vec<Polygon>;

    fn union(self, other: Self) -> Self::Output {
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = other.to_bounding_box();
        if bounding_box.disjoint_with(&other_bounding_box)
            || bounding_box.touches(&other_bounding_box)
        {
            return vec![self.clone(), other.clone()];
        }
        let mut operation = Operation::<PolygonalVertex<Polygon>, UNION>::from((self, other));
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe { maybe_events_count.unwrap_unchecked() })
        };
        while let Some(event) = operation.next() {
            events.push(event)
        }
        operation.events_to_polygons(events)
    }
}
