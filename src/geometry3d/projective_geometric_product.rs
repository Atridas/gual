use std::{
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

use num::traits::ConstZero;

use crate::{GeometricProduct, Projective, WedgeProduct, geometry3d::Trivector};

use super::{Bivector, Evenvector, Multivector, Vector};

// ----------------------------------------------------------------------------------------------------
// Vector
// ----------------------------------------------------------------------------------------------------

impl<T> GeometricProduct<Vector<T, Projective>> for Vector<T, Projective>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T, Projective>;

    fn geometric_product(&self, rhs: &Vector<T, Projective>) -> Self::Output {
        Evenvector {
            s: self.x * rhs.x + self.y * rhs.y,
            b: self.wedge(rhs),
        }
    }
}

impl<T> GeometricProduct<Bivector<T, Projective>> for Vector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Bivector<T, Projective>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: Vector {
                x: -self.y * rhs.xy,
                y: self.x * rhs.xy,
                z: self.y * rhs.yz - self.x * rhs.zx,
                _metric: PhantomData,
            },
            b: Bivector::ZERO,
            t: self.wedge(rhs),
        }
    }
}

impl<T> GeometricProduct<Trivector<T, Projective>> for Vector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type Output = Bivector<T, Projective>;

    fn geometric_product(&self, rhs: &Trivector<T, Projective>) -> Self::Output {
        Bivector {
            yz: self.x * rhs.xyz,
            zx: self.y * rhs.xyz,
            xy: T::ZERO,
            _metric: PhantomData,
        }
    }
}

impl<T> GeometricProduct<Evenvector<T, Projective>> for Vector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Evenvector<T, Projective>) -> Self::Output {
        let a = self.geometric_product(&rhs.s);
        let c = Vector {
            x: -self.y * rhs.b.xy,
            y: self.x * rhs.b.xy,
            z: self.y * rhs.b.yz - self.x * rhs.b.zx,
            _metric: PhantomData,
        };
        let d = self.wedge(&rhs.b);

        Multivector {
            s: T::ZERO,
            v: a + c,
            b: Bivector::ZERO,
            t: d,
        }
    }
}

impl<T> GeometricProduct<Multivector<T, Projective>> for Vector<T, Projective>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Multivector<T, Projective>) -> Self::Output {
        let a = self.geometric_product(&rhs.s);
        let b = self.geometric_product(&rhs.v);
        let c = Vector {
            x: -self.y * rhs.b.xy,
            y: self.x * rhs.b.xy,
            z: self.y * rhs.b.yz - self.x * rhs.b.zx,
            _metric: PhantomData,
        };
        let d = self.wedge(&rhs.b);

        let yz = self.x * rhs.t.xyz;
        let zx = self.y * rhs.t.xyz;

        Multivector {
            s: b.s,
            v: a + c,
            b: Bivector {
                yz: b.b.yz + yz,
                zx: b.b.zx + zx,
                xy: b.b.xy,
                _metric: PhantomData,
            },
            t: d,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Bivector
// ----------------------------------------------------------------------------------------------------

impl<T> GeometricProduct<Vector<T, Projective>> for Bivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Vector<T, Projective>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: Vector {
                x: self.xy * rhs.y,
                y: -(self.xy * rhs.x),
                z: self.zx * rhs.x - self.yz * rhs.y,
                _metric: PhantomData,
            },
            b: Bivector::ZERO,
            t: self.wedge(rhs),
        }
    }
}

impl<T> GeometricProduct<Bivector<T, Projective>> for Bivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T, Projective>;

    fn geometric_product(&self, rhs: &Bivector<T, Projective>) -> Self::Output {
        Evenvector {
            s: -(self.xy * rhs.xy),
            b: Bivector {
                yz: self.xy * rhs.zx - self.zx * rhs.xy,
                zx: self.yz * rhs.xy - self.xy * rhs.yz,
                xy: T::ZERO,
                _metric: PhantomData,
            },
        }
    }
}

impl<T> GeometricProduct<Trivector<T, Projective>> for Bivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Vector<T, Projective>;

    fn geometric_product(&self, rhs: &Trivector<T, Projective>) -> Self::Output {
        Vector {
            x: T::ZERO,
            y: T::ZERO,
            z: self.xy * -rhs.xyz,
            _metric: PhantomData,
        }
    }
}

impl<T> GeometricProduct<Evenvector<T, Projective>> for Bivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T, Projective>;

    fn geometric_product(&self, rhs: &Evenvector<T, Projective>) -> Self::Output {
        self.geometric_product(&rhs.s) + self.geometric_product(&rhs.b)
    }
}

