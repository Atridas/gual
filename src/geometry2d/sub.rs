use std::ops::{Add, Neg, Sub};

use num::traits::ConstZero;

use crate::default_sub;

use super::{Bivector, Evenvector, Multivector, Point, Vector};

impl<T, M> Sub<T> for Vector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: T) -> Self::Output {
        Multivector {
            s: -rhs,
            v: self,
            b: Bivector::ZERO,
        }
    }
}

default_sub!(Vector, Vector);
default_sub!(Vector, Bivector);
default_sub!(Vector, Evenvector);
default_sub!(Vector, Multivector);

impl<T> Sub<Vector<T>> for Point<T>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Point<T>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Point(self.0 - rhs)
    }
}

impl<T> Sub<Point<T>> for Point<T>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Vector<T>;
    fn sub(self, rhs: Point<T>) -> Self::Output {
        self.0 - rhs.0
    }
}

impl<T, M> Sub<T> for Bivector<T, M>
where
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn sub(self, rhs: T) -> Self::Output {
        Evenvector { s: -rhs, b: self }
    }
}

default_sub!(Bivector, Vector);
default_sub!(Bivector, Bivector);
default_sub!(Bivector, Evenvector);
default_sub!(Bivector, Multivector);

impl<T, M> Sub<T> for Evenvector<T, M>
where
    T: ConstZero,
    T: Sub<Output = T>,
{
    type Output = Evenvector<T, M>;
    fn sub(self, rhs: T) -> Self::Output {
        Evenvector {
            s: self.s - rhs,
            b: self.b,
        }
    }
}

default_sub!(Evenvector, Vector);
default_sub!(Evenvector, Bivector);
default_sub!(Evenvector, Evenvector);
default_sub!(Evenvector, Multivector);

impl<T, M> Sub<T> for Multivector<T, M>
where
    T: ConstZero,
    T: Sub<Output = T>,
{
    type Output = Multivector<T, M>;
    fn sub(self, rhs: T) -> Self::Output {
        Multivector {
            s: self.s - rhs,
            v: self.v,
            b: self.b,
        }
    }
}

default_sub!(Multivector, Vector);
default_sub!(Multivector, Bivector);
default_sub!(Multivector, Evenvector);
default_sub!(Multivector, Multivector);
