use std::{
    marker::PhantomData,
    ops::{Add, Mul, Sub},
};

use num::traits::ConstZero;

use crate::{AntiwedgeProduct, reverse_antiwedge_metric};

use super::{Bivector, Evenvector, Multivector, Vector};

// ----------------------------------------------------------------------------------------------------
// Scalar
// ----------------------------------------------------------------------------------------------------

impl<T, M> AntiwedgeProduct<Vector<T, M>> for T {
    type Output = ();

    fn antiwedge(&self, _rhs: &Vector<T, M>) -> Self::Output {}
}

impl<T, M> AntiwedgeProduct<Bivector<T, M>> for T
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &Bivector<T, M>) -> Self::Output {
        *self * rhs.xy
    }
}

impl<T, M> AntiwedgeProduct<Evenvector<T, M>> for T
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        self.antiwedge(&rhs.b)
    }
}

impl<T, M> AntiwedgeProduct<Multivector<T, M>> for T
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        self.antiwedge(&rhs.b)
    }
}

// ----------------------------------------------------------------------------------------------------
// Vector
// ----------------------------------------------------------------------------------------------------

impl<T, M> AntiwedgeProduct<T> for Vector<T, M> {
    type Output = ();

    fn antiwedge(&self, _rhs: &T) -> Self::Output {}
}

impl<T, M> AntiwedgeProduct<Vector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &Vector<T, M>) -> Self::Output {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl<T, M> AntiwedgeProduct<Bivector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Vector<T, M>;

    fn antiwedge(&self, rhs: &Bivector<T, M>) -> Self::Output {
        Vector {
            x: self.x * rhs.xy,
            y: self.y * rhs.xy,
            _metric: PhantomData,
        }
    }
}

impl<T, M> AntiwedgeProduct<Evenvector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Vector<T, M>;

    fn antiwedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        self.antiwedge(&rhs.b)
    }
}

impl<T, M> AntiwedgeProduct<Multivector<T, M>> for Vector<T, M>
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
            b: Bivector::ZERO,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Bivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> AntiwedgeProduct<T> for Bivector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &T) -> Self::Output {
        self.xy * *rhs
    }
}

reverse_antiwedge_metric!(Bivector, Vector);

impl<T, M> AntiwedgeProduct<Bivector<T, M>> for Bivector<T, M>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector<T, M>;

    fn antiwedge(&self, rhs: &Bivector<T, M>) -> Self::Output {
        Bivector {
            xy: self.xy * rhs.xy,
            _metric: PhantomData,
        }
    }
}

impl<T, M> AntiwedgeProduct<Evenvector<T, M>> for Bivector<T, M>
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
            s: self.antiwedge(&rhs.s),
            v: self.antiwedge(&rhs.v),
            b: self.antiwedge(&rhs.b),
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Evenvector
// ----------------------------------------------------------------------------------------------------

impl<T, M> AntiwedgeProduct<T> for Evenvector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &T) -> Self::Output {
        self.b.antiwedge(rhs)
    }
}

impl<T, M> AntiwedgeProduct<Vector<T, M>> for Evenvector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Vector<T, M>;

    fn antiwedge(&self, rhs: &Vector<T, M>) -> Self::Output {
        self.b.antiwedge(rhs)
    }
}

impl<T, M> AntiwedgeProduct<Bivector<T, M>> for Evenvector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T, M>;

    fn antiwedge(&self, rhs: &Bivector<T, M>) -> Self::Output {
        Evenvector {
            s: self.s.antiwedge(rhs),
            b: self.b.antiwedge(rhs),
        }
    }
}

impl<T, M> AntiwedgeProduct<Evenvector<T, M>> for Evenvector<T, M>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T, M>;

    fn antiwedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        Evenvector {
            s: self.s.antiwedge(&rhs.b) + self.b.antiwedge(&rhs.s),
            b: self.b.antiwedge(&rhs.b),
        }
    }
}

impl<T, M> AntiwedgeProduct<Multivector<T, M>> for Evenvector<T, M>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn antiwedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        Multivector {
            s: self.s.antiwedge(&rhs.b) + self.b.antiwedge(&rhs.s),
            v: self.b.antiwedge(&rhs.v),
            b: self.b.antiwedge(&rhs.b),
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Multivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> AntiwedgeProduct<T> for Multivector<T, M>
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type Output = T;

    fn antiwedge(&self, rhs: &T) -> Self::Output {
        self.b.antiwedge(rhs)
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
            s: self.v.antiwedge(rhs),
            v: self.b.antiwedge(rhs),
            b: Bivector::ZERO,
        }
    }
}

impl<T, M> AntiwedgeProduct<Bivector<T, M>> for Multivector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn antiwedge(&self, rhs: &Bivector<T, M>) -> Self::Output {
        Multivector {
            s: self.s.antiwedge(rhs),
            v: self.v.antiwedge(rhs),
            b: self.b.antiwedge(rhs),
        }
    }
}

impl<T, M> AntiwedgeProduct<Evenvector<T, M>> for Multivector<T, M>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, M>;

    fn antiwedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        Multivector {
            s: self.s.antiwedge(&rhs.b) + self.b.antiwedge(&rhs.s),
            v: self.v.antiwedge(&rhs.b),
            b: self.b.antiwedge(&rhs.b),
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
            s: self.s.antiwedge(&rhs.b) + self.v.antiwedge(&rhs.v) + self.b.antiwedge(&rhs.s),
            v: self.v.antiwedge(&rhs.b) + self.b.antiwedge(&rhs.v),
            b: self.b.antiwedge(&rhs.b),
        }
    }
}
