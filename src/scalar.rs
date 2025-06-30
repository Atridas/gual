use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{Metric, Projective, Scalar};

impl<const D: u32, T, M> Scalar<D, T, M> {
    pub fn new(v: T) -> Self {
        Scalar(v, PhantomData)
    }
}

impl<const D: u32, T, M> Zero for Scalar<D, T, M>
where
    T: Zero,
{
    fn zero() -> Self {
        Scalar(T::zero(), PhantomData)
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<const D: u32, T, M> ConstZero for Scalar<D, T, M>
where
    T: ConstZero,
{
    const ZERO: Self = Scalar(T::ZERO, PhantomData);
}

impl<const D: u32, T, M> One for Scalar<D, T, M>
where
    T: One,
{
    fn one() -> Self {
        Scalar(T::one(), PhantomData)
    }
}

impl<const D: u32, T, M> ConstOne for Scalar<D, T, M>
where
    T: ConstOne,
    T: PartialEq,
{
    const ONE: Self = Scalar(T::ONE, PhantomData);
}

impl<const D: u32, T, M> Add<Scalar<D, T, M>> for Scalar<D, T, M>
where
    T: Add<Output = T>,
{
    type Output = Scalar<D, T, M>;

    fn add(self, rhs: Scalar<D, T, M>) -> Self::Output {
        Scalar::<D, T, M>(self.0 + rhs.0, PhantomData)
    }
}

impl<const D: u32, T, M> Mul<Scalar<D, T, M>> for Scalar<D, T, M>
where
    T: Mul<Output = T>,
{
    type Output = Scalar<D, T, M>;

    fn mul(self, rhs: Scalar<D, T, M>) -> Self::Output {
        Scalar::<D, T, M>(self.0 * rhs.0, PhantomData)
    }
}

impl<const D: u32, T: Clone> Metric for Scalar<D, T>
where
    T: Zero,
{
    type Bulk = Self;
    type Weight = Self;

    fn bulk(&self) -> Self {
        self.clone()
    }

    fn weight(&self) -> Self {
        self.clone()
    }

    fn from_bulk(bulk: &Self) -> Self {
        bulk.clone()
    }

    fn from_weight(weight: &Self) -> Self {
        weight.clone()
    }

    fn from_bulk_and_weight(bulk: &Self::Bulk, weight: &Self::Weight) -> Self {
        assert!(weight.is_zero());
        bulk.clone()
    }

    fn proper_bulk(&self) -> Self {
        self.clone()
    }

    fn proper_weight(&self) -> Self {
        self.clone()
    }
}

impl<const D: u32, T: Clone> Metric for Scalar<D, T, Projective>
where
    T: ConstZero,
{
    type Bulk = T;
    type Weight = ();

    fn bulk(&self) -> T {
        self.0.clone()
    }

    fn weight(&self) {}

    fn from_bulk(bulk: &T) -> Self {
        Scalar(bulk.clone(), PhantomData)
    }

    fn from_weight(_weight: &()) -> Self {
        Self::ZERO
    }

    fn from_bulk_and_weight(bulk: &T, _weight: &()) -> Self {
        Scalar(bulk.clone(), PhantomData)
    }

    fn proper_bulk(&self) -> Self {
        self.clone()
    }

    fn proper_weight(&self) -> Self {
        self.clone()
    }
}
