use std::{
    marker::PhantomData,
    ops::{Add, Mul, Sub},
};

use num::traits::ConstZero;

use crate::{AntiwedgeProduct, geometry3d::Trivector, reverse_antiwedge_metric};

use super::{Bivector, Evenvector, Multivector, Vector};

// ----------------------------------------------------------------------------------------------------
// Scalar
// ----------------------------------------------------------------------------------------------------

impl<T, M> AntiwedgeProduct<Vector<T, M>> for T {
    type Output = ();

    fn antiwedge(&self, _rhs: &Vector<T, M>) -> Self::Output {}
}

impl<T, M> AntiwedgeProduct<Bivector<T, M>> for T {
    type Output = ();

    fn antiwedge(&self, _rhs: &Bivector<T, M>) -> Self::Output {}
}

impl<T, M> AntiwedgeProduct<Trivector<T, M>> for T
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &Trivector<T, M>) -> Self::Output {
        *self * rhs.xyz
    }
}

impl<T, M> AntiwedgeProduct<Evenvector<T, M>> for T {
    type Output = ();

    fn antiwedge(&self, _rhs: &Evenvector<T, M>) -> Self::Output {}
}

impl<T, M> AntiwedgeProduct<Multivector<T, M>> for T
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        self.antiwedge(&rhs.t)
    }
}

// ----------------------------------------------------------------------------------------------------
// Vector
// ----------------------------------------------------------------------------------------------------

impl<T, M> AntiwedgeProduct<T> for Vector<T, M> {
    type Output = ();

    fn antiwedge(&self, _rhs: &T) -> Self::Output {}
}

impl<T, M> AntiwedgeProduct<Vector<T, M>> for Vector<T, M> {
    type Output = ();

    fn antiwedge(&self, _rhs: &Vector<T, M>) -> Self::Output {}
}

impl<T, M> AntiwedgeProduct<Bivector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &Bivector<T, M>) -> Self::Output {
        self.x * rhs.yz + self.y * rhs.zx + self.z * rhs.xy
    }
}

impl<T, M> AntiwedgeProduct<Trivector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Vector<T, M>;

    fn antiwedge(&self, rhs: &Trivector<T, M>) -> Self::Output {
        Vector {
            x: self.x * rhs.xyz,
            y: self.y * rhs.xyz,
            z: self.z * rhs.xyz,
            _metric: PhantomData,
        }
    }
}

impl<T, M> AntiwedgeProduct<Evenvector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        self.antiwedge(&rhs.b)
    }
}

impl<T, M> AntiwedgeProduct<Multivector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn antiwedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        Multivector {
            s: self.antiwedge(&rhs.b),
            v: self.antiwedge(&rhs.t),
            b: Bivector::ZERO,
            t: Trivector::ZERO,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Bivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> AntiwedgeProduct<T> for Bivector<T, M> {
    type Output = ();

    fn antiwedge(&self, _rhs: &T) -> Self::Output {}
}

reverse_antiwedge_metric!(Bivector, Vector);

impl<T, M> AntiwedgeProduct<Bivector<T, M>> for Bivector<T, M>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Vector<T, M>;

    fn antiwedge(&self, rhs: &Bivector<T, M>) -> Self::Output {
        Vector {
            x: self.zx * rhs.xy - self.xy * rhs.zx,
            y: self.xy * rhs.yz - self.yz * rhs.xy,
            z: self.yz * rhs.zx - self.zx * rhs.yz,
            _metric: PhantomData,
        }
    }
}

impl<T, M> AntiwedgeProduct<Trivector<T, M>> for Bivector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Bivector<T, M>;

    fn antiwedge(&self, rhs: &Trivector<T, M>) -> Self::Output {
        Bivector {
            yz: self.yz * rhs.xyz,
            zx: self.zx * rhs.xyz,
            xy: self.xy * rhs.xyz,
            _metric: PhantomData,
        }
    }
}

impl<T, M> AntiwedgeProduct<Evenvector<T, M>> for Bivector<T, M>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Vector<T, M>;

    fn antiwedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        self.antiwedge(&rhs.b)
    }
}

impl<T, M> AntiwedgeProduct<Multivector<T, M>> for Bivector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn antiwedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        Multivector {
            s: self.antiwedge(&rhs.v),
            v: self.antiwedge(&rhs.b),
            b: self.antiwedge(&rhs.t),
            t: Trivector::ZERO,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Trivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> AntiwedgeProduct<T> for Trivector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &T) -> Self::Output {
        self.xyz * *rhs
    }
}

