use std::ops::{Add, Mul, Neg, Sub};

use num::{
    Float, Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{
    AntiwedgeProduct, Epsilon, GeometricProduct, KVector, WedgeProduct, reverse_antiwedge,
};

use super::{Bivector, Evenvector, Multivector, Quadvector, Scalar, Trivector, Vector};

impl<T> Zero for Bivector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Bivector {
            wx: T::zero(),
            wy: T::zero(),
            wz: T::zero(),
            yz: T::zero(),
            zx: T::zero(),
            xy: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.wx.is_zero()
            && self.wy.is_zero()
            && self.wz.is_zero()
            && self.yz.is_zero()
            && self.zx.is_zero()
            && self.xy.is_zero()
    }
}

impl<T> ConstZero for Bivector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Bivector {
        wx: T::ZERO,
        wy: T::ZERO,
        wz: T::ZERO,
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ZERO,
    };
}

impl<T> Bivector<T>
where
    T: ConstZero,
    T: ConstOne,
{
    pub const WX: Self = Bivector {
        wx: T::ONE,
        wy: T::ZERO,
        wz: T::ZERO,
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ZERO,
    };

    pub const WY: Self = Bivector {
        wx: T::ZERO,
        wy: T::ONE,
        wz: T::ZERO,
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ZERO,
    };

    pub const WZ: Self = Bivector {
        wx: T::ZERO,
        wy: T::ZERO,
        wz: T::ONE,
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ZERO,
    };

    pub const YZ: Self = Bivector {
        wx: T::ZERO,
        wy: T::ZERO,
        wz: T::ZERO,
        yz: T::ONE,
        zx: T::ZERO,
        xy: T::ZERO,
    };

    pub const ZX: Self = Bivector {
        wx: T::ZERO,
        wy: T::ZERO,
        wz: T::ZERO,
        yz: T::ZERO,
        zx: T::ONE,
        xy: T::ZERO,
    };

    pub const XY: Self = Bivector {
        wx: T::ZERO,
        wy: T::ZERO,
        wz: T::ZERO,
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ONE,
    };
}

impl<T> Bivector<T>
where
    T: Float,
    T: Epsilon,
{
    pub fn is_2_blade(&self) -> bool {
        let dot = (self.wx * self.yz + self.wy * self.zx + self.wz * self.xy).abs();
        dot.is_near_zero()
    }
}

impl<T> Add for Bivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Bivector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Bivector {
            wx: self.wx + rhs.wx,
            wy: self.wy + rhs.wy,
            wz: self.wz + rhs.wz,
            yz: self.yz + rhs.yz,
            zx: self.zx + rhs.zx,
            xy: self.xy + rhs.xy,
        }
    }
}

impl<T> Sub for Bivector<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Bivector<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Bivector {
            wx: self.wx - rhs.wx,
            wy: self.wy - rhs.wy,
            wz: self.wz - rhs.wz,
            yz: self.yz - rhs.yz,
            zx: self.zx - rhs.zx,
            xy: self.xy - rhs.xy,
        }
    }
}

impl<T> Neg for Bivector<T>
where
    T: Neg<Output = T>,
{
    type Output = Bivector<T>;
    fn neg(self) -> Self::Output {
        Bivector {
            wx: -self.wx,
            wy: -self.wy,
            wz: -self.wz,
            yz: -self.yz,
            zx: -self.zx,
            xy: -self.xy,
        }
    }
}

impl<T: Copy + Neg<Output = T>> KVector for Bivector<T> {
    type AntiKVector = Bivector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Bivector {
            wx: -self.yz,
            wy: -self.zx,
            wz: -self.xy,
            yz: -self.wx,
            zx: -self.wy,
            xy: -self.wz,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Bivector {
            wx: -self.yz,
            wy: -self.zx,
            wz: -self.xy,
            yz: -self.wx,
            zx: -self.wy,
            xy: -self.wz,
        }
    }
}

impl<T> WedgeProduct<Bivector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Quadvector<T>;

    fn wedge(&self, rhs: &Bivector<T>) -> Self::Output {
        Quadvector {
            xyzw: -(self.wx * rhs.yz
                + self.wy * rhs.zx
                + self.wz * rhs.xy
                + self.yz * rhs.wx
                + self.zx * rhs.wy
                + self.xy * rhs.wz),
        }
    }
}

impl<T> AntiwedgeProduct<Bivector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Scalar<T>;

    fn antiwedge(&self, rhs: &Bivector<T>) -> Self::Output {
        Scalar(
            -(self.wx * rhs.yz
                + self.wy * rhs.zx
                + self.wz * rhs.xy
                + self.yz * rhs.wx
                + self.zx * rhs.wy
                + self.xy * rhs.wz),
        )
    }
}

