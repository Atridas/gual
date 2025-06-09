use std::ops::{Add, Mul, Neg, Sub};

use num::traits::ConstZero;

use crate::{Antiscalar, GeometricProduct, VectorSpace};

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
