use std::ops::{Add, Mul, Neg, Sub};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{GeometricProduct, reverse_add};

use super::{Bivector, Evenvector, Quadvector, Scalar};

impl<T> Zero for Evenvector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Evenvector {
            s: Scalar::zero(),
            b: Bivector::zero(),
            a: Quadvector::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.s.is_zero() && self.b.is_zero() && self.a.is_zero()
    }
}

impl<T> ConstZero for Evenvector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Evenvector {
        s: Scalar::ZERO,
        b: Bivector::ZERO,
        a: Quadvector::ZERO,
    };
}

impl<T> One for Evenvector<T>
where
    T: Copy,
    T: Zero,
    T: One,
    Evenvector<T>: Mul<Evenvector<T>, Output = Evenvector<T>>,
{
    fn one() -> Self {
        Evenvector {
            s: Scalar::one(),
            b: Bivector::zero(),
            a: Quadvector::zero(),
        }
    }
}

impl<T> ConstOne for Evenvector<T>
where
    T: Copy,
    T: ConstZero,
    T: ConstOne,
    Evenvector<T>: One,
{
    const ONE: Self = Evenvector {
        s: Scalar::ONE,
        b: Bivector::ZERO,
        a: Quadvector::ZERO,
    };
}

impl<T> Add for Evenvector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Evenvector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Evenvector {
            s: self.s + rhs.s,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl<T> Add<Scalar<T>> for Evenvector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Evenvector<T>;
    fn add(self, rhs: Scalar<T>) -> Self::Output {
        Evenvector {
            s: self.s + rhs,
            b: self.b,
            a: self.a,
        }
    }
}

impl<T> Add<Bivector<T>> for Evenvector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Evenvector<T>;
    fn add(self, rhs: Bivector<T>) -> Self::Output {
        Evenvector {
            s: self.s,
            b: self.b + rhs,
            a: self.a,
        }
    }
}

impl<T> Add<Quadvector<T>> for Evenvector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Evenvector<T>;
    fn add(self, rhs: Quadvector<T>) -> Self::Output {
        Evenvector {
            s: self.s,
            b: self.b,
            a: self.a + rhs,
        }
    }
}

impl<T> Sub for Evenvector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Evenvector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Evenvector {
            s: self.s - rhs.s,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl<T> Neg for Evenvector<T>
where
    T: Neg<Output = T>,
{
    type Output = Evenvector<T>;
    fn neg(self) -> Self::Output {
        Evenvector {
            s: -self.s,
            b: -self.b,
            a: -self.a,
        }
    }
}

impl<T> Mul<Evenvector<T>> for Evenvector<T>
where
    Evenvector<T>: GeometricProduct<Evenvector<T>, Output = Evenvector<T>>,
{
    type Output = Evenvector<T>;

    fn mul(self, rhs: Evenvector<T>) -> Self::Output {
        self.geometric_product(&rhs)
    }
}

impl<T> GeometricProduct<Evenvector<T>> for Evenvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Evenvector<T>;

    fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
        Evenvector {
            s: self.s * rhs.s,
            b: self.s * rhs.b + self.b * rhs.s,
            a: self.s * rhs.a + self.a * rhs.s,
        } + self.b.geometric_product(&rhs.b)
            + self.b.geometric_product(&rhs.a)
            + self.a.geometric_product(&rhs.b)
    }
}

reverse_add!(Scalar, Evenvector);
reverse_add!(Bivector, Evenvector);
reverse_add!(Quadvector, Evenvector);
