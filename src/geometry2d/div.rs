use std::{
    marker::PhantomData,
    ops::{Div, Mul},
};

use num::traits::ConstOne;

use super::{Bivector, Evenvector, Multivector, Vector};

impl<T: Copy, M> Div<T> for Vector<T, M>
where
    T: ConstOne,
    T: Mul<Output = T>,
    T: Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let inv = T::ONE / rhs;
        Vector {
            x: self.x * inv,
            y: self.y * inv,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy, M> Div<T> for Bivector<T, M>
where
    T: Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Bivector {
            xy: self.xy / rhs,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy, M> Div<T> for Evenvector<T, M>
where
    T: ConstOne,
    T: Mul<Output = T>,
    T: Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let inv = T::ONE / rhs;
        Evenvector {
            s: self.s * inv,
            b: self.b * inv,
        }
    }
}

impl<T: Copy, M> Div<T> for Multivector<T, M>
where
    T: ConstOne,
    T: Mul<Output = T>,
    T: Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let inv = T::ONE / rhs;
        Multivector {
            s: self.s * inv,
            v: self.v * inv,
            b: self.b * inv,
        }
    }
}
