use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Float, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{Antiscalar, AntiwedgeProduct, KVector};

use super::{Quadvector, Scalar};

impl<T> Zero for Quadvector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Quadvector { xyzw: T::zero() }
    }

    fn is_zero(&self) -> bool {
        self.xyzw.is_zero()
    }
}

impl<T> ConstZero for Quadvector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Quadvector { xyzw: T::ZERO };
}

impl<T> Antiscalar for Quadvector<T>
where
    T: ConstOne,
    T: Float,
    Scalar<T>: Mul<Output = Scalar<T>>,
{
    const UNIT_VOLUME: Self = Quadvector { xyzw: T::ONE };

    fn sqrt(&self) -> Self {
        Quadvector {
            xyzw: self.xyzw.sqrt(),
        }
    }
}

impl<T> Quadvector<T>
where
    T: ConstOne,
{
    pub const XYZW: Self = Quadvector { xyzw: T::ONE };
}

impl<T> Add for Quadvector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Quadvector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Quadvector {
            xyzw: self.xyzw + rhs.xyzw,
        }
    }
}

impl<T> Sub for Quadvector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Quadvector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Quadvector {
            xyzw: self.xyzw - rhs.xyzw,
        }
    }
}

impl<T> Neg for Quadvector<T>
where
    T: Neg<Output = T>,
{
    type Output = Quadvector<T>;
    fn neg(self) -> Self::Output {
        Quadvector { xyzw: -self.xyzw }
    }
}

impl<T: Clone> KVector for Quadvector<T> {
    type AntiKVector = Scalar<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Scalar(self.xyzw.clone())
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Scalar(self.xyzw.clone())
    }
}

impl<T> AntiwedgeProduct<Quadvector<T>> for Quadvector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Quadvector<T>;

    fn antiwedge(&self, rhs: &Quadvector<T>) -> Self::Output {
        Quadvector {
            xyzw: self.xyzw * rhs.xyzw,
        }
    }
}
