use std::hash::{BuildHasher, Hash, Hasher};

use crate::traits::Multisegmental;

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
    ((hash ^ 89_869_747) ^ (hash.wrapping_shl(16))).wrapping_mul(3_644_798_167)
}

pub struct MultisegmentalsSegments<Multisegments, Segments> {
    multisegments: Multisegments,
    segments: Segments,
}

impl<Multisegments, Segments> MultisegmentalsSegments<Multisegments, Segments> {
    #[must_use]
    pub(crate) fn new(segments: Segments, multisegments: Multisegments) -> Self {
        Self {
            segments,
            multisegments,
        }
    }
}

impl<Multisegments: Iterator<Item = Multisegment>, Multisegment: Multisegmental> Iterator
    for MultisegmentalsSegments<Multisegments, <Multisegment as Multisegmental>::Segments>
{
    type Item = <Multisegment as Multisegmental>::Segment;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = self.segments.next();
        if result.is_none() {
            if let Some(next_multisegment) = self.multisegments.next() {
                self.segments = next_multisegment.segments();
            } else {
                return None;
            }
            result = self.segments.next();
        }
        result
    }
}
