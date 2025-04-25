use std::ops::{Add, Mul, Neg, Sub};

use num::{Zero, traits::ConstZero};

use crate::{KVector, WedgeProduct};

use super::{Bivector3D, Scalar3D, Trivector3D, Vector3D};

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

impl<T> WedgeProduct<Scalar3D<T>> for Bivector3D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector3D<T>;

    fn wedge(self, rhs: Scalar3D<T>) -> Self::Output {
        Bivector3D {
            yz: self.yz * rhs.0,
            zx: self.zx * rhs.0,
            xy: self.xy * rhs.0,
        }
    }
}

impl<T> WedgeProduct<Vector3D<T>> for Bivector3D<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Trivector3D<T>;

    fn wedge(self, rhs: Vector3D<T>) -> Self::Output {
        Trivector3D {
            xyz: self.yz * rhs.x + self.zx * rhs.y + self.xy * rhs.z,
        }
    }
}
