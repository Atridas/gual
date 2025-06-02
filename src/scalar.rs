use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use num::traits::{ConstOne, ConstZero};

use crate::Scalar;

impl<const D: u32, T, M> Scalar<D, T, M> {
    pub fn new(v: T) -> Self {
        Scalar(v, PhantomData)
    }
}

impl<const D: u32, T, M> Scalar<D, T, M>
where
    T: ConstZero,
{
    pub const ZERO: Self = Scalar(T::ZERO, PhantomData);
}

impl<const D: u32, T, M> Scalar<D, T, M>
where
    T: ConstOne,
{
    pub const ONE: Self = Scalar(T::ONE, PhantomData);
}

impl<const D: u32, T, M, V> Add<V> for Scalar<D, T, M>
where
    V: Add<Scalar<D, T, M>>,
{
    type Output = <V as Add<Scalar<D, T, M>>>::Output;

    fn add(self, rhs: V) -> Self::Output {
        rhs + self
    }
}

impl<const D: u32, T, M, V> Mul<V> for Scalar<D, T, M>
where
    V: Mul<Scalar<D, T, M>>,
{
    type Output = <V as Mul<Scalar<D, T, M>>>::Output;

    fn mul(self, rhs: V) -> Self::Output {
        rhs * self
    }
}
