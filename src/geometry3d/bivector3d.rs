use std::ops::{Add, Neg, Sub};

use num::{Zero, traits::ConstZero};

use crate::KVector;

use super::{Bivector3D, Vector3D};

impl<T> Zero for Bivector3D<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Bivector3D {
            yz: T::zero(),
            zx: T::zero(),
            xy: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.yz.is_zero() && self.zx.is_zero() && self.xy.is_zero()
    }
}

impl<T> ConstZero for Bivector3D<T>
where
    T: ConstZero,
{
    const ZERO: Self = Bivector3D {
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ZERO,
    };
}

impl<T> Add for Bivector3D<T>
where
    T: Add<T, Output = T>,
{
    type Output = Bivector3D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Bivector3D {
            yz: self.yz + rhs.yz,
            zx: self.zx + rhs.zx,
            xy: self.xy + rhs.xy,
        }
    }
}

impl<T> Sub for Bivector3D<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Bivector3D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Bivector3D {
            yz: self.yz - rhs.yz,
            zx: self.zx - rhs.zx,
            xy: self.xy - rhs.xy,
        }
    }
}

impl<T> Neg for Bivector3D<T>
where
    T: Neg<Output = T>,
{
    type Output = Bivector3D<T>;
    fn neg(self) -> Self::Output {
        Bivector3D {
            yz: -self.yz,
            zx: -self.zx,
            xy: -self.xy,
        }
    }
}

impl<T: Clone> KVector for Bivector3D<T> {
    type AntiKVector = Vector3D<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        unimplemented!()
    }

    fn left_complement(&self) -> Self::AntiKVector {
        unimplemented!()
    }
}
