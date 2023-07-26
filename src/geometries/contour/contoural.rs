use crate::traits::{
    Contoural, Elemental, Multisegmental, Multivertexal2, Segmental,
};

use super::types::Contour;

impl<Scalar> Contoural for &Contour<Scalar>
where
    Self: Multisegmental + Multivertexal2,
    for<'a> &'a <Self as Multivertexal2>::IndexVertex: Elemental,
    for<'a> &'a <Self as Multisegmental>::IndexSegment: Segmental,
{
}

impl<Scalar> Contoural for Contour<Scalar>
where
    Self: Multisegmental + Multivertexal2,
    for<'a> &'a <Self as Multivertexal2>::IndexVertex: Elemental,
    for<'a> &'a <Self as Multisegmental>::IndexSegment: Segmental,
{
}
