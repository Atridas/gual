use std::ops::{Neg, Sub};

use num::traits::ConstZero;

use crate::projective2d::{DirVector, Point};
// ----------------------------------------------------------------------------------------------------
// Point
// ----------------------------------------------------------------------------------------------------

impl<T> Sub<DirVector<T>> for Point<T>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Point<T>;
    fn sub(self, rhs: DirVector<T>) -> Self::Output {
        Point(self.0 - rhs)
    }
}

impl<T> Sub<Point<T>> for Point<T>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = DirVector<T>;
    fn sub(self, rhs: Point<T>) -> Self::Output {
        self.0 - rhs.0
    }
}
