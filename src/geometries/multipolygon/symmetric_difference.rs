use crate::bounded::{Bounded, Box};
use crate::clipping::shaped::Operation;
use crate::clipping::traits::ReduceEvents;
use crate::clipping::{Event, SYMMETRIC_DIFFERENCE};
use crate::geometries::{Empty, Point, Polygon};
use crate::operations::{
    do_boxes_have_no_common_continuum, flags_to_false_indices,
    flags_to_true_indices, merge_boxes, to_boxes_have_common_continuum,
};
use crate::relatable::Relatable;
use crate::traits::{Elemental, SymmetricDifference};

use super::types::Multipolygon;

impl<Scalar> SymmetricDifference<Empty> for Multipolygon<Scalar> {
    type Output = Self;

    fn symmetric_difference(self, _other: Empty) -> Self::Output {
        self
    }
}

impl<Scalar> SymmetricDifference<&Empty> for Multipolygon<Scalar> {
    type Output = Self;

    fn symmetric_difference(self, _other: &Empty) -> Self::Output {
        self
    }
}

impl<Scalar> SymmetricDifference<Empty> for &Multipolygon<Scalar>
where
    Multipolygon<Scalar>: Clone,
{
    type Output = Multipolygon<Scalar>;

    fn symmetric_difference(self, _other: Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> SymmetricDifference<&Empty> for &Multipolygon<Scalar>
where
    Multipolygon<Scalar>: Clone,
{
    type Output = Multipolygon<Scalar>;

    fn symmetric_difference(self, _other: &Empty) -> Self::Output {
        self.clone()
    }
}

impl<Scalar> SymmetricDifference for &Multipolygon<Scalar>
where
    Scalar: Clone + Ord,
    Multipolygon<Scalar>: Clone,
    Operation<Point<Scalar>, SYMMETRIC_DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Polygon<Scalar>>>
        + for<'a> From<(&'a [&'a Polygon<Scalar>], &'a [&'a Polygon<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Polygon<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multipolygon<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Polygon<Scalar>>;

    fn symmetric_difference(self, other: Self) -> Self::Output {
        let bounding_boxes = self
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let other_bounding_boxes = other
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let bounding_box = merge_boxes(&bounding_boxes);
        let other_bounding_box = merge_boxes(&other_bounding_boxes);
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            let mut result = self.polygons.clone();
            result.extend_from_slice(&other.polygons);
            return result;
        }
        let boxes_have_common_continuum = to_boxes_have_common_continuum(
            &bounding_boxes,
            &other_bounding_box,
        );
        let common_continuum_polygons_ids =
            flags_to_true_indices(&boxes_have_common_continuum);
        if common_continuum_polygons_ids.is_empty() {
            let mut result = self.polygons.clone();
            result.extend_from_slice(&other.polygons);
            return result;
        }
        let other_boxes_have_common_continuum = to_boxes_have_common_continuum(
            &other_bounding_boxes,
            &bounding_box,
        );
        let other_common_continuum_polygons_ids =
            flags_to_true_indices(&other_boxes_have_common_continuum);
        if other_common_continuum_polygons_ids.is_empty() {
            let mut result = self.polygons.clone();
            result.extend_from_slice(&other.polygons);
            return result;
        }
        let common_continuum_polygons = common_continuum_polygons_ids
            .into_iter()
            .map(|index| &self.polygons[index])
            .collect::<Vec<_>>();
        let other_common_continuum_polygons =
            other_common_continuum_polygons_ids
                .into_iter()
                .map(|index| &other.polygons[index])
                .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, SYMMETRIC_DIFFERENCE>::from(
            (&common_continuum_polygons, &other_common_continuum_polygons),
        );
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        for event in operation.by_ref() {
            events.push(event);
        }
        let mut result = operation.reduce_events(events);
        result.reserve(
            (self.polygons.len() - common_continuum_polygons.len())
                + (other.polygons.len()
                    - other_common_continuum_polygons.len()),
        );
        result.extend(
            flags_to_false_indices(&boxes_have_common_continuum)
                .into_iter()
                .map(|index| self.polygons[index].clone()),
        );
        result.extend(
            flags_to_false_indices(&other_boxes_have_common_continuum)
                .into_iter()
                .map(|index| other.polygons[index].clone()),
        );
        result
    }
}

impl<Scalar> SymmetricDifference<&Polygon<Scalar>> for &Multipolygon<Scalar>
where
    Scalar: Clone + Ord,
    Operation<Point<Scalar>, SYMMETRIC_DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Polygon<Scalar>>>
        + for<'a> From<(&'a [&'a Polygon<Scalar>], &'a Polygon<Scalar>)>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Polygon<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multipolygon<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Polygon<Scalar>>;

    fn symmetric_difference(self, other: &Polygon<Scalar>) -> Self::Output {
        let bounding_boxes = self
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let bounding_box = merge_boxes(&bounding_boxes);
        let other_bounding_box = other.to_bounding_box();
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            let mut result = self.polygons.clone();
            result.push(other.clone());
            return result;
        }
        let boxes_have_common_continuum = to_boxes_have_common_continuum(
            &bounding_boxes,
            &other_bounding_box,
        );
        let common_continuum_polygons_ids =
            flags_to_true_indices(&boxes_have_common_continuum);
        if common_continuum_polygons_ids.is_empty() {
            let mut result = self.polygons.clone();
            result.push(other.clone());
            return result;
        }
        let common_continuum_polygons = common_continuum_polygons_ids
            .into_iter()
            .map(|index| &self.polygons[index])
            .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, SYMMETRIC_DIFFERENCE>::from(
            (&common_continuum_polygons, other),
        );
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        for event in operation.by_ref() {
            events.push(event);
        }
        let mut result = operation.reduce_events(events);
        result.reserve(self.polygons.len() - common_continuum_polygons.len());
        result.extend(
            flags_to_false_indices(&boxes_have_common_continuum)
                .into_iter()
                .map(|index| self.polygons[index].clone()),
        );
        result
    }
}

