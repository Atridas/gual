use std::ops::{Add, Mul, Neg, Sub};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{
    AntiwedgeProduct, KVector, WedgeProduct, reverse_antiwedge, reverse_mul, reverse_wedge,
};

use super::{Bivector3D, Multivector3D, Scalar3D, Trivector3D, Vector3D};

impl<T> Zero for Scalar3D<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Scalar3D(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T> ConstZero for Scalar3D<T>
where
    T: ConstZero,
{
    const ZERO: Self = Scalar3D(T::ZERO);
}

impl<T> One for Scalar3D<T>
where
    T: One,
    Scalar3D<T>: Mul<Output = Scalar3D<T>>,
{
    fn one() -> Self {
        Scalar3D(T::one())
    }
}

impl<T> ConstOne for Scalar3D<T>
where
    T: ConstOne,
{
    const ONE: Self = Scalar3D(T::ONE);
}

impl<T> Add for Scalar3D<T>
where
    T: Add<T, Output = T>,
{
    type Output = Scalar3D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Scalar3D(self.0 + rhs.0)
    }
}

impl<T> Sub for Scalar3D<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Scalar3D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Scalar3D(self.0 - rhs.0)
    }
}

impl<T> Neg for Scalar3D<T>
where
    T: Neg<Output = T>,
{
    type Output = Scalar3D<T>;
    fn neg(self) -> Self::Output {
        Scalar3D(-self.0)
    }
}

impl<T: Clone> KVector for Scalar3D<T> {
    type AntiKVector = Trivector3D<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Trivector3D {
            xyz: self.0.clone(),
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Trivector3D {
            xyz: self.0.clone(),
        }
    }
}

impl<T> Mul<Scalar3D<T>> for Scalar3D<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Scalar3D<T>;

    fn mul(self, rhs: Scalar3D<T>) -> Self::Output {
        Scalar3D(self.0 * rhs.0)
    }
}

impl<T> Mul<Vector3D<T>> for Scalar3D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Vector3D<T>;

    fn mul(self, rhs: Vector3D<T>) -> Self::Output {
        Vector3D {
            x: self.0 * rhs.x,
            y: self.0 * rhs.y,
            z: self.0 * rhs.z,
        }
    }
}

impl<T> Mul<Bivector3D<T>> for Scalar3D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector3D<T>;

    fn mul(self, rhs: Bivector3D<T>) -> Self::Output {
        Bivector3D {
            yz: self.0 * rhs.yz,
            zx: self.0 * rhs.zx,
            xy: self.0 * rhs.xy,
        }
    }
}

impl<T> Mul<Trivector3D<T>> for Scalar3D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Trivector3D<T>;

    fn mul(self, rhs: Trivector3D<T>) -> Self::Output {
        Trivector3D {
            xyz: self.0 * rhs.xyz,
        }
    }
}

impl<T> Mul<Multivector3D<T>> for Scalar3D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector3D<T>;

    fn mul(self, rhs: Multivector3D<T>) -> Self::Output {
        Multivector3D {
            s: self * rhs.s,
            v: self * rhs.v,
            b: self * rhs.b,
            a: self * rhs.a,
        }
    }
}

impl<T, V> WedgeProduct<V> for Scalar3D<T>
where
    Scalar3D<T>: Mul<V, Output = V>,
{
    type Output = V;
    fn wedge(self, rhs: V) -> Self::Output {
        self * rhs
    }
}

impl<T> AntiwedgeProduct<Trivector3D<T>> for Scalar3D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Scalar3D<T>;

    fn antiwedge(self, rhs: Trivector3D<T>) -> Self::Output {
        Scalar3D(self.0 * rhs.xyz)
    }
}

reverse_mul!(Vector3D, Scalar3D);
reverse_mul!(Bivector3D, Scalar3D);
reverse_mul!(Trivector3D, Scalar3D);
reverse_mul!(Multivector3D, Scalar3D);

reverse_wedge!(Vector3D, Scalar3D);
reverse_wedge!(Bivector3D, Scalar3D);
reverse_wedge!(Trivector3D, Scalar3D);
reverse_wedge!(Multivector3D, Scalar3D);

reverse_antiwedge!(Trivector3D, Scalar3D);
