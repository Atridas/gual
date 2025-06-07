use std::ops::{Add, Mul, Neg, Sub};

use num::{Zero, traits::ConstZero};

use crate::{Antiscalar, AntiwedgeProduct, GeometricProduct, VectorSpace, WedgeProduct};

use super::{Bivector, Multivector, Trivector, Vector};

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
        self.t
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

impl<T> WedgeProduct<Vector<T>> for Multivector<T>
where
    T: Zero,
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Multivector<T>;

    fn wedge(&self, _rhs: &Vector<T>) -> Self::Output {
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

    fn wedge(&self, _rhs: &Bivector<T>) -> Self::Output {
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

    fn wedge(&self, _rhs: &Trivector<T>) -> Self::Output {
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

    fn wedge(&self, _rhs: &Multivector<T>) -> Self::Output {
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

    fn geometric_product(&self, _rhs: &Vector<T>) -> Self::Output {
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

    fn geometric_product(&self, _rhs: &Bivector<T>) -> Self::Output {
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

    fn geometric_product(&self, _rhs: &Trivector<T>) -> Self::Output {
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

    fn geometric_product(&self, _rhs: &Multivector<T>) -> Self::Output {
        unimplemented!();
        // self.geometric_product(&rhs.s)
        //     + self.geometric_product(&rhs.v)
        //     + self.geometric_product(&rhs.b)
        //     + self.geometric_product(&rhs.a)
    }
}
