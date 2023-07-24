use std::ops::Index;

pub trait Iterable {
    type Item;
    type Output<'a>: Iterator<Item = &'a Self::Item>
    where
        Self: 'a,
        Self::Item: 'a;

    fn iter(&self) -> Self::Output<'_>;
}

pub trait Lengthsome {
    type Output;

    fn len(&self) -> Self::Output;
}

pub trait Sequence:
    Index<usize, Output = Self::IndexItem>
    + IntoIterator<Item = Self::IntoIteratorItem>
    + Iterable<Item = Self::IndexItem>
    + Lengthsome<Output = usize>
{
    type IndexItem;
    type IntoIteratorItem;
}

impl<T> Iterable for Vec<T> {
    type Item = T;
    type Output<'a> = std::slice::Iter<'a, T>
    where
        Self: 'a,
        T: 'a;

    fn iter(&self) -> Self::Output<'_> {
        self.as_slice().iter()
    }
}

impl<T> Lengthsome for Vec<T> {
    type Output = usize;

    fn len(&self) -> Self::Output {
        Vec::<T>::len(self)
    }
}

impl<T> Sequence for Vec<T> {
    type IndexItem = T;
    type IntoIteratorItem = T;
}

pub trait Elemental {
    type Coordinate;

    fn coordinates(self) -> (Self::Coordinate, Self::Coordinate);
    fn x(self) -> Self::Coordinate;
    fn y(self) -> Self::Coordinate;
}

pub trait Segmental {
    type Endpoint: Elemental;

    fn start(self) -> Self::Endpoint;
    fn end(self) -> Self::Endpoint;
    fn endpoints(self) -> (Self::Endpoint, Self::Endpoint);
}

pub trait Multisegmental {
    type Segment: Segmental;
    type Segments: Iterator<Item = Self::Segment>;

    fn segments(self) -> Self::Segments;
    fn segments_count(self) -> usize;
}

pub trait Multivertexal {
    type Vertex: Elemental;
    type Vertices: Iterator<Item = Self::Vertex>;

    fn vertices(self) -> Self::Vertices;
    fn vertices_count(self) -> usize;
}

pub trait Contoural: Multisegmental + Multivertexal {}

pub trait Polygonal {
    type Contour: Contoural;
    type Holes: Iterator<Item = Self::Contour>;

    fn border(self) -> Self::Contour;
    fn holes(self) -> Self::Holes;
    fn holes_count(self) -> usize;
}

pub trait Multipolygonal {
    type Polygon: Polygonal;
    type Polygons: Iterator<Item = Self::Polygon>;

    fn polygons(self) -> Self::Polygons;
    fn polygons_count(self) -> usize;
}

pub trait Multisegmental2
where
    for<'a> &'a Self::IndexSegment: Segmental,
{
    type IndexSegment;
    type IntoIteratorSegment: Segmental;
    type Segments: Sequence<
        IndexItem = Self::IndexSegment,
        IntoIteratorItem = Self::IntoIteratorSegment,
    >;

    fn segments2(self) -> Self::Segments;
}

pub trait Multivertexal2
where
    for<'a> &'a Self::IndexVertex: Elemental,
{
    type IndexVertex;
    type IntoIteratorVertex: Elemental;
    type Vertices: Sequence<
        IndexItem = Self::IndexVertex,
        IntoIteratorItem = Self::IntoIteratorVertex,
    >;

    fn vertices2(self) -> Self::Vertices;
}

pub trait Contoural2: Multisegmental2 + Multivertexal2
where
    for<'a> &'a Self::IndexSegment: Segmental,
    for<'a> &'a Self::IndexVertex: Elemental,
{
}

pub trait Polygonal2
where
    for<'a, 'b> &'a <&'b Self::IndexHole as Multisegmental2>::IndexSegment:
        Segmental,
    for<'a, 'b> &'a <&'b Self::IndexHole as Multivertexal2>::IndexVertex:
        Elemental,
    for<'a> &'a <Self::Contour as Multisegmental2>::IndexSegment: Segmental,
    for<'a> &'a <Self::Contour as Multivertexal2>::IndexVertex: Elemental,
    for<'a> &'a <Self::IntoIteratorHole as Multisegmental2>::IndexSegment:
        Segmental,
    for<'a> &'a <Self::IntoIteratorHole as Multivertexal2>::IndexVertex:
        Elemental,
    for<'a> &'a Self::IndexHole: Contoural2,
{
    type Contour: Contoural2;
    type IndexHole;
    type IntoIteratorHole: Contoural2;
    type Holes: Sequence<
        IndexItem = Self::IndexHole,
        IntoIteratorItem = Self::IntoIteratorHole,
    >;

    fn border2(self) -> Self::Contour;
    fn holes2(self) -> Self::Holes;
}