impl<T> AntiwedgeProduct<Trivector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    type Output = Vector<T>;

    fn antiwedge(&self, rhs: &Trivector<T>) -> Self::Output {
        Vector {
            x: self.wx * rhs.zyx + self.zx * rhs.wxy - self.xy * rhs.wzx,
            y: self.wy * rhs.zyx + self.xy * rhs.wyz - self.yz * rhs.wxy,
            z: self.wz * rhs.zyx + self.yz * rhs.wzx - self.zx * rhs.wyz,
            w: -(self.wx * rhs.wyz + self.wy * rhs.wzx + self.wz * rhs.wxy),
        }
    }
}

impl<T> AntiwedgeProduct<Quadvector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector<T>;

    fn antiwedge(&self, rhs: &Quadvector<T>) -> Self::Output {
        Bivector {
            wx: self.wx * rhs.xyzw,
            wy: self.wy * rhs.xyzw,
            wz: self.wz * rhs.xyzw,
            yz: self.yz * rhs.xyzw,
            zx: self.zx * rhs.xyzw,
            xy: self.xy * rhs.xyzw,
        }
    }
}

impl<T> GeometricProduct<Vector<T>> for Bivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Vector<T>) -> Self::Output {
        Multivector {
            s: Scalar::ZERO,
            v: Vector {
                x: self.xy * rhs.y - self.zx * rhs.z,
                y: self.yz * rhs.z - self.xy * rhs.x,
                z: self.zx * rhs.x - self.yz * rhs.y,
                w: -(self.wx * rhs.x + self.wy * rhs.y + self.wz * rhs.z),
            },
            b: Bivector::ZERO,
            t: self.wedge(rhs),
            a: Quadvector::ZERO,
        }
    }
}

impl<T> GeometricProduct<Bivector<T>> for Bivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Evenvector<T>;

    fn geometric_product(&self, rhs: &Bivector<T>) -> Self::Output {
        Evenvector {
            s: -Scalar(self.yz * rhs.yz + self.zx * rhs.zx + self.xy * rhs.xy),
            b: Bivector {
                wx: self.wz * rhs.zx + self.xy * rhs.wy - self.wy * rhs.xy - self.zx * rhs.wz,
                wy: self.wx * rhs.xy + self.yz * rhs.wz - self.wz * rhs.yz - self.xy * rhs.wx,
                wz: self.wy * rhs.yz + self.zx * rhs.wx - self.wx * rhs.zx - self.yz * rhs.wy,
                yz: self.xy * rhs.zx - self.zx * rhs.xy,
                zx: self.yz * rhs.xy - self.xy * rhs.yz,
                xy: self.zx * rhs.yz - self.yz * rhs.zx,
            },
            a: self.wedge(rhs),
        }
    }
}

impl<T> GeometricProduct<Trivector<T>> for Bivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Multivector<T>;

    fn geometric_product(&self, rhs: &Trivector<T>) -> Self::Output {
        Multivector {
            s: Scalar::ZERO,
            v: Vector {
                x: self.yz * rhs.zyx,
                y: self.zx * rhs.zyx,
                z: self.xy * rhs.zyx,
                w: -(self.yz * rhs.wyz + self.zx * rhs.wzx + self.xy * rhs.wxy),
            },
            b: Bivector::ZERO,
            t: Trivector {
                wyz: self.xy * rhs.wzx - self.zx * rhs.wxy - self.wx * rhs.zyx,
                wzx: self.yz * rhs.wxy - self.xy * rhs.wyz - self.wy * rhs.zyx,
                wxy: self.zx * rhs.wyz - self.yz * rhs.wzx - self.wz * rhs.zyx,
                zyx: T::ZERO,
            },
            a: Quadvector::ZERO,
        }
    }
}

impl<T> GeometricProduct<Quadvector<T>> for Bivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Bivector<T>;

    fn geometric_product(&self, rhs: &Quadvector<T>) -> Self::Output {
        Bivector {
            wx: self.yz * -rhs.xyzw,
            wy: self.zx * -rhs.xyzw,
            wz: self.xy * -rhs.xyzw,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        }
    }
}

impl<T> GeometricProduct<Evenvector<T>> for Bivector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Evenvector<T>;

    fn geometric_product(&self, rhs: &Evenvector<T>) -> Self::Output {
        self.geometric_product(&rhs.s)
            + self.geometric_product(&rhs.b)
            + self.geometric_product(&rhs.a)
    }
}

impl<T> GeometricProduct<Multivector<T>> for Bivector<T>
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
        self.geometric_product(&rhs.s)
            + self.geometric_product(&rhs.b)
            + self.geometric_product(&rhs.a)
            + self.geometric_product(&rhs.v)
            + self.geometric_product(&rhs.t)
    }
}

reverse_antiwedge!(Trivector, Bivector);
reverse_antiwedge!(Quadvector, Bivector);
