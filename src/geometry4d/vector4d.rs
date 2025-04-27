use std::ops::{Add, Mul, Neg, Sub};

use num::{Zero, traits::ConstZero};

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
        self.x.is_zero() == self.y.is_zero()
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
    type AntiKVector = Bivector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        unimplemented!();
    }

    fn left_complement(&self) -> Self::AntiKVector {
        unimplemented!();
    }
}

impl<T> WedgeProduct<Vector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Bivector<T>;

    fn wedge(self, rhs: Vector<T>) -> Self::Output {
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

    fn wedge(self, rhs: Bivector<T>) -> Self::Output {
        Trivector {
            wyz: self.w * rhs.yz - self.y * rhs.wz + self.z * rhs.wy,
            wzx: self.w * rhs.zx - self.z * rhs.wx + self.x * rhs.wz,
            wxy: self.w * rhs.xy - self.x * rhs.wy + self.y * rhs.wx,
            zyx: -self.x * rhs.yz - self.y * rhs.zx - self.z * rhs.xy,
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

    fn wedge(self, rhs: Trivector<T>) -> Self::Output {
        Quadvector {
            xyzw: self.x * rhs.wyz + self.y * rhs.wzx + self.z * rhs.wxy + self.w * rhs.zyx,
        }
    }
}

impl<T> AntiwedgeProduct<Trivector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Scalar<T>;

    fn antiwedge(self, _rhs: Trivector<T>) -> Self::Output {
        unimplemented!()
    }
}

reverse_wedge!(Bivector, Vector);
reverse_wedge!(Trivector, Vector);

reverse_antiwedge!(Trivector, Vector);
