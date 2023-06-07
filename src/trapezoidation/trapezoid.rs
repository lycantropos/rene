use super::edge::Edge;

#[derive(Clone)]
pub(crate) struct Trapezoid<Point> {
    pub(super) left_point: Point,
    pub(super) right_point: Point,
    pub(super) below_edge_index: usize,
    pub(super) above_edge_index: usize,
    leaf_index: usize,
    lower_left_leaf_index: Option<usize>,
    lower_right_leaf_index: Option<usize>,
    upper_left_leaf_index: Option<usize>,
    upper_right_leaf_index: Option<usize>,
}

impl<Point> Trapezoid<Point> {
    pub(super) fn new(
        left_point: Point,
        right_point: Point,
        below_edge_index: usize,
        above_edge_index: usize,
        leaf_index: usize,
    ) -> Self {
        Self {
            left_point,
            right_point,
            below_edge_index,
            above_edge_index,
            leaf_index,
            lower_left_leaf_index: None,
            lower_right_leaf_index: None,
            upper_left_leaf_index: None,
            upper_right_leaf_index: None,
        }
    }

    pub(super) fn get_lower_left_leaf_index(&self) -> Option<usize> {
        self.lower_left_leaf_index
    }

    pub(super) fn get_upper_left_leaf_index(&self) -> Option<usize> {
        self.upper_left_leaf_index
    }

    pub(super) fn get_lower_right_leaf_index(&self) -> Option<usize> {
        self.lower_right_leaf_index
    }

    pub(super) fn get_upper_right_leaf_index(&self) -> Option<usize> {
        self.upper_right_leaf_index
    }

    pub(super) fn leaf_index(&self) -> usize {
        self.leaf_index
    }

    pub(super) fn set_as_lower_left(&mut self, value: Option<&mut Self>) {
        match value {
            Some(value) => {
                self.lower_left_leaf_index = Some(value.leaf_index);
                value.lower_right_leaf_index = Some(self.leaf_index);
            }
            None => {
                self.lower_left_leaf_index = None;
            }
        }
    }

    pub(super) fn set_as_lower_right(&mut self, value: Option<&mut Self>) {
        match value {
            Some(value) => {
                self.lower_right_leaf_index = Some(value.leaf_index);
                value.lower_left_leaf_index = Some(self.leaf_index);
            }
            None => {
                self.lower_right_leaf_index = None;
            }
        }
    }

    pub(super) fn set_as_upper_left(&mut self, value: Option<&mut Self>) {
        match value {
            Some(value) => {
                self.upper_left_leaf_index = Some(value.leaf_index);
                value.upper_right_leaf_index = Some(self.leaf_index);
            }
            None => {
                self.upper_left_leaf_index = None;
            }
        };
    }

    pub(super) fn set_as_upper_right(&mut self, value: Option<&mut Self>) {
        match value {
            Some(value) => {
                self.upper_right_leaf_index = Some(value.leaf_index);
                value.upper_left_leaf_index = Some(self.leaf_index);
            }
            None => {
                self.upper_right_leaf_index = None;
            }
        }
    }
}
