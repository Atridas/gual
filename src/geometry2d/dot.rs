use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::Dot;

use super::{Bivector, Vector};

impl<T> Dot for Vector<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Scalar = T;
    type Antiscalar = Bivector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.x * rhs.x + self.y * rhs.y
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        Bivector {
            xy: self.x * rhs.x + self.y * rhs.y,
            _metric: PhantomData,
        }
    }
}

impl<T> Dot for Bivector<T>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type Scalar = T;
    type Antiscalar = Bivector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.xy * rhs.xy
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        Bivector {
            xy: self.xy * rhs.xy,
            _metric: PhantomData,
        }
    }
}
