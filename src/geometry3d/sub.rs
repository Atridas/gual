use std::{
    marker::PhantomData,
    ops::{Neg, Sub},
};

use num::traits::ConstZero;

use crate::Scalar;

use super::{Bivector, Evenvector, Multivector, Point, Trivector, Vector};

// ----------------------------------------------------------------------------------------------------
// Vector
// ----------------------------------------------------------------------------------------------------

impl<T, M> Sub<T> for Vector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: T) -> Self::Output {
        Multivector {
            s: -rhs,
            v: self,
            b: Bivector::ZERO,
            t: Trivector::ZERO,
        }
    }
}

impl<T, M> Sub<Scalar<3, T, M>> for Vector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Multivector {
            s: -rhs.0,
            v: self,
            b: Bivector::ZERO,
            t: Trivector::ZERO,
        }
    }
}

impl<T, M> Sub<Vector<T, M>> for Vector<T, M>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector<T, M>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Sub<Bivector<T, M>> for Vector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Bivector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: self,
            b: -rhs,
            t: Trivector::ZERO,
        }
    }
}

impl<T, M> Sub<Trivector<T, M>> for Vector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Trivector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: self,
            b: Bivector::ZERO,
            t: -rhs,
        }
    }
}

impl<T, M> Sub<Evenvector<T, M>> for Vector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Evenvector<T, M>) -> Self::Output {
        Multivector {
            s: -rhs.s,
            v: self,
            b: -rhs.b,
            t: Trivector::ZERO,
        }
    }
}

impl<T, M> Sub<Multivector<T, M>> for Vector<T, M>
where
    T: ConstZero,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Multivector<T, M>) -> Self::Output {
        Multivector {
            s: -rhs.s,
            v: self - rhs.v,
            b: -rhs.b,
            t: Trivector::ZERO,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Point
// ----------------------------------------------------------------------------------------------------

impl<T> Sub for Point<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl<T> Sub<Vector<T>> for Point<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Point<T>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Point(self.0 - rhs)
    }
}

// ----------------------------------------------------------------------------------------------------
// Bivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> Sub<T> for Bivector<T, M>
where
    T: Neg<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn sub(self, rhs: T) -> Self::Output {
        Evenvector { s: -rhs, b: self }
    }
}

impl<T, M> Sub<Scalar<3, T, M>> for Bivector<T, M>
where
    T: Neg<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn sub(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Evenvector { s: -rhs.0, b: self }
    }
}

impl<T, M> Sub<Vector<T, M>> for Bivector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Vector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: -rhs,
            b: self,
            t: Trivector::ZERO,
        }
    }
}

impl<T, M> Sub<Bivector<T, M>> for Bivector<T, M>
where
    T: Sub<T, Output = T>,
{
    type Output = Bivector<T, M>;
    fn sub(self, rhs: Self) -> Self::Output {
        Bivector {
            yz: self.yz - rhs.yz,
            zx: self.zx - rhs.zx,
            xy: self.xy - rhs.xy,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Sub<Trivector<T, M>> for Bivector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Trivector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: Vector::ZERO,
            b: self,
            t: -rhs,
        }
    }
}

impl<T, M> Sub<Evenvector<T, M>> for Bivector<T, M>
where
    T: Sub<Output = T>,
    T: Neg<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn sub(self, rhs: Evenvector<T, M>) -> Self::Output {
        Evenvector {
            s: -rhs.s,
            b: self - rhs.b,
        }
    }
}

impl<T, M> Sub<Multivector<T, M>> for Bivector<T, M>
where
    T: ConstZero,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Multivector<T, M>) -> Self::Output {
        Multivector {
            s: -rhs.s,
            v: -rhs.v,
            b: self - rhs.b,
            t: -rhs.t,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Trivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> Sub<T> for Trivector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: T) -> Self::Output {
        Multivector {
            s: -rhs,
            v: Vector::ZERO,
            b: Bivector::ZERO,
            t: self,
        }
    }
}

impl<T, M> Sub<Scalar<3, T, M>> for Trivector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Multivector {
            s: -rhs.0,
            v: Vector::ZERO,
            b: Bivector::ZERO,
            t: self,
        }
    }
}

impl<T, M> Sub<Vector<T, M>> for Trivector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Vector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: -rhs,
            b: Bivector::ZERO,
            t: self,
        }
    }
}

impl<T, M> Sub<Bivector<T, M>> for Trivector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Bivector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: Vector::ZERO,
            b: -rhs,
            t: self,
        }
    }
}

