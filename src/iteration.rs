use core::convert::From;

pub(crate) struct PairwiseCombinations<T> {
    values: Vec<T>,
    index: usize,
    next_index: usize,
}

impl<T> Default for PairwiseCombinations<T> {
    fn default() -> Self {
        Self {
            values: Vec::default(),
            index: 0,
            next_index: 1,
        }
    }
}

impl<T> From<Vec<T>> for PairwiseCombinations<T> {
    fn from(values: Vec<T>) -> Self {
        Self {
            values,
            index: 0,
            next_index: 1,
        }
    }
}

impl<T: Copy> Iterator for PairwiseCombinations<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + 1 >= self.values.len() {
            None
        } else {
            let result =
                Some((self.values[self.index], self.values[self.next_index]));
            if self.next_index == self.values.len() - 1 {
                self.index += 1;
                self.next_index = self.index + 1;
            } else {
                self.next_index += 1;
            }
            result
        }
    }
}
