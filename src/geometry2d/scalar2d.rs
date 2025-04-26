use std::ops::{Add, Mul, Neg, Sub};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{
    AntiwedgeProduct, KVector, WedgeProduct, reverse_antiwedge, reverse_mul, reverse_wedge,
};

use super::{Bivector2D, Multivector2D, Scalar2D, Vector2D};

impl<T> Zero for Scalar2D<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Scalar2D(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T> ConstZero for Scalar2D<T>
where
    T: ConstZero,
{
    const ZERO: Self = Scalar2D(T::ZERO);
}

impl<T> One for Scalar2D<T>
where
    T: One,
    Scalar2D<T>: Mul<Output = Scalar2D<T>>,
{
    fn one() -> Self {
        Scalar2D(T::one())
    }
}

impl<T> ConstOne for Scalar2D<T>
where
    T: ConstOne,
{
    const ONE: Self = Scalar2D(T::ONE);
}

impl<T> Add for Scalar2D<T>
where
    T: Add<T, Output = T>,
{
    type Output = Scalar2D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Scalar2D(self.0 + rhs.0)
    }
}

impl<T> Sub for Scalar2D<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Scalar2D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Scalar2D(self.0 - rhs.0)
    }
}

impl<T> Neg for Scalar2D<T>
where
    T: Neg<Output = T>,
{
    type Output = Scalar2D<T>;
    fn neg(self) -> Self::Output {
        Scalar2D(-self.0)
    }
}

impl<T: Clone> KVector for Scalar2D<T> {
    type AntiKVector = Bivector2D<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Bivector2D { xy: self.0.clone() }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Bivector2D { xy: self.0.clone() }
    }
}

impl<T> Mul<Scalar2D<T>> for Scalar2D<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Scalar2D<T>;

    fn mul(self, rhs: Scalar2D<T>) -> Self::Output {
        Scalar2D(self.0 * rhs.0)
    }
}

impl<T> Mul<Vector2D<T>> for Scalar2D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Vector2D<T>;

    fn mul(self, rhs: Vector2D<T>) -> Self::Output {
        Vector2D {
            x: self.0 * rhs.x,
            y: self.0 * rhs.y,
        }
    }
}

impl<T> Mul<Bivector2D<T>> for Scalar2D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector2D<T>;

    fn mul(self, rhs: Bivector2D<T>) -> Self::Output {
        Bivector2D {
            xy: self.0 * rhs.xy,
        }
    }
}

impl<T> Mul<Multivector2D<T>> for Scalar2D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector2D<T>;

    fn mul(self, rhs: Multivector2D<T>) -> Self::Output {
        Multivector2D {
            s: self * rhs.s,
            v: self * rhs.v,
            a: self * rhs.a,
        }
    }
}

impl<T, V> WedgeProduct<V> for Scalar2D<T>
where
    Scalar2D<T>: Mul<V, Output = V>,
{
    type Output = V;
    fn wedge(self, rhs: V) -> Self::Output {
        self * rhs
    }
}

impl<T> AntiwedgeProduct<Bivector2D<T>> for Scalar2D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Scalar2D<T>;

    fn antiwedge(self, rhs: Bivector2D<T>) -> Self::Output {
        Scalar2D(self.0 * rhs.xy)
    }
}

reverse_mul!(Vector2D, Scalar2D);
reverse_mul!(Bivector2D, Scalar2D);
reverse_mul!(Multivector2D, Scalar2D);

reverse_wedge!(Vector2D, Scalar2D);
reverse_wedge!(Bivector2D, Scalar2D);
reverse_wedge!(Multivector2D, Scalar2D);

reverse_antiwedge!(Bivector2D, Scalar2D);
