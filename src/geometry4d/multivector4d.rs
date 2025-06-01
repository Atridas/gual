use std::ops::{Add, Mul, Neg, Sub};

use num::{
    One, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{Antiscalar, AntiwedgeProduct, KVector, VectorSpace, WedgeProduct, reverse_add};

use super::{Bivector, Evenvector, Multivector, Quadvector, Scalar, Trivector, Vector};

impl<T: Copy> VectorSpace for Multivector<T>
where
    T: Neg<Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    Quadvector<T>: Antiscalar,
{
    type Scalar = Scalar<T>;
    type Vector = Vector<T>;
    type Antivector = Trivector<T>;
    type Antiscalar = Quadvector<T>;

    fn scalar(&self) -> Self::Scalar {
        self.s
    }

    fn vector(&self) -> Self::Vector {
        self.v
    }

    fn antivector(&self) -> Self::Antivector {
        self.t
    }

    fn antiscalar(&self) -> Self::Antiscalar {
        self.a
    }

    fn right_complement(&self) -> Self {
        Multivector {
            s: self.a.right_complement(),
            v: self.t.right_complement(),
            b: self.b.right_complement(),
            t: self.v.right_complement(),
            a: self.s.right_complement(),
        }
    }

    fn left_complement(&self) -> Self {
        Multivector {
            s: self.a.left_complement(),
            v: self.t.left_complement(),
            b: self.b.left_complement(),
            t: self.v.left_complement(),
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
            b: Bivector::zero(),
            t: Trivector::zero(),
            a: Quadvector::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.s.is_zero()
            && self.v.is_zero()
            && self.b.is_zero()
            && self.t.is_zero()
            && self.a.is_zero()
    }
}

impl<T> ConstZero for Multivector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZERO,
        t: Trivector::ZERO,
        a: Quadvector::ZERO,
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
            b: Bivector::zero(),
            t: Trivector::zero(),
            a: Quadvector::zero(),
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
        b: Bivector::ZERO,
        t: Trivector::ZERO,
        a: Quadvector::ZERO,
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
        b: Bivector::ZERO,
        t: Trivector::ZERO,
        a: Quadvector::ZERO,
    };

    pub const Y: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::Y,
        b: Bivector::ZERO,
        t: Trivector::ZERO,
        a: Quadvector::ZERO,
    };

    pub const Z: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::Z,
        b: Bivector::ZERO,
        t: Trivector::ZERO,
        a: Quadvector::ZERO,
    };

    pub const W: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::W,
        b: Bivector::ZERO,
        t: Trivector::ZERO,
        a: Quadvector::ZERO,
    };

    pub const WX: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        b: Bivector::WX,
        t: Trivector::ZERO,
        a: Quadvector::ZERO,
    };

    pub const WY: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        b: Bivector::WY,
        t: Trivector::ZERO,
        a: Quadvector::ZERO,
    };

    pub const WZ: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        b: Bivector::WZ,
        t: Trivector::ZERO,
        a: Quadvector::ZERO,
    };

    pub const YZ: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        b: Bivector::YZ,
        t: Trivector::ZERO,
        a: Quadvector::ZERO,
    };

    pub const ZX: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZX,
        t: Trivector::ZERO,
        a: Quadvector::ZERO,
    };

    pub const XY: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        b: Bivector::XY,
        t: Trivector::ZERO,
        a: Quadvector::ZERO,
    };

    pub const WYZ: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZERO,
        t: Trivector::WYZ,
        a: Quadvector::ZERO,
    };

    pub const WZX: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZERO,
        t: Trivector::WZX,
        a: Quadvector::ZERO,
    };

    pub const WXY: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZERO,
        t: Trivector::WXY,
        a: Quadvector::ZERO,
    };

    pub const ZYX: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZERO,
        t: Trivector::ZYX,
        a: Quadvector::ZERO,
    };

    pub const XYZW: Self = Multivector {
        s: Scalar::ZERO,
        v: Vector::ZERO,
        b: Bivector::ZERO,
        t: Trivector::ZERO,
        a: Quadvector::XYZW,
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
            t: self.t + rhs.t,
            a: self.a + rhs.a,
        }
    }
}

impl<T> Add<Scalar<T>> for Multivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Multivector<T>;
    fn add(self, rhs: Scalar<T>) -> Self::Output {
        Multivector {
            s: self.s + rhs,
            v: self.v,
            b: self.b,
            t: self.t,
            a: self.a,
        }
    }
}

