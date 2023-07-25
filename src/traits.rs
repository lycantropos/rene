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

pub trait Contoural: Multisegmental2 + Multivertexal2
where
    for<'a> &'a Self::IndexSegment: Segmental,
    for<'a> &'a Self::IndexVertex: Elemental,
{
}

pub trait Polygonal
where
    for<'a> &'a Self::IndexHole: Contoural,
    for<'a> &'a Multisegmental2IndexSegment<Self::Contour>: Segmental,
    for<'a> &'a Multivertexal2IndexVertex<Self::Contour>: Elemental,
    for<'a> &'a Multisegmental2IndexSegment<Self::IntoIteratorHole>: Segmental,
    for<'a> &'a Multivertexal2IndexVertex<Self::IntoIteratorHole>: Elemental,
    for<'a, 'b> &'a Multisegmental2IndexSegment<&'b Self::IndexHole>:
        Segmental,
    for<'a, 'b> &'a Multivertexal2IndexVertex<&'b Self::IndexHole>: Elemental,
{
    type Contour: Contoural;
    type IndexHole;
    type IntoIteratorHole: Contoural;
    type Holes: Sequence<
        IndexItem = Self::IndexHole,
        IntoIteratorItem = Self::IntoIteratorHole,
    >;

    fn border(self) -> Self::Contour;
    fn components(self) -> (Self::Contour, Self::Holes);
    fn holes(self) -> Self::Holes;
}

pub trait Multipolygonal
where
    for<'a> &'a Multisegmental2IndexSegment<
        PolygonalContour<Self::IntoIteratorPolygon>,
    >: Segmental,
    for<'a> &'a Multivertexal2IndexVertex<PolygonalContour<Self::IntoIteratorPolygon>>:
        Elemental,
    for<'a> &'a Multisegmental2IndexSegment<
        PolygonalIntoIteratorHole<Self::IntoIteratorPolygon>,
    >: Segmental,
    for<'a> &'a Multivertexal2IndexVertex<
        PolygonalIntoIteratorHole<Self::IntoIteratorPolygon>,
    >: Elemental,
    for<'a> &'a PolygonalIndexHole<Self::IntoIteratorPolygon>: Contoural,
    for<'a> &'a Self::IndexPolygon: Polygonal,
    for<'a, 'b> &'a Multisegmental2IndexSegment<
        &'b PolygonalIndexHole<Self::IntoIteratorPolygon>,
    >: Segmental,
    for<'a, 'b> &'a Multivertexal2IndexVertex<
        &'b PolygonalIndexHole<Self::IntoIteratorPolygon>,
    >: Elemental,
{
    type IndexPolygon;
    type IntoIteratorPolygon: Polygonal;
    type Polygons: Sequence<
        IndexItem = Self::IndexPolygon,
        IntoIteratorItem = Self::IntoIteratorPolygon,
    >;

    fn polygons(self) -> Self::Polygons;
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

pub type Multisegmental2IndexSegment<T> = <T as Multisegmental2>::IndexSegment;
pub type Multivertexal2IndexVertex<T> = <T as Multivertexal2>::IndexVertex;
pub type PolygonalContour<T> = <T as Polygonal>::Contour;
pub type PolygonalIndexHole<T> = <T as Polygonal>::IndexHole;
pub type PolygonalIntoIteratorHole<T> = <T as Polygonal>::IntoIteratorHole;

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
