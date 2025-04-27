use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{Antiscalar, AntiwedgeProduct, KVector};

use super::{Bivector, Scalar};

impl<T> Zero for Bivector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Bivector { xy: T::zero() }
    }

    fn is_zero(&self) -> bool {
        self.xy.is_zero()
    }
}

impl<T> ConstZero for Bivector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Bivector { xy: T::ZERO };
}

impl<T> Antiscalar for Bivector<T>
where
    T: ConstOne,
    Scalar<T>: Mul<Output = Scalar<T>>,
{
    const UNIT_VOLUME: Self = Bivector { xy: T::ONE };
}

impl<T> Bivector<T>
where
    T: ConstOne,
{
    pub const XY: Self = Bivector { xy: T::ONE };
}

impl<T> Add for Bivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Bivector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Bivector {
            xy: self.xy + rhs.xy,
        }
    }
}

impl<T> Sub for Bivector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Bivector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Bivector {
            xy: self.xy - rhs.xy,
        }
    }
}

impl<T> Neg for Bivector<T>
where
    T: Neg<Output = T>,
{
    type Output = Bivector<T>;
    fn neg(self) -> Self::Output {
        Bivector { xy: -self.xy }
    }
}

impl<T: Clone> KVector for Bivector<T> {
    type AntiKVector = Scalar<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Scalar(self.xy.clone())
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Scalar(self.xy.clone())
    }
}

impl<T> AntiwedgeProduct<Bivector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector<T>;

    fn antiwedge(&self, rhs: Bivector<T>) -> Self::Output {
        Bivector {
            xy: self.xy * rhs.xy,
        }
    }
}
