use std::{marker::PhantomData, ops::Neg};

use crate::geometry3d::Trivector;

use super::{Bivector, Evenvector, Multivector, Point, UnitVector, Vector};

impl<T, M> Neg for Vector<T, M>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            _metric: PhantomData,
        }
    }
}

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

impl<T, M> Neg for Bivector<T, M>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Bivector {
            yz: -self.yz,
            zx: -self.zx,
            xy: -self.xy,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Neg for Trivector<T, M>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Trivector {
            xyz: -self.xyz,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Neg for Evenvector<T, M>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Evenvector {
            s: -self.s,
            b: -self.b,
        }
    }
}

impl<T, M> Neg for Multivector<T, M>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Multivector {
            s: -self.s,
            v: -self.v,
            b: -self.b,
            t: -self.t,
        }
    }
}
