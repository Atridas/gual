use std::ops::{Add, Mul, Neg, Sub};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::GeometricProduct;

use super::{Bivector, Evenvector, Scalar};

impl<T> Zero for Evenvector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Evenvector {
            s: T::zero(),
            b: Bivector::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.s.is_zero() && self.b.is_zero()
    }
}

impl<T> ConstZero for Evenvector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Evenvector {
        s: T::ZERO,
        b: Bivector::ZERO,
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
            s: T::one(),
            b: Bivector::zero(),
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
        s: T::ONE,
        b: Bivector::ZERO,
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

// impl<T> GeometricProduct<Evenvector<T>> for Evenvector<T>
// where
//     T: Copy,
//     T: Add<T, Output = T>,
//     T: Sub<T, Output = T>,
//     T: Neg<Output = T>,
//     T: Mul<T, Output = T>,
// {
//     type Output = Evenvector<T>;

//     fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
//         self.b.geometric_product(&rhs.b)
//             + Evenvector {
//                 s: self.s * rhs.s,
//                 b: self.s * rhs.b + self.b * rhs.s,
//             }
//     }
// }
