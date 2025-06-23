use num::{Float, traits::ConstOne};

use crate::{Epsilon, Norm, Unitizable};

use super::{DirVector, Line, UnitLine, UnitVector};

impl<T> Unitizable for DirVector<T>
where
    T: Float,
    T: ConstOne,
    T: Epsilon,
    DirVector<T>: Norm<Scalar = T>,
{
    type Output = UnitVector<T>;

    fn unitize(&self) -> Option<Self::Output> {
        let len2 = self.norm_squared();
        if len2.is_near_zero() {
            None
        } else {
            Some(UnitVector(*self / len2.sqrt()))
        }
    }
}

impl<T> Unitizable for Line<T>
where
    T: Float,
    T: ConstOne,
    T: Epsilon,
    Line<T>: Norm<Scalar = T>,
{
    type Output = UnitLine<T>;

    fn unitize(&self) -> Option<Self::Output> {
        let len2 = self.norm_squared();
        if len2.is_near_zero() {
            None
        } else {
            Some(UnitLine(*self / len2.sqrt()))
        }
    }
}
