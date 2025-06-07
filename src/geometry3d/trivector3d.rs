use std::{
    marker::PhantomData,
    ops::{Mul, Neg},
};

use num::traits::ConstOne;

use crate::{Antiscalar, AntiwedgeProduct, GeometricProduct, KVector};

use super::{Bivector, Multivector, Scalar, Trivector, Vector};

impl<T: Clone> Antiscalar for Trivector<T>
where
    T: ConstOne,
{
    const UNIT_VOLUME: Self = Trivector {
        xyz: T::ONE,
        _metric: PhantomData,
    };

    type T = T;

    fn volume(&self) -> T {
        self.xyz.clone()
    }

    fn from_volume(volume: Self::T) -> Self {
        Trivector {
            xyz: volume,
            _metric: PhantomData,
        }
    }
}

impl<T: Clone> KVector for Trivector<T> {
    type AntiKVector = Scalar<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Scalar(self.xyz.clone())
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Scalar(self.xyz.clone())
    }
}

impl<T> AntiwedgeProduct<Trivector<T>> for Trivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Trivector<T>;

    fn antiwedge(&self, rhs: &Trivector<T>) -> Self::Output {
        Trivector {
            xyz: self.xyz * rhs.xyz,
            _metric: PhantomData,
        }
    }
}

impl<T> GeometricProduct<Vector<T>> for Trivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector<T>;

    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        rhs.geometric_product(self)
    }
}

impl<T> GeometricProduct<Bivector<T>> for Trivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Vector<T>;

    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        Vector {
            x: -self.xyz * rhs.yz,
            y: -self.xyz * rhs.zx,
            z: -self.xyz * rhs.xy,
            _metric: PhantomData,
        }
    }
}

impl<T> GeometricProduct<Trivector<T>> for Trivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Scalar<T>;

    fn geometric_product(&self, rhs: &Trivector<T>) -> Self::Output {
        -Scalar(self.xyz * rhs.xyz)
    }
}

impl<T> GeometricProduct<Multivector<T>> for Trivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, _rhs: &Multivector<T>) -> Self::Output {
        unimplemented!();
        // Multivector {
        //     s: self.geometric_product(&rhs.a),
        //     v: self.geometric_product(&rhs.b),
        //     b: self.geometric_product(&rhs.v),
        //     a: self.geometric_product(&rhs.s),
        // }
    }
}
