use std::{
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

use num::traits::ConstZero;

use crate::{
    AntiwedgeProduct, GeometricProduct, KVector, WedgeProduct, reverse_antiwedge, reverse_wedge,
};

use super::{Bivector, Evenvector, Multivector, Scalar, Trivector, Vector};

impl<T> Sub for Vector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            _metric: PhantomData,
        }
    }
}

impl<T> Neg for Vector<T>
where
    T: Neg<Output = T>,
{
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            _metric: PhantomData,
        }
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            _metric: PhantomData,
        }
    }
}

impl<T> KVector for Vector<T>
where
    T: Copy,
{
    type AntiKVector = Bivector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Bivector {
            yz: self.x,
            zx: self.y,
            xy: self.z,
            _metric: PhantomData,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Bivector {
            yz: self.x,
            zx: self.y,
            xy: self.z,
            _metric: PhantomData,
        }
    }
}

impl<T> WedgeProduct<Vector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Bivector<T>;

    fn wedge(&self, rhs: &Vector<T>) -> Self::Output {
        Bivector {
            yz: self.y * rhs.z - self.z * rhs.y,
            zx: self.z * rhs.x - self.x * rhs.z,
            xy: self.x * rhs.y - self.y * rhs.x,
            _metric: PhantomData,
        }
    }
}

impl<T> WedgeProduct<Bivector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
{
    type Output = Trivector<T>;

    fn wedge(&self, rhs: &Bivector<T>) -> Self::Output {
        Trivector {
            xyz: self.x * rhs.yz + self.y * rhs.zx + self.z * rhs.xy,
            _metric: PhantomData,
        }
    }
}

impl<T> AntiwedgeProduct<Bivector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
{
    type Output = Scalar<T>;

    fn antiwedge(&self, rhs: &Bivector<T>) -> Self::Output {
        Scalar(self.x * rhs.yz + self.y * rhs.zx + self.z * rhs.xy)
    }
}

impl<T> AntiwedgeProduct<Trivector<T>> for Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Vector<T>;

    fn antiwedge(&self, rhs: &Trivector<T>) -> Self::Output {
        Vector {
            x: self.x * rhs.xyz,
            y: self.y * rhs.xyz,
            z: self.z * rhs.xyz,
            _metric: PhantomData,
        }
    }
}

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

reverse_wedge!(Bivector, Vector);

reverse_antiwedge!(Bivector, Vector);
reverse_antiwedge!(Trivector, Vector);
