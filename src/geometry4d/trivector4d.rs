use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{AntiwedgeProduct, KVector, reverse_antiwedge};

use super::{Quadvector, Trivector, Vector};

impl<T> Zero for Trivector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Trivector {
            wyz: T::zero(),
            wzx: T::zero(),
            wxy: T::zero(),
            zyx: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.wyz.is_zero() && self.wzx.is_zero() && self.wxy.is_zero() && self.zyx.is_zero()
    }
}

impl<T> ConstZero for Trivector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Trivector {
        wyz: T::ZERO,
        wzx: T::ZERO,
        wxy: T::ZERO,
        zyx: T::ZERO,
    };
}

impl<T> Trivector<T>
where
    T: ConstZero,
    T: ConstOne,
{
    pub const WYZ: Self = Trivector {
        wyz: T::ONE,
        wzx: T::ZERO,
        wxy: T::ZERO,
        zyx: T::ZERO,
    };

    pub const WZX: Self = Trivector {
        wyz: T::ZERO,
        wzx: T::ONE,
        wxy: T::ZERO,
        zyx: T::ZERO,
    };

    pub const WXY: Self = Trivector {
        wyz: T::ZERO,
        wzx: T::ZERO,
        wxy: T::ONE,
        zyx: T::ZERO,
    };

    pub const ZYX: Self = Trivector {
        wyz: T::ZERO,
        wzx: T::ZERO,
        wxy: T::ZERO,
        zyx: T::ONE,
    };
}

impl<T> Add for Trivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Trivector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Trivector {
            wyz: self.wyz + rhs.wyz,
            wzx: self.wzx + rhs.wzx,
            wxy: self.wxy + rhs.wxy,
            zyx: self.zyx + rhs.zyx,
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
            wyz: self.wyz - rhs.wyz,
            wzx: self.wzx - rhs.wzx,
            wxy: self.wxy - rhs.wxy,
            zyx: self.zyx - rhs.zyx,
        }
    }
}

impl<T> Neg for Trivector<T>
where
    T: Neg<Output = T>,
{
    type Output = Trivector<T>;
    fn neg(self) -> Self::Output {
        Trivector {
            wyz: -self.wyz,
            wzx: -self.wzx,
            wxy: -self.wxy,
            zyx: -self.zyx,
        }
    }
}

impl<T> KVector for Trivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
{
    type AntiKVector = Vector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Vector {
            x: -self.wyz,
            y: -self.wzx,
            z: -self.wxy,
            w: -self.zyx,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Vector {
            x: self.wyz,
            y: self.wzx,
            z: self.wxy,
            w: self.zyx,
        }
    }
}

impl<T> AntiwedgeProduct<Quadvector<T>> for Trivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Trivector<T>;

    fn antiwedge(self, rhs: Quadvector<T>) -> Self::Output {
        Trivector {
            wyz: self.wyz * rhs.xyzw,
            wzx: self.wzx * rhs.xyzw,
            wxy: self.wxy * rhs.xyzw,
            zyx: self.zyx * rhs.xyzw,
        }
    }
}

reverse_antiwedge!(Quadvector, Trivector);
