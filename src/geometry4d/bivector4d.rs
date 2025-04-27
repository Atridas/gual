use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Float, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{AntiwedgeProduct, Epsilon, KVector, WedgeProduct, reverse_antiwedge};

use super::{Bivector, Quadvector, Scalar, Trivector, Vector};

impl<T> Zero for Bivector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Bivector {
            wx: T::zero(),
            wy: T::zero(),
            wz: T::zero(),
            yz: T::zero(),
            zx: T::zero(),
            xy: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.wx.is_zero()
            && self.wy.is_zero()
            && self.wz.is_zero()
            && self.yz.is_zero()
            && self.zx.is_zero()
            && self.xy.is_zero()
    }
}

impl<T> ConstZero for Bivector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Bivector {
        wx: T::ZERO,
        wy: T::ZERO,
        wz: T::ZERO,
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
    pub const WX: Self = Bivector {
        wx: T::ONE,
        wy: T::ZERO,
        wz: T::ZERO,
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ZERO,
    };

    pub const WY: Self = Bivector {
        wx: T::ZERO,
        wy: T::ONE,
        wz: T::ZERO,
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ZERO,
    };

    pub const WZ: Self = Bivector {
        wx: T::ZERO,
        wy: T::ZERO,
        wz: T::ONE,
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ZERO,
    };

    pub const YZ: Self = Bivector {
        wx: T::ZERO,
        wy: T::ZERO,
        wz: T::ZERO,
        yz: T::ONE,
        zx: T::ZERO,
        xy: T::ZERO,
    };

    pub const ZX: Self = Bivector {
        wx: T::ZERO,
        wy: T::ZERO,
        wz: T::ZERO,
        yz: T::ZERO,
        zx: T::ONE,
        xy: T::ZERO,
    };

    pub const XY: Self = Bivector {
        wx: T::ZERO,
        wy: T::ZERO,
        wz: T::ZERO,
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ONE,
    };
}

impl<T> Bivector<T>
where
    T: Float,
    T: Epsilon,
{
    pub fn is_2_blade(&self) -> bool {
        let dot = (self.wx * self.yz + self.wy * self.zx + self.wz * self.xy).abs();
        dot.is_near_zero()
    }
}

impl<T> Add for Bivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Bivector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Bivector {
            wx: self.wx + rhs.wx,
            wy: self.wy + rhs.wy,
            wz: self.wz + rhs.wz,
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
            wx: self.wx - rhs.wx,
            wy: self.wy - rhs.wy,
            wz: self.wz - rhs.wz,
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
            wx: -self.wx,
            wy: -self.wy,
            wz: -self.wz,
            yz: -self.yz,
            zx: -self.zx,
            xy: -self.xy,
        }
    }
}

impl<T: Copy + Neg<Output = T>> KVector for Bivector<T> {
    type AntiKVector = Bivector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Bivector {
            wx: -self.yz,
            wy: -self.zx,
            wz: -self.xy,
            yz: -self.wx,
            zx: -self.wy,
            xy: -self.wz,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Bivector {
            wx: -self.yz,
            wy: -self.zx,
            wz: -self.xy,
            yz: -self.wx,
            zx: -self.wy,
            xy: -self.wz,
        }
    }
}

impl<T> WedgeProduct<Bivector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Quadvector<T>;

    fn wedge(self, rhs: Bivector<T>) -> Self::Output {
        Quadvector {
            xyzw: -(self.wx * rhs.yz
                + self.wy * rhs.zx
                + self.wz * rhs.xy
                + self.yz * rhs.wx
                + self.zx * rhs.wy
                + self.xy * rhs.wz),
        }
    }
}

impl<T> AntiwedgeProduct<Bivector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Scalar<T>;

    fn antiwedge(self, rhs: Bivector<T>) -> Self::Output {
        Scalar(
            -(self.wx * rhs.yz
                + self.wy * rhs.zx
                + self.wz * rhs.xy
                + self.yz * rhs.wx
                + self.zx * rhs.wy
                + self.xy * rhs.wz),
        )
    }
}

impl<T> AntiwedgeProduct<Trivector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Vector<T>;

    fn antiwedge(self, rhs: Trivector<T>) -> Self::Output {
        Vector {
            x: self.wx * rhs.zyx + self.zx * rhs.wxy - self.xy * rhs.wzx,
            y: self.wy * rhs.zyx + self.xy * rhs.wyz - self.yz * rhs.wxy,
            z: self.wz * rhs.zyx + self.yz * rhs.wzx - self.zx * rhs.wyz,
            w: -(self.wx * rhs.wyz + self.wy * rhs.wzx + self.wz * rhs.wxy),
        }
    }
}

impl<T> AntiwedgeProduct<Quadvector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector<T>;

    fn antiwedge(self, rhs: Quadvector<T>) -> Self::Output {
        Bivector {
            wx: self.wx * rhs.xyzw,
            wy: self.wy * rhs.xyzw,
            wz: self.wz * rhs.xyzw,
            yz: self.yz * rhs.xyzw,
            zx: self.zx * rhs.xyzw,
            xy: self.xy * rhs.xyzw,
        }
    }
}

reverse_antiwedge!(Trivector, Bivector);
reverse_antiwedge!(Quadvector, Bivector);