reverse_antiwedge_metric!(Trivector, Vector);

reverse_antiwedge_metric!(Trivector, Bivector);

impl<T, M> AntiwedgeProduct<Trivector<T, M>> for Trivector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Trivector<T, M>;

    fn antiwedge(&self, rhs: &Trivector<T, M>) -> Self::Output {
        Trivector {
            xyz: self.xyz * rhs.xyz,
            _metric: PhantomData,
        }
    }
}

impl<T, M> AntiwedgeProduct<Evenvector<T, M>> for Trivector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T, M>;

    fn antiwedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        Evenvector {
            s: self.antiwedge(&rhs.s),
            b: self.antiwedge(&rhs.b),
        }
    }
}

impl<T, M> AntiwedgeProduct<Multivector<T, M>> for Trivector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn antiwedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        Multivector {
            s: self.antiwedge(&rhs.s),
            v: self.antiwedge(&rhs.v),
            b: self.antiwedge(&rhs.b),
            t: self.antiwedge(&rhs.t),
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Evenvector
// ----------------------------------------------------------------------------------------------------

impl<T, M> AntiwedgeProduct<T> for Evenvector<T, M> {
    type Output = ();

    fn antiwedge(&self, _rhs: &T) -> Self::Output {}
}

impl<T, M> AntiwedgeProduct<Vector<T, M>> for Evenvector<T, M>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &Vector<T, M>) -> Self::Output {
        self.b.antiwedge(rhs)
    }
}

impl<T, M> AntiwedgeProduct<Bivector<T, M>> for Evenvector<T, M>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Vector<T, M>;

    fn antiwedge(&self, rhs: &Bivector<T, M>) -> Self::Output {
        self.b.antiwedge(rhs)
    }
}

impl<T, M> AntiwedgeProduct<Evenvector<T, M>> for Evenvector<T, M>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Vector<T, M>;

    fn antiwedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        self.b.antiwedge(&rhs.b)
    }
}

impl<T, M> AntiwedgeProduct<Multivector<T, M>> for Evenvector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn antiwedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        Multivector {
            s: self.b.antiwedge(&rhs.v) + self.s.antiwedge(&rhs.t),
            v: self.b.antiwedge(&rhs.b),
            b: self.b.antiwedge(&rhs.t),
            t: Trivector::ZERO,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Multivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> AntiwedgeProduct<T> for Multivector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &T) -> Self::Output {
        self.t.antiwedge(rhs)
    }
}

impl<T, M> AntiwedgeProduct<Vector<T, M>> for Multivector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn antiwedge(&self, rhs: &Vector<T, M>) -> Self::Output {
        Multivector {
            s: self.b.antiwedge(rhs),
            v: self.t.antiwedge(rhs),
            b: Bivector::ZERO,
            t: Trivector::ZERO,
        }
    }
}

impl<T, M> AntiwedgeProduct<Bivector<T, M>> for Multivector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn antiwedge(&self, rhs: &Bivector<T, M>) -> Self::Output {
        Multivector {
            s: self.v.antiwedge(rhs),
            v: self.b.antiwedge(rhs),
            b: self.t.antiwedge(rhs),
            t: Trivector::ZERO,
        }
    }
}

impl<T, M> AntiwedgeProduct<Evenvector<T, M>> for Multivector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn antiwedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        Multivector {
            s: self.t.antiwedge(&rhs.s) + self.v.antiwedge(&rhs.b),
            v: self.b.antiwedge(&rhs.b),
            b: self.t.antiwedge(&rhs.b),
            t: Trivector::ZERO,
        }
    }
}

impl<T, M> AntiwedgeProduct<Multivector<T, M>> for Multivector<T, M>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn antiwedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        Multivector {
            s: self.t.antiwedge(&rhs.s)
                + self.b.antiwedge(&rhs.v)
                + self.v.antiwedge(&rhs.b)
                + self.s.antiwedge(&rhs.t),
            v: self.t.antiwedge(&rhs.v) + self.b.antiwedge(&rhs.b) + self.v.antiwedge(&rhs.t),
            b: self.t.antiwedge(&rhs.b) + self.b.antiwedge(&rhs.t),
            t: self.t.antiwedge(&rhs.t),
        }
    }
}
