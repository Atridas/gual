use std::ops::Mul;

use crate::{
    Scalar,
    projective2d::{DirVector, Point, UnitVector},
};
// ----------------------------------------------------------------------------------------------------
// UnitVector
// ----------------------------------------------------------------------------------------------------

impl<T: Copy> Mul<T> for UnitVector<T>
where
    T: Mul<Output = T>,
{
    type Output = DirVector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        self.0 * rhs
    }
}

impl<T: Copy> Mul<Scalar<2, T>> for UnitVector<T>
where
    T: Mul<Output = T>,
{
    type Output = DirVector<T>;
    fn mul(self, rhs: Scalar<2, T>) -> Self::Output {
        self.0 * rhs.0
    }
}

// ----------------------------------------------------------------------------------------------------
// Point
// ----------------------------------------------------------------------------------------------------

impl<T: Copy> Mul<T> for Point<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Point(self.0 * rhs)
    }
}

impl<T: Copy> Mul<Scalar<2, T>> for Point<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Scalar<2, T>) -> Self::Output {
        Point(self.0 * rhs.0)
    }
}
