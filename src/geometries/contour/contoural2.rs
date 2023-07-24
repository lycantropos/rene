use crate::traits::{
    Contoural2, Elemental, Multisegmental2, Multivertexal2, Segmental,
};

use super::types::Contour;

impl<Scalar> Contoural2 for &Contour<Scalar>
where
    Self: Multisegmental2 + Multivertexal2,
    for<'a> &'a <Self as Multivertexal2>::IndexVertex: Elemental,
    for<'a> &'a <Self as Multisegmental2>::IndexSegment: Segmental,
{
}

impl<Scalar> Contoural2 for Contour<Scalar>
where
    Self: Multisegmental2 + Multivertexal2,
    for<'a> &'a <Self as Multivertexal2>::IndexVertex: Elemental,
    for<'a> &'a <Self as Multisegmental2>::IndexSegment: Segmental,
{
}
