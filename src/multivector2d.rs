use std::ops::{Add, Mul, Neg, Sub};

use num::{One, Zero};

use crate::{Antiscalar, KVector, Multivector, WedgeProduct};

mod bivector2d;
mod scalar2d;
mod vector2d;

#[derive(Debug, Clone, Copy)]
pub struct Scalar2D<T>(pub T);

#[derive(Debug, Clone, Copy)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Bivector2D<T> {
    pub xy: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Multivector2D<T> {
    pub s: Scalar2D<T>,
    pub v: Vector2D<T>,
    pub a: Bivector2D<T>,
}

impl<T: Copy> Multivector for Multivector2D<T>
where
    T: Neg<Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    Bivector2D<T>: Antiscalar,
{
    type Scalar = Scalar2D<T>;
    type Vector = Vector2D<T>;
    type Antivector = Vector2D<T>;
    type Antiscalar = Bivector2D<T>;

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
        self.a
    }

    fn right_complement(&self) -> Self {
        Multivector2D {
            s: self.a.right_complement(),
            v: self.v.right_complement(),
            a: self.s.right_complement(),
        }
    }

    fn left_complement(&self) -> Self {
        Multivector2D {
            s: self.a.left_complement(),
            v: self.v.left_complement(),
            a: self.s.left_complement(),
        }
    }
}

impl<T> Zero for Multivector2D<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Multivector2D {
            s: Scalar2D::zero(),
            v: Vector2D::zero(),
            a: Bivector2D::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.s.is_zero() && self.v.is_zero() && self.a.is_zero()
    }
}

impl<T> One for Multivector2D<T>
where
    T: Zero,
    T: One,
    Scalar2D<T>: Mul<Output = Scalar2D<T>>,
    Multivector2D<T>: Mul<Output = Multivector2D<T>>, // TODO!
{
    fn one() -> Self {
        Multivector2D {
            s: Scalar2D::one(),
            v: Vector2D::zero(),
            a: Bivector2D::zero(),
        }
    }
}

impl<T> Add for Multivector2D<T>
where
    T: Add<T, Output = T>,
{
    type Output = Multivector2D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Multivector2D {
            s: self.s + rhs.s,
            v: self.v + rhs.v,
            a: self.a + rhs.a,
        }
    }
}

impl<T> Sub for Multivector2D<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Multivector2D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Multivector2D {
            s: self.s - rhs.s,
            v: self.v - rhs.v,
            a: self.a - rhs.a,
        }
    }
}

impl<T> Neg for Multivector2D<T>
where
    T: Neg<Output = T>,
{
    type Output = Multivector2D<T>;
    fn neg(self) -> Self::Output {
        Multivector2D {
            s: -self.s,
            v: -self.v,
            a: -self.a,
        }
    }
}

impl<T> WedgeProduct<Scalar2D<T>> for Multivector2D<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector2D<T>;

    fn wedge(self, rhs: Scalar2D<T>) -> Self::Output {
        Multivector2D {
            s: self.s.wedge(rhs),
            v: self.v.wedge(rhs),
            a: self.a.wedge(rhs),
        }
    }
}

impl<T> WedgeProduct<Vector2D<T>> for Multivector2D<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Multivector2D<T>;

    fn wedge(self, rhs: Vector2D<T>) -> Self::Output {
        Multivector2D {
            s: Scalar2D::zero(),
            v: self.s.wedge(rhs),
            a: self.v.wedge(rhs),
        }
    }
}

impl<T> WedgeProduct<Bivector2D<T>> for Multivector2D<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector2D<T>;

    fn wedge(self, rhs: Bivector2D<T>) -> Self::Output {
        Multivector2D {
            s: Scalar2D::zero(),
            v: Vector2D::zero(),
            a: self.s.wedge(rhs),
        }
    }
}

impl<T> WedgeProduct<Multivector2D<T>> for Multivector2D<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Multivector2D<T>;

    fn wedge(self, rhs: Multivector2D<T>) -> Self::Output {
        let s = self.s.wedge(rhs.s);
        let v1 = self.s.wedge(rhs.v);
        let a1 = self.s.wedge(rhs.a);
        let v2 = self.v.wedge(rhs.s);
        let a2 = self.v.wedge(rhs.v);
        let a3 = self.a.wedge(rhs.s);

        Multivector2D {
            s: s,
            v: v1 + v2,
            a: a1 + a2 + a3,
        }
    }
}
