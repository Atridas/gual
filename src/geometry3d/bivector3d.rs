use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{AntiwedgeProduct, KVector, reverse_antiwedge};

use super::{Bivector3D, Trivector3D, Vector3D};

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

impl<T> Bivector3D<T>
where
    T: ConstZero,
    T: ConstOne,
{
    pub const YZ: Self = Bivector3D {
        yz: T::ONE,
        zx: T::ZERO,
        xy: T::ZERO,
    };

    pub const ZX: Self = Bivector3D {
        yz: T::ZERO,
        zx: T::ONE,
        xy: T::ZERO,
    };

    pub const XY: Self = Bivector3D {
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ONE,
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

impl<T: Copy> KVector for Bivector3D<T> {
    type AntiKVector = Vector3D<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Vector3D {
            x: self.yz,
            y: self.zx,
            z: self.xy,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Vector3D {
            x: self.yz,
            y: self.zx,
            z: self.xy,
        }
    }
}

impl<T> AntiwedgeProduct<Bivector3D<T>> for Bivector3D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Vector3D<T>;

    fn antiwedge(self, rhs: Bivector3D<T>) -> Self::Output {
        Vector3D {
            x: self.zx * rhs.xy - self.xy * rhs.zx,
            y: self.xy * rhs.yz - self.yz * rhs.xy,
            z: self.yz * rhs.zx - self.zx * rhs.yz,
        }
    }
}

impl<T> AntiwedgeProduct<Trivector3D<T>> for Bivector3D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Bivector3D<T>;

    fn antiwedge(self, rhs: Trivector3D<T>) -> Self::Output {
        Bivector3D {
            yz: self.yz * rhs.xyz,
            zx: self.zx * rhs.xyz,
            xy: self.xy * rhs.xyz,
        }
    }
}

reverse_antiwedge!(Trivector3D, Bivector3D);
