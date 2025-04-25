use std::ops::{Add, Mul, Neg, Sub};

use num::{One, Zero, traits::ConstZero};

use crate::{Antiscalar, KVector};

use super::{Bivector2D, Scalar2D};

impl<T> Zero for Bivector2D<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Bivector2D { xy: T::zero() }
    }

    fn is_zero(&self) -> bool {
        self.xy.is_zero()
    }
}

impl<T> ConstZero for Bivector2D<T>
where
    T: ConstZero,
{
    const ZERO: Self = Bivector2D { xy: T::ZERO };
}

impl<T> Antiscalar for Bivector2D<T>
where
    T: One,
    Scalar2D<T>: Mul<Output = Scalar2D<T>>,
{
    fn unit_volume() -> Self {
        Bivector2D { xy: T::one() }
    }
}

impl<T> Add for Bivector2D<T>
where
    T: Add<T, Output = T>,
{
    type Output = Bivector2D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Bivector2D {
            xy: self.xy + rhs.xy,
        }
    }
}

impl<T> Sub for Bivector2D<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Bivector2D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Bivector2D {
            xy: self.xy - rhs.xy,
        }
    }
}

impl<T> Neg for Bivector2D<T>
where
    T: Neg<Output = T>,
{
    type Output = Bivector2D<T>;
    fn neg(self) -> Self::Output {
        Bivector2D { xy: -self.xy }
    }
}

impl<T: Clone> KVector for Bivector2D<T> {
    type AntiKVector = Scalar2D<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Scalar2D(self.xy.clone())
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Scalar2D(self.xy.clone())
    }
}
