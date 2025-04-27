use std::ops::{Add, Sub};

use num::traits::{ConstOne, ConstZero};

use super::{Point, Vector};

impl<T> Point<T>
where
    T: ConstZero,
    T: ConstOne,
{
    pub const ZERO: Self = Point(Vector::ZERO);
    pub const X: Self = Point(Vector::X);
    pub const Y: Self = Point(Vector::Y);
    pub const Z: Self = Point(Vector::X);
    pub fn zero() -> Self {
        Self::ZERO
    }
}

impl<T> Add<Vector<T>> for Point<T>
where
    T: Add<T, Output = T>,
{
    type Output = Point<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        Point(self.0 + rhs)
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl<T> Sub<Vector<T>> for Point<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Point<T>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Point(self.0 - rhs)
    }
}
