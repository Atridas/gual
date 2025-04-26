use std::ops::{Add, Mul, Neg, Sub};

use num::{Zero, traits::ConstZero};

use crate::{AntiwedgeProduct, KVector, WedgeProduct, reverse_antiwedge, reverse_wedge};

use super::{Bivector3D, Scalar3D, Trivector3D, Vector3D};

impl<T> Zero for Vector3D<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Vector3D {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() == self.y.is_zero()
    }
}

impl<T> ConstZero for Vector3D<T>
where
    T: ConstZero,
{
    const ZERO: Self = Vector3D {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ZERO,
    };
}

impl<T> Add for Vector3D<T>
where
    T: Add<T, Output = T>,
{
    type Output = Vector3D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Vector3D<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector3D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Neg for Vector3D<T>
where
    T: Neg<Output = T>,
{
    type Output = Vector3D<T>;
    fn neg(self) -> Self::Output {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> KVector for Vector3D<T>
where
    T: Copy,
{
    type AntiKVector = Bivector3D<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Bivector3D {
            yz: self.x,
            zx: self.y,
            xy: self.z,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Bivector3D {
            yz: self.x,
            zx: self.y,
            xy: self.z,
        }
    }
}

impl<T> WedgeProduct<Vector3D<T>> for Vector3D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Bivector3D<T>;

    fn wedge(self, rhs: Vector3D<T>) -> Self::Output {
        Bivector3D {
            yz: self.y * rhs.z - self.z * rhs.y,
            zx: self.z * rhs.x - self.x * rhs.z,
            xy: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T> WedgeProduct<Bivector3D<T>> for Vector3D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
{
    type Output = Trivector3D<T>;

    fn wedge(self, rhs: Bivector3D<T>) -> Self::Output {
        Trivector3D {
            xyz: self.x * rhs.yz + self.y * rhs.zx + self.z * rhs.xy,
        }
    }
}

impl<T> AntiwedgeProduct<Bivector3D<T>> for Vector3D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
{
    type Output = Scalar3D<T>;

    fn antiwedge(self, rhs: Bivector3D<T>) -> Self::Output {
        Scalar3D(self.x * rhs.yz + self.y * rhs.zx + self.z * rhs.xy)
    }
}

impl<T> AntiwedgeProduct<Trivector3D<T>> for Vector3D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Vector3D<T>;

    fn antiwedge(self, rhs: Trivector3D<T>) -> Self::Output {
        Vector3D {
            x: self.x * rhs.xyz,
            y: self.y * rhs.xyz,
            z: self.z * rhs.xyz,
        }
    }
}

reverse_wedge!(Bivector3D, Vector3D);

reverse_antiwedge!(Bivector3D, Vector3D);
reverse_antiwedge!(Trivector3D, Vector3D);
