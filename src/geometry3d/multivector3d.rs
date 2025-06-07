use std::ops::{Add, Mul, Neg, Sub};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{Antiscalar, AntiwedgeProduct, GeometricProduct, KVector, VectorSpace, WedgeProduct};

use super::{Bivector, Multivector, Scalar, Trivector, Vector};

impl<T: Copy> VectorSpace for Multivector<T>
where
    T: Neg<Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    Trivector<T>: Antiscalar,
{
    type Scalar = T;
    type Vector = Vector<T>;
    type Antivector = Bivector<T>;
    type Antiscalar = Trivector<T>;

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
        unimplemented!();
        // Multivector {
        //     s: self.a.right_complement(),
        //     v: self.b.right_complement(),
        //     b: self.v.right_complement(),
        //     a: self.s.right_complement(),
        // }
    }

    fn left_complement(&self) -> Self {
        unimplemented!();
        // Multivector {
        //     s: self.a.left_complement(),
        //     v: self.b.left_complement(),
        //     b: self.v.left_complement(),
        //     a: self.s.left_complement(),
        // }
    }
}

impl<T> Zero for Multivector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Multivector {
            s: T::zero(),
            v: Vector::zero(),
            b: Bivector::zero(),
            a: Trivector::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.s.is_zero() && self.v.is_zero() && self.b.is_zero() && self.a.is_zero()
    }
}

impl<T> ConstZero for Multivector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Multivector {
        s: T::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZERO,
        a: Trivector::ZERO,
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
            s: T::one(),
            v: Vector::zero(),
            b: Bivector::zero(),
            a: Trivector::zero(),
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
        s: T::ONE,
        v: Vector::ZERO,
        b: Bivector::ZERO,
        a: Trivector::ZERO,
    };
}

impl<T> Multivector<T>
where
    T: ConstZero,
    T: ConstOne,
{
    pub const X: Self = Multivector {
        s: T::ZERO,
        v: Vector::X,
        b: Bivector::ZERO,
        a: Trivector::ZERO,
    };

    pub const Y: Self = Multivector {
        s: T::ZERO,
        v: Vector::Y,
        b: Bivector::ZERO,
        a: Trivector::ZERO,
    };

    pub const Z: Self = Multivector {
        s: T::ZERO,
        v: Vector::Z,
        b: Bivector::ZERO,
        a: Trivector::ZERO,
    };

    pub const YZ: Self = Multivector {
        s: T::ZERO,
        v: Vector::ZERO,
        b: Bivector::YZ,
        a: Trivector::ZERO,
    };

    pub const ZX: Self = Multivector {
        s: T::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZX,
        a: Trivector::ZERO,
    };

    pub const XY: Self = Multivector {
        s: T::ZERO,
        v: Vector::ZERO,
        b: Bivector::XY,
        a: Trivector::ZERO,
    };

    pub const XYZ: Self = Multivector {
        s: T::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZERO,
        a: Trivector::XYZ,
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
            b: self.b + rhs.b,
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
            b: self.b - rhs.b,
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
            b: -self.b,
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

    fn wedge(&self, rhs: &Vector<T>) -> Self::Output {
        unimplemented!();
        // Multivector {
        //     s: T::zero(),
        //     v: self.s.wedge(rhs),
        //     b: self.v.wedge(rhs),
        //     a: self.b.wedge(rhs),
        // }
    }
}

impl<T> WedgeProduct<Bivector<T>> for Multivector<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn wedge(&self, rhs: &Bivector<T>) -> Self::Output {
        unimplemented!();
        // Multivector {
        //     s: T::zero(),
        //     v: Vector::zero(),
        //     b: self.s.wedge(rhs),
        //     a: self.v.wedge(rhs),
        // }
    }
}

impl<T> WedgeProduct<Trivector<T>> for Multivector<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn wedge(&self, rhs: &Trivector<T>) -> Self::Output {
        unimplemented!();
        // Multivector {
        //     s: T::zero(),
        //     v: Vector::zero(),
        //     b: Bivector::zero(),
        //     a: self.s.wedge(rhs),
        // }
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

    fn wedge(&self, rhs: &Multivector<T>) -> Self::Output {
        unimplemented!();
        // let s = self.s.wedge(&rhs.s);
        // let v1 = self.s.wedge(&rhs.v);
        // let b1 = self.s.wedge(&rhs.b);
        // let a1 = self.s.wedge(&rhs.a);

        // let v2 = self.v.wedge(&rhs.s);
        // let b2 = self.v.wedge(&rhs.v);
        // let a2 = self.v.wedge(&rhs.b);

        // let b3 = self.b.wedge(&rhs.s);
        // let a3 = self.b.wedge(&rhs.v);

        // let a4 = self.a.wedge(&rhs.s);

        // Multivector {
        //     s: s,
        //     v: v1 + v2,
        //     b: b1 + b2 + b3,
        //     a: a1 + a2 + a3 + a4,
        // }
    }
}

impl<T> AntiwedgeProduct<Multivector<T>> for Multivector<T>
where
    Multivector<T>: VectorSpace,
    Multivector<T>: WedgeProduct<Multivector<T>, Output = Multivector<T>>,
{
    type Output = Multivector<T>;

    fn antiwedge(&self, rhs: &Multivector<T>) -> Self::Output {
        self.left_complement()
            .wedge(&rhs.left_complement())
            .right_complement()
    }
}

impl<T> GeometricProduct<Vector<T>> for Multivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        unimplemented!();
        // let v = self.s.geometric_product(rhs);
        // let sb = self.v.geometric_product(rhs);
        // let vt = self.b.geometric_product(rhs);
        // let b = self.a.geometric_product(rhs);

        // Multivector {
        //     s: sb.s,
        //     v: v + vt.v,
        //     b: sb.b + b,
        //     a: vt.a,
        // }
    }
}

impl<T> GeometricProduct<Bivector<T>> for Multivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        unimplemented!();
        // let b = self.s.geometric_product(rhs);
        // let vt = self.v.geometric_product(rhs);
        // let sb = self.b.geometric_product(rhs);
        // let v = self.a.geometric_product(rhs);

        // Multivector {
        //     s: sb.s,
        //     v: v + vt.v,
        //     b: sb.b + b,
        //     a: vt.a,
        // }
    }
}

impl<T> GeometricProduct<Trivector<T>> for Multivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Trivector<T>) -> Self::Output {
        unimplemented!();
        // Multivector {
        //     s: self.a.geometric_product(rhs),
        //     v: self.b.geometric_product(rhs),
        //     b: self.v.geometric_product(rhs),
        //     a: self.s.geometric_product(rhs),
        // }
    }
}

impl<T> GeometricProduct<Multivector<T>> for Multivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Multivector<T>) -> Self::Output {
        unimplemented!();
        // self.geometric_product(&rhs.s)
        //     + self.geometric_product(&rhs.v)
        //     + self.geometric_product(&rhs.b)
        //     + self.geometric_product(&rhs.a)
    }
}
