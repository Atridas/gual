use std::ops::{Add, Mul, Neg, Sub};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{Antiscalar, AntiwedgeProduct, KVector, VectorSpace, WedgeProduct};

use super::{Bivector, Multivector, Scalar, Vector};

impl<T: Copy> VectorSpace for Multivector<T>
where
    T: Neg<Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    Bivector<T>: Antiscalar,
{
    type Scalar = Scalar<T>;
    type Vector = Vector<T>;
    type Antivector = Vector<T>;
    type Antiscalar = Bivector<T>;

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
        Multivector {
            s: self.a.right_complement(),
            v: self.v.right_complement(),
            a: self.s.right_complement(),
        }
    }

    fn left_complement(&self) -> Self {
        Multivector {
            s: self.a.left_complement(),
            v: self.v.left_complement(),
            a: self.s.left_complement(),
        }
    }
}

impl<T> Zero for Multivector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Multivector {
            s: Scalar::zero(),
            v: Vector::zero(),
            a: Bivector::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.s.is_zero() && self.v.is_zero() && self.a.is_zero()
    }
}

impl<T> ConstZero for Multivector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        a: Bivector::ZERO,
    };
}

impl<T> One for Multivector<T>
where
    T: Zero,
    T: One,
    Scalar<T>: Mul<Output = Scalar<T>>,
    Multivector<T>: Mul<Output = Multivector<T>>, // TODO!
{
    fn one() -> Self {
        Multivector {
            s: Scalar::one(),
            v: Vector::zero(),
            a: Bivector::zero(),
        }
    }
}

impl<T> ConstOne for Multivector<T>
where
    T: ConstZero,
    T: ConstOne,
    Multivector<T>: Mul<Output = Multivector<T>>, // TODO!
{
    const ONE: Self = Multivector {
        s: Scalar::ONE,
        v: Vector::ZERO,
        a: Bivector::ZERO,
    };
}

impl<T> Multivector<T>
where
    T: ConstZero,
    T: ConstOne,
{
    pub const X: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::X,
        a: Bivector::ZERO,
    };

    pub const Y: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::Y,
        a: Bivector::ZERO,
    };

    pub const XY: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        a: Bivector::XY,
    };
}

impl<T> Add for Multivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Multivector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Multivector {
            s: self.s + rhs.s,
            v: self.v + rhs.v,
            a: self.a + rhs.a,
        }
    }
}

impl<T> Sub for Multivector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Multivector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Multivector {
            s: self.s - rhs.s,
            v: self.v - rhs.v,
            a: self.a - rhs.a,
        }
    }
}

impl<T> Neg for Multivector<T>
where
    T: Neg<Output = T>,
{
    type Output = Multivector<T>;
    fn neg(self) -> Self::Output {
        Multivector {
            s: -self.s,
            v: -self.v,
            a: -self.a,
        }
    }
}

impl<T> WedgeProduct<Vector<T>> for Multivector<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Multivector<T>;

    fn wedge(&self, rhs: Vector<T>) -> Self::Output {
        Multivector {
            s: Scalar::zero(),
            v: self.s.wedge(rhs),
            a: self.v.wedge(rhs),
        }
    }
}

impl<T> WedgeProduct<Bivector<T>> for Multivector<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn wedge(&self, rhs: Bivector<T>) -> Self::Output {
        Multivector {
            s: Scalar::zero(),
            v: Vector::zero(),
            a: self.s.wedge(rhs),
        }
    }
}

impl<T> WedgeProduct<Multivector<T>> for Multivector<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Multivector<T>;

    fn wedge(&self, rhs: Multivector<T>) -> Self::Output {
        let s = self.s.wedge(rhs.s);
        let v1 = self.s.wedge(rhs.v);
        let a1 = self.s.wedge(rhs.a);
        let v2 = self.v.wedge(rhs.s);
        let a2 = self.v.wedge(rhs.v);
        let a3 = self.a.wedge(rhs.s);

        Multivector {
            s: s,
            v: v1 + v2,
            a: a1 + a2 + a3,
        }
    }
}

impl<T> AntiwedgeProduct<Multivector<T>> for Multivector<T>
where
    Multivector<T>: VectorSpace,
    Multivector<T>: WedgeProduct<Multivector<T>, Output = Multivector<T>>,
{
    type Output = Multivector<T>;

    fn antiwedge(&self, rhs: Multivector<T>) -> Self::Output {
        self.left_complement()
            .wedge(rhs.left_complement())
            .right_complement()
    }
}
