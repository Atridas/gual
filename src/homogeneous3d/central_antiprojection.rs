use std::ops::{Add, Mul, Neg, Sub};

use num::Float;
use num::traits::ConstOne;

use crate::{Antisupport, CentralAntiprojection, Epsilon, Unitizable};

use super::{HomogeneusLine, Line, NormalizedLine, NormalizedPlane};
use super::{HomogeneusPlane, HomogeneusPoint, Plane};
use crate::geometry3d as d3;
use crate::geometry4d as d4;

impl<T> CentralAntiprojection<HomogeneusPoint<T>> for HomogeneusPlane<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn central_antiprojection(&self, rhs: &HomogeneusPoint<T>) -> Self::Output {
        let x2 = rhs.x * rhs.x;
        let y2 = rhs.y * rhs.y;
        let z2 = rhs.z * rhs.z;
        let xx = self.wyz * rhs.x;
        let yy = self.wzx * rhs.y;
        let zz = self.wxy * rhs.z;
        let ww = self.zyx * rhs.w;

        HomogeneusPlane {
            wyz: (y2 + z2) * self.wyz - (yy + zz + ww) * rhs.x,
            wzx: (z2 + x2) * self.wzx - (zz + xx + ww) * rhs.y,
            wxy: (x2 + y2) * self.wxy - (xx + yy + ww) * rhs.z,
            zyx: (x2 + y2 + z2) * self.zyx,
        }
    }
}

impl<T> CentralAntiprojection<d3::Point<T>> for Plane<T>
where
    T: Copy,
    T: ConstOne,
    T: Float,
    T: Epsilon,
{
    type Output = Option<NormalizedPlane<T>>;

    fn central_antiprojection(&self, rhs: &d3::Point<T>) -> Self::Output {
        let x2 = rhs.0.x * rhs.0.x;
        let y2 = rhs.0.y * rhs.0.y;
        let z2 = rhs.0.z * rhs.0.z;
        let xx = self.0.wyz * rhs.0.x;
        let yy = self.0.wzx * rhs.0.y;
        let zz = self.0.wxy * rhs.0.z;

        HomogeneusPlane {
            wyz: (y2 + z2) * self.0.wyz - (yy + zz + T::ONE) * rhs.0.x,
            wzx: (z2 + x2) * self.0.wzx - (zz + xx + T::ONE) * rhs.0.y,
            wxy: (x2 + y2) * self.0.wxy - (xx + yy + T::ONE) * rhs.0.z,
            zyx: (x2 + y2 + z2) * self.0.zyx,
        }
        .unitize()
    }
}

impl<T> CentralAntiprojection<HomogeneusPoint<T>> for HomogeneusLine<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn central_antiprojection(&self, rhs: &HomogeneusPoint<T>) -> Self::Output {
        let x2 = rhs.x * rhs.x;
        let y2 = rhs.y * rhs.y;
        let z2 = rhs.z * rhs.z;
        let a = rhs.x * self.wx + rhs.y * self.wy + rhs.z * self.wz;
        HomogeneusLine {
            wx: a * rhs.x + (rhs.z * self.zx - rhs.y * self.xy) * rhs.w,
            wy: a * rhs.y + (rhs.x * self.xy - rhs.z * self.yz) * rhs.w,
            wz: a * rhs.z + (rhs.y * self.yz - rhs.x * self.zx) * rhs.w,
            yz: (y2 + z2) * self.yz - (rhs.y * self.zx + rhs.z * self.xy) * rhs.x,
            zx: (z2 + x2) * self.zx - (rhs.z * self.xy + rhs.x * self.yz) * rhs.y,
            xy: (x2 + y2) * self.xy - (rhs.x * self.yz + rhs.y * self.zx) * rhs.z,
        }
    }
}

impl<T> CentralAntiprojection<d3::Point<T>> for Line<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type Output = Option<NormalizedLine<T>>;

    fn central_antiprojection(&self, rhs: &d3::Point<T>) -> Self::Output {
        let x2 = rhs.0.x * rhs.0.x;
        let y2 = rhs.0.y * rhs.0.y;
        let z2 = rhs.0.z * rhs.0.z;
        let a = rhs.0.x * self.0.wx + rhs.0.y * self.0.wy + rhs.0.z * self.0.wz;
        HomogeneusLine {
            wx: a * rhs.0.x + (rhs.0.z * self.0.zx - rhs.0.y * self.0.xy),
            wy: a * rhs.0.y + (rhs.0.x * self.0.xy - rhs.0.z * self.0.yz),
            wz: a * rhs.0.z + (rhs.0.y * self.0.yz - rhs.0.x * self.0.zx),
            yz: (y2 + z2) * self.0.yz - (rhs.0.y * self.0.zx + rhs.0.z * self.0.xy) * rhs.0.x,
            zx: (z2 + x2) * self.0.zx - (rhs.0.z * self.0.xy + rhs.0.x * self.0.yz) * rhs.0.y,
            xy: (x2 + y2) * self.0.xy - (rhs.0.x * self.0.yz + rhs.0.y * self.0.zx) * rhs.0.z,
        }
        .unitize()
    }
}

