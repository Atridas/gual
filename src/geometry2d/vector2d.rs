use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{AntiwedgeProduct, KVector, WedgeProduct, reverse_antiwedge};

use super::{Bivector, Scalar, Vector};

impl<T> Zero for Vector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Vector {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T> ConstZero for Vector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Vector {
        x: T::ZERO,
        y: T::ZERO,
    };
}

impl<T> Vector<T>
where
    T: ConstZero,
    T: ConstOne,
{
    pub const X: Self = Vector {
        x: T::ONE,
        y: T::ZERO,
    };

    pub const Y: Self = Vector {
        x: T::ZERO,
        y: T::ONE,
    };
}

impl<T> Add for Vector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Vector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Neg for Vector<T>
where
    T: Neg<Output = T>,
{
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> KVector for Vector<T>
where
    T: Copy,
    T: Neg<Output = T>,
{
    type AntiKVector = Vector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Vector {
            x: -self.y,
            y: self.x,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Vector {
            x: self.y,
            y: -self.x,
        }
    }
}

impl<T> WedgeProduct<Vector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Bivector<T>;

    fn wedge(self, rhs: Vector<T>) -> Self::Output {
        Bivector {
            xy: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T> AntiwedgeProduct<Vector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Scalar<T>;

    fn antiwedge(self, rhs: Vector<T>) -> Self::Output {
        Scalar(self.x * rhs.y - self.y * rhs.x)
    }
}

impl<T> AntiwedgeProduct<Bivector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Vector<T>;

    fn antiwedge(self, rhs: Bivector<T>) -> Self::Output {
        Vector {
            x: self.x * rhs.xy,
            y: self.y * rhs.xy,
        }
    }
}

reverse_antiwedge!(Bivector, Vector);
