use rithm::big_int::BigInt;
use rithm::fraction::Fraction;

use crate::geometries::Segment;
use crate::traits::Multisegmental;

use super::types::Contour;

impl<Digit, const SHIFT: usize> Multisegmental for Contour<Fraction<BigInt<Digit, SHIFT>>>
where
    BigInt<Digit, SHIFT>: Clone,
{
    type Segment = self::Segment<Fraction<BigInt<Digit, SHIFT>>>;

    fn segments(&self) -> Vec<Self::Segment> {
        let mut result = Vec::<Self::Segment>::with_capacity(self.vertices.len());
        for index in 0..self.vertices.len() - 1 {
            result.push(Segment::new(
                self.vertices[index].clone(),
                self.vertices[index + 1].clone(),
            ));
        }
        result.push(Segment::new(
            self.vertices[self.vertices.len() - 1].clone(),
            self.vertices[0].clone(),
        ));
        result
    }

    fn segments_count(&self) -> usize {
        self.vertices.len()
    }
}