impl<T> CentralAntiprojection<HomogeneusLine<T>> for HomogeneusPlane<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn central_antiprojection(&self, rhs: &HomogeneusLine<T>) -> Self::Output {
        let a = rhs.yz * self.wyz + rhs.zx * self.wzx + rhs.xy * self.wxy;
        HomogeneusPlane {
            wyz: a * rhs.yz + (rhs.zx * rhs.wz - rhs.xy * rhs.wy) * self.zyx,
            wzx: a * rhs.zx + (rhs.xy * rhs.wx - rhs.yz * rhs.wz) * self.zyx,
            wxy: a * rhs.xy + (rhs.yz * rhs.wy - rhs.zx * rhs.wx) * self.zyx,
            zyx: (rhs.yz * rhs.yz + rhs.zx * rhs.zx + rhs.xy * rhs.xy) * self.zyx,
        }
    }
}

impl<T> CentralAntiprojection<Line<T>> for Plane<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type Output = Option<NormalizedPlane<T>>;

    fn central_antiprojection(&self, rhs: &Line<T>) -> Self::Output {
        let a = rhs.0.yz * self.0.wyz + rhs.0.zx * self.0.wzx + rhs.0.xy * self.0.wxy;
        HomogeneusPlane {
            wyz: a * rhs.0.yz + (rhs.0.zx * rhs.0.wz - rhs.0.xy * rhs.0.wy) * self.0.zyx,
            wzx: a * rhs.0.zx + (rhs.0.xy * rhs.0.wx - rhs.0.yz * rhs.0.wz) * self.0.zyx,
            wxy: a * rhs.0.xy + (rhs.0.yz * rhs.0.wy - rhs.0.zx * rhs.0.wx) * self.0.zyx,
            zyx: (rhs.0.yz * rhs.0.yz + rhs.0.zx * rhs.0.zx + rhs.0.xy * rhs.0.xy) * self.0.zyx,
        }
        .unitize()
    }
}

impl<T> Antisupport for HomogeneusPoint<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Plane = HomogeneusPlane<T>;

    fn antisupport(&self) -> Self::Plane {
        HomogeneusPlane {
            wyz: -self.w * self.x,
            wzx: -self.w * self.y,
            wxy: -self.w * self.z,
            zyx: self.x * self.x + self.y * self.y + self.z * self.z,
        }
    }
}

impl<T> Antisupport for d3::Point<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type Plane = Option<Plane<T>>;

    fn antisupport(&self) -> Self::Plane {
        let len2 = self.0.x * self.0.x + self.0.y * self.0.y + self.0.z * self.0.z;
        if len2.is_near_zero() {
            None
        } else {
            let len = len2.sqrt();
            let invlen = len.recip();
            Some(Plane(d4::Trivector {
                wyz: -self.0.x * invlen,
                wzx: -self.0.y * invlen,
                wxy: -self.0.z * invlen,
                zyx: len,
            }))
        }
    }
}

impl<T> Antisupport for HomogeneusLine<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Plane = HomogeneusPlane<T>;

    fn antisupport(&self) -> Self::Plane {
        HomogeneusPlane {
            wyz: self.zx * self.wz - self.xy * self.wy,
            wzx: self.xy * self.wx - self.yz * self.wz,
            wxy: self.yz * self.wy - self.zx * self.wx,
            zyx: self.yz * self.yz + self.zx * self.zx + self.xy * self.xy,
        }
    }
}

impl<T> Antisupport for Line<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type Plane = Option<NormalizedPlane<T>>;

    fn antisupport(&self) -> Self::Plane {
        HomogeneusPlane {
            wyz: self.0.zx * self.0.wz - self.0.xy * self.0.wy,
            wzx: self.0.xy * self.0.wx - self.0.yz * self.0.wz,
            wxy: self.0.yz * self.0.wy - self.0.zx * self.0.wx,
            zyx: self.0.yz * self.0.yz + self.0.zx * self.0.zx + self.0.xy * self.0.xy,
        }
        .unitize()
    }
}
