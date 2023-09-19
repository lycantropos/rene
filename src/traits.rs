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

pub trait Multisegmental
where
    for<'a> &'a Self::IndexSegment: Segmental,
{
    type IndexSegment;
    type IntoIteratorSegment: Segmental;
    type Segments: Sequence<
        IndexItem = Self::IndexSegment,
        IntoIteratorItem = Self::IntoIteratorSegment,
    >;

    fn segments(self) -> Self::Segments;
}

pub trait Multivertexal
where
    for<'a> &'a Self::IndexVertex: Elemental,
{
    type IndexVertex;
    type IntoIteratorVertex: Elemental;
    type Vertices: Sequence<
        IndexItem = Self::IndexVertex,
        IntoIteratorItem = Self::IntoIteratorVertex,
    >;

    fn vertices(self) -> Self::Vertices;
}

pub trait Contoural: Multisegmental + Multivertexal
where
    for<'a> &'a Self::IndexSegment: Segmental,
    for<'a> &'a Self::IndexVertex: Elemental,
{
}

pub trait Polygonal
where
    for<'a> &'a Self::IndexHole: Contoural,
    for<'a> &'a MultisegmentalIndexSegment<Self::Contour>: Segmental,
    for<'a> &'a MultivertexalIndexVertex<Self::Contour>: Elemental,
    for<'a> &'a MultisegmentalIndexSegment<Self::IntoIteratorHole>: Segmental,
    for<'a> &'a MultivertexalIndexVertex<Self::IntoIteratorHole>: Elemental,
    for<'a, 'b> &'a MultisegmentalIndexSegment<&'b Self::IndexHole>: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<&'b Self::IndexHole>: Elemental,
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
    for<'a> &'a MultisegmentalIndexSegment<
        PolygonalContour<Self::IntoIteratorPolygon>,
    >: Segmental,
    for<'a> &'a MultivertexalIndexVertex<PolygonalContour<Self::IntoIteratorPolygon>>:
        Elemental,
    for<'a> &'a MultisegmentalIndexSegment<
        PolygonalIntoIteratorHole<Self::IntoIteratorPolygon>,
    >: Segmental,
    for<'a> &'a MultivertexalIndexVertex<
        PolygonalIntoIteratorHole<Self::IntoIteratorPolygon>,
    >: Elemental,
    for<'a> &'a PolygonalIndexHole<Self::IntoIteratorPolygon>: Contoural,
    for<'a> &'a Self::IndexPolygon: Polygonal,
    for<'a, 'b> &'a MultisegmentalIndexSegment<
        &'b PolygonalIndexHole<Self::IntoIteratorPolygon>,
    >: Segmental,
    for<'a, 'b> &'a MultivertexalIndexVertex<
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
pub type MultipolygonalIntoIteratorPolygon<T> =
    <T as Multipolygonal>::IntoIteratorPolygon;
pub type MultisegmentalIndexSegment<T> = <T as Multisegmental>::IndexSegment;
pub type MultivertexalIndexVertex<T> = <T as Multivertexal>::IndexVertex;
pub type PolygonalContour<T> = <T as Polygonal>::Contour;
pub type PolygonalHoles<T> = <T as Polygonal>::Holes;
pub type PolygonalIndexHole<T> = <T as Polygonal>::IndexHole;
pub type PolygonalIntoIteratorHole<T> = <T as Polygonal>::IntoIteratorHole;
pub type SegmentalCoordinate<T> = ElementalCoordinate<SegmentalEndpoint<T>>;
pub type SegmentalEndpoint<T> = <T as Segmental>::Endpoint;

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
