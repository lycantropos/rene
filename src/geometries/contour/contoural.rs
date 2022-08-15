use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::traits::{Contoural, Multisegmental, Multivertexal};

use super::types::Contour;

impl<Digit, const SEPARATOR: char, const SHIFT: usize> Contoural
    for Contour<Fraction<BigInt<Digit, SEPARATOR, SHIFT>>>
where
    Self: Multisegmental + Multivertexal,
{
}
