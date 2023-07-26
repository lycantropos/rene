use crate::bounded;
use crate::bounded::Bounded;
use crate::geometries::Polygon;
use crate::operations::{coordinates_iterator_to_bounds, merge_bounds};
use crate::traits::{
    Contoural, Elemental, Iterable, MultisegmentalIndexSegment,
    MultivertexalIndexVertex, Polygonal, PolygonalIndexHole,
    PolygonalIntoIteratorHole, Segmental,
};

use super::types::Multipolygon;

impl<'a, Scalar: Ord> Bounded<&'a Scalar> for &'a Multipolygon<Scalar>
where
    &'a Polygon<Scalar>: Bounded<&'a Scalar>,
{
    fn to_bounding_box(self) -> bounded::Box<&'a Scalar> {
        bounded::Box::new(
            self.to_min_x(),
            self.to_max_x(),
            self.to_min_y(),
            self.to_max_y(),
        )
    }

    fn to_max_x(self) -> &'a Scalar {
        unsafe {
            self.polygons
                .iter()
                .map(Bounded::to_max_x)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_max_y(self) -> &'a Scalar {
        unsafe {
            self.polygons
                .iter()
                .map(Bounded::to_max_y)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_min_x(self) -> &'a Scalar {
        unsafe {
            self.polygons
                .iter()
                .map(Bounded::to_min_x)
                .min()
                .unwrap_unchecked()
        }
    }

    fn to_min_y(self) -> &'a Scalar {
        unsafe {
            self.polygons
                .iter()
                .map(Bounded::to_min_y)
                .min()
                .unwrap_unchecked()
        }
    }
}

impl<Contour, Point, Scalar: Ord> Bounded<Scalar> for Multipolygon<Scalar>
where
    Contour: Contoural<IntoIteratorVertex = Point>,
    Point: Elemental<Coordinate = Scalar>,
    Polygon<Scalar>: Bounded<Scalar> + Polygonal<Contour = Contour>,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b PolygonalIndexHole<Polygon<Scalar>>>:
        Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b PolygonalIndexHole<Polygon<Scalar>>>:
        Elemental,
    for<'a> &'a MultisegmentalIndexSegment<PolygonalIntoIteratorHole<Polygon<Scalar>>>:
        Segmental,
    for<'a> &'a MultivertexalIndexVertex<PolygonalIntoIteratorHole<Polygon<Scalar>>>:
        Elemental,
    for<'a> &'a MultisegmentalIndexSegment<Contour>: Segmental,
    for<'a> &'a MultivertexalIndexVertex<Contour>: Elemental,
    for<'a> &'a PolygonalIndexHole<Polygon<Scalar>>: Contoural,
{
    fn to_bounding_box(self) -> bounded::Box<Scalar> {
        let (min_x, max_x, min_y, max_y) =
            merge_bounds(self.polygons.into_iter().map(|polygon| {
                coordinates_iterator_to_bounds(
                    polygon
                        .border()
                        .vertices()
                        .into_iter()
                        .map(Elemental::coordinates),
                )
            }));
        bounded::Box::new(min_x, max_x, min_y, max_y)
    }

    fn to_max_x(self) -> Scalar {
        unsafe {
            self.polygons
                .into_iter()
                .map(Bounded::to_max_x)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_max_y(self) -> Scalar {
        unsafe {
            self.polygons
                .into_iter()
                .map(Bounded::to_max_y)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_min_x(self) -> Scalar {
        unsafe {
            self.polygons
                .into_iter()
                .map(Bounded::to_min_x)
                .max()
                .unwrap_unchecked()
        }
    }

    fn to_min_y(self) -> Scalar {
        unsafe {
            self.polygons
                .into_iter()
                .map(Bounded::to_min_y)
                .max()
                .unwrap_unchecked()
        }
    }
}
