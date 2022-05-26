use std::hash::{BuildHasher, Hash, Hasher};

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid};

use crate::geometries::Point;

pub(super) fn cross_multiply<Scalar: AdditiveGroup + MultiplicativeMonoid>(
    first_start: Point<Scalar>,
    first_end: Point<Scalar>,
    second_start: Point<Scalar>,
    second_end: Point<Scalar>,
) -> Scalar {
    (first_end.x - first_start.x) * (second_end.y - second_start.y)
        - (first_end.y - first_start.y) * (second_end.x - second_start.x)
}

pub(super) fn hash_slice_unordered<
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

fn shuffle_bits(hash: u64) -> u64 {
    ((hash ^ 89869747) ^ (hash.wrapping_shl(16))).wrapping_mul(3644798167)
}
