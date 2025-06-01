use std::{marker::PhantomData, ops::Neg};

use super::{Bivector, Evenvector, Multivector, Vector};

impl<T, M> Neg for Vector<T, M>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Neg for Bivector<T, M>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Bivector {
            xy: -self.xy,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Neg for Evenvector<T, M>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Evenvector {
            s: -self.s,
            b: -self.b,
        }
    }
}

impl<T, M> Neg for Multivector<T, M>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Multivector {
            s: -self.s,
            v: -self.v,
            b: -self.b,
        }
    }
}