impl<T> GeometricProduct<Multivector<T, Projective>> for Bivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;
    fn geometric_product(&self, rhs: &Multivector<T, Projective>) -> Self::Output {
        self.geometric_product(&rhs.s)
            + self.geometric_product(&rhs.v)
            + self.geometric_product(&rhs.b)
            + self.geometric_product(&rhs.t)
    }
}

// ----------------------------------------------------------------------------------------------------
// Trivector
// ----------------------------------------------------------------------------------------------------

impl<T> GeometricProduct<Vector<T, Projective>> for Trivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type Output = Bivector<T, Projective>;

    fn geometric_product(&self, rhs: &Vector<T, Projective>) -> Self::Output {
        rhs.geometric_product(self)
    }
}

impl<T> GeometricProduct<Bivector<T, Projective>> for Trivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Vector<T, Projective>;

    fn geometric_product(&self, rhs: &Bivector<T, Projective>) -> Self::Output {
        rhs.geometric_product(self)
    }
}

impl<T> GeometricProduct<Trivector<T, Projective>> for Trivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
{
    type Output = T;

    fn geometric_product(&self, _rhs: &Trivector<T, Projective>) -> Self::Output {
        T::ZERO
    }
}

impl<T> GeometricProduct<Evenvector<T, Projective>> for Trivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Evenvector<T, Projective>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: self.geometric_product(&rhs.b),
            b: Bivector::ZERO,
            t: self.geometric_product(&rhs.s),
        }
    }
}

impl<T> GeometricProduct<Multivector<T, Projective>> for Trivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Multivector<T, Projective>) -> Self::Output {
        Multivector {
            s: self.geometric_product(&rhs.t),
            v: self.geometric_product(&rhs.b),
            b: self.geometric_product(&rhs.v),
            t: self.geometric_product(&rhs.s),
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Evenvector
// ----------------------------------------------------------------------------------------------------

impl<T> GeometricProduct<Vector<T, Projective>> for Evenvector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Vector<T, Projective>) -> Self::Output {
        self.s.geometric_product(rhs) + self.b.geometric_product(rhs)
    }
}

impl<T> GeometricProduct<Bivector<T, Projective>> for Evenvector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Evenvector<T, Projective>;

    fn geometric_product(&self, rhs: &Bivector<T, Projective>) -> Self::Output {
        self.s.geometric_product(rhs) + self.b.geometric_product(rhs)
    }
}

impl<T> GeometricProduct<Trivector<T, Projective>> for Evenvector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Trivector<T, Projective>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: self.b.geometric_product(rhs),
            b: Bivector::ZERO,
            t: self.s.geometric_product(rhs),
        }
    }
}

impl<T> GeometricProduct<Multivector<T, Projective>> for Evenvector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Neg<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Multivector<T, Projective>) -> Self::Output {
        self.s.geometric_product(rhs) + self.b.geometric_product(rhs)
    }
}

// ----------------------------------------------------------------------------------------------------
// Multivector
// ----------------------------------------------------------------------------------------------------

impl<T> GeometricProduct<Vector<T, Projective>> for Multivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Vector<T, Projective>) -> Self::Output {
        self.s.geometric_product(rhs)
            + self.v.geometric_product(rhs)
            + self.b.geometric_product(rhs)
            + self.t.geometric_product(rhs)
    }
}

impl<T> GeometricProduct<Bivector<T, Projective>> for Multivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Bivector<T, Projective>) -> Self::Output {
        self.s.geometric_product(rhs)
            + self.v.geometric_product(rhs)
            + self.b.geometric_product(rhs)
            + self.t.geometric_product(rhs)
    }
}

impl<T> GeometricProduct<Trivector<T, Projective>> for Multivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Trivector<T, Projective>) -> Self::Output {
        self.s.geometric_product(rhs)
            + self.v.geometric_product(rhs)
            + self.b.geometric_product(rhs)
            + self.t.geometric_product(rhs)
    }
}

impl<T> GeometricProduct<Multivector<T, Projective>> for Multivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Add<Output = T>,
    T: Neg<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Multivector<T, Projective>;

    fn geometric_product(&self, rhs: &Multivector<T, Projective>) -> Self::Output {
        self.s.geometric_product(rhs)
            + self.v.geometric_product(rhs)
            + self.b.geometric_product(rhs)
            + self.t.geometric_product(rhs)
    }
}
