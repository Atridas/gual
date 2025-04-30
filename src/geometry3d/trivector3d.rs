use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Float, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{Antiscalar, AntiwedgeProduct, KVector};

use super::{Scalar, Trivector};

impl<T> Zero for Trivector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Trivector { xyz: T::zero() }
    }

    fn is_zero(&self) -> bool {
        self.xyz.is_zero()
    }
}

impl<T> ConstZero for Trivector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Trivector { xyz: T::ZERO };
}

impl<T> Antiscalar for Trivector<T>
where
    T: ConstOne,
    T: Float,
    Scalar<T>: Mul<Output = Scalar<T>>,
{
    const UNIT_VOLUME: Self = Trivector { xyz: T::ONE };

    fn sqrt(&self) -> Self {
        Trivector {
            xyz: self.xyz.sqrt(),
        }
    }
}

impl<T> Trivector<T>
where
    T: ConstOne,
{
    pub const XYZ: Self = Trivector { xyz: T::ONE };
}

impl<T> Add for Trivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Trivector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Trivector {
            xyz: self.xyz + rhs.xyz,
        }
    }
}

impl<T> Sub for Trivector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Trivector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Trivector {
            xyz: self.xyz - rhs.xyz,
        }
    }
}

impl<T> Neg for Trivector<T>
where
    T: Neg<Output = T>,
{
    type Output = Trivector<T>;
    fn neg(self) -> Self::Output {
        Trivector { xyz: -self.xyz }
    }
}

impl<T> Mul<T> for Trivector<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Trivector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Trivector {
            xyz: self.xyz * rhs,
        }
    }
}

impl<T: Clone> KVector for Trivector<T> {
    type AntiKVector = Scalar<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Scalar(self.xyz.clone())
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Scalar(self.xyz.clone())
    }
}

impl<T> AntiwedgeProduct<Trivector<T>> for Trivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Trivector<T>;

    fn antiwedge(&self, rhs: Trivector<T>) -> Self::Output {
        Trivector {
            xyz: self.xyz * rhs.xyz,
        }
    }
}
