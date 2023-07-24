use crate::traits::{Contoural, Multisegmental, Multivertexal};

use super::types::Contour;

impl<Scalar> Contoural for &Contour<Scalar> where
    Self: Multisegmental + Multivertexal
{
}

impl<Scalar> Contoural for Contour<Scalar> where
    Self: Multisegmental + Multivertexal
{
}
