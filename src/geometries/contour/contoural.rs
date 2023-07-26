use crate::traits::{
    Contoural, Elemental, Multisegmental, Multivertexal, Segmental,
};

use super::types::Contour;

impl<Scalar> Contoural for &Contour<Scalar>
where
    Self: Multisegmental + Multivertexal,
    for<'a> &'a <Self as Multivertexal>::IndexVertex: Elemental,
    for<'a> &'a <Self as Multisegmental>::IndexSegment: Segmental,
{
}

impl<Scalar> Contoural for Contour<Scalar>
where
    Self: Multisegmental + Multivertexal,
    for<'a> &'a <Self as Multivertexal>::IndexVertex: Elemental,
    for<'a> &'a <Self as Multisegmental>::IndexSegment: Segmental,
{
}
