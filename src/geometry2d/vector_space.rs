use std::{
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

use num::{Float, traits::ConstOne};

use crate::{Antiscalar, Complement, VectorSpace};

use super::{Bivector, Multivector, Vector};

impl<T: Clone, M> Antiscalar for Bivector<T, M>
where
    T: ConstOne,
{
    const UNIT_VOLUME: Self = Bivector {
        xy: T::ONE,
        _metric: PhantomData,
    };

    type T = T;

    fn volume(&self) -> T {
        self.xy.clone()
    }

    fn from_volume(volume: Self::T) -> Self {
        Bivector {
            xy: volume,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy, M> VectorSpace for Multivector<T, M>
where
    T: Neg<Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Float,
    Bivector<T, M>: Antiscalar,
{
    type Scalar = T;
    type Vector = Vector<T, M>;
    type Antivector = Vector<T, M>;
    type Antiscalar = Bivector<T, M>;

    fn scalar(&self) -> Self::Scalar {
        self.s
    }

    fn vector(&self) -> Self::Vector {
        self.v
    }

    fn antivector(&self) -> Self::Antivector {
        self.v
    }

    fn antiscalar(&self) -> Self::Antiscalar {
        self.b
    }

    fn right_complement(&self) -> Self {
        Multivector {
            s: self.b.right_complement(),
            v: self.v.right_complement(),
            b: Bivector::new(self.s),
        }
    }

    fn left_complement(&self) -> Self {
        Multivector {
            s: self.b.left_complement(),
            v: self.v.left_complement(),
            b: Bivector::new(self.s),
        }
    }
}
