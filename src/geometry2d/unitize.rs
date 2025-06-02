use num::{Float, traits::ConstOne};

use crate::{Epsilon, Norm, Unitizable};

use super::{UnitVector, Vector};

impl<T> Unitizable for Vector<T>
where
    T: Float,
    T: ConstOne,
    T: Epsilon,
    Vector<T>: Norm<Scalar = T>,
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
