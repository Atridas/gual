use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use num::traits::ConstZero;

use crate::{Dot, Projective};

use super::{Bivector, Trivector, Vector};

// ----------------------------------------------------------------------------------------------------
// Euclidean metric
// ----------------------------------------------------------------------------------------------------

impl<T> Dot for Vector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        Trivector {
            xyz: self.x * rhs.x + self.y * rhs.y + self.z * rhs.z,
            _metric: PhantomData,
        }
    }
}

impl<T> Dot for Bivector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.yz * rhs.yz + self.zx * rhs.zx + self.xy * rhs.xy
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        Trivector {
            xyz: self.yz * rhs.yz + self.zx * rhs.zx + self.xy * rhs.xy,
            _metric: PhantomData,
        }
    }
}

impl<T> Dot for Trivector<T>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.xyz * rhs.xyz
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        Trivector {
            xyz: self.xyz * rhs.xyz,
            _metric: PhantomData,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Projective metric
// ----------------------------------------------------------------------------------------------------

impl<T> Dot for Vector<T, Projective>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Scalar = T;
    type Antiscalar = Trivector<T, Projective>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.x * rhs.x + self.y * rhs.y
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        Trivector {
            xyz: self.z * rhs.z,
            _metric: PhantomData,
        }
    }
}

impl<T> Dot for Bivector<T, Projective>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Scalar = T;
    type Antiscalar = Trivector<T, Projective>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.xy * rhs.xy
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        Trivector {
            xyz: self.yz * rhs.yz + self.zx * rhs.zx,
            _metric: PhantomData,
        }
    }
}

impl<T> Dot for Trivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type Scalar = T;
    type Antiscalar = Trivector<T, Projective>;

    fn dot(&self, _rhs: &Self) -> Self::Scalar {
        T::ZERO
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        Trivector {
            xyz: self.xyz * rhs.xyz,
            _metric: PhantomData,
        }
    }
}
