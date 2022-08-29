#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Relation {
    /// intersection is empty
    Disjoint,
    /// intersection is a strict subset of each of the geometries,
    /// has dimension less than at least of one of the geometries
    /// and only boundaries intersect, but do not cross
    Touch,
    /// intersection is a strict subset of each of the geometries,
    /// has dimension less than at least of one of the geometries,
    /// one of the geometries lies in interior & exterior of the other geometry
    /// or boundaries cross
    Cross,
    /// intersection is a strict subset of each of the geometries
    /// and has the same dimension as geometries
    Overlap,
    /// interior of the geometry is a superset of the other
    Cover,
    /// boundary of the geometry contains
    /// at least one boundary point of the other, but not all,
    /// interior of the geometry contains other points of the other
    Encloses,
    /// geometry is a strict superset of the other
    /// and interior/boundary of the geometry is a superset
    /// of interior/boundary of the other
    Composite,
    /// geometries are equal
    Equal,
    /// geometry is a strict subset of the other
    /// and interior/boundary of the geometry is a subset
    /// of interior/boundary of the other
    Component,
    /// at least one boundary point of the geometry
    /// lies on the boundary of the other, but not all,
    /// other points of the geometry lie in the interior of the other
    Enclosed,
    /// geometry is a subset of the interior of the other
    Within,
}

pub trait Relatable<Other = Self> {
    fn relate_to(self, other: Other) -> Relation;
}
