use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Float, One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{
    AntiwedgeProduct, KVector, WedgeProduct, reverse_antiwedge, reverse_mul, reverse_wedge,
};

use super::{Bivector, Multivector, Scalar, Trivector, Vector};

impl<T> Zero for Scalar<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Scalar(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T> ConstZero for Scalar<T>
where
    T: ConstZero,
{
    const ZERO: Self = Scalar(T::ZERO);
}

impl<T> One for Scalar<T>
where
    T: One,
    Scalar<T>: Mul<Output = Scalar<T>>,
{
    fn one() -> Self {
        Scalar(T::one())
    }
}

impl<T> ConstOne for Scalar<T>
where
    T: ConstOne,
{
    const ONE: Self = Scalar(T::ONE);
}

impl<T: Float> crate::Scalar for Scalar<T> {
    fn sqrt(&self) -> Self {
        Scalar(self.0.sqrt())
    }
}

impl<T> Add for Scalar<T>
where
    T: Add<T, Output = T>,
{
    type Output = Scalar<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Scalar(self.0 + rhs.0)
    }
}

impl<T> Sub for Scalar<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Scalar<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Scalar(self.0 - rhs.0)
    }
}

impl<T> Neg for Scalar<T>
where
    T: Neg<Output = T>,
{
    type Output = Scalar<T>;
    fn neg(self) -> Self::Output {
        Scalar(-self.0)
    }
}

impl<T: Clone> KVector for Scalar<T> {
    type AntiKVector = Trivector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Trivector {
            xyz: self.0.clone(),
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Trivector {
            xyz: self.0.clone(),
        }
    }
}

impl<T> Mul<Scalar<T>> for Scalar<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Scalar<T>;

    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        Scalar(self.0 * rhs.0)
    }
}

impl<T> Mul<Vector<T>> for Scalar<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Vector {
            x: self.0 * rhs.x,
            y: self.0 * rhs.y,
            z: self.0 * rhs.z,
        }
    }
}

impl<T> Mul<Bivector<T>> for Scalar<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector<T>;

    fn mul(self, rhs: Bivector<T>) -> Self::Output {
        Bivector {
            yz: self.0 * rhs.yz,
            zx: self.0 * rhs.zx,
            xy: self.0 * rhs.xy,
        }
    }
}

impl<T> Mul<Trivector<T>> for Scalar<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Trivector<T>;

    fn mul(self, rhs: Trivector<T>) -> Self::Output {
        Trivector {
            xyz: self.0 * rhs.xyz,
        }
    }
}

impl<T> Mul<Multivector<T>> for Scalar<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn mul(self, rhs: Multivector<T>) -> Self::Output {
        Multivector {
            s: self * rhs.s,
            v: self * rhs.v,
            b: self * rhs.b,
            a: self * rhs.a,
        }
    }
}

impl<T, V> WedgeProduct<V> for Scalar<T>
where
    T: Copy,
    Scalar<T>: Mul<V, Output = V>,
{
    type Output = V;
    fn wedge(&self, rhs: V) -> Self::Output {
        *self * rhs
    }
}

impl<T> AntiwedgeProduct<Trivector<T>> for Scalar<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Scalar<T>;

    fn antiwedge(&self, rhs: Trivector<T>) -> Self::Output {
        Scalar(self.0 * rhs.xyz)
    }
}

reverse_mul!(Vector, Scalar);
reverse_mul!(Bivector, Scalar);
reverse_mul!(Trivector, Scalar);
reverse_mul!(Multivector, Scalar);

reverse_wedge!(Vector, Scalar);
reverse_wedge!(Bivector, Scalar);
reverse_wedge!(Trivector, Scalar);
reverse_wedge!(Multivector, Scalar);

reverse_antiwedge!(Trivector, Scalar);
