use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use num::Float;
use num::zero;

use crate::Epsilon;
use crate::Join;

use crate::WedgeProduct;
use crate::geometry3d as d3;
use crate::geometry4d as d4;

use super::HomogeneusPlane;
use super::Line;
use super::{HomogeneusLine, HomogeneusPoint};

impl<T> Join<HomogeneusPoint<T>> for HomogeneusPoint<T>
where
    d4::Vector<T>: WedgeProduct<d4::Vector<T>, Output = d4::Bivector<T>>,
{
    type Output = HomogeneusLine<T>;
    fn join(&self, rhs: HomogeneusPoint<T>) -> Self::Output {
        self.wedge(rhs)
    }
}

impl<T> Join<HomogeneusLine<T>> for HomogeneusPoint<T>
where
    d4::Vector<T>: WedgeProduct<d4::Bivector<T>, Output = d4::Trivector<T>>,
{
    type Output = HomogeneusPlane<T>;
    fn join(&self, rhs: HomogeneusLine<T>) -> Self::Output {
        self.wedge(rhs)
    }
}

impl<T> Join<HomogeneusPoint<T>> for HomogeneusLine<T>
where
    d4::Bivector<T>: WedgeProduct<d4::Vector<T>, Output = d4::Trivector<T>>,
{
    type Output = HomogeneusPlane<T>;
    fn join(&self, rhs: HomogeneusPoint<T>) -> Self::Output {
        self.wedge(rhs)
    }
}

impl<T> HomogeneusPoint<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    pub fn join(a: Self, b: Self, c: Self) -> HomogeneusPlane<T> {
        let acwz = a.w * c.z;
        let abwz = a.w * b.z;
        let bawz = b.w * a.z;
        let bcwz = b.w * c.z;
        let cawz = b.w * a.z;
        let cbwz = b.w * b.z;
        let bcxy = b.x * c.y - b.y * c.x;
        let caxy = c.x * a.y - c.y * a.x;
        let abxy = a.x * b.y - a.y * b.x;
        HomogeneusPlane {
            wyz: acwz * b.y - abwz * c.y + bawz * c.y - bcwz * a.y + cbwz * a.y - cawz * b.y,
            wzx: abwz * c.x - acwz * b.x + bcwz * a.x - bawz * c.x + cawz * b.x - cbwz * b.x,
            wxy: a.w * bcxy + b.w * caxy + c.w * abxy,
            zyx: -(a.z * bcxy + b.z * caxy + c.z * abxy),
        }
    }
}

impl<T> Join<d3::Point<T>> for d3::Point<T>
where
    T: Float,
    T: Epsilon,
{
    type Output = HomogeneusLine<T>;
    fn join(&self, rhs: d3::Point<T>) -> Self::Output {
        let dx = rhs.0.x - self.0.x;
        let dy = rhs.0.y - self.0.y;
        let dz = rhs.0.z - self.0.z;
        HomogeneusLine {
            wx: dx,
            wy: dy,
            wz: dz,
            yz: self.0.y * rhs.0.z - self.0.z * rhs.0.y,
            zx: self.0.z * rhs.0.x - self.0.x * rhs.0.z,
            xy: self.0.x * rhs.0.y - self.0.y * rhs.0.x,
        }
    }
}

impl<T> Join<Line<T>> for d3::Point<T>
where
    T: Float,
    T: Epsilon,
{
    type Output = HomogeneusPlane<T>;
    fn join(&self, rhs: Line<T>) -> Self::Output {
        let yz = rhs.0.yz + self.0.z * rhs.0.wy - self.0.y * rhs.0.wz;
        let zx = rhs.0.zx + self.0.x * rhs.0.wz - self.0.z * rhs.0.wx;
        let xy = rhs.0.xy + self.0.y * rhs.0.wx - self.0.x * rhs.0.wy;
        HomogeneusPlane {
            wyz: yz,
            wzx: zx,
            wxy: xy,
            zyx: -(self.0.x * rhs.0.yz + self.0.y * rhs.0.zx + self.0.z * rhs.0.xy),
        }
    }
}

impl<T> Join<d3::Point<T>> for Line<T>
where
    Self: Copy,
    T: Float,
    T: Epsilon,
{
    type Output = HomogeneusPlane<T>;
    fn join(&self, rhs: d3::Point<T>) -> Self::Output {
        rhs.join(*self)
    }
}

impl<T> d3::Point<T>
where
    T: Float,
    T: Epsilon,
{
    pub fn join(p: Self, q: Self, r: Self) -> HomogeneusPlane<T> {
        let pq = q - p;
        let pr = r - p;
        let n = pq.wedge(pr);
        HomogeneusPlane {
            wyz: n.yz,
            wzx: n.zx,
            wxy: n.xy,
            zyx: -(p.0.x * n.yz + p.0.y * n.zx + p.0.z * n.xy),
        }
    }
}

pub enum Winding {
    Clockwise,
    CounterClockwise,
}

impl<T> HomogeneusLine<T>
where
    T: Float,
    T: Epsilon,
{
    pub fn get_winding(&self, rhs: HomogeneusLine<T>) -> Option<Winding> {
        //let volume = -self.antiwedge(rhs);
        let dot = self.wx * rhs.yz
            + self.wy * rhs.zx
            + self.wz * rhs.xy
            + self.yz * rhs.wx
            + self.zx * rhs.wy
            + self.xy * rhs.wz;
        if dot.is_near_zero() {
            None
        } else if dot < zero() {
            Some(Winding::Clockwise)
        } else {
            Some(Winding::CounterClockwise)
        }
    }
}
