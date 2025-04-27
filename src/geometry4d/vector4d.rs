use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{AntiwedgeProduct, KVector, WedgeProduct, reverse_antiwedge, reverse_wedge};

use super::{Bivector, Quadvector, Scalar, Trivector, Vector};

impl<T> Zero for Vector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Vector {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero()
    }
}

impl<T> ConstZero for Vector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Vector {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ZERO,
        w: T::ZERO,
    };
}

impl<T> Vector<T>
where
    T: ConstZero,
    T: ConstOne,
{
    pub const X: Self = Vector {
        x: T::ONE,
        y: T::ZERO,
        z: T::ZERO,
        w: T::ZERO,
    };

    pub const Y: Self = Vector {
        x: T::ZERO,
        y: T::ONE,
        z: T::ZERO,
        w: T::ZERO,
    };

    pub const Z: Self = Vector {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ONE,
        w: T::ZERO,
    };

    pub const W: Self = Vector {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ZERO,
        w: T::ONE,
    };
}

impl<T> Add for Vector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Vector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T> Sub for Vector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<T> Neg for Vector<T>
where
    T: Neg<Output = T>,
{
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T> KVector for Vector<T>
where
    T: Copy,
    T: Neg<Output = T>,
{
    type AntiKVector = Trivector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Trivector {
            wyz: self.x,
            wzx: self.y,
            wxy: self.z,
            zyx: self.w,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Trivector {
            wyz: -self.x,
            wzx: -self.y,
            wxy: -self.z,
            zyx: -self.w,
        }
    }
}

impl<T> WedgeProduct<Vector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Bivector<T>;

    fn wedge(&self, rhs: Vector<T>) -> Self::Output {
        Bivector {
            wx: self.w * rhs.x - self.x * rhs.w,
            wy: self.w * rhs.y - self.y * rhs.w,
            wz: self.w * rhs.z - self.z * rhs.w,
            yz: self.y * rhs.z - self.z * rhs.y,
            zx: self.z * rhs.x - self.x * rhs.z,
            xy: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T> WedgeProduct<Bivector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Trivector<T>;

    fn wedge(&self, rhs: Bivector<T>) -> Self::Output {
        Trivector {
            wyz: self.w * rhs.yz - self.y * rhs.wz + self.z * rhs.wy,
            wzx: self.w * rhs.zx - self.z * rhs.wx + self.x * rhs.wz,
            wxy: self.w * rhs.xy - self.x * rhs.wy + self.y * rhs.wx,
            zyx: -(self.x * rhs.yz + self.y * rhs.zx + self.z * rhs.xy),
        }
    }
}

impl<T> WedgeProduct<Trivector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Quadvector<T>;

    fn wedge(&self, rhs: Trivector<T>) -> Self::Output {
        Quadvector {
            xyzw: self.x * rhs.wyz + self.y * rhs.wzx + self.z * rhs.wxy + self.w * rhs.zyx,
        }
    }
}

impl<T> WedgeProduct<Vector<T>> for Trivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Quadvector<T>;

    fn wedge(&self, rhs: Vector<T>) -> Self::Output {
        -rhs.wedge(*self)
    }
}

impl<T> AntiwedgeProduct<Trivector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
{
    type Output = Scalar<T>;

    fn antiwedge(&self, rhs: Trivector<T>) -> Self::Output {
        Scalar(self.x * rhs.wyz + self.y * rhs.wzx + self.z * rhs.wxy + self.w * rhs.zyx)
    }
}

impl<T> AntiwedgeProduct<Vector<T>> for Trivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    Scalar<T>: Neg<Output = Scalar<T>>,
{
    type Output = Scalar<T>;

    fn antiwedge(&self, rhs: Vector<T>) -> Self::Output {
        -rhs.antiwedge(*self)
    }
}

impl<T> AntiwedgeProduct<Quadvector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Vector<T>;

    fn antiwedge(&self, rhs: Quadvector<T>) -> Self::Output {
        Vector {
            x: self.x * rhs.xyzw,
            y: self.y * rhs.xyzw,
            z: self.z * rhs.xyzw,
            w: self.w * rhs.xyzw,
        }
    }
}

reverse_wedge!(Bivector, Vector);

reverse_antiwedge!(Quadvector, Vector);
