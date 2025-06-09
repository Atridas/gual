use std::{
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

use num::traits::ConstZero;

use crate::{GeometricProduct, WedgeProduct};

use super::{Bivector, Evenvector, Multivector, Trivector, Vector};

impl<T> GeometricProduct<Vector<T>> for Bivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        Multivector {
            s: T::ZERO,
            v: Vector {
                x: self.xy * rhs.y - self.zx * rhs.z,
                y: self.yz * rhs.z - self.xy * rhs.x,
                z: self.zx * rhs.x - self.yz * rhs.y,
                _metric: PhantomData,
            },
            b: Bivector::ZERO,
            t: rhs.wedge(self),
        }
    }
}

impl<T> GeometricProduct<Bivector<T>> for Bivector<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Evenvector<T>;

    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        Evenvector {
            s: -(self.yz * rhs.yz + self.zx * rhs.zx + self.xy * rhs.xy),
            b: Bivector {
                yz: self.xy * rhs.zx - self.zx * rhs.xy,
                zx: self.yz * rhs.xy - self.xy * rhs.yz,
                xy: self.zx * rhs.yz - self.yz * rhs.zx,
                _metric: PhantomData,
            },
        }
    }
}

impl<T> GeometricProduct<Trivector<T>> for Bivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Vector<T>;

    fn geometric_product(&self, rhs: &Trivector<T>) -> Self::Output {
        Vector {
            x: self.yz * -rhs.xyz,
            y: self.zx * -rhs.xyz,
            z: self.xy * -rhs.xyz,
            _metric: PhantomData,
        }
    }
}

// impl<T> GeometricProduct<Multivector<T>> for Bivector<T>
// where
//     T: Copy,
//     T: ConstZero,
//     T: Add<T, Output = T>,
//     T: Sub<T, Output = T>,
//     T: Neg<Output = T>,
//     T: Mul<T, Output = T>,
// {
//     type Output = Multivector<T>;

//     fn geometric_product(&self, rhs: &Multivector<T>) -> Self::Output {
//         let b = self.geometric_product(&rhs.s);
//         let vt = self.geometric_product(&rhs.v);
//         let sb = self.geometric_product(&rhs.b);
//         let v = self.geometric_product(&rhs.a);

//         Multivector {
//             s: sb.s,
//             v: v + vt.v,
//             b: sb.b + b,
//             a: vt.a,
//         }
//     }
// }
