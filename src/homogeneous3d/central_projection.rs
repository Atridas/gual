use std::ops::{Add, Mul, Neg, Sub};

use num::Float;

use crate::{CentralProjection, Epsilon, Unitizable};

use super::{HomogeneusLine, Line, NormalizedLine};
use super::{HomogeneusPlane, HomogeneusPoint, Plane};
use crate::geometry3d as d3;

impl<T> CentralProjection<HomogeneusLine<T>> for HomogeneusPoint<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn central_projection(&self, rhs: &HomogeneusLine<T>) -> Self::Output {
        let a = rhs.yz * rhs.yz + rhs.zx * rhs.zx + rhs.xy * rhs.xy;
        let b = rhs.yz * self.x + rhs.zx * self.y + rhs.xy * self.z;
        let w = rhs.yz * (rhs.wz * self.y - rhs.wy * self.z)
            + rhs.zx * (rhs.wx * self.z - rhs.wz * self.x)
            + rhs.xy * (rhs.wy * self.x - rhs.wx * self.y);
        HomogeneusPoint {
            x: self.x * a - rhs.yz * b,
            y: self.y * a - rhs.zx * b,
            z: self.z * a - rhs.xy * b,
            w,
        }
    }
}

impl<T> CentralProjection<Line<T>> for d3::Point<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type Output = Option<Self>;

    fn central_projection(&self, rhs: &Line<T>) -> Self::Output {
        let a = rhs.0.yz * rhs.0.yz + rhs.0.zx * rhs.0.zx + rhs.0.xy * rhs.0.xy;
        let b = rhs.0.yz * self.0.x + rhs.0.zx * self.0.y + rhs.0.xy * self.0.z;
        let w = rhs.0.yz * (rhs.0.wz * self.0.y - rhs.0.wy * self.0.z)
            + rhs.0.zx * (rhs.0.wx * self.0.z - rhs.0.wz * self.0.x)
            + rhs.0.xy * (rhs.0.wy * self.0.x - rhs.0.wx * self.0.y);
        if w.is_near_zero() {
            None
        } else {
            Some(d3::Point(d3::Vector {
                x: (self.0.x * a - rhs.0.yz * b) / w,
                y: (self.0.y * a - rhs.0.zx * b) / w,
                z: (self.0.z * a - rhs.0.xy * b) / w,
            }))
        }
    }
}

impl<T> CentralProjection<Line<T>> for d3::Vector<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type Output = Option<d3::Point<T>>;

    fn central_projection(&self, rhs: &Line<T>) -> Self::Output {
        let a = rhs.0.yz * rhs.0.yz + rhs.0.zx * rhs.0.zx + rhs.0.xy * rhs.0.xy;
        let b = rhs.0.yz * self.x + rhs.0.zx * self.y + rhs.0.xy * self.z;
        let w = rhs.0.yz * (rhs.0.wz * self.y - rhs.0.wy * self.z)
            + rhs.0.zx * (rhs.0.wx * self.z - rhs.0.wz * self.x)
            + rhs.0.xy * (rhs.0.wy * self.x - rhs.0.wx * self.y);
        if w.is_near_zero() {
            None
        } else {
            Some(d3::Point(d3::Vector {
                x: (self.x * a - rhs.0.yz * b) / w,
                y: (self.y * a - rhs.0.zx * b) / w,
                z: (self.z * a - rhs.0.xy * b) / w,
            }))
        }
    }
}

impl<T> CentralProjection<HomogeneusPlane<T>> for HomogeneusPoint<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn central_projection(&self, rhs: &HomogeneusPlane<T>) -> Self::Output {
        let zyx2 = rhs.zyx * rhs.zyx;
        HomogeneusPoint {
            x: zyx2 * self.x,
            y: zyx2 * self.y,
            z: zyx2 * self.z,
            w: -(self.x * rhs.wyz + self.y * rhs.wzx + self.z * rhs.wxy) * rhs.zyx,
        }
    }
}

impl<T> CentralProjection<Plane<T>> for d3::Point<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type Output = Option<Self>;

    fn central_projection(&self, rhs: &Plane<T>) -> Self::Output {
        let w = -(self.0.x * rhs.0.wyz + self.0.y * rhs.0.wzx + self.0.z * rhs.0.wxy) * rhs.0.zyx;
        if w.is_near_zero() {
            None
        } else {
            let s = rhs.0.zyx * rhs.0.zyx / w;
            Some(d3::Point(d3::Vector {
                x: s * self.0.x,
                y: s * self.0.y,
                z: s * self.0.z,
            }))
        }
    }
}

impl<T> CentralProjection<Plane<T>> for d3::Vector<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type Output = Option<d3::Point<T>>;

    fn central_projection(&self, rhs: &Plane<T>) -> Self::Output {
        let w = -(self.x * rhs.0.wyz + self.y * rhs.0.wzx + self.z * rhs.0.wxy) * rhs.0.zyx;
        if w.is_near_zero() {
            None
        } else {
            let s = rhs.0.zyx * rhs.0.zyx / w;
            Some(d3::Point(d3::Vector {
                x: s * self.x,
                y: s * self.y,
                z: s * self.z,
            }))
        }
    }
}

impl<T> CentralProjection<HomogeneusPlane<T>> for HomogeneusLine<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn central_projection(&self, rhs: &HomogeneusPlane<T>) -> Self::Output {
        let w2 = rhs.zyx * rhs.zyx;
        HomogeneusLine {
            wx: (rhs.wzx * self.xy - rhs.wxy * self.zx) * rhs.zyx,
            wy: (rhs.wxy * self.yz - rhs.wyz * self.xy) * rhs.zyx,
            wz: (rhs.wyz * self.zx - rhs.wzx * self.yz) * rhs.zyx,
            yz: w2 * self.wx,
            zx: w2 * self.wy,
            xy: w2 * self.wz,
        }
    }
}

impl<T> CentralProjection<Plane<T>> for Line<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type Output = Option<NormalizedLine<T>>;

    fn central_projection(&self, rhs: &Plane<T>) -> Self::Output {
        let w2 = rhs.0.zyx * rhs.0.zyx;
        HomogeneusLine {
            wx: (rhs.0.wzx * self.0.xy - rhs.0.wxy * self.0.zx) * rhs.0.zyx,
            wy: (rhs.0.wxy * self.0.yz - rhs.0.wyz * self.0.xy) * rhs.0.zyx,
            wz: (rhs.0.wyz * self.0.zx - rhs.0.wzx * self.0.yz) * rhs.0.zyx,
            yz: w2 * self.0.wx,
            zx: w2 * self.0.wy,
            xy: w2 * self.0.wz,
        }
        .unitize()
    }
}
