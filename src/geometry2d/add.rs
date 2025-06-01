use std::{marker::PhantomData, ops::Add};

use num::traits::ConstZero;

use crate::reverse_add_metric;

use super::{Bivector, Evenvector, Multivector, Vector};

// ----------------------------------------------------------------------------------------------------
// Vector
// ----------------------------------------------------------------------------------------------------

impl<T, M> Add<T> for Vector<T, M>
where
    T: ConstZero,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: T) -> Self::Output {
        Multivector {
            s: rhs,
            v: self,
            b: Bivector::ZERO,
        }
    }
}

impl<T, M> Add<Vector<T, M>> for Vector<T, M>
where
    T: Add<T, Output = T>,
{
    type Output = Vector<T, M>;
    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Add<Bivector<T, M>> for Vector<T, M>
where
    T: ConstZero,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Bivector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: self,
            b: rhs,
        }
    }
}

impl<T, M> Add<Evenvector<T, M>> for Vector<T, M> {
    type Output = Multivector<T, M>;
    fn add(self, rhs: Evenvector<T, M>) -> Self::Output {
        Multivector {
            s: rhs.s,
            v: self,
            b: rhs.b,
        }
    }
}

impl<T, M> Add<Multivector<T, M>> for Vector<T, M>
where
    T: Add<Output = T>,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Multivector<T, M>) -> Self::Output {
        Multivector {
            s: rhs.s,
            v: self + rhs.v,
            b: rhs.b,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Bivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> Add<T> for Bivector<T, M> {
    type Output = Evenvector<T, M>;
    fn add(self, rhs: T) -> Self::Output {
        Evenvector { s: rhs, b: self }
    }
}

reverse_add_metric!(Bivector, Vector);

impl<T, M> Add<Bivector<T, M>> for Bivector<T, M>
where
    T: Add<T, Output = T>,
{
    type Output = Bivector<T, M>;
    fn add(self, rhs: Self) -> Self::Output {
        Bivector {
            xy: self.xy + rhs.xy,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Add<Evenvector<T, M>> for Bivector<T, M>
where
    T: Add<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn add(self, rhs: Evenvector<T, M>) -> Self::Output {
        Evenvector {
            s: rhs.s,
            b: self + rhs.b,
        }
    }
}

impl<T, M> Add<Multivector<T, M>> for Bivector<T, M>
where
    T: Add<Output = T>,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Multivector<T, M>) -> Self::Output {
        Multivector {
            s: rhs.s,
            v: rhs.v,
            b: self + rhs.b,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Evenvector
// ----------------------------------------------------------------------------------------------------

impl<T, M> Add<T> for Evenvector<T, M>
where
    T: Add<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn add(self, rhs: T) -> Self::Output {
        Evenvector {
            s: self.s + rhs,
            b: self.b,
        }
    }
}

reverse_add_metric!(Evenvector, Vector);

reverse_add_metric!(Evenvector, Bivector);

impl<T, M> Add<Evenvector<T, M>> for Evenvector<T, M>
where
    T: Add<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn add(self, rhs: Evenvector<T, M>) -> Self::Output {
        Evenvector {
            s: self.s + rhs.s,
            b: self.b + rhs.b,
        }
    }
}

impl<T, M> Add<Multivector<T, M>> for Evenvector<T, M>
where
    T: Add<Output = T>,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Multivector<T, M>) -> Self::Output {
        Multivector {
            s: self.s + rhs.s,
            v: rhs.v,
            b: self.b + rhs.b,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Multivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> Add<T> for Multivector<T, M>
where
    T: Add<Output = T>,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: T) -> Self::Output {
        Multivector {
            s: self.s + rhs,
            v: self.v,
            b: self.b,
        }
    }
}

reverse_add_metric!(Multivector, Vector);

reverse_add_metric!(Multivector, Bivector);

reverse_add_metric!(Multivector, Evenvector);

impl<T, M> Add<Multivector<T, M>> for Multivector<T, M>
where
    T: Add<Output = T>,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Multivector<T, M>) -> Self::Output {
        Multivector {
            s: self.s + rhs.s,
            v: self.v + rhs.v,
            b: self.b + rhs.b,
        }
    }
}
