use std::ops::Neg;

use crate::projective2d::{Point, UnitVector};

impl<T> Neg for UnitVector<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        UnitVector(-self.0)
    }
}

impl<T> Neg for Point<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Point(-self.0)
    }
}
