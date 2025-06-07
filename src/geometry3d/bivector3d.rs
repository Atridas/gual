use std::{
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::{AntiwedgeProduct, GeometricProduct, KVector, WedgeProduct, reverse_antiwedge};

use super::{Bivector, Evenvector, Multivector, Trivector, Vector};

impl<T> Zero for Bivector<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Bivector {
            yz: T::zero(),
            zx: T::zero(),
            xy: T::zero(),
            _metric: PhantomData,
        }
    }

    fn is_zero(&self) -> bool {
        self.yz.is_zero() && self.zx.is_zero() && self.xy.is_zero()
    }
}

impl<T> ConstZero for Bivector<T>
where
    T: ConstZero,
{
    const ZERO: Self = Bivector {
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ZERO,
        _metric: PhantomData,
    };
}

impl<T> Bivector<T>
where
    T: ConstZero,
    T: ConstOne,
{
    pub const YZ: Self = Bivector {
        yz: T::ONE,
        zx: T::ZERO,
        xy: T::ZERO,
        _metric: PhantomData,
    };

    pub const ZX: Self = Bivector {
        yz: T::ZERO,
        zx: T::ONE,
        xy: T::ZERO,
        _metric: PhantomData,
    };

    pub const XY: Self = Bivector {
        yz: T::ZERO,
        zx: T::ZERO,
        xy: T::ONE,
        _metric: PhantomData,
    };
}

impl<T> Add for Bivector<T>
where
    T: Add<T, Output = T>,
{
    type Output = Bivector<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Bivector {
            yz: self.yz + rhs.yz,
            zx: self.zx + rhs.zx,
            xy: self.xy + rhs.xy,
            _metric: PhantomData,
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
            yz: self.yz - rhs.yz,
            zx: self.zx - rhs.zx,
            xy: self.xy - rhs.xy,
            _metric: PhantomData,
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
            yz: -self.yz,
            zx: -self.zx,
            xy: -self.xy,
            _metric: PhantomData,
        }
    }
}

impl<T> Mul<T> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Bivector {
            yz: self.yz * rhs,
            zx: self.zx * rhs,
            xy: self.xy * rhs,
            _metric: PhantomData,
        }
    }
}

impl<T: Copy> KVector for Bivector<T> {
    type AntiKVector = Vector<T>;

    fn right_complement(&self) -> Self::AntiKVector {
        Vector {
            x: self.yz,
            y: self.zx,
            z: self.xy,
            _metric: PhantomData,
        }
    }

    fn left_complement(&self) -> Self::AntiKVector {
        Vector {
            x: self.yz,
            y: self.zx,
            z: self.xy,
            _metric: PhantomData,
        }
    }
}

impl<T> AntiwedgeProduct<Bivector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Sub<T, Output = T>,
{
    type Output = Vector<T>;

    fn antiwedge(&self, rhs: &Bivector<T>) -> Self::Output {
        Vector {
            x: self.zx * rhs.xy - self.xy * rhs.zx,
            y: self.xy * rhs.yz - self.yz * rhs.xy,
            z: self.yz * rhs.zx - self.zx * rhs.yz,
            _metric: PhantomData,
        }
    }
}

impl<T> AntiwedgeProduct<Trivector<T>> for Bivector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type Output = Bivector<T>;

    fn antiwedge(&self, rhs: &Trivector<T>) -> Self::Output {
        Bivector {
            yz: self.yz * rhs.xyz,
            zx: self.zx * rhs.xyz,
            xy: self.xy * rhs.xyz,
            _metric: PhantomData,
        }
    }
}

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
            a: rhs.wedge(self),
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

reverse_antiwedge!(Trivector, Bivector);
