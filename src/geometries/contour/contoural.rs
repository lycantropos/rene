use crate::traits::{
    Contoural, Elemental, Multisegmental2, Multivertexal2, Segmental,
};

use super::types::Contour;

impl<Scalar> Contoural for &Contour<Scalar>
where
    Self: Multisegmental2 + Multivertexal2,
    for<'a> &'a <Self as Multivertexal2>::IndexVertex: Elemental,
    for<'a> &'a <Self as Multisegmental2>::IndexSegment: Segmental,
{
}

impl<Scalar> Contoural for Contour<Scalar>
where
    Self: Multisegmental2 + Multivertexal2,
    for<'a> &'a <Self as Multivertexal2>::IndexVertex: Elemental,
    for<'a> &'a <Self as Multisegmental2>::IndexSegment: Segmental,
{
}
