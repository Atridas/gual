use std::ops::Div;

use crate::projective2d::{DirVector, Point, UnitVector};

// ----------------------------------------------------------------------------------------------------
// Point
// ----------------------------------------------------------------------------------------------------

impl<T: Copy> Div<T> for UnitVector<T>
where
    DirVector<T>: Div<T, Output = DirVector<T>>,
{
    type Output = DirVector<T>;
    fn div(self, rhs: T) -> Self::Output {
        self.0 / rhs
    }
}

impl<T: Copy> Div<T> for Point<T>
where
    DirVector<T>: Div<T, Output = DirVector<T>>,
{
    type Output = Point<T>;
    fn div(self, rhs: T) -> Self::Output {
        Point(self.0 / rhs)
    }
}