impl<T, M> Sub<Trivector<T, M>> for Trivector<T, M>
where
    T: Sub<T, Output = T>,
{
    type Output = Trivector<T, M>;
    fn sub(self, rhs: Self) -> Self::Output {
        Trivector {
            xyz: self.xyz - rhs.xyz,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Sub<Evenvector<T, M>> for Trivector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Evenvector<T, M>) -> Self::Output {
        Multivector {
            s: -rhs.s,
            v: Vector::ZERO,
            b: -rhs.b,
            t: self,
        }
    }
}

impl<T, M> Sub<Multivector<T, M>> for Trivector<T, M>
where
    T: ConstZero,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Multivector<T, M>) -> Self::Output {
        Multivector {
            s: -rhs.s,
            v: -rhs.v,
            b: -rhs.b,
            t: self - rhs.t,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Evenvector
// ----------------------------------------------------------------------------------------------------

impl<T, M> Sub<T> for Evenvector<T, M>
where
    T: Sub<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn sub(self, rhs: T) -> Self::Output {
        Evenvector {
            s: self.s - rhs,
            b: self.b,
        }
    }
}

impl<T, M> Sub<Scalar<3, T, M>> for Evenvector<T, M>
where
    T: Sub<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn sub(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Evenvector {
            s: self.s - rhs.0,
            b: self.b,
        }
    }
}

impl<T, M> Sub<Vector<T, M>> for Evenvector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Vector<T, M>) -> Self::Output {
        Multivector {
            s: self.s,
            v: -rhs,
            b: self.b,
            t: Trivector::ZERO,
        }
    }
}

impl<T, M> Sub<Bivector<T, M>> for Evenvector<T, M>
where
    T: Sub<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn sub(self, rhs: Bivector<T, M>) -> Self::Output {
        Evenvector {
            s: self.s,
            b: self.b - rhs,
        }
    }
}

impl<T, M> Sub<Trivector<T, M>> for Evenvector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Trivector<T, M>) -> Self::Output {
        Multivector {
            s: self.s,
            v: Vector::ZERO,
            b: self.b,
            t: -rhs,
        }
    }
}

impl<T, M> Sub<Evenvector<T, M>> for Evenvector<T, M>
where
    T: Sub<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn sub(self, rhs: Evenvector<T, M>) -> Self::Output {
        Evenvector {
            s: self.s - rhs.s,
            b: self.b - rhs.b,
        }
    }
}

impl<T, M> Sub<Multivector<T, M>> for Evenvector<T, M>
where
    T: Sub<Output = T>,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Multivector<T, M>) -> Self::Output {
        Multivector {
            s: self.s - rhs.s,
            v: -rhs.v,
            b: self.b - rhs.b,
            t: -rhs.t,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Multivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> Sub<T> for Multivector<T, M>
where
    T: Sub<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: T) -> Self::Output {
        Multivector {
            s: self.s - rhs,
            v: self.v,
            b: self.b,
            t: self.t,
        }
    }
}

impl<T, M> Sub<Scalar<3, T, M>> for Multivector<T, M>
where
    T: Sub<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Multivector {
            s: self.s - rhs.0,
            v: self.v,
            b: self.b,
            t: self.t,
        }
    }
}

impl<T, M> Sub<Vector<T, M>> for Multivector<T, M>
where
    T: Sub<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Vector<T, M>) -> Self::Output {
        Multivector {
            s: self.s,
            v: self.v - rhs,
            b: self.b,
            t: self.t,
        }
    }
}

impl<T, M> Sub<Bivector<T, M>> for Multivector<T, M>
where
    T: Sub<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Bivector<T, M>) -> Self::Output {
        Multivector {
            s: self.s,
            v: self.v,
            b: self.b - rhs,
            t: self.t,
        }
    }
}

impl<T, M> Sub<Trivector<T, M>> for Multivector<T, M>
where
    T: Sub<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Trivector<T, M>) -> Self::Output {
        Multivector {
            s: self.s,
            v: self.v,
            b: self.b,
            t: self.t - rhs,
        }
    }
}

impl<T, M> Sub<Evenvector<T, M>> for Multivector<T, M>
where
    T: Sub<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Evenvector<T, M>) -> Self::Output {
        Multivector {
            s: self.s - rhs.s,
            v: self.v,
            b: self.b - rhs.b,
            t: self.t,
        }
    }
}

impl<T, M> Sub<Multivector<T, M>> for Multivector<T, M>
where
    T: Sub<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: Multivector<T, M>) -> Self::Output {
        Multivector {
            s: self.s - rhs.s,
            v: self.v - rhs.v,
            b: self.b - rhs.b,
            t: self.t - rhs.t,
        }
    }
}
