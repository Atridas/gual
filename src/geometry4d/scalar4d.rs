use std::ops::{Add, Mul, Neg, Sub};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{KVector, WedgeProduct, reverse_mul, reverse_wedge};

use super::{Bivector4D, Multivector4D, Quadvector4D, Scalar4D, Trivector4D, Vector4D};

impl<T> Zero for Scalar4D<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Scalar4D(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T> ConstZero for Scalar4D<T>
where
    T: ConstZero,
{
    const ZERO: Self = Scalar4D(T::ZERO);
}

impl<T> One for Scalar4D<T>
where
    T: One,
    Scalar4D<T>: Mul<Output = Scalar4D<T>>,
{
    fn one() -> Self {
        Scalar4D(T::one())
    }
}

impl<T> ConstOne for Scalar4D<T>
where
    T: ConstOne,
{
    const ONE: Self = Scalar4D(T::ONE);
}

impl<T> Add for Scalar4D<T>
where
    T: Add<T, Output = T>,
{
    type Output = Scalar4D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Scalar4D(self.0 + rhs.0)
    }
}

impl<T> Sub for Scalar4D<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Scalar4D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Scalar4D(self.0 - rhs.0)
    }
}

impl<T> Neg for Scalar4D<T>
where
    T: Neg<Output = T>,
{
    type Output = Scalar4D<T>;
    fn neg(self) -> Self::Output {
        Scalar4D(-self.0)
    }
}

impl<T: Clone> KVector for Scalar4D<T> {
    type AntiKVector = Quadvector4D<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Quadvector4D {
            xyzw: self.0.clone(),
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Quadvector4D {
            xyzw: self.0.clone(),
        }
    }
}

impl<T> Mul<Scalar4D<T>> for Scalar4D<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Scalar4D<T>;

    fn mul(self, rhs: Scalar4D<T>) -> Self::Output {
        Scalar4D(self.0 * rhs.0)
    }
}

impl<T> Mul<Vector4D<T>> for Scalar4D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Vector4D<T>;

    fn mul(self, rhs: Vector4D<T>) -> Self::Output {
        Vector4D {
            x: self.0 * rhs.x,
            y: self.0 * rhs.y,
            z: self.0 * rhs.z,
            w: self.0 * rhs.w,
        }
    }
}

impl<T> Mul<Bivector4D<T>> for Scalar4D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector4D<T>;

    fn mul(self, rhs: Bivector4D<T>) -> Self::Output {
        Bivector4D {
            wx: self.0 * rhs.wx,
            wy: self.0 * rhs.wy,
            wz: self.0 * rhs.wz,
            yz: self.0 * rhs.yz,
            zx: self.0 * rhs.zx,
            xy: self.0 * rhs.xy,
        }
    }
}

impl<T> Mul<Trivector4D<T>> for Scalar4D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Trivector4D<T>;

    fn mul(self, rhs: Trivector4D<T>) -> Self::Output {
        Trivector4D {
            wyz: self.0 * rhs.wyz,
            wzx: self.0 * rhs.wzx,
            wxy: self.0 * rhs.wxy,
            zyx: self.0 * rhs.zyx,
        }
    }
}

impl<T> Mul<Quadvector4D<T>> for Scalar4D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Quadvector4D<T>;

    fn mul(self, rhs: Quadvector4D<T>) -> Self::Output {
        Quadvector4D {
            xyzw: self.0 * rhs.xyzw,
        }
    }
}

impl<T> Mul<Multivector4D<T>> for Scalar4D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector4D<T>;

    fn mul(self, rhs: Multivector4D<T>) -> Self::Output {
        Multivector4D {
            s: self * rhs.s,
            v: self * rhs.v,
            b: self * rhs.b,
            t: self * rhs.t,
            a: self * rhs.a,
        }
    }
}

impl<T, V> WedgeProduct<V> for Scalar4D<T>
where
    Scalar4D<T>: Mul<V, Output = V>,
{
    type Output = V;
    fn wedge(self, rhs: V) -> Self::Output {
        self * rhs
    }
}

reverse_mul!(Vector4D, Scalar4D);
reverse_mul!(Bivector4D, Scalar4D);
reverse_mul!(Trivector4D, Scalar4D);
reverse_mul!(Quadvector4D, Scalar4D);
reverse_mul!(Multivector4D, Scalar4D);

reverse_wedge!(Vector4D, Scalar4D);
reverse_wedge!(Bivector4D, Scalar4D);
reverse_wedge!(Trivector4D, Scalar4D);
reverse_wedge!(Quadvector4D, Scalar4D);
reverse_wedge!(Multivector4D, Scalar4D);
