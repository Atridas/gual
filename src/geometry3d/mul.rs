use std::{marker::PhantomData, ops::Mul};

use crate::{GeometricProduct, Scalar, geometry3d::Trivector};

use super::{Bivector, Evenvector, Multivector, Point, UnitVector, Vector};

// ----------------------------------------------------------------------------------------------------
// Vector
// ----------------------------------------------------------------------------------------------------

impl<T: Copy, M> Mul<T> for Vector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy, M> Mul<Scalar<3, T, M>> for Vector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Vector {
            x: self.x * rhs.0,
            y: self.y * rhs.0,
            z: self.z * rhs.0,
            _metric: PhantomData,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// UnitVector
// ----------------------------------------------------------------------------------------------------

impl<T: Copy> Mul<T> for UnitVector<T>
where
    T: Mul<Output = T>,
{
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        self.0 * rhs
    }
}

impl<T: Copy> Mul<Scalar<3, T>> for UnitVector<T>
where
    T: Mul<Output = T>,
{
    type Output = Vector<T>;
    fn mul(self, rhs: Scalar<3, T>) -> Self::Output {
        self.0 * rhs.0
    }
}

// ----------------------------------------------------------------------------------------------------
// Point
// ----------------------------------------------------------------------------------------------------

impl<T: Copy> Mul<T> for Point<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Point(self.0 * rhs)
    }
}

impl<T: Copy> Mul<Scalar<3, T>> for Point<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Scalar<3, T>) -> Self::Output {
        Point(self.0 * rhs.0)
    }
}

// ----------------------------------------------------------------------------------------------------
// Bivector
// ----------------------------------------------------------------------------------------------------

impl<T: Copy, M> Mul<T> for Bivector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Bivector {
            yz: self.yz * rhs,
            zx: self.zx * rhs,
            xy: self.xy * rhs,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy, M> Mul<Scalar<3, T, M>> for Bivector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Bivector {
            yz: self.yz * rhs.0,
            zx: self.zx * rhs.0,
            xy: self.xy * rhs.0,
            _metric: PhantomData,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Trivector
// ----------------------------------------------------------------------------------------------------

impl<T: Copy, M> Mul<T> for Trivector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Trivector {
            xyz: self.xyz * rhs,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy, M> Mul<Scalar<3, T, M>> for Trivector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Trivector {
            xyz: self.xyz * rhs.0,
            _metric: PhantomData,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Evenvector
// ----------------------------------------------------------------------------------------------------

impl<T: Copy, M> Mul<T> for Evenvector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Evenvector {
            s: self.s * rhs,
            b: self.b * rhs,
        }
    }
}

impl<T: Copy, M> Mul<Scalar<3, T, M>> for Evenvector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Evenvector {
            s: self.s * rhs.0,
            b: self.b * rhs.0,
        }
    }
}

impl<T, M> Mul<Vector<T, M>> for Evenvector<T, M>
where
    Self: GeometricProduct<Vector<T, M>>,
{
    type Output = <Self as GeometricProduct<Vector<T, M>>>::Output;
    fn mul(self, rhs: Vector<T, M>) -> Self::Output {
        self.geometric_product(&rhs)
    }
}

impl<T, M> Mul<Bivector<T, M>> for Evenvector<T, M>
where
    Self: GeometricProduct<Bivector<T, M>>,
{
    type Output = <Self as GeometricProduct<Bivector<T, M>>>::Output;
    fn mul(self, rhs: Bivector<T, M>) -> Self::Output {
        self.geometric_product(&rhs)
    }
}

impl<T, M> Mul<Evenvector<T, M>> for Evenvector<T, M>
where
    Self: GeometricProduct<Evenvector<T, M>>,
{
    type Output = <Self as GeometricProduct<Evenvector<T, M>>>::Output;
    fn mul(self, rhs: Evenvector<T, M>) -> Self::Output {
        self.geometric_product(&rhs)
    }
}

impl<T, M> Mul<Multivector<T, M>> for Evenvector<T, M>
where
    Self: GeometricProduct<Multivector<T, M>>,
{
    type Output = <Self as GeometricProduct<Multivector<T, M>>>::Output;
    fn mul(self, rhs: Multivector<T, M>) -> Self::Output {
        self.geometric_product(&rhs)
    }
}

// ----------------------------------------------------------------------------------------------------
// Multivector
// ----------------------------------------------------------------------------------------------------

impl<T: Copy, M> Mul<T> for Multivector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Multivector {
            s: self.s * rhs,
            v: self.v * rhs,
            b: self.b * rhs,
            t: self.t * rhs,
        }
    }
}

impl<T: Copy, M> Mul<Scalar<3, T, M>> for Multivector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Multivector {
            s: self.s * rhs.0,
            v: self.v * rhs.0,
            b: self.b * rhs.0,
            t: self.t * rhs.0,
        }
    }
}

impl<T, M> Mul<Vector<T, M>> for Multivector<T, M>
where
    Self: GeometricProduct<Vector<T, M>>,
{
    type Output = <Self as GeometricProduct<Vector<T, M>>>::Output;
    fn mul(self, rhs: Vector<T, M>) -> Self::Output {
        self.geometric_product(&rhs)
    }
}

impl<T, M> Mul<Bivector<T, M>> for Multivector<T, M>
where
    Self: GeometricProduct<Bivector<T, M>>,
{
    type Output = <Self as GeometricProduct<Bivector<T, M>>>::Output;
    fn mul(self, rhs: Bivector<T, M>) -> Self::Output {
        self.geometric_product(&rhs)
    }
}

impl<T, M> Mul<Evenvector<T, M>> for Multivector<T, M>
where
    Self: GeometricProduct<Evenvector<T, M>>,
{
    type Output = <Self as GeometricProduct<Evenvector<T, M>>>::Output;
    fn mul(self, rhs: Evenvector<T, M>) -> Self::Output {
        self.geometric_product(&rhs)
    }
}

impl<T, M> Mul<Multivector<T, M>> for Multivector<T, M>
where
    Self: GeometricProduct<Multivector<T, M>>,
{
    type Output = <Self as GeometricProduct<Multivector<T, M>>>::Output;
    fn mul(self, rhs: Multivector<T, M>) -> Self::Output {
        self.geometric_product(&rhs)
    }
}
