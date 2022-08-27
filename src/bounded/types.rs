pub struct Box<Scalar> {
    max_x: Scalar,
    max_y: Scalar,
    min_x: Scalar,
    min_y: Scalar,
}

impl<Scalar: Clone> Clone for Box<Scalar> {
    fn clone(&self) -> Self {
        Self {
            max_x: self.max_x.clone(),
            max_y: self.max_y.clone(),
            min_x: self.min_x.clone(),
            min_y: self.min_y.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self)
    where
        Self:,
    {
        (self.max_x, self.max_y, self.min_x, self.min_y) = (
            source.max_x.clone(),
            source.max_y.clone(),
            source.min_x.clone(),
            source.min_y.clone(),
        );
    }
}

impl<Scalar> Box<Scalar> {
    pub fn get_max_x(&self) -> &Scalar {
        &self.max_x
    }

    pub fn get_max_y(&self) -> &Scalar {
        &self.max_y
    }

    pub fn get_min_x(&self) -> &Scalar {
        &self.min_x
    }

    pub fn get_min_y(&self) -> &Scalar {
        &self.min_y
    }
}

impl<Scalar> Box<Scalar> {
    pub fn new(max_x: Scalar, max_y: Scalar, min_x: Scalar, min_y: Scalar) -> Self {
        Self {
            max_x,
            max_y,
            min_x,
            min_y,
        }
    }
}
