use std::ops::{Add, Mul, Neg, Sub};

use num::{Zero, traits::ConstZero};

use crate::{AntiwedgeProduct, KVector, WedgeProduct, reverse_antiwedge};

use super::{Bivector2D, Scalar2D, Vector2D};

impl<T> Zero for Vector2D<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Vector2D {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T> ConstZero for Vector2D<T>
where
    T: ConstZero,
{
    const ZERO: Self = Vector2D {
        x: T::ZERO,
        y: T::ZERO,
    };
}

impl<T> Add for Vector2D<T>
where
    T: Add<T, Output = T>,
{
    type Output = Vector2D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vector2D<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector2D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Neg for Vector2D<T>
where
    T: Neg<Output = T>,
{
    type Output = Vector2D<T>;
    fn neg(self) -> Self::Output {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> KVector for Vector2D<T>
where
    T: Copy,
    T: Neg<Output = T>,
{
    type AntiKVector = Vector2D<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Vector2D {
            x: -self.y,
            y: self.x,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Vector2D {
            x: self.y,
            y: -self.x,
        }
    }
}

impl<T> WedgeProduct<Vector2D<T>> for Vector2D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Bivector2D<T>;

    fn wedge(self, rhs: Vector2D<T>) -> Self::Output {
        Bivector2D {
            xy: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T> AntiwedgeProduct<Vector2D<T>> for Vector2D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Scalar2D<T>;

    fn antiwedge(self, rhs: Vector2D<T>) -> Self::Output {
        Scalar2D(self.x * rhs.y - self.y * rhs.x)
    }
}

impl<T> AntiwedgeProduct<Bivector2D<T>> for Vector2D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Vector2D<T>;

    fn antiwedge(self, rhs: Bivector2D<T>) -> Self::Output {
        Vector2D {
            x: self.x * rhs.xy,
            y: self.y * rhs.xy,
        }
    }
}

reverse_antiwedge!(Bivector2D, Vector2D);
