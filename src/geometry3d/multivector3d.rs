use std::ops::{Add, Mul, Neg, Sub};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{Antiscalar, AntiwedgeProduct, KVector, Multivector, WedgeProduct};

use super::{Bivector3D, Multivector3D, Scalar3D, Trivector3D, Vector3D};

impl<T: Copy> Multivector for Multivector3D<T>
where
    T: Neg<Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    Trivector3D<T>: Antiscalar,
{
    type Scalar = Scalar3D<T>;
    type Vector = Vector3D<T>;
    type Antivector = Bivector3D<T>;
    type Antiscalar = Trivector3D<T>;

    fn scalar(&self) -> Self::Scalar {
        self.s
    }

    fn vector(&self) -> Self::Vector {
        self.v
    }

    fn antivector(&self) -> Self::Antivector {
        self.b
    }

    fn antiscalar(&self) -> Self::Antiscalar {
        self.a
    }

    fn right_complement(&self) -> Self {
        Multivector3D {
            s: self.a.right_complement(),
            v: self.b.right_complement(),
            b: self.v.right_complement(),
            a: self.s.right_complement(),
        }
    }

    fn left_complement(&self) -> Self {
        Multivector3D {
            s: self.a.left_complement(),
            v: self.b.left_complement(),
            b: self.v.left_complement(),
            a: self.s.left_complement(),
        }
    }
}

impl<T> Zero for Multivector3D<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Multivector3D {
            s: Scalar3D::zero(),
            v: Vector3D::zero(),
            b: Bivector3D::zero(),
            a: Trivector3D::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.s.is_zero() && self.v.is_zero() && self.b.is_zero() && self.a.is_zero()
    }
}

impl<T> ConstZero for Multivector3D<T>
where
    T: ConstZero,
{
    const ZERO: Self = Multivector3D {
        s: Scalar3D::ZERO,
        v: Vector3D::ZERO,
        b: Bivector3D::ZERO,
        a: Trivector3D::ZERO,
    };
}

impl<T> One for Multivector3D<T>
where
    T: Zero,
    T: One,
    Scalar3D<T>: Mul<Output = Scalar3D<T>>,
    Multivector3D<T>: Mul<Output = Multivector3D<T>>, // TODO!
{
    fn one() -> Self {
        Multivector3D {
            s: Scalar3D::one(),
            v: Vector3D::zero(),
            b: Bivector3D::zero(),
            a: Trivector3D::zero(),
        }
    }
}

impl<T> ConstOne for Multivector3D<T>
where
    T: ConstZero,
    T: ConstOne,
    Multivector3D<T>: Mul<Output = Multivector3D<T>>, // TODO!
{
    const ONE: Self = Multivector3D {
        s: Scalar3D::ONE,
        v: Vector3D::ZERO,
        b: Bivector3D::ZERO,
        a: Trivector3D::ZERO,
    };
}

impl<T> Multivector3D<T>
where
    T: ConstZero,
    T: ConstOne,
{
    pub const X: Self = Multivector3D {
        s: Scalar3D::ZERO,
        v: Vector3D::X,
        b: Bivector3D::ZERO,
        a: Trivector3D::ZERO,
    };

    pub const Y: Self = Multivector3D {
        s: Scalar3D::ZERO,
        v: Vector3D::Y,
        b: Bivector3D::ZERO,
        a: Trivector3D::ZERO,
    };

    pub const Z: Self = Multivector3D {
        s: Scalar3D::ZERO,
        v: Vector3D::Z,
        b: Bivector3D::ZERO,
        a: Trivector3D::ZERO,
    };

    pub const YZ: Self = Multivector3D {
        s: Scalar3D::ZERO,
        v: Vector3D::ZERO,
        b: Bivector3D::YZ,
        a: Trivector3D::ZERO,
    };

    pub const ZX: Self = Multivector3D {
        s: Scalar3D::ZERO,
        v: Vector3D::ZERO,
        b: Bivector3D::ZX,
        a: Trivector3D::ZERO,
    };

    pub const XY: Self = Multivector3D {
        s: Scalar3D::ZERO,
        v: Vector3D::ZERO,
        b: Bivector3D::XY,
        a: Trivector3D::ZERO,
    };

    pub const XYZ: Self = Multivector3D {
        s: Scalar3D::ZERO,
        v: Vector3D::ZERO,
        b: Bivector3D::ZERO,
        a: Trivector3D::XYZ,
    };
}

impl<T> Add for Multivector3D<T>
where
    T: Add<T, Output = T>,
{
    type Output = Multivector3D<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Multivector3D {
            s: self.s + rhs.s,
            v: self.v + rhs.v,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl<T> Sub for Multivector3D<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Multivector3D<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Multivector3D {
            s: self.s - rhs.s,
            v: self.v - rhs.v,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl<T> Neg for Multivector3D<T>
where
    T: Neg<Output = T>,
{
    type Output = Multivector3D<T>;
    fn neg(self) -> Self::Output {
        Multivector3D {
            s: -self.s,
            v: -self.v,
            a: -self.a,
            b: -self.b,
        }
    }
}

impl<T> WedgeProduct<Vector3D<T>> for Multivector3D<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Multivector3D<T>;

    fn wedge(self, rhs: Vector3D<T>) -> Self::Output {
        Multivector3D {
            s: Scalar3D::zero(),
            v: self.s.wedge(rhs),
            b: self.v.wedge(rhs),
            a: self.b.wedge(rhs),
        }
    }
}

impl<T> WedgeProduct<Bivector3D<T>> for Multivector3D<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector3D<T>;

    fn wedge(self, rhs: Bivector3D<T>) -> Self::Output {
        Multivector3D {
            s: Scalar3D::zero(),
            v: Vector3D::zero(),
            b: self.s.wedge(rhs),
            a: self.v.wedge(rhs),
        }
    }
}

impl<T> WedgeProduct<Trivector3D<T>> for Multivector3D<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector3D<T>;

    fn wedge(self, rhs: Trivector3D<T>) -> Self::Output {
        Multivector3D {
            s: Scalar3D::zero(),
            v: Vector3D::zero(),
            b: Bivector3D::zero(),
            a: self.s.wedge(rhs),
        }
    }
}

impl<T> WedgeProduct<Multivector3D<T>> for Multivector3D<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Multivector3D<T>;

    fn wedge(self, rhs: Multivector3D<T>) -> Self::Output {
        let s = self.s.wedge(rhs.s);
        let v1 = self.s.wedge(rhs.v);
        let b1 = self.s.wedge(rhs.b);
        let a1 = self.s.wedge(rhs.a);

        let v2 = self.v.wedge(rhs.s);
        let b2 = self.v.wedge(rhs.v);
        let a2 = self.v.wedge(rhs.b);

        let b3 = self.b.wedge(rhs.s);
        let a3 = self.b.wedge(rhs.v);

        let a4 = self.a.wedge(rhs.s);

        Multivector3D {
            s: s,
            v: v1 + v2,
            b: b1 + b2 + b3,
            a: a1 + a2 + a3 + a4,
        }
    }
}

impl<T> AntiwedgeProduct<Multivector3D<T>> for Multivector3D<T>
where
    Multivector3D<T>: Multivector,
    Multivector3D<T>: WedgeProduct<Multivector3D<T>, Output = Multivector3D<T>>,
{
    type Output = Multivector3D<T>;

    fn antiwedge(self, rhs: Multivector3D<T>) -> Self::Output {
        self.left_complement()
            .wedge(rhs.left_complement())
            .right_complement()
    }
}
