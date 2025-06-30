//! Wedge product implementation
//!
//! | `^` |  1  |  x   |  y   |  z   |  yz  |  zx  |  xy  | xyz |
//! | --- | --- | ---- | ---- | ---- | ---- | ---- | ---- | --- |
//! |  1  |  1  |  x   |  y   |  z   |  yz  |  zx  |  xy  | xyz |
//! |  x  |  x  |  0   | xy   | -zx  | -xyz |  0   |  0   |  0  |
//! |  y  |  y  | -xy  |  0   |  yz  |   0  | -xyz |  0   |  0  |
//! |  z  |  z  |  zx  | -yz  |  0   |   0  |  0   | -xyz |  0  |
//! |  yz |  yz | -xyz |  0   |  0   |   0  |  0   |  0   |  0  |
//! |  zx |  zx |  0   | -xyz |  0   |   0  |  0   |  0   |  0  |
//! |  xy |  xy |  0   |  0   | -xyz |   0  |  0   |  0   |  0  |

use std::{
    marker::PhantomData,
    ops::{Add, Mul, Sub},
};

use num::traits::ConstZero;

use crate::{Scalar, WedgeProduct, null_wedge, reverse_wedge_metric, wedge_scalar};

use super::{Bivector, Evenvector, Multivector, Trivector, Vector};

// ----------------------------------------------------------------------------------------------------
// Scalar
// ----------------------------------------------------------------------------------------------------

wedge_scalar!(Vector, 3);
wedge_scalar!(Bivector, 3);
wedge_scalar!(Trivector, 3);
wedge_scalar!(Evenvector, 3);
wedge_scalar!(Multivector, 3);

// ----------------------------------------------------------------------------------------------------
// Vector
// ----------------------------------------------------------------------------------------------------

impl<T, M> WedgeProduct<Vector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Bivector<T, M>;

    fn wedge(&self, rhs: &Vector<T, M>) -> Self::Output {
        Bivector {
            yz: self.y * rhs.z - self.z * rhs.y,
            zx: self.z * rhs.x - self.x * rhs.z,
            xy: self.x * rhs.y - self.y * rhs.x,
            _metric: PhantomData,
        }
    }
}

impl<T, M> WedgeProduct<Bivector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Trivector<T, M>;

    fn wedge(&self, rhs: &Bivector<T, M>) -> Self::Output {
        Trivector {
            xyz: self.x * rhs.yz + self.y * rhs.zx + self.z * rhs.xy,
            _metric: PhantomData,
        }
    }
}

null_wedge!(Vector, Trivector);

impl<T, M> WedgeProduct<Evenvector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn wedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        self.wedge(&rhs.s) + self.wedge(&rhs.b)
    }
}

impl<T, M> WedgeProduct<Multivector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn wedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: self.wedge(&rhs.s),
            b: self.wedge(&rhs.v),
            t: self.wedge(&rhs.b),
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Bivector
// ----------------------------------------------------------------------------------------------------

reverse_wedge_metric!(Bivector, Vector);
null_wedge!(Bivector, Bivector);
null_wedge!(Bivector, Trivector);

impl<T, M> WedgeProduct<Evenvector<T, M>> for Bivector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Bivector<T, M>;

    fn wedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        self.wedge(&rhs.s)
    }
}

impl<T, M> WedgeProduct<Multivector<T, M>> for Bivector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn wedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: Vector::ZERO,
            b: self.wedge(&rhs.s),
            t: self.wedge(&rhs.v),
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Bivector
// ----------------------------------------------------------------------------------------------------

null_wedge!(Trivector, Vector);
null_wedge!(Trivector, Bivector);
null_wedge!(Trivector, Trivector);

impl<T, M> WedgeProduct<Evenvector<T, M>> for Trivector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Trivector<T, M>;

    fn wedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        self.wedge(&rhs.s)
    }
}

impl<T, M> WedgeProduct<Multivector<T, M>> for Trivector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type Output = Trivector<T, M>;

    fn wedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        self.wedge(&rhs.s)
    }
}

// ----------------------------------------------------------------------------------------------------
// Evenvector
// ----------------------------------------------------------------------------------------------------

reverse_wedge_metric!(Evenvector, Vector);
reverse_wedge_metric!(Evenvector, Bivector);
reverse_wedge_metric!(Evenvector, Trivector);

impl<T, M> WedgeProduct<Evenvector<T, M>> for Evenvector<T, M>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T, M>;

    fn wedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        Evenvector {
            s: self.s * rhs.s,
            b: self.s.wedge(&rhs.b) + self.b.wedge(&rhs.s),
        }
    }
}

impl<T, M> WedgeProduct<Multivector<T, M>> for Evenvector<T, M>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn wedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        Multivector {
            s: self.s * rhs.s,
            v: self.s.wedge(&rhs.v),
            b: self.s.wedge(&rhs.b) + self.b.wedge(&rhs.s),
            t: self.s.wedge(&rhs.t) + self.b.wedge(&rhs.v),
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Multivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> WedgeProduct<Vector<T, M>> for Multivector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn wedge(&self, rhs: &Vector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: self.s.wedge(rhs),
            b: self.v.wedge(rhs),
            t: self.b.wedge(rhs),
        }
    }
}

reverse_wedge_metric!(Multivector, Bivector);

impl<T, M> WedgeProduct<Trivector<T, M>> for Multivector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn wedge(&self, rhs: &Trivector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: Vector::ZERO,
            b: Bivector::ZERO,
            t: self.s.wedge(rhs),
        }
    }
}

reverse_wedge_metric!(Multivector, Evenvector);

impl<T, M> WedgeProduct<Multivector<T, M>> for Multivector<T, M>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn wedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        Multivector {
            s: self.s * rhs.s,
            v: self.s.wedge(&rhs.v) + self.v.wedge(&rhs.s),
            b: self.s.wedge(&rhs.b) + self.v.wedge(&rhs.v) + self.b.wedge(&rhs.s),
            t: self.s.wedge(&rhs.t)
                + self.v.wedge(&rhs.b)
                + self.b.wedge(&rhs.v)
                + self.t.wedge(&rhs.s),
        }
    }
}
