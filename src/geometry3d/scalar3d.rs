use std::{
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{
    AntiwedgeProduct, GeometricProduct, KVector, WedgeProduct, reverse_antiwedge,
    reverse_geometric, reverse_mul, reverse_wedge,
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

impl<T> Mul<T> for Scalar<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Scalar<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Scalar(self.0 * rhs)
    }
}

impl<T: Clone> KVector for Scalar<T> {
    type AntiKVector = Trivector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Trivector {
            xyz: self.0.clone(),
            _metric: PhantomData,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Trivector {
            xyz: self.0.clone(),
            _metric: PhantomData,
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
            _metric: PhantomData,
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
            _metric: PhantomData,
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
            _metric: PhantomData,
        }
    }
}

impl<T> Mul<Multivector<T>> for Scalar<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn mul(self, _rhs: Multivector<T>) -> Self::Output {
        unimplemented!();
        // Multivector {
        //     s: self * rhs.s,
        //     v: self * rhs.v,
        //     b: self * rhs.b,
        //     a: self * rhs.a,
        // }
    }
}

impl<T, V> WedgeProduct<V> for Scalar<T>
where
    T: Copy,
    V: Copy,
    Scalar<T>: Mul<V, Output = V>,
{
    type Output = V;
    fn wedge(&self, rhs: &V) -> Self::Output {
        *self * *rhs
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

    fn antiwedge(&self, rhs: &Trivector<T>) -> Self::Output {
        Scalar(self.0 * rhs.xyz)
    }
}

impl<T, V> GeometricProduct<V> for Scalar<T>
where
    T: Copy,
    V: Copy,
    Scalar<T>: Mul<V, Output = V>,
{
    type Output = V;
    fn geometric_product(&self, rhs: &V) -> Self::Output {
        *self * *rhs
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

reverse_geometric!(Vector, Scalar);
reverse_geometric!(Bivector, Scalar);
reverse_geometric!(Trivector, Scalar);
reverse_geometric!(Multivector, Scalar);
