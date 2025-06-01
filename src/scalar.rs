use std::ops::{Add, Mul, Neg, Sub};

use crate::Scalar;

impl<T, V> Add<V> for Scalar<T>
where
    V: Add<T>,
{
    type Output = <V as Add<T>>::Output;
    fn add(self, rhs: V) -> Self::Output {
        rhs + self.0
    }
}

impl<T, V> Sub<V> for Scalar<T>
where
    V: Neg<Output = V>,
    V: Add<T>,
{
    type Output = <V as Add<T>>::Output;
    fn sub(self, rhs: V) -> Self::Output {
        -rhs + self.0
    }
}

impl<T, V> Mul<V> for Scalar<T>
where
    V: Mul<T>,
{
    type Output = <V as Mul<T>>::Output;
    fn mul(self, rhs: V) -> Self::Output {
        rhs * self.0
    }
}
