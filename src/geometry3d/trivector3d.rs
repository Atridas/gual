use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{Antiscalar, AntiwedgeProduct, KVector};

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
    T: ConstOne,
    Scalar3D<T>: Mul<Output = Scalar3D<T>>,
{
    const UNIT_VOLUME: Self = Trivector3D { xyz: T::ONE };
}

impl<T> Trivector3D<T>
where
    T: ConstOne,
{
    pub const XYZ: Self = Trivector3D { xyz: T::ONE };
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

impl<T> AntiwedgeProduct<Trivector3D<T>> for Trivector3D<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Trivector3D<T>;

    fn antiwedge(self, rhs: Trivector3D<T>) -> Self::Output {
        Trivector3D {
            xyz: self.xyz * rhs.xyz,
        }
    }
}
