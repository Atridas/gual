use crate::Epsilon;
use crate::Normalizable;
use crate::geometry3d as d3;
use crate::geometry3d::Vector;
use crate::geometry4d as d4;

pub use d4::Bivector as HomogeneusLine;
pub use d4::Trivector as HomogeneusPlane;
pub use d4::Vector as Point;
use num::Float;

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

impl<T> Normalizable for Point<T>
where
    T: Float,
    T: Epsilon,
{
    type Output = NormalizedPoint<T>;
    fn normalize(self) -> Option<Self::Output> {
        if self.w.is_small() {
            let len = self.x * self.x + self.y * self.y + self.z * self.z;
            if len.is_small() {
                None
            } else {
                let len = len.sqrt();
                Some(NormalizedPoint::DirVector(DirVector(Vector {
                    x: self.x / len,
                    y: self.y / len,
                    z: self.z / len,
                })))
            }
        } else {
            Some(NormalizedPoint::Point(d3::Point(Vector {
                x: self.x / self.w,
                y: self.y / self.w,
                z: self.z / self.w,
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
    fn normalize(self) -> Option<Self::Output> {
        if self.is_2_blade() {
            let len = self.wx * self.wx + self.wy * self.wy + self.wz * self.wz;
            if len.is_small() {
                let len = self.yz * self.yz + self.zx * self.zx + self.xy * self.xy;
                if len.is_small() {
                    None
                } else {
                    let len = len.sqrt();
                    Some(NormalizedLine::HorizonLine(HorizonLine(d3::Bivector {
                        yz: self.yz / len,
                        zx: self.zx / len,
                        xy: self.xy / len,
                    })))
                }
            } else {
                let len = len.sqrt();
                Some(NormalizedLine::Line(Line(d4::Bivector {
                    wx: self.wx / len,
                    wy: self.wy / len,
                    wz: self.wz / len,
                    yz: self.yz / len,
                    zx: self.zx / len,
                    xy: self.xy / len,
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
    fn normalize(self) -> Option<Self::Output> {
        let len = self.wyz * self.wyz + self.wzx * self.wzx + self.wxy * self.wxy;
        if len.is_small() {
            if self.zyx.is_small() {
                Some(NormalizedPlane::Horizon)
            } else {
                None
            }
        } else {
            let len = len.sqrt();
            Some(NormalizedPlane::Plane(Plane(d4::Trivector {
                wyz: self.wyz / len,
                wzx: self.wzx / len,
                wxy: self.wxy / len,
                zyx: self.zyx / len,
            })))
        }
    }
}
