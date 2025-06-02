use std::{marker::PhantomData, ops::Neg};

use num::traits::ConstOne;

use crate::{Antiscalar, Complement, Scalar};

use super::{Bivector, Evenvector, Multivector, Vector};

impl<T, M> Complement for Scalar<2, T, M>
where
    T: Copy,
    T: Neg<Output = T>,
{
    type Output = Bivector<T, M>;

    fn right_complement(&self) -> Self::Output {
        Bivector {
            xy: self.0,
            _metric: PhantomData,
        }
    }

    fn left_complement(&self) -> Self::Output {
        Bivector {
            xy: self.0,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Complement for Vector<T, M>
where
    T: Copy,
    T: Neg<Output = T>,
{
    type Output = Vector<T, M>;

    fn right_complement(&self) -> Self::Output {
        Vector {
            x: -self.y,
            y: self.x,
            _metric: PhantomData,
        }
    }

    fn left_complement(&self) -> Self::Output {
        Vector {
            x: self.y,
            y: -self.x,
            _metric: PhantomData,
        }
    }
}

impl<T: Clone, M> Complement for Bivector<T, M> {
    type Output = T;

    fn right_complement(&self) -> Self::Output {
        self.xy.clone()
    }

    fn left_complement(&self) -> Self::Output {
        self.xy.clone()
    }
}

impl<T: Clone + ConstOne, M> Complement for Evenvector<T, M> {
    type Output = Evenvector<T, M>;

    fn right_complement(&self) -> Self::Output {
        Evenvector {
            s: self.b.right_complement(),
            b: Bivector::from_volume(self.s.clone()),
        }
    }

    fn left_complement(&self) -> Self::Output {
        Evenvector {
            s: self.b.left_complement(),
            b: Bivector::from_volume(self.s.clone()),
        }
    }
}

impl<T, M> Complement for Multivector<T, M>
where
    T: Copy,
    T: ConstOne,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;

    fn right_complement(&self) -> Self::Output {
        Multivector {
            s: self.b.right_complement(),
            v: self.v.right_complement(),
            b: Bivector::from_volume(self.s.clone()),
        }
    }

    fn left_complement(&self) -> Self::Output {
        Multivector {
            s: self.b.left_complement(),
            v: self.v.left_complement(),
            b: Bivector::from_volume(self.s.clone()),
        }
    }
}
