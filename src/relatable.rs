#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Relation {
    /// geometry is a strict subset of the other
    /// and interior/boundary of the geometry is a subset
    /// of interior/boundary of the other
    Component,
    /// geometry is a strict superset of the other
    /// and interior/boundary of the geometry is a superset
    /// of interior/boundary of the other
    Composite,
    /// interior of the geometry is a superset of the other
    Cover,
    /// intersection is a strict subset of each of the geometries,
    /// has dimension less than at least of one of the geometries,
    /// one of the geometries lies in interior & exterior of the other geometry
    /// or boundaries cross
    Cross,
    /// at least one geometry is non-empty and intersection is empty
    Disjoint,
    /// at least one boundary point of the geometry
    /// lies on the boundary of the other, but not all,
    /// other points of the geometry lie in the interior of the other
    Enclosed,
    /// boundary of the geometry contains
    /// at least one boundary point of the other, but not all,
    /// interior of the geometry contains other points of the other
    Encloses,
    /// geometries are equal
    Equal,
    /// intersection is a strict subset of each of the geometries
    /// and has the same dimension as geometries
    Overlap,
    /// intersection is a strict subset of each of the geometries,
    /// has dimension less than at least of one of the geometries
    /// and only boundaries intersect, but do not cross
    Touch,
    /// geometry is a subset of the interior of the other
    Within,
}

pub trait Relatable<Other = Self> {
    fn component_of(self, other: Other) -> bool
    where
        Self: Sized,
    {
        self.relate_to(other) == Relation::Component
    }

    fn composite_with(self, other: Other) -> bool
    where
        Self: Sized,
    {
        self.relate_to(other) == Relation::Composite
    }

    fn covers(self, other: Other) -> bool
    where
        Self: Sized,
    {
        self.relate_to(other) == Relation::Cover
    }

    fn crosses(self, other: Other) -> bool
    where
        Self: Sized,
    {
        self.relate_to(other) == Relation::Cross
    }

    fn disjoint_with(self, other: Other) -> bool
    where
        Self: Sized,
    {
        self.relate_to(other) == Relation::Disjoint
    }

    fn enclosed_by(self, other: Other) -> bool
    where
        Self: Sized,
    {
        self.relate_to(other) == Relation::Enclosed
    }

    fn encloses(self, other: Other) -> bool
    where
        Self: Sized,
    {
        self.relate_to(other) == Relation::Encloses
    }

    fn equals_to(self, other: Other) -> bool
    where
        Self: Sized,
    {
        self.relate_to(other) == Relation::Equal
    }

    fn overlaps(self, other: Other) -> bool
    where
        Self: Sized,
    {
        self.relate_to(other) == Relation::Overlap
    }

    fn touches(self, other: Other) -> bool
    where
        Self: Sized,
    {
        self.relate_to(other) == Relation::Touch
    }

    fn within(self, other: Other) -> bool
    where
        Self: Sized,
    {
        self.relate_to(other) == Relation::Within
    }

    fn relate_to(self, other: Other) -> Relation;
}
