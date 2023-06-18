#[derive(Clone)]
pub(crate) struct Trapezoid {
    pub(super) left_point_index: usize,
    pub(super) right_point_index: usize,
    pub(super) below_edge_index: usize,
    pub(super) above_edge_index: usize,
    pub(super) is_component: bool,
    leaf_index: usize,
    lower_left_leaf_index: Option<usize>,
    lower_right_leaf_index: Option<usize>,
    upper_left_leaf_index: Option<usize>,
    upper_right_leaf_index: Option<usize>,
}

impl Trapezoid {
    pub(super) fn new(
        is_component: bool,
        left_point_index: usize,
        right_point_index: usize,
        below_edge_index: usize,
        above_edge_index: usize,
        leaf_index: usize,
    ) -> Self {
        Self {
            left_point_index,
            right_point_index,
            below_edge_index,
            above_edge_index,
            is_component,
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

    pub(super) fn get_lower_right_leaf_index(&self) -> Option<usize> {
        self.lower_right_leaf_index
    }

    pub(super) fn get_upper_left_leaf_index(&self) -> Option<usize> {
        self.upper_left_leaf_index
    }

    pub(super) fn get_upper_right_leaf_index(&self) -> Option<usize> {
        self.upper_right_leaf_index
    }

    pub(super) fn get_leaf_index(&self) -> usize {
        self.leaf_index
    }

    pub(super) fn reset_lower_left(&mut self) {
        self.lower_left_leaf_index = None;
    }

    pub(super) fn reset_lower_right(&mut self) {
        self.lower_right_leaf_index = None;
    }

    pub(super) fn reset_upper_left(&mut self) {
        self.upper_left_leaf_index = None;
    }

    pub(super) fn reset_upper_right(&mut self) {
        self.upper_right_leaf_index = None;
    }

    pub(super) fn set_as_lower_left(&mut self, value: &mut Self) {
        self.lower_left_leaf_index = Some(value.leaf_index);
        value.lower_right_leaf_index = Some(self.leaf_index);
    }

    pub(super) fn set_as_lower_right(&mut self, value: &mut Self) {
        self.lower_right_leaf_index = Some(value.leaf_index);
        value.lower_left_leaf_index = Some(self.leaf_index);
    }

    pub(super) fn set_as_upper_left(&mut self, value: &mut Self) {
        self.upper_left_leaf_index = Some(value.leaf_index);
        value.upper_right_leaf_index = Some(self.leaf_index);
    }

    pub(super) fn set_as_upper_right(&mut self, value: &mut Self) {
        self.upper_right_leaf_index = Some(value.leaf_index);
        value.upper_left_leaf_index = Some(self.leaf_index);
    }
}
