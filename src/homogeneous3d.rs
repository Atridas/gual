use crate::Epsilon;
use crate::Normalizable;
use crate::geometry3d as d3;
use crate::geometry3d::Vector;
use crate::geometry4d as d4;

pub use d4::Bivector as HomogeneusLine;
pub use d4::Trivector as HomogeneusPlane;
pub use d4::Vector as HomogeneusPoint;
use num::Float;

mod bulk_and_weight;
mod conversions;
mod distance;
mod dot;
mod join;
mod meet;
mod norm;

pub struct DirVector<T>(d3::Vector<T>);

pub enum NormalizedPoint<T> {
    Point(d3::Point<T>),
    DirVector(DirVector<T>),
}

pub struct Line<T>(d4::Bivector<T>);
pub struct HorizonLine<T>(d3::Bivector<T>);

pub enum NormalizedLine<T> {
    Line(Line<T>),
    HorizonLine(HorizonLine<T>),
}

pub struct Plane<T>(d4::Trivector<T>);

pub enum NormalizedPlane<T> {
    Plane(Plane<T>),
    Horizon,
}

impl<T> Normalizable for HomogeneusPoint<T>
where
    T: Float,
    T: Epsilon,
{
    type Output = NormalizedPoint<T>;
    fn normalize(&self) -> Option<Self::Output> {
        if self.w.is_near_zero() {
            let len2 = self.x * self.x + self.y * self.y + self.z * self.z;
            if len2.is_near_zero() {
                None
            } else {
                let invlen = len2.sqrt().recip();
                Some(NormalizedPoint::DirVector(DirVector(Vector {
                    x: self.x * invlen,
                    y: self.y * invlen,
                    z: self.z * invlen,
                })))
            }
        } else {
            let w = self.w.recip();
            Some(NormalizedPoint::Point(d3::Point(Vector {
                x: self.x * w,
                y: self.y * w,
                z: self.z * w,
            })))
        }
    }
}

impl<T> Normalizable for HomogeneusLine<T>
where
    T: Float,
    T: Epsilon,
{
    type Output = NormalizedLine<T>;
    fn normalize(&self) -> Option<Self::Output> {
        if self.is_2_blade() {
            let len2 = self.wx * self.wx + self.wy * self.wy + self.wz * self.wz;
            if len2.is_near_zero() {
                let len2 = self.yz * self.yz + self.zx * self.zx + self.xy * self.xy;
                if len2.is_near_zero() {
                    None
                } else {
                    let invlen = len2.sqrt().recip();
                    Some(NormalizedLine::HorizonLine(HorizonLine(d3::Bivector {
                        yz: self.yz * invlen,
                        zx: self.zx * invlen,
                        xy: self.xy * invlen,
                    })))
                }
            } else {
                let invlen = len2.sqrt().recip();
                Some(NormalizedLine::Line(Line(d4::Bivector {
                    wx: self.wx * invlen,
                    wy: self.wy * invlen,
                    wz: self.wz * invlen,
                    yz: self.yz * invlen,
                    zx: self.zx * invlen,
                    xy: self.xy * invlen,
                })))
            }
        } else {
            None
        }
    }
}

impl<T> Normalizable for HomogeneusPlane<T>
where
    T: Float,
    T: Epsilon,
{
    type Output = NormalizedPlane<T>;
    fn normalize(&self) -> Option<Self::Output> {
        let len2 = self.wyz * self.wyz + self.wzx * self.wzx + self.wxy * self.wxy;
        if len2.is_near_zero() {
            if self.zyx.is_near_zero() {
                Some(NormalizedPlane::Horizon)
            } else {
                None
            }
        } else {
            let invlen = len2.sqrt().recip();
            Some(NormalizedPlane::Plane(Plane(d4::Trivector {
                wyz: self.wyz * invlen,
                wzx: self.wzx * invlen,
                wxy: self.wxy * invlen,
                zyx: self.zyx * invlen,
            })))
        }
    }
}
