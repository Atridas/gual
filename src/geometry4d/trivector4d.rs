use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{
    AntiwedgeProduct, GeometricProduct, KVector, WedgeProduct, reverse_add, reverse_antiwedge,
};

use super::{Bivector, Evenvector, Multivector, Quadvector, Scalar, Trivector, Vector};

impl<T> Zero for Trivector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Trivector {
            wyz: T::zero(),
            wzx: T::zero(),
            wxy: T::zero(),
            zyx: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.wyz.is_zero() && self.wzx.is_zero() && self.wxy.is_zero() && self.zyx.is_zero()
    }
}

impl<T> ConstZero for Trivector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Trivector {
        wyz: T::ZERO,
        wzx: T::ZERO,
        wxy: T::ZERO,
        zyx: T::ZERO,
    };
}

impl<T> Trivector<T>
where
    T: ConstZero,
    T: ConstOne,
{
    pub const WYZ: Self = Trivector {
        wyz: T::ONE,
        wzx: T::ZERO,
        wxy: T::ZERO,
        zyx: T::ZERO,
    };

    pub const WZX: Self = Trivector {
        wyz: T::ZERO,
        wzx: T::ONE,
        wxy: T::ZERO,
        zyx: T::ZERO,
    };

    pub const WXY: Self = Trivector {
        wyz: T::ZERO,
        wzx: T::ZERO,
        wxy: T::ONE,
        zyx: T::ZERO,
    };

    pub const ZYX: Self = Trivector {
        wyz: T::ZERO,
        wzx: T::ZERO,
        wxy: T::ZERO,
        zyx: T::ONE,
    };
}

impl<T> Add for Trivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Trivector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Trivector {
            wyz: self.wyz + rhs.wyz,
            wzx: self.wzx + rhs.wzx,
            wxy: self.wxy + rhs.wxy,
            zyx: self.zyx + rhs.zyx,
        }
    }
}

impl<T> Add<Quadvector<T>> for Trivector<T>
where
    T: ConstZero,
{
    type Output = Multivector<T>;
    fn add(self, rhs: Quadvector<T>) -> Self::Output {
        Multivector {
            s: Scalar::ZERO,
            v: Vector::ZERO,
            b: Bivector::ZERO,
            t: self,
            a: rhs,
        }
    }
}

impl<T> Sub for Trivector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Trivector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Trivector {
            wyz: self.wyz - rhs.wyz,
            wzx: self.wzx - rhs.wzx,
            wxy: self.wxy - rhs.wxy,
            zyx: self.zyx - rhs.zyx,
        }
    }
}

impl<T> Neg for Trivector<T>
where
    T: Neg<Output = T>,
{
    type Output = Trivector<T>;
    fn neg(self) -> Self::Output {
        Trivector {
            wyz: -self.wyz,
            wzx: -self.wzx,
            wxy: -self.wxy,
            zyx: -self.zyx,
        }
    }
}

impl<T> KVector for Trivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
{
    type AntiKVector = Vector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Vector {
            x: -self.wyz,
            y: -self.wzx,
            z: -self.wxy,
            w: -self.zyx,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Vector {
            x: self.wyz,
            y: self.wzx,
            z: self.wxy,
            w: self.zyx,
        }
    }
}

impl<T> AntiwedgeProduct<Quadvector<T>> for Trivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Trivector<T>;

    fn antiwedge(&self, rhs: &Quadvector<T>) -> Self::Output {
        Trivector {
            wyz: self.wyz * rhs.xyzw,
            wzx: self.wzx * rhs.xyzw,
            wxy: self.wxy * rhs.xyzw,
            zyx: self.zyx * rhs.xyzw,
        }
    }
}

impl<T> GeometricProduct<Vector<T>> for Trivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Evenvector<T>;

    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        Evenvector {
            s: Scalar::ZERO,
            b: Bivector {
                wx: self.wxy * rhs.y - self.wzx * rhs.z,
                wy: self.wyz * rhs.z - self.wxy * rhs.x,
                wz: self.wzx * rhs.x - self.wyz * rhs.y,
                yz: -self.zyx * rhs.x,
                zx: -self.zyx * rhs.y,
                xy: -self.zyx * rhs.z,
            },
            a: self.wedge(rhs),
        }
    }
}

impl<T> GeometricProduct<Bivector<T>> for Trivector<T>
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
        Multivector {
            s: Scalar::ZERO,
            v: Vector {
                x: self.zyx * rhs.yz,
                y: self.zyx * rhs.zx,
                z: self.zyx * rhs.xy,
                w: -(self.wyz * rhs.yz + self.wzx * rhs.zx + self.wxy * rhs.xy),
            },
            b: Bivector::ZERO,
            t: Trivector {
                wyz: self.zyx * rhs.wx + self.wxy * rhs.zx - self.wzx * rhs.xy,
                wzx: self.zyx * rhs.wy + self.wyz * rhs.xy - self.wxy * rhs.yz,
                wxy: self.zyx * rhs.wz + self.wzx * rhs.yz - self.wyz * rhs.zx,
                zyx: T::ZERO,
            },
            a: Quadvector::ZERO,
        }
    }
}

impl<T> GeometricProduct<Quadvector<T>> for Trivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Mul<T, Output = T>,
{
    type Output = Vector<T>;

    fn geometric_product(&self, rhs: &Quadvector<T>) -> Self::Output {
        Vector {
            x: T::ZERO,
            y: T::ZERO,
            z: T::ZERO,
            w: self.zyx * rhs.xyzw,
        }
    }
}

impl<T> GeometricProduct<Evenvector<T>> for Trivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
        self.geometric_product(&rhs.s)
            + self.geometric_product(&rhs.b)
            + self.geometric_product(&rhs.a)
    }
}

reverse_add!(Quadvector, Trivector);

reverse_antiwedge!(Quadvector, Trivector);
