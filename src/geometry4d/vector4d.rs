use std::ops::{Add, Mul, Neg, Sub};

use num::{Zero, traits::ConstZero};

use crate::{AntiwedgeProduct, KVector, WedgeProduct, reverse_antiwedge, reverse_wedge};

use super::{Bivector4D, Quadvector4D, Scalar4D, Trivector4D, Vector4D};

impl<T> Zero for Vector4D<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Vector4D {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() == self.y.is_zero()
    }
}

impl<T> ConstZero for Vector4D<T>
where
    T: ConstZero,
{
    const ZERO: Self = Vector4D {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ZERO,
        w: T::ZERO,
    };
}

impl<T> Add for Vector4D<T>
where
    T: Add<T, Output = T>,
{
    type Output = Vector4D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vector4D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T> Sub for Vector4D<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector4D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector4D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<T> Neg for Vector4D<T>
where
    T: Neg<Output = T>,
{
    type Output = Vector4D<T>;
    fn neg(self) -> Self::Output {
        Vector4D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T> KVector for Vector4D<T>
where
    T: Copy,
    T: Neg<Output = T>,
{
    type AntiKVector = Bivector4D<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        unimplemented!();
    }

    fn left_complement(&self) -> Self::AntiKVector {
        unimplemented!();
    }
}

impl<T> WedgeProduct<Vector4D<T>> for Vector4D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Bivector4D<T>;

    fn wedge(self, rhs: Vector4D<T>) -> Self::Output {
        Bivector4D {
            wx: self.w * rhs.x - self.x * rhs.w,
            wy: self.w * rhs.y - self.y * rhs.w,
            wz: self.w * rhs.z - self.z * rhs.w,
            yz: self.y * rhs.z - self.z * rhs.y,
            zx: self.z * rhs.x - self.x * rhs.z,
            xy: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T> WedgeProduct<Bivector4D<T>> for Vector4D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Trivector4D<T>;

    fn wedge(self, rhs: Bivector4D<T>) -> Self::Output {
        Trivector4D {
            wyz: self.w * rhs.yz - self.y * rhs.wz + self.z * rhs.wy,
            wzx: self.w * rhs.zx - self.z * rhs.wx + self.x * rhs.wz,
            wxy: self.w * rhs.xy - self.x * rhs.wy + self.y * rhs.wx,
            zyx: -self.x * rhs.yz - self.y * rhs.zx - self.z * rhs.xy,
        }
    }
}

impl<T> WedgeProduct<Trivector4D<T>> for Vector4D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Quadvector4D<T>;

    fn wedge(self, rhs: Trivector4D<T>) -> Self::Output {
        Quadvector4D {
            xyzw: self.x * rhs.wyz + self.y * rhs.wzx + self.z * rhs.wxy + self.w * rhs.zyx,
        }
    }
}

impl<T> AntiwedgeProduct<Trivector4D<T>> for Vector4D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Scalar4D<T>;

    fn antiwedge(self, _rhs: Trivector4D<T>) -> Self::Output {
        unimplemented!()
    }
}

reverse_wedge!(Bivector4D, Vector4D);
reverse_wedge!(Trivector4D, Vector4D);

reverse_antiwedge!(Trivector4D, Vector4D);
