use std::ops::{Add, Mul, Neg, Sub};

use num::Float;

use crate::{Epsilon, OrthogonalAntiprojection};

use super::{HomogeneusLine, Line};
use super::{HomogeneusPlane, HomogeneusPoint, Plane};
use crate::geometry3d as d3;
use crate::geometry4d as d4;

impl<T> OrthogonalAntiprojection<HomogeneusPoint<T>> for HomogeneusPlane<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn orthogonal_antiprojection(&self, rhs: &HomogeneusPoint<T>) -> Self::Output {
        let w2 = rhs.w * rhs.w;
        HomogeneusPlane {
            wyz: w2 * self.wyz,
            wzx: w2 * self.wzx,
            wxy: w2 * self.wxy,
            zyx: -(self.wyz * rhs.x + self.wzx * rhs.y + self.wxy * rhs.z) * rhs.w,
        }
    }
}

impl<T> OrthogonalAntiprojection<d3::Point<T>> for Plane<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn orthogonal_antiprojection(&self, rhs: &d3::Point<T>) -> Self::Output {
        Plane(d4::Trivector {
            wyz: self.0.wyz,
            wzx: self.0.wzx,
            wxy: self.0.wxy,
            zyx: -(self.0.wyz * rhs.0.x + self.0.wzx * rhs.0.y + self.0.wxy * rhs.0.z),
        })
    }
}

impl<T> OrthogonalAntiprojection<HomogeneusPoint<T>> for HomogeneusLine<T>
where
    T: Copy,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn orthogonal_antiprojection(&self, rhs: &HomogeneusPoint<T>) -> Self::Output {
        let w2 = rhs.w * rhs.w;
        HomogeneusLine {
            wx: self.wx * w2,
            wy: self.wy * w2,
            wz: self.wz * w2,
            yz: (rhs.y * self.wz - rhs.z * self.wy) * rhs.w,
            zx: (rhs.z * self.wx - rhs.x * self.wz) * rhs.w,
            xy: (rhs.x * self.wy - rhs.y * self.wx) * rhs.w,
        }
    }
}

impl<T> OrthogonalAntiprojection<d3::Point<T>> for Line<T>
where
    T: Copy,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn orthogonal_antiprojection(&self, rhs: &d3::Point<T>) -> Self::Output {
        Line(d4::Bivector {
            wx: self.0.wx,
            wy: self.0.wy,
            wz: self.0.wz,
            yz: (rhs.0.y * self.0.wz - rhs.0.z * self.0.wy),
            zx: (rhs.0.z * self.0.wx - rhs.0.x * self.0.wz),
            xy: (rhs.0.x * self.0.wy - rhs.0.y * self.0.wx),
        })
    }
}

impl<T> OrthogonalAntiprojection<HomogeneusLine<T>> for HomogeneusPlane<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn orthogonal_antiprojection(&self, rhs: &HomogeneusLine<T>) -> Self::Output {
        let w2 = rhs.wx * rhs.wx + rhs.wy * rhs.wy + rhs.wz * rhs.wz;
        let b = rhs.wx * self.wyz + rhs.wy * self.wzx + rhs.wz * self.wxy;
        HomogeneusPlane {
            wyz: w2 * self.wyz - b * rhs.wx,
            wzx: w2 * self.wzx - b * rhs.wy,
            wxy: w2 * self.wxy - b * rhs.wz,
            zyx: (rhs.wx * rhs.xy + rhs.wy * rhs.yz + rhs.wy * rhs.yz
                - rhs.wx * rhs.zx
                - rhs.wy * rhs.xy
                - rhs.wz * rhs.yz)
                * self.zyx,
        }
    }
}

impl<T> OrthogonalAntiprojection<Line<T>> for Plane<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type Output = Option<Self>;

    fn orthogonal_antiprojection(&self, rhs: &Line<T>) -> Self::Output {
        let w2 = rhs.0.wx * rhs.0.wx + rhs.0.wy * rhs.0.wy + rhs.0.wz * rhs.0.wz;
        let b = rhs.0.wx * self.0.wyz + rhs.0.wy * self.0.wzx + rhs.0.wz * self.0.wxy;
        let wyz = w2 * self.0.wyz - b * rhs.0.wx;
        let wzx: T = w2 * self.0.wzx - b * rhs.0.wy;
        let wxy = w2 * self.0.wxy - b * rhs.0.wz;
        let len2 = wyz * wyz + wzx * wzx + wxy * wxy;
        if len2.is_near_zero() {
            None
        } else {
            let invlen = len2.sqrt().recip();
            Some(Plane(d4::Trivector {
                wyz: wyz * invlen,
                wzx: wzx * invlen,
                wxy: wxy * invlen,
                zyx: (rhs.0.wx * rhs.0.xy + rhs.0.wy * rhs.0.yz + rhs.0.wy * rhs.0.yz
                    - rhs.0.wx * rhs.0.zx
                    - rhs.0.wy * rhs.0.xy
                    - rhs.0.wz * rhs.0.yz)
                    * self.0.zyx
                    * invlen,
            }))
        }
    }
}
