use std::hash::{BuildHasher, Hash, Hasher};

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Sign, Signed};

use crate::oriented::Orientation;
use crate::traits;

pub(crate) fn cross_multiply<
    Scalar: AdditiveGroup + MultiplicativeMonoid,
    Point: traits::Point<Scalar>,
>(
    first_start: &Point,
    first_end: &Point,
    second_start: &Point,
    second_end: &Point,
) -> Scalar {
    (first_end.x() - first_start.x()) * (second_end.y() - second_start.y())
        - (first_end.y() - first_start.y()) * (second_end.x() - second_start.x())
}

pub(crate) fn hash_slice_unordered<
    Value: Hash,
    H: Hasher,
    ValueBuildHasher: BuildHasher + Default,
>(
    values: &[Value],
    state: &mut H,
) {
    let mut hash = 0;
    let value_build_hasher = ValueBuildHasher::default();
    for value in values {
        hash ^= shuffle_bits(value_build_hasher.hash_one(value));
    }
    state.write_u64(hash);
}

pub(crate) fn orient<
    Scalar: AdditiveGroup + MultiplicativeMonoid + Signed,
    Point: Clone + traits::Point<Scalar>,
>(
    vertex: &Point,
    first_ray_point: &Point,
    second_ray_point: &Point,
) -> Orientation {
    match cross_multiply::<Scalar, Point>(vertex, first_ray_point, vertex, second_ray_point).sign()
    {
        Sign::Negative => Orientation::Clockwise,
        Sign::Positive => Orientation::Counterclockwise,
        Sign::Zero => Orientation::Collinear,
    }
}

fn shuffle_bits(hash: u64) -> u64 {
    ((hash ^ 89869747) ^ (hash.wrapping_shl(16))).wrapping_mul(3644798167)
}
