use std::ops::{Add, Div, Mul, Neg};

use num::traits::{ConstOne, ConstZero};

use crate::{Antiscalar, Dot};

use super::{HomogeneusLine, HomogeneusPlane, HomogeneusPoint, Plane};
use super::{HorizonLine, Line};
use crate::geometry3d as d3;
use crate::geometry4d as d4;

impl<T> Dot for d4::Scalar<T>
where
    T: Copy,
    T: ConstZero,
    T: Mul<T, Output = T>,
    d4::Quadvector<T>: Antiscalar,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        d4::Scalar(self.0 * rhs.0)
    }

    fn antidot(&self, _rhs: &Self) -> Self::Antiscalar {
        d4::Quadvector::ZERO
    }
}

impl<T> Dot for HomogeneusPoint<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    d4::Quadvector<T>: Antiscalar,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        d4::Scalar(self.x * rhs.x + self.y * rhs.y + self.z * rhs.z)
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        d4::Quadvector {
            xyzw: self.w * rhs.w,
        }
    }
}

impl<T> Dot for d3::Point<T>
where
    T: Copy,
    T: ConstOne,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Div<T, Output = T>,
    d4::Quadvector<T>: Antiscalar,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        d4::Scalar(self.0.x * rhs.0.x + self.0.y * rhs.0.y + self.0.z * rhs.0.z)
    }

    fn antidot(&self, _rhs: &Self) -> Self::Antiscalar {
        d4::Quadvector::UNIT_VOLUME
    }
}

impl<T> Dot for d3::Vector<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
    d4::Quadvector<T>: Antiscalar,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        d4::Scalar(self.x * rhs.x + self.y * rhs.y + self.z * rhs.z)
    }

    fn antidot(&self, _rhs: &Self) -> Self::Antiscalar {
        d4::Quadvector::ZERO
    }
}

impl<T> Dot for HomogeneusLine<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
    d4::Quadvector<T>: Antiscalar,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        d4::Scalar(self.yz * rhs.yz + self.zx * rhs.zx + self.xy * rhs.xy)
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        d4::Quadvector {
            xyzw: self.wx * rhs.wx + self.wy * rhs.wy + self.wz * rhs.wz,
        }
    }
}

impl<T> Dot for Line<T>
where
    T: Copy,
    T: ConstOne,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
    d4::Quadvector<T>: Antiscalar,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        d4::Scalar(self.0.yz * rhs.0.yz + self.0.zx * rhs.0.zx + self.0.xy * rhs.0.xy)
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        d4::Quadvector {
            xyzw: self.0.wx * rhs.0.wx + self.0.wy * rhs.0.wy + self.0.wz * rhs.0.wz,
        }
    }
}

impl<T> Dot for HorizonLine<T>
where
    T: Copy,
    T: ConstOne,
    d4::Quadvector<T>: ConstZero,
    d4::Quadvector<T>: Antiscalar,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn dot(&self, _rhs: &Self) -> Self::Scalar {
        d4::Scalar::ONE
    }

    fn antidot(&self, _rhs: &Self) -> Self::Antiscalar {
        d4::Quadvector::ZERO
    }
}

impl<T> Dot for HomogeneusPlane<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
    d4::Quadvector<T>: Antiscalar,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        d4::Scalar(self.zyx * rhs.zyx)
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        d4::Quadvector {
            xyzw: self.wyz * rhs.wyz + self.wzx * rhs.wzx + self.wxy * rhs.wxy,
        }
    }
}

impl<T> Dot for Plane<T>
where
    T: Copy,
    T: ConstOne,
    T: Neg<Output = T>,
    T: Add<T, Output = T>,
    T: Mul<T, Output = T>,
    d4::Quadvector<T>: Antiscalar,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn dot(&self, rhs: &Self) -> Self::Scalar {
        d4::Scalar(self.0.zyx * rhs.0.zyx)
    }

    fn antidot(&self, _rhs: &Self) -> Self::Antiscalar {
        d4::Quadvector { xyzw: T::ONE }
    }
}

impl<T> Dot for d4::Quadvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
    d4::Quadvector<T>: Antiscalar,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn dot(&self, _rhs: &Self) -> Self::Scalar {
        d4::Scalar::ZERO
    }

    fn antidot(&self, rhs: &Self) -> Self::Antiscalar {
        d4::Quadvector {
            xyzw: self.xyzw * rhs.xyzw,
        }
    }
}
