use std::marker::PhantomData;

use num::{
    Float,
    traits::{ConstOne, ConstZero},
};

use crate::{Dot, Norm, Projective};

use super::{Bivector, Trivector, UnitVector, Vector};

// ----------------------------------------------------------------------------------------------------
// Both metrics
// ----------------------------------------------------------------------------------------------------

impl<T, M> Norm for Vector<T, M>
where
    T: Float,
    T: ConstOne,
    Vector<T, M>: Dot<Scalar = T, Antiscalar = Trivector<T, M>>,
{
    type Scalar = T;
    type Antiscalar = Trivector<T, M>;

    fn bulk_norm_squared(&self) -> T {
        self.dot(self)
    }

    fn weight_norm_squared(&self) -> Trivector<T, M> {
        self.antidot(self)
    }

    fn bulk_norm(&self) -> T {
        self.bulk_norm_squared().sqrt()
    }

    fn weight_norm(&self) -> Trivector<T, M> {
        Trivector {
            xyz: self.weight_norm_squared().xyz.sqrt(),
            _metric: PhantomData,
        }
    }
}

impl<T, M> Norm for Bivector<T, M>
where
    T: Float,
    T: ConstOne,
    Bivector<T, M>: Dot<Scalar = T, Antiscalar = Trivector<T, M>>,
{
    type Scalar = T;
    type Antiscalar = Trivector<T, M>;

    fn bulk_norm_squared(&self) -> T {
        self.dot(self)
    }

    fn weight_norm_squared(&self) -> Trivector<T, M> {
        self.antidot(self)
    }

    fn bulk_norm(&self) -> T {
        self.bulk_norm_squared().sqrt()
    }

    fn weight_norm(&self) -> Trivector<T, M> {
        Trivector {
            xyz: self.weight_norm_squared().xyz.sqrt(),
            _metric: PhantomData,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Euclidean metric
// ----------------------------------------------------------------------------------------------------

impl<T> Norm for UnitVector<T>
where
    T: Clone,
    T: ConstOne,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn bulk_norm_squared(&self) -> T {
        T::ONE
    }

    fn weight_norm_squared(&self) -> Trivector<T> {
        Trivector {
            xyz: T::ONE,
            _metric: PhantomData,
        }
    }

    fn bulk_norm(&self) -> T {
        T::ONE
    }

    fn weight_norm(&self) -> Trivector<T> {
        Trivector {
            xyz: T::ONE,
            _metric: PhantomData,
        }
    }
}

impl<T> Norm for Trivector<T>
where
    T: Float,
    T: ConstOne,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn bulk_norm_squared(&self) -> T {
        self.xyz * self.xyz
    }

    fn weight_norm_squared(&self) -> Trivector<T> {
        Trivector {
            xyz: self.xyz * self.xyz,
            _metric: PhantomData,
        }
    }

    fn bulk_norm(&self) -> T {
        self.xyz.abs()
    }

    fn weight_norm(&self) -> Trivector<T> {
        Trivector {
            xyz: self.xyz.abs(),
            _metric: PhantomData,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Projective metric
// ----------------------------------------------------------------------------------------------------

impl<T> Norm for Trivector<T, Projective>
where
    T: ConstZero,
    T: ConstOne,
    T: Float,
{
    type Scalar = T;
    type Antiscalar = Trivector<T, Projective>;

    fn bulk_norm_squared(&self) -> T {
        T::ZERO
    }

    fn weight_norm_squared(&self) -> Trivector<T, Projective> {
        Trivector {
            xyz: self.xyz * self.xyz,
            _metric: PhantomData,
        }
    }

    fn bulk_norm(&self) -> T {
        T::ZERO
    }

    fn weight_norm(&self) -> Trivector<T, Projective> {
        Trivector {
            xyz: self.xyz.abs(),
            _metric: PhantomData,
        }
    }
}
