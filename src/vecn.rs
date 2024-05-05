use std::{
    array,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

/// Simple N-dimensional vector
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VecN<const DIMS: usize, T: Number>(pub [T; DIMS]);

pub trait Number:
    Copy
    + Default
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
}

impl<T> Number for T where
    T: Copy
        + Default
        + PartialEq
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
{
}

impl<const DIMS: usize, T: Number> VecN<DIMS, T> {
    pub fn merge<M>(self, rhs: Self, merger: M) -> Self
    where
        M: Fn(T, T) -> T,
    {
        Self(array::from_fn(|i| merger(self[i], rhs[i])))
    }

    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(T) -> T,
    {
        Self(self.0.map(f))
    }
}

impl<const DIMS: usize, T: Number> Index<usize> for VecN<DIMS, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const DIMS: usize, T: Number> IndexMut<usize> for VecN<DIMS, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

macro_rules! math_op {
    ($op_name:ident, $fname: ident, $code:expr) => {
        // Implement operator for vectors
        impl<const DIMS: usize, T: Number> $op_name for VecN<DIMS, T> {
            type Output = Self;
            fn $fname(self, rhs: Self) -> Self::Output {
                self.merge(rhs, $code)
            }
        }

        // Implement operator for scalars
        impl<const DIMS: usize, T: Number> $op_name<T> for VecN<DIMS, T> {
            type Output = Self;
            fn $fname(self, rhs: T) -> Self::Output {
                self.map(|v| (($code)(v, rhs)))
            }
        }
    };
}

math_op!(Add, add, |a, b| a + b);
math_op!(Sub, sub, |a, b| a - b);
math_op!(Div, div, |a, b| a / b);
math_op!(Mul, mul, |a, b| a * b);


#[cfg(test)]
mod tests {
    use super::VecN;

    #[test]
    fn test_ivec3_add() {
        let a: VecN<3, i32> = VecN([1, 2, 3]);
        let b: VecN<3, i32> = VecN([4, 5, 6]);
        let a_plus_b = a + b;
        assert_eq!(a_plus_b, VecN([5, 7, 9]));
    }

    #[test]
    fn test_ivec3_add_scalar() {
        let a: VecN<3, i32> = VecN([1, 2, 3]);
        let a_plus_b = a + 8;
        assert_eq!(a_plus_b, VecN([9, 10, 11]));
    }

    #[test]
    fn test_vec3_add_scalar() {
        let a: VecN<3, f32> = VecN([1., 2., 3.]);
        let a_plus_b = a + 8.;
        assert_eq!(a_plus_b, VecN([9., 10., 11.]));
    }
}
