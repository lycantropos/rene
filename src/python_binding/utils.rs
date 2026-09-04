/// Based on "Ranking and unranking permutations in linear time"
/// by W. Myrvold, F. Ruskey
///
/// Time complexity: O(values.len())
/// Memory complexity: O(1)
///
/// More at: http://webhome.cs.uvic.ca/~ruskey/Publications/RankPerm/MyrvoldRuskey.pdf
pub(super) fn permute<T>(values: &mut [T], mut seed: usize) {
    for step in (1..=values.len()).rev() {
        values.swap(step - 1, seed % step);
        seed /= step;
    }
}
