use std::{
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

use num::traits::ConstOne;

use crate::{
    Antiscalar, Complement, VectorSpace,
    geometry3d::{Bivector, Multivector, Vector},
};

use super::Trivector;

impl<T: Clone> Antiscalar for Trivector<T>
where
    T: ConstOne,
{
    const UNIT_VOLUME: Self = Trivector {
        xyz: T::ONE,
        _metric: PhantomData,
    };

    type T = T;

    fn volume(&self) -> T {
        self.xyz.clone()
    }

    fn from_volume(volume: Self::T) -> Self {
        Trivector {
            xyz: volume,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy> VectorSpace for Multivector<T>
where
    T: Neg<Output = T>,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    Trivector<T>: Antiscalar,
{
    type Scalar = T;
    type Vector = Vector<T>;
    type Antivector = Bivector<T>;
    type Antiscalar = Trivector<T>;

    fn scalar(&self) -> Self::Scalar {
        self.s
    }

    fn vector(&self) -> Self::Vector {
        self.v
    }

    fn antivector(&self) -> Self::Antivector {
        self.b
    }

    fn antiscalar(&self) -> Self::Antiscalar {
        self.t
    }

    fn right_complement(&self) -> Self {
        Multivector {
            s: self.t.right_complement(),
            v: self.b.right_complement(),
            b: self.v.right_complement(),
            t: Trivector::new(self.s),
        }
    }

    fn left_complement(&self) -> Self {
        Multivector {
            s: self.t.left_complement(),
            v: self.b.left_complement(),
            b: self.v.left_complement(),
            t: Trivector::new(self.s),
        }
    }
}
