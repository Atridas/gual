use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use num::Float;

use crate::{Dot, Epsilon, Unitizable};

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

impl<T> Unitizable for Vector<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type Output = Vector<T>;

    fn unitize(&self) -> Option<Self::Output> {
        let len2 = self.dot(self);
        if len2.is_near_zero() {
            None
        } else {
            Some(*self * len2.recip().sqrt())
        }
    }
}
