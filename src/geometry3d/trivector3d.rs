use std::ops::{Add, Mul, Neg, Sub};

use num::{One, Zero, traits::ConstZero};

use crate::{Antiscalar, KVector};

use super::{Scalar3D, Trivector3D};

impl<T> Zero for Trivector3D<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Trivector3D { xyz: T::zero() }
    }

    fn is_zero(&self) -> bool {
        self.xyz.is_zero()
    }
}

impl<T> ConstZero for Trivector3D<T>
where
    T: ConstZero,
{
    const ZERO: Self = Trivector3D { xyz: T::ZERO };
}

impl<T> Antiscalar for Trivector3D<T>
where
    T: One,
    Scalar3D<T>: Mul<Output = Scalar3D<T>>,
{
    fn unit_volume() -> Self {
        Trivector3D { xyz: T::one() }
    }
}

impl<T> Add for Trivector3D<T>
where
    T: Add<T, Output = T>,
{
    type Output = Trivector3D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Trivector3D {
            xyz: self.xyz + rhs.xyz,
        }
    }
}

impl<T> Sub for Trivector3D<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Trivector3D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Trivector3D {
            xyz: self.xyz - rhs.xyz,
        }
    }
}

impl<T> Neg for Trivector3D<T>
where
    T: Neg<Output = T>,
{
    type Output = Trivector3D<T>;
    fn neg(self) -> Self::Output {
        Trivector3D { xyz: -self.xyz }
    }
}

impl<T: Clone> KVector for Trivector3D<T> {
    type AntiKVector = Scalar3D<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Scalar3D(self.xyz.clone())
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Scalar3D(self.xyz.clone())
    }
}
