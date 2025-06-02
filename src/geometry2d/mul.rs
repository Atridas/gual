use std::{marker::PhantomData, ops::Mul};

use crate::{GeometricProduct, Scalar};

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
            _metric: PhantomData,
        }
    }
}

impl<T: Copy, M> Mul<Scalar<2, T, M>> for Vector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Scalar<2, T, M>) -> Self::Output {
        Vector {
            x: self.x * rhs.0,
            y: self.y * rhs.0,
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

impl<T: Copy> Mul<Scalar<2, T>> for UnitVector<T>
where
    T: Mul<Output = T>,
{
    type Output = Vector<T>;
    fn mul(self, rhs: Scalar<2, T>) -> Self::Output {
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

impl<T: Copy> Mul<Scalar<2, T>> for Point<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Scalar<2, T>) -> Self::Output {
        Point(self.0 * rhs.0)
    }
}

// ----------------------------------------------------------------------------------------------------
// Bivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> Mul<T> for Bivector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Bivector {
            xy: self.xy * rhs,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Mul<Scalar<2, T, M>> for Bivector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Scalar<2, T, M>) -> Self::Output {
        Bivector {
            xy: self.xy * rhs.0,
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

impl<T: Copy, M> Mul<Scalar<2, T, M>> for Evenvector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Scalar<2, T, M>) -> Self::Output {
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
        }
    }
}

impl<T: Copy, M> Mul<Scalar<2, T, M>> for Multivector<T, M>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Scalar<2, T, M>) -> Self::Output {
        Multivector {
            s: self.s * rhs.0,
            v: self.v * rhs.0,
            b: self.b * rhs.0,
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
