use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{AntiwedgeProduct, KVector, WedgeProduct, reverse_antiwedge, reverse_wedge};

use super::{Bivector, Scalar, Trivector, Vector};

impl<T> Zero for Vector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Vector {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
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
    };

    pub const Y: Self = Vector {
        x: T::ZERO,
        y: T::ONE,
        z: T::ZERO,
    };

    pub const Z: Self = Vector {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ONE,
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
        }
    }
}

impl<T> KVector for Vector<T>
where
    T: Copy,
{
    type AntiKVector = Bivector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Bivector {
            yz: self.x,
            zx: self.y,
            xy: self.z,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Bivector {
            yz: self.x,
            zx: self.y,
            xy: self.z,
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
{
    type Output = Trivector<T>;

    fn wedge(&self, rhs: Bivector<T>) -> Self::Output {
        Trivector {
            xyz: self.x * rhs.yz + self.y * rhs.zx + self.z * rhs.xy,
        }
    }
}

impl<T> AntiwedgeProduct<Bivector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
{
    type Output = Scalar<T>;

    fn antiwedge(&self, rhs: Bivector<T>) -> Self::Output {
        Scalar(self.x * rhs.yz + self.y * rhs.zx + self.z * rhs.xy)
    }
}

impl<T> AntiwedgeProduct<Trivector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Vector<T>;

    fn antiwedge(&self, rhs: Trivector<T>) -> Self::Output {
        Vector {
            x: self.x * rhs.xyz,
            y: self.y * rhs.xyz,
            z: self.z * rhs.xyz,
        }
    }
}

reverse_wedge!(Bivector, Vector);

reverse_antiwedge!(Bivector, Vector);
reverse_antiwedge!(Trivector, Vector);
