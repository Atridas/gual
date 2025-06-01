use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{Antiscalar, AntiwedgeProduct, GeometricProduct, KVector};

use super::{Bivector, Evenvector, Quadvector, Scalar, Trivector, Vector};

impl<T> Zero for Quadvector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Quadvector { xyzw: T::zero() }
    }

    fn is_zero(&self) -> bool {
        self.xyzw.is_zero()
    }
}

impl<T> ConstZero for Quadvector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Quadvector { xyzw: T::ZERO };
}

impl<T: Clone> Antiscalar for Quadvector<T>
where
    T: ConstOne,
{
    const UNIT_VOLUME: Self = Quadvector { xyzw: T::ONE };

    type T = T;

    fn volume(&self) -> T {
        self.xyzw.clone()
    }

    fn from_volume(volume: Self::T) -> Self {
        Quadvector { xyzw: volume }
    }
}

impl<T> Quadvector<T>
where
    T: ConstOne,
{
    pub const XYZW: Self = Quadvector { xyzw: T::ONE };
}

impl<T> Add for Quadvector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Quadvector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Quadvector {
            xyzw: self.xyzw + rhs.xyzw,
        }
    }
}

impl<T> Sub for Quadvector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Quadvector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Quadvector {
            xyzw: self.xyzw - rhs.xyzw,
        }
    }
}

impl<T> Neg for Quadvector<T>
where
    T: Neg<Output = T>,
{
    type Output = Quadvector<T>;
    fn neg(self) -> Self::Output {
        Quadvector { xyzw: -self.xyzw }
    }
}

impl<T: Clone> KVector for Quadvector<T> {
    type AntiKVector = Scalar<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Scalar(self.xyzw.clone())
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Scalar(self.xyzw.clone())
    }
}

impl<T> AntiwedgeProduct<Quadvector<T>> for Quadvector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Quadvector<T>;

    fn antiwedge(&self, rhs: &Quadvector<T>) -> Self::Output {
        Quadvector {
            xyzw: self.xyzw * rhs.xyzw,
        }
    }
}

impl<T> GeometricProduct<Vector<T>> for Quadvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Trivector<T>;

    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        Trivector {
            wyz: -self.xyzw * rhs.x,
            wzx: -self.xyzw * rhs.y,
            wxy: -self.xyzw * rhs.z,
            zyx: T::ZERO,
        }
    }
}

impl<T> GeometricProduct<Bivector<T>> for Quadvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Bivector<T>;

    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        Bivector {
            wx: self.xyzw * rhs.yz,
            wy: self.xyzw * rhs.zx,
            wz: self.xyzw * rhs.xy,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        }
    }
}

impl<T> GeometricProduct<Quadvector<T>> for Quadvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = ();

    fn geometric_product(&self, _rhs: &Quadvector<T>) -> Self::Output {}
}

impl<T> GeometricProduct<Trivector<T>> for Quadvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Vector<T>;

    fn geometric_product(&self, rhs: &Trivector<T>) -> Self::Output {
        Vector {
            x: T::ZERO,
            y: T::ZERO,
            z: T::ZERO,
            w: -self.xyzw * rhs.zyx,
        }
    }
}

impl<T> GeometricProduct<Evenvector<T>> for Quadvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Evenvector<T>;

    fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
        self.geometric_product(&rhs.s) + self.geometric_product(&rhs.b)
    }
}