impl<T> Add<Vector<T>> for Multivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Multivector<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        Multivector {
            s: self.s,
            v: self.v + rhs,
            b: self.b,
            t: self.t,
            a: self.a,
        }
    }
}

impl<T> Add<Bivector<T>> for Multivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Multivector<T>;
    fn add(self, rhs: Bivector<T>) -> Self::Output {
        Multivector {
            s: self.s,
            v: self.v,
            b: self.b + rhs,
            t: self.t,
            a: self.a,
        }
    }
}

impl<T> Add<Trivector<T>> for Multivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Multivector<T>;
    fn add(self, rhs: Trivector<T>) -> Self::Output {
        Multivector {
            s: self.s,
            v: self.v,
            b: self.b,
            t: self.t + rhs,
            a: self.a,
        }
    }
}

impl<T> Add<Quadvector<T>> for Multivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Multivector<T>;
    fn add(self, rhs: Quadvector<T>) -> Self::Output {
        Multivector {
            s: self.s,
            v: self.v,
            b: self.b,
            t: self.t,
            a: self.a + rhs,
        }
    }
}

impl<T> Add<Evenvector<T>> for Multivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Multivector<T>;
    fn add(self, rhs: Evenvector<T>) -> Self::Output {
        Multivector {
            s: self.s + rhs.s,
            v: self.v,
            b: self.b + rhs.b,
            t: self.t,
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
            t: self.t - rhs.t,
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
            t: -self.t,
            b: -self.b,
        }
    }
}

impl<T> WedgeProduct<Vector<T>> for Multivector<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Multivector<T>;

    fn wedge(&self, rhs: &Vector<T>) -> Self::Output {
        Multivector {
            s: Scalar::zero(),
            v: self.s.wedge(rhs),
            b: self.v.wedge(rhs),
            t: self.b.wedge(rhs),
            a: self.t.wedge(rhs),
        }
    }
}

impl<T> WedgeProduct<Bivector<T>> for Multivector<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Multivector<T>;

    fn wedge(&self, rhs: &Bivector<T>) -> Self::Output {
        Multivector {
            s: Scalar::zero(),
            v: Vector::zero(),
            b: self.s.wedge(rhs),
            t: self.v.wedge(rhs),
            a: self.b.wedge(rhs),
        }
    }
}

impl<T> WedgeProduct<Trivector<T>> for Multivector<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Multivector<T>;

    fn wedge(&self, rhs: &Trivector<T>) -> Self::Output {
        Multivector {
            s: Scalar::zero(),
            v: Vector::zero(),
            b: Bivector::zero(),
            t: self.s.wedge(rhs),
            a: self.v.wedge(rhs),
        }
    }
}

impl<T> WedgeProduct<Quadvector<T>> for Multivector<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn wedge(&self, rhs: &Quadvector<T>) -> Self::Output {
        Multivector {
            s: Scalar::zero(),
            v: Vector::zero(),
            b: Bivector::zero(),
            t: Trivector::zero(),
            a: self.s.wedge(rhs),
        }
    }
}

impl<T> WedgeProduct<Multivector<T>> for Multivector<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Multivector<T>;

    fn wedge(&self, rhs: &Multivector<T>) -> Self::Output {
        let s = self.s.wedge(&rhs.s);
        let v1 = self.s.wedge(&rhs.v);
        let b1 = self.s.wedge(&rhs.b);
        let t1 = self.s.wedge(&rhs.t);
        let a1 = self.s.wedge(&rhs.a);

        let v2 = self.v.wedge(&rhs.s);
        let b2 = self.v.wedge(&rhs.v);
        let t2 = self.v.wedge(&rhs.b);
        let a2 = self.v.wedge(&rhs.t);

        let b3 = self.b.wedge(&rhs.s);
        let t3 = self.b.wedge(&rhs.v);
        let a3 = self.b.wedge(&rhs.b);

        let t4 = self.t.wedge(&rhs.s);
        let a4 = self.t.wedge(&rhs.v);

        let a5 = self.a.wedge(&rhs.s);

        Multivector {
            s: s,
            v: v1 + v2,
            b: b1 + b2 + b3,
            t: t1 + t2 + t3 + t4,
            a: a1 + a2 + a3 + a4 + a5,
        }
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

reverse_add!(Scalar, Multivector);
reverse_add!(Vector, Multivector);
reverse_add!(Bivector, Multivector);
reverse_add!(Trivector, Multivector);
reverse_add!(Quadvector, Multivector);
reverse_add!(Evenvector, Multivector);