impl<Scalar> SymmetricDifference<&Multipolygon<Scalar>> for &Polygon<Scalar>
where
    Scalar: Clone + Ord,
    Operation<Point<Scalar>, SYMMETRIC_DIFFERENCE>: Iterator<Item = Event>
        + ReduceEvents<Output = Vec<Polygon<Scalar>>>
        + for<'a> From<(&'a Polygon<Scalar>, &'a [&'a Polygon<Scalar>])>,
    Point<Scalar>: Elemental<Coordinate = Scalar>,
    Polygon<Scalar>: Clone,
    for<'a> &'a Box<&'a Scalar>: Relatable,
    for<'a> &'a Multipolygon<Scalar>: Bounded<&'a Scalar>,
    for<'a> &'a Polygon<Scalar>: Bounded<&'a Scalar>,
{
    type Output = Vec<Polygon<Scalar>>;

    fn symmetric_difference(
        self,
        other: &Multipolygon<Scalar>,
    ) -> Self::Output {
        let other_bounding_boxes = other
            .polygons
            .iter()
            .map(Bounded::to_bounding_box)
            .collect::<Vec<_>>();
        let bounding_box = self.to_bounding_box();
        let other_bounding_box = merge_boxes(&other_bounding_boxes);
        if do_boxes_have_no_common_continuum(
            &bounding_box,
            &other_bounding_box,
        ) {
            let mut result = other.polygons.clone();
            result.push(self.clone());
            return result;
        }
        let other_boxes_have_common_continuum = to_boxes_have_common_continuum(
            &other_bounding_boxes,
            &other_bounding_box,
        );
        let other_common_continuum_polygons_ids =
            flags_to_true_indices(&other_boxes_have_common_continuum);
        if other_common_continuum_polygons_ids.is_empty() {
            let mut result = other.polygons.clone();
            result.push(self.clone());
            return result;
        }
        let other_common_continuum_polygons =
            other_common_continuum_polygons_ids
                .into_iter()
                .map(|index| &other.polygons[index])
                .collect::<Vec<_>>();
        let mut operation = Operation::<Point<_>, SYMMETRIC_DIFFERENCE>::from(
            (self, &other_common_continuum_polygons),
        );
        let mut events = {
            let (_, maybe_events_count) = operation.size_hint();
            debug_assert!(maybe_events_count.is_some());
            Vec::with_capacity(unsafe {
                maybe_events_count.unwrap_unchecked()
            })
        };
        for event in operation.by_ref() {
            events.push(event);
        }
        let mut result = operation.reduce_events(events);
        result.reserve(
            other.polygons.len() - other_common_continuum_polygons.len(),
        );
        result.extend(
            flags_to_false_indices(&other_boxes_have_common_continuum)
                .into_iter()
                .map(|index| other.polygons[index].clone()),
        );
        result
    }
}
