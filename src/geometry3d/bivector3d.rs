use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{AntiwedgeProduct, KVector, reverse_antiwedge};

use super::{Bivector, Trivector, Vector};

impl<T> Zero for Bivector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Bivector {
            yz: T::zero(),
            zx: T::zero(),
            xy: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.yz.is_zero() && self.zx.is_zero() && self.xy.is_zero()
    }
}

impl<T> ConstZero for Bivector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Bivector {
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ZERO,
    };
}

impl<T> Bivector<T>
where
    T: ConstZero,
    T: ConstOne,
{
    pub const YZ: Self = Bivector {
        yz: T::ONE,
        zx: T::ZERO,
        xy: T::ZERO,
    };

    pub const ZX: Self = Bivector {
        yz: T::ZERO,
        zx: T::ONE,
        xy: T::ZERO,
    };

    pub const XY: Self = Bivector {
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ONE,
    };
}

impl<T> Add for Bivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Bivector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Bivector {
            yz: self.yz + rhs.yz,
            zx: self.zx + rhs.zx,
            xy: self.xy + rhs.xy,
        }
    }
}

impl<T> Sub for Bivector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Bivector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Bivector {
            yz: self.yz - rhs.yz,
            zx: self.zx - rhs.zx,
            xy: self.xy - rhs.xy,
        }
    }
}

impl<T> Neg for Bivector<T>
where
    T: Neg<Output = T>,
{
    type Output = Bivector<T>;
    fn neg(self) -> Self::Output {
        Bivector {
            yz: -self.yz,
            zx: -self.zx,
            xy: -self.xy,
        }
    }
}

impl<T: Copy> KVector for Bivector<T> {
    type AntiKVector = Vector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Vector {
            x: self.yz,
            y: self.zx,
            z: self.xy,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Vector {
            x: self.yz,
            y: self.zx,
            z: self.xy,
        }
    }
}

impl<T> AntiwedgeProduct<Bivector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Vector<T>;

    fn antiwedge(self, rhs: Bivector<T>) -> Self::Output {
        Vector {
            x: self.zx * rhs.xy - self.xy * rhs.zx,
            y: self.xy * rhs.yz - self.yz * rhs.xy,
            z: self.yz * rhs.zx - self.zx * rhs.yz,
        }
    }
}

impl<T> AntiwedgeProduct<Trivector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector<T>;

    fn antiwedge(self, rhs: Trivector<T>) -> Self::Output {
        Bivector {
            yz: self.yz * rhs.xyz,
            zx: self.zx * rhs.xyz,
            xy: self.xy * rhs.xyz,
        }
    }
}

reverse_antiwedge!(Trivector, Bivector);
