pub trait Elemental {
    type Coordinate;

    fn x(&self) -> Self::Coordinate;
    fn y(&self) -> Self::Coordinate;
}

pub trait Segmental {
    type Endpoint: Elemental;

    fn start(&self) -> Self::Endpoint;
    fn end(&self) -> Self::Endpoint;
}

pub trait Multisegmental {
    type Segment: Segmental;

    fn segments(&self) -> Vec<Self::Segment>;
    fn segments_count(&self) -> usize;
}

pub trait Multivertexal {
    type Vertex: Elemental;

    fn vertices(&self) -> Vec<Self::Vertex>;
    fn vertices_count(&self) -> usize;
}

pub trait Contoural: Multisegmental + Multivertexal {}

pub trait Polygonal {
    type Contour: Contoural;

    fn border(&self) -> Self::Contour;
    fn holes(&self) -> Vec<Self::Contour>;
    fn holes_count(&self) -> usize;
}

pub trait Multipolygonal {
    type Polygon: Polygonal;

    fn polygons(&self) -> Vec<Self::Polygon>;
    fn polygons_count(&self) -> usize;
}

pub trait Intersection<Other = Self> {
    type Output;

    fn intersection(self, other: Other) -> Self::Output;
}
