use std::marker::PhantomData;

use num::Float;

use crate::{Dot, Norm};

use super::{Bivector, Vector};

impl<T> Norm for Vector<T>
where
    T: Float,
    Vector<T>: Dot<Scalar = T, Antiscalar = Bivector<T>>,
{
    type Scalar = T;
    type Antiscalar = Bivector<T>;

    fn bulk_norm_squared(&self) -> T {
        self.dot(self)
    }

    fn weight_norm_squared(&self) -> Bivector<T> {
        self.antidot(self)
    }

    fn bulk_norm(&self) -> T {
        self.bulk_norm_squared().sqrt()
    }

    fn weight_norm(&self) -> Bivector<T> {
        Bivector {
            xy: self.weight_norm_squared().xy.sqrt(),
            _metric: PhantomData,
        }
    }
}

impl<T> Norm for Bivector<T>
where
    T: Float,
{
    type Scalar = T;
    type Antiscalar = Bivector<T>;

    fn bulk_norm_squared(&self) -> T {
        self.xy * self.xy
    }

    fn weight_norm_squared(&self) -> Bivector<T> {
        Bivector {
            xy: self.xy * self.xy,
            _metric: PhantomData,
        }
    }

    fn bulk_norm(&self) -> T {
        self.xy.abs()
    }

    fn weight_norm(&self) -> Bivector<T> {
        Bivector {
            xy: self.xy.abs(),
            _metric: PhantomData,
        }
    }
}