pub trait Multipolygonal2
where
    for<'a, 'b> &'a <&'b <Self::IntoIteratorPolygon as Polygonal2>::IndexHole as Multisegmental2>::IndexSegment: Segmental,
    for<'a, 'b> &'a <&'b <Self::IntoIteratorPolygon as Polygonal2>::IndexHole as Multivertexal2>::IndexVertex: Elemental,
    for<'a> &'a <<Self::IntoIteratorPolygon as Polygonal2>::Contour as Multisegmental2>::IndexSegment: Segmental,
    for<'a> &'a <<Self::IntoIteratorPolygon as Polygonal2>::Contour as Multivertexal2>::IndexVertex: Elemental,
    for<'a> &'a <<Self::IntoIteratorPolygon as Polygonal2>::IntoIteratorHole as Multisegmental2>::IndexSegment: Segmental,
    for<'a> &'a <<Self::IntoIteratorPolygon as Polygonal2>::IntoIteratorHole as Multivertexal2>::IndexVertex: Elemental,
    for<'a> &'a <Self::IntoIteratorPolygon as Polygonal2>::IndexHole: Contoural2,
    for<'a> &'a Self::IndexPolygon: Polygonal2,
{
    type IndexPolygon;
    type IntoIteratorPolygon: Polygonal2;
    type Polygons: Sequence<
        IndexItem = Self::IndexPolygon,
        IntoIteratorItem = Self::IntoIteratorPolygon,
    >;

    fn polygons2(self) -> Self::Polygons;
}

pub type ElementalCoordinate<T> = <T as Elemental>::Coordinate;
pub type SegmentalCoordinate<T> = ElementalCoordinate<SegmentalEndpoint<T>>;
pub type SegmentalEndpoint<T> = <T as Segmental>::Endpoint;
pub type MultisegmentalCoordinate<T> =
    SegmentalCoordinate<MultisegmentalSegment<T>>;
pub type MultisegmentalSegment<T> = <T as Multisegmental>::Segment;
pub type MultivertexalCoordinate<T> =
    ElementalCoordinate<MultivertexalVertex<T>>;
pub type MultivertexalVertex<T> = <T as Multivertexal>::Vertex;
pub type PolygonalCoordinate<T> = MultivertexalCoordinate<PolygonalContour<T>>;
pub type PolygonalSegment<T> = MultisegmentalSegment<PolygonalContour<T>>;
pub type PolygonalVertex<T> = MultivertexalVertex<PolygonalContour<T>>;
pub type PolygonalContour<T> = <T as Polygonal>::Contour;
pub type MultipolygonalCoordinate<T> =
    PolygonalCoordinate<MultipolygonalPolygon<T>>;
pub type MultipolygonalVertex<T> = PolygonalVertex<MultipolygonalPolygon<T>>;
pub type MultipolygonalContour<T> = PolygonalContour<MultipolygonalPolygon<T>>;
pub type MultipolygonalPolygon<T> = <T as Multipolygonal>::Polygon;

pub type Multisegmental2IndexSegment<T> = <T as Multisegmental2>::IndexSegment;
pub type Multivertexal2IndexVertex<T> = <T as Multivertexal2>::IndexVertex;
pub type Polygonal2IntoIteratorHole<T> = <T as Polygonal2>::IntoIteratorHole;

pub trait Intersection<Other = Self> {
    type Output;

    fn intersection(self, other: Other) -> Self::Output;
}

pub trait Difference<Other = Self> {
    type Output;

    fn difference(self, other: Other) -> Self::Output;
}

pub trait SymmetricDifference<Other = Self> {
    type Output;

    fn symmetric_difference(self, other: Other) -> Self::Output;
}

pub trait Union<Other = Self> {
    type Output;

    fn union(self, other: Other) -> Self::Output;
}
