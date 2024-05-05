use crate::vecn::{Number, VecN};

impl<const DIMS: usize, T: Number> VecN<DIMS, T> {
    pub fn dot(self, rhs: Self) -> T
    where
        T: std::iter::Sum,
    {
        (self * rhs).0.into_iter().sum()
    }

    pub fn length_squared(self) -> T
    where
        T: std::iter::Sum,
    {
        self.dot(self)
    }
}
