use std::ops::{Add, Mul, Neg, Sub};

use num::Float;

use crate::{Epsilon, OrthogonalProjection, Support};

use super::{HomogeneusLine, Line};
use super::{HomogeneusPlane, HomogeneusPoint, Plane};
use crate::geometry3d as d3;

impl<T> OrthogonalProjection<HomogeneusPlane<T>> for HomogeneusPoint<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn orthogonal_projection(&self, rhs: &HomogeneusPlane<T>) -> Self::Output {
        let a = rhs.wyz * rhs.wyz + rhs.wzx * rhs.wzx + rhs.wxy * rhs.wxy;
        let b = self.x * rhs.wyz + self.y * rhs.wzx + self.z * rhs.wxy + self.w * rhs.zyx;
        HomogeneusPoint {
            x: a * self.x - b * rhs.wyz,
            y: a * self.y - b * rhs.wzx,
            z: a * self.z - b * rhs.wxy,
            w: a * self.w,
        }
    }
}

impl<T> OrthogonalProjection<Plane<T>> for d3::Point<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn orthogonal_projection(&self, rhs: &Plane<T>) -> Self::Output {
        let b = self.0.x * rhs.0.wyz + self.0.y * rhs.0.wzx + self.0.z * rhs.0.wxy + rhs.0.zyx;
        d3::Point(d3::Vector::new(
            self.0.x - b * rhs.0.wyz,
            self.0.y - b * rhs.0.wzx,
            self.0.z - b * rhs.0.wxy,
        ))
    }
}

impl<T> OrthogonalProjection<HomogeneusLine<T>> for HomogeneusPoint<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn orthogonal_projection(&self, rhs: &HomogeneusLine<T>) -> Self::Output {
        let a = self.x * rhs.wx + self.y * rhs.wy + self.z * rhs.wz;

        HomogeneusPoint {
            x: a * rhs.wx + self.w * (rhs.wy * rhs.xy - rhs.wz * rhs.zx),
            y: a * rhs.wy + self.w * (rhs.wz * rhs.yz - rhs.wx * rhs.xy),
            z: a * rhs.wz + self.w * (rhs.wx * rhs.zx - rhs.wy * rhs.yz),
            w: self.w * (rhs.wx * rhs.wx + rhs.wy * rhs.wy + rhs.wz * rhs.wz),
        }
    }
}

impl<T> OrthogonalProjection<Line<T>> for d3::Point<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn orthogonal_projection(&self, rhs: &Line<T>) -> Self::Output {
        let a = self.0.x * rhs.0.wx + self.0.y * rhs.0.wy + self.0.z * rhs.0.wz;

        d3::Point(d3::Vector::new(
            a * rhs.0.wx + (rhs.0.wy * rhs.0.xy - rhs.0.wz * rhs.0.zx),
            a * rhs.0.wy + (rhs.0.wz * rhs.0.yz - rhs.0.wx * rhs.0.xy),
            a * rhs.0.wz + (rhs.0.wx * rhs.0.zx - rhs.0.wy * rhs.0.yz),
        ))
    }
}

impl<T> OrthogonalProjection<HomogeneusPlane<T>> for HomogeneusLine<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Output = Self;

    fn orthogonal_projection(&self, rhs: &HomogeneusPlane<T>) -> Self::Output {
        let a = rhs.wyz * rhs.wyz + rhs.wzx * rhs.wzx + rhs.wxy * rhs.wxy;
        let b = self.wx * rhs.wyz + self.wy * rhs.wzx + self.wz * rhs.wxy;
        let c = self.yz * rhs.wyz + self.zx * rhs.wzx + self.xy * rhs.wxy;
        HomogeneusLine {
            wx: a * self.wx - b * rhs.wyz,
            wy: a * self.wy - b * rhs.wzx,
            wz: a * self.wz - b * rhs.wxy,
            yz: c * self.yz + rhs.zyx * (rhs.wxy * self.wy - rhs.wzx * self.wz),
            zx: c * self.zx + rhs.zyx * (rhs.wyz * self.wz - rhs.wxy * self.wx),
            xy: c * self.xy + rhs.zyx * (rhs.wzx * self.wx - rhs.wyz * self.wy),
        }
    }
}

impl<T> OrthogonalProjection<Plane<T>> for Line<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type Output = Option<Self>;

    fn orthogonal_projection(&self, rhs: &Plane<T>) -> Self::Output {
        let a = rhs.0.wyz * rhs.0.wyz + rhs.0.wzx * rhs.0.wzx + rhs.0.wxy * rhs.0.wxy;
        let b = self.0.wx * rhs.0.wyz + self.0.wy * rhs.0.wzx + self.0.wz * rhs.0.wxy;
        let c = self.0.yz * rhs.0.wyz + self.0.zx * rhs.0.wzx + self.0.xy * rhs.0.wxy;
        let wx = a * self.0.wx - b * rhs.0.wyz;
        let wy = a * self.0.wy - b * rhs.0.wzx;
        let wz = a * self.0.wz - b * rhs.0.wxy;
        let len2 = wx * wx + wy * wy + wz * wz;
        if len2.is_near_zero() {
            None
        } else {
            let invlen = len2.sqrt().recip();
            Some(Line(HomogeneusLine {
                wx: wx * invlen,
                wy: wy * invlen,
                wz: wz * invlen,
                yz: (c * self.0.yz + rhs.0.zyx * (rhs.0.wxy * self.0.wy - rhs.0.wzx * self.0.wz))
                    * invlen,
                zx: (c * self.0.zx + rhs.0.zyx * (rhs.0.wyz * self.0.wz - rhs.0.wxy * self.0.wx))
                    * invlen,
                xy: (c * self.0.xy + rhs.0.zyx * (rhs.0.wzx * self.0.wx - rhs.0.wyz * self.0.wy))
                    * invlen,
            }))
        }
    }
}

impl<T> Support for HomogeneusLine<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Point = HomogeneusPoint<T>;

    fn support(&self) -> Self::Point {
        HomogeneusPoint {
            x: self.wy * self.xy - self.wz * self.zx,
            y: self.wz * self.yz - self.wx * self.xy,
            z: self.wx * self.zx - self.wy * self.yz,
            w: self.wx * self.wx + self.wy * self.wy + self.wz * self.wz,
        }
    }
}

impl<T> Support for Line<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
{
    type Point = d3::Point<T>;

    fn support(&self) -> Self::Point {
        d3::Point(d3::Vector::new(
            self.0.wy * self.0.xy - self.0.wz * self.0.zx,
            self.0.wz * self.0.yz - self.0.wx * self.0.xy,
            self.0.wx * self.0.zx - self.0.wy * self.0.yz,
        ))
    }
}

impl<T> Support for HomogeneusPlane<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Point = HomogeneusPoint<T>;

    fn support(&self) -> Self::Point {
        HomogeneusPoint {
            x: -self.zyx * self.wyz,
            y: -self.zyx * self.wzx,
            z: -self.zyx * self.wxy,
            w: self.wyz * self.wyz + self.wzx * self.wzx + self.wxy * self.wxy,
        }
    }
}

impl<T> Support for Plane<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type Point = d3::Point<T>;

    fn support(&self) -> Self::Point {
        d3::Point(d3::Vector::new(
            -self.0.zyx * self.0.wyz,
            -self.0.zyx * self.0.wzx,
            -self.0.zyx * self.0.wxy,
        ))
    }
}
