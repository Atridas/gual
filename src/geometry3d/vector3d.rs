use std::{
    marker::PhantomData,
    ops::{Add, Mul, Sub},
};

use num::traits::ConstZero;

use crate::{GeometricProduct, WedgeProduct};

use super::{Bivector, Evenvector, Multivector, Trivector, Vector};

impl<T> GeometricProduct<Vector<T>> for Vector<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Evenvector<T>;

    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        Evenvector {
            s: self.x * rhs.x + self.y * rhs.y + self.z * rhs.z,
            b: self.wedge(rhs),
        }
    }
}

impl<T> GeometricProduct<Bivector<T>> for Vector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: Vector {
                x: self.z * rhs.zx - self.y * rhs.xy,
                y: self.x * rhs.xy - self.z * rhs.yz,
                z: self.y * rhs.yz - self.x * rhs.zx,
                _metric: PhantomData,
            },
            b: Bivector::ZERO,
            t: self.wedge(rhs),
        }
    }
}

impl<T> GeometricProduct<Trivector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector<T>;

    fn geometric_product(&self, rhs: &Trivector<T>) -> Self::Output {
        Bivector {
            yz: self.x * rhs.xyz,
            zx: self.y * rhs.xyz,
            xy: self.z * rhs.xyz,
            _metric: PhantomData,
        }
    }
}

impl<T> GeometricProduct<Multivector<T>> for Vector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, _rhs: &Multivector<T>) -> Self::Output {
        unimplemented!();
        // let v = self.geometric_product(&rhs.s);
        // let sb = self.geometric_product(&rhs.v);
        // let vt = self.geometric_product(&rhs.b);
        // let b = self.geometric_product(&rhs.a);

        // Multivector {
        //     s: sb.s,
        //     v: v + vt.v,
        //     b: sb.b + b,
        //     a: vt.a,
        // }
    }
}
