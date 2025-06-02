use std::{
    marker::PhantomData,
    ops::{Add, Mul, Sub},
};

use num::traits::ConstZero;

use crate::{Scalar, WedgeProduct, null_wedge, reverse_wedge_metric, wedge_scalar};

use super::{Bivector, Evenvector, Multivector, Vector};

// ----------------------------------------------------------------------------------------------------
// Scalar
// ----------------------------------------------------------------------------------------------------

wedge_scalar!(Vector, 2);
wedge_scalar!(Bivector, 2);
wedge_scalar!(Evenvector, 2);
wedge_scalar!(Multivector, 2);

// ----------------------------------------------------------------------------------------------------
// Vector
// ----------------------------------------------------------------------------------------------------

impl<T, M> WedgeProduct<Vector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
    T: Sub<Output = T>,
{
    type Output = Bivector<T, M>;

    fn wedge(&self, rhs: &Vector<T, M>) -> Self::Output {
        Bivector {
            xy: self.x * rhs.y - self.y * rhs.x,
            _metric: PhantomData,
        }
    }
}

null_wedge!(Vector, Bivector);

impl<T, M> WedgeProduct<Evenvector<T, M>> for Vector<T, M>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Output = Vector<T, M>;

    fn wedge(&self, rhs: &Evenvector<T, M>) -> Self::Output {
        self.wedge(&rhs.s)
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
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Bivector
// ----------------------------------------------------------------------------------------------------

null_wedge!(Bivector, Vector);
null_wedge!(Bivector, Bivector);

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
    T: Mul<Output = T>,
{
    type Output = Bivector<T, M>;

    fn wedge(&self, rhs: &Multivector<T, M>) -> Self::Output {
        self.wedge(&rhs.s)
    }
}

// ----------------------------------------------------------------------------------------------------
// Evenvector
// ----------------------------------------------------------------------------------------------------

reverse_wedge_metric!(Evenvector, Vector);
reverse_wedge_metric!(Evenvector, Bivector);

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
        }
    }
}

reverse_wedge_metric!(Multivector, Bivector);
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
        }
    }
}
