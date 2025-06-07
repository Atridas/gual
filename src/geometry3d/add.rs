use std::{marker::PhantomData, ops::Add};

use num::traits::ConstZero;

use crate::{Scalar, reverse_add_metric};

use super::{Bivector, Evenvector, Multivector, Point, Trivector, Vector};

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
            t: Trivector::ZERO,
        }
    }
}

impl<T, M> Add<Scalar<3, T, M>> for Vector<T, M>
where
    T: ConstZero,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Multivector {
            s: rhs.0,
            v: self,
            b: Bivector::ZERO,
            t: Trivector::ZERO,
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
            z: self.z + rhs.z,
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
            t: Trivector::ZERO,
        }
    }
}

impl<T, M> Add<Trivector<T, M>> for Vector<T, M>
where
    T: ConstZero,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Trivector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: self,
            b: Bivector::ZERO,
            t: rhs,
        }
    }
}

impl<T, M> Add<Evenvector<T, M>> for Vector<T, M>
where
    T: ConstZero,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Evenvector<T, M>) -> Self::Output {
        Multivector {
            s: rhs.s,
            v: self,
            b: rhs.b,
            t: Trivector::ZERO,
        }
    }
}

impl<T, M> Add<Multivector<T, M>> for Vector<T, M>
where
    T: ConstZero,
    T: Add<Output = T>,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Multivector<T, M>) -> Self::Output {
        Multivector {
            s: rhs.s,
            v: self + rhs.v,
            b: rhs.b,
            t: Trivector::ZERO,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Point
// ----------------------------------------------------------------------------------------------------

impl<T> Add<Vector<T>> for Point<T>
where
    T: Add<T, Output = T>,
{
    type Output = Point<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        Point(self.0 + rhs)
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

impl<T, M> Add<Scalar<3, T, M>> for Bivector<T, M> {
    type Output = Evenvector<T, M>;
    fn add(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Evenvector { s: rhs.0, b: self }
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
            yz: self.yz + rhs.yz,
            zx: self.zx + rhs.zx,
            xy: self.xy + rhs.xy,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Add<Trivector<T, M>> for Bivector<T, M>
where
    T: ConstZero,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Trivector<T, M>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: Vector::ZERO,
            b: self,
            t: rhs,
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
    T: ConstZero,
    T: Add<Output = T>,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Multivector<T, M>) -> Self::Output {
        Multivector {
            s: rhs.s,
            v: rhs.v,
            b: self + rhs.b,
            t: rhs.t,
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// Trivector
// ----------------------------------------------------------------------------------------------------

impl<T, M> Add<T> for Trivector<T, M>
where
    T: ConstZero,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: T) -> Self::Output {
        Multivector {
            s: rhs,
            v: Vector::ZERO,
            b: Bivector::ZERO,
            t: self,
        }
    }
}

impl<T, M> Add<Scalar<3, T, M>> for Trivector<T, M>
where
    T: ConstZero,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Multivector {
            s: rhs.0,
            v: Vector::ZERO,
            b: Bivector::ZERO,
            t: self,
        }
    }
}

reverse_add_metric!(Trivector, Vector);
reverse_add_metric!(Trivector, Bivector);

impl<T, M> Add<Trivector<T, M>> for Trivector<T, M>
where
    T: Add<T, Output = T>,
{
    type Output = Trivector<T, M>;
    fn add(self, rhs: Self) -> Self::Output {
        Trivector {
            xyz: self.xyz + rhs.xyz,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Add<Evenvector<T, M>> for Trivector<T, M>
where
    T: ConstZero,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Evenvector<T, M>) -> Self::Output {
        Multivector {
            s: rhs.s,
            v: Vector::ZERO,
            b: rhs.b,
            t: self,
        }
    }
}

impl<T, M> Add<Multivector<T, M>> for Trivector<T, M>
where
    T: ConstZero,
    T: Add<Output = T>,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Multivector<T, M>) -> Self::Output {
        Multivector {
            s: rhs.s,
            v: rhs.v,
            b: rhs.b,
            t: self + rhs.t,
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

impl<T, M> Add<Scalar<3, T, M>> for Evenvector<T, M>
where
    T: Add<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn add(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Evenvector {
            s: self.s + rhs.0,
            b: self.b,
        }
    }
}

reverse_add_metric!(Evenvector, Vector);

reverse_add_metric!(Evenvector, Bivector);

reverse_add_metric!(Evenvector, Trivector);

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
            t: rhs.t,
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
            t: self.t,
        }
    }
}

impl<T, M> Add<Scalar<3, T, M>> for Multivector<T, M>
where
    T: Add<Output = T>,
{
    type Output = Multivector<T, M>;
    fn add(self, rhs: Scalar<3, T, M>) -> Self::Output {
        Multivector {
            s: self.s + rhs.0,
            v: self.v,
            b: self.b,
            t: self.t,
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
            t: self.t + rhs.t,
        }
    }
}
