use std::ops::Add;

use crate::projective2d::{DirVector, Point};

// ----------------------------------------------------------------------------------------------------
// Point
// ----------------------------------------------------------------------------------------------------

impl<T> Add<DirVector<T>> for Point<T>
where
    T: Add<T, Output = T>,
{
    type Output = Point<T>;
    fn add(self, rhs: DirVector<T>) -> Self::Output {
        Point(DirVector::new(self.0.x + rhs.x, self.0.y + rhs.y))
    }
}
