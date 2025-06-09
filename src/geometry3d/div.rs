use std::{
    marker::PhantomData,
    ops::{Div, Mul},
};

use num::traits::ConstOne;

use super::{Bivector, Evenvector, Multivector, Point, Trivector, UnitVector, Vector};

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
            z: self.z * inv,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy> Div<T> for UnitVector<T>
where
    Vector<T>: Div<T, Output = Vector<T>>,
{
    type Output = Vector<T>;
    fn div(self, rhs: T) -> Self::Output {
        self.0 / rhs
    }
}

impl<T: Copy> Div<T> for Point<T>
where
    Vector<T>: Div<T, Output = Vector<T>>,
{
    type Output = Point<T>;
    fn div(self, rhs: T) -> Self::Output {
        Point(self.0 / rhs)
    }
}

impl<T: Copy, M> Div<T> for Bivector<T, M>
where
    T: ConstOne,
    T: Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let inv = T::ONE / rhs;
        Bivector {
            yz: self.yz * inv,
            zx: self.zx * inv,
            xy: self.xy * inv,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy, M> Div<T> for Trivector<T, M>
where
    T: Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Trivector {
            xyz: self.xyz / rhs,
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
            t: self.t * inv,
        }
    }
}
