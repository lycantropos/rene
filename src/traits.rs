pub trait Elemental {
    type Coordinate;

    fn x(&self) -> Self::Coordinate;
    fn y(&self) -> Self::Coordinate;
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

pub type ElementalCoordinate<T> = <T as Elemental>::Coordinate;
pub type SegmentalCoordinate<T> = ElementalCoordinate<SegmentalEndpoint<T>>;
pub type SegmentalEndpoint<T> = <T as Segmental>::Endpoint;
pub type MultisegmentalCoordinate<T> = SegmentalCoordinate<MultisegmentalSegment<T>>;
pub type MultisegmentalSegment<T> = <T as Multisegmental>::Segment;
pub type MultivertexalCoordinate<T> = ElementalCoordinate<MultivertexalVertex<T>>;
pub type MultivertexalVertex<T> = <T as Multivertexal>::Vertex;
pub type PolygonalCoordinate<T> = MultivertexalCoordinate<PolygonalContour<T>>;
pub type PolygonalSegment<T> = MultisegmentalSegment<PolygonalContour<T>>;
pub type PolygonalVertex<T> = MultivertexalVertex<PolygonalContour<T>>;
pub type PolygonalContour<T> = <T as Polygonal>::Contour;
pub type MultipolygonalCoordinate<T> = PolygonalCoordinate<MultipolygonalPolygon<T>>;
pub type MultipolygonalVertex<T> = PolygonalVertex<MultipolygonalPolygon<T>>;
pub type MultipolygonalContour<T> = PolygonalContour<MultipolygonalPolygon<T>>;
pub type MultipolygonalPolygon<T> = <T as Multipolygonal>::Polygon;

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
