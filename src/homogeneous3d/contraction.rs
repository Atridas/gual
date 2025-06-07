use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use num::Float;
use num::traits::ConstZero;

use crate::Contraction;

use crate::Epsilon;
use crate::geometry3d as d3;
use crate::geometry4d as d4;

use super::HomogeneusLine;
use super::HomogeneusPlane;
use super::HomogeneusPoint;
use super::HorizonLine;
use super::Line;
use super::NormalizedLine;
use super::NormalizedPoint;
use super::Plane;

impl<T> Contraction<d4::Scalar<T>> for HomogeneusPoint<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusPoint<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d4::Scalar<T>) -> Self::BulkOutput {
        HomogeneusPoint {
            x: self.x * rhs.0,
            y: self.y * rhs.0,
            z: self.z * rhs.0,
            w: self.w * rhs.0,
        }
    }

    fn weight_contraction(&self, _rhs: &d4::Scalar<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<d4::Scalar<T>> for d3::Point<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type BulkOutput = d3::Point<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, _rhs: &d4::Scalar<T>) -> Self::BulkOutput {
        *self
    }

    fn weight_contraction(&self, _rhs: &d4::Scalar<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<d4::Scalar<T>> for d3::Vector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type BulkOutput = d3::Vector<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d4::Scalar<T>) -> Self::BulkOutput {
        d3::Vector::new(self.x * rhs.0, self.y * rhs.0, self.z * rhs.0)
    }

    fn weight_contraction(&self, _rhs: &d4::Scalar<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<d4::Scalar<T>> for HomogeneusLine<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusLine<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d4::Scalar<T>) -> Self::BulkOutput {
        HomogeneusLine {
            wx: self.wx * rhs.0,
            wy: self.wy * rhs.0,
            wz: self.wz * rhs.0,
            yz: self.yz * rhs.0,
            zx: self.zx * rhs.0,
            xy: self.xy * rhs.0,
        }
    }

    fn weight_contraction(&self, _rhs: &d4::Scalar<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<d4::Scalar<T>> for Line<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type BulkOutput = Line<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d4::Scalar<T>) -> Self::BulkOutput {
        Line(d4::Bivector {
            wx: self.0.wx * rhs.0,
            wy: self.0.wy * rhs.0,
            wz: self.0.wz * rhs.0,
            yz: self.0.yz * rhs.0,
            zx: self.0.zx * rhs.0,
            xy: self.0.xy * rhs.0,
        })
    }

    fn weight_contraction(&self, _rhs: &d4::Scalar<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<d4::Scalar<T>> for HorizonLine<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HorizonLine<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d4::Scalar<T>) -> Self::BulkOutput {
        HorizonLine(d3::Bivector::new(
            self.0.yz * rhs.0,
            self.0.zx * rhs.0,
            self.0.xy * rhs.0,
        ))
    }

    fn weight_contraction(&self, _rhs: &d4::Scalar<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<HomogeneusPoint<T>> for HomogeneusLine<T>
where
    T: Copy,
    T: ConstZero,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusPoint<T>;
    type WeightOutput = HomogeneusPoint<T>;

    fn bulk_contraction(&self, rhs: &HomogeneusPoint<T>) -> Self::BulkOutput {
        HomogeneusPoint {
            x: self.zx * rhs.z - self.xy * rhs.y,
            y: self.xy * rhs.x - self.yz * rhs.z,
            z: self.yz * rhs.y - self.zx * rhs.x,
            w: -(self.wx * rhs.x + self.wy * rhs.y + self.wz * rhs.z),
        }
    }

    fn weight_contraction(&self, rhs: &HomogeneusPoint<T>) -> Self::WeightOutput {
        HomogeneusPoint {
            x: self.wx * rhs.w,
            y: self.wy * rhs.w,
            z: self.wz * rhs.w,
            w: T::ZERO,
        }
    }
}

impl<T> Contraction<d3::Point<T>> for Line<T>
where
    T: Copy,
    T: Epsilon,
    T: Float,
{
    type BulkOutput = Option<NormalizedPoint<T>>;
    type WeightOutput = d3::Vector<T>;

    fn bulk_contraction(&self, rhs: &d3::Point<T>) -> Self::BulkOutput {
        self.bulk_contraction(&rhs.0)
    }

    fn weight_contraction(&self, _rhs: &d3::Point<T>) -> Self::WeightOutput {
        d3::Vector::new(self.0.wx, self.0.wy, self.0.wz)
    }
}

impl<T> Contraction<d3::Vector<T>> for Line<T>
where
    T: Copy,
    T: Epsilon,
    T: Float,
{
    type BulkOutput = Option<NormalizedPoint<T>>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d3::Vector<T>) -> Self::BulkOutput {
        let w = -(self.0.wx * rhs.x + self.0.wy * rhs.y + self.0.wz * rhs.z);

        if w.is_near_zero() {
            let x = self.0.zx * rhs.z - self.0.xy * rhs.y;
            let y = self.0.xy * rhs.x - self.0.yz * rhs.z;
            let z = self.0.yz * rhs.y - self.0.zx * rhs.x;
            let len2 = x * x + y * y + z * z;
            if len2.is_near_zero() {
                None
            } else {
                let invlen = len2.sqrt().recip();
                Some(NormalizedPoint::DirVector(d3::DirVector(d3::Vector::new(
                    x * invlen,
                    y * invlen,
                    z * invlen,
                ))))
            }
        } else {
            let inv_w = w.recip();
            Some(NormalizedPoint::Point(d3::Point(d3::Vector::new(
                (self.0.zx * rhs.z - self.0.xy * rhs.y) * inv_w,
                (self.0.xy * rhs.x - self.0.yz * rhs.z) * inv_w,
                (self.0.yz * rhs.y - self.0.zx * rhs.x) * inv_w,
            ))))
        }
    }

    fn weight_contraction(&self, _rhs: &d3::Vector<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<d3::Point<T>> for HorizonLine<T>
where
    T: Copy,
    T: Epsilon,
    T: Float,
{
    type BulkOutput = d3::Vector<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d3::Point<T>) -> Self::BulkOutput {
        d3::Vector::new(
            self.0.zx * rhs.0.z - self.0.xy * rhs.0.y,
            self.0.xy * rhs.0.x - self.0.yz * rhs.0.z,
            self.0.yz * rhs.0.y - self.0.zx * rhs.0.x,
        )
    }

    fn weight_contraction(&self, _rhs: &d3::Point<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<d3::Vector<T>> for HorizonLine<T>
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type BulkOutput = d3::Vector<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d3::Vector<T>) -> Self::BulkOutput {
        d3::Vector::new(
            self.0.zx * rhs.z - self.0.xy * rhs.y,
            self.0.xy * rhs.x - self.0.yz * rhs.z,
            self.0.yz * rhs.y - self.0.zx * rhs.x,
        )
    }

    fn weight_contraction(&self, _rhs: &d3::Vector<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<d4::Scalar<T>> for HomogeneusPlane<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusPlane<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d4::Scalar<T>) -> Self::BulkOutput {
        HomogeneusPlane {
            wyz: self.wyz * rhs.0,
            wzx: self.wzx * rhs.0,
            wxy: self.wxy * rhs.0,
            zyx: self.zyx * rhs.0,
        }
    }

    fn weight_contraction(&self, _rhs: &d4::Scalar<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<d4::Scalar<T>> for Plane<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type BulkOutput = Plane<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, _rhs: &d4::Scalar<T>) -> Self::BulkOutput {
        Plane(d4::Trivector {
            wyz: self.0.wyz,
            wzx: self.0.wzx,
            wxy: self.0.wxy,
            zyx: self.0.zyx,
        })
    }

    fn weight_contraction(&self, _rhs: &d4::Scalar<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<HomogeneusPoint<T>> for HomogeneusPlane<T>
where
    T: Copy,
    T: ConstZero,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusLine<T>;
    type WeightOutput = HomogeneusLine<T>;

    fn bulk_contraction(&self, rhs: &HomogeneusPoint<T>) -> Self::BulkOutput {
        HomogeneusLine {
            wx: self.wxy * rhs.y - self.wzx * rhs.z,
            wy: self.wyz * rhs.z - self.wxy * rhs.x,
            wz: self.wzx * rhs.x - self.wyz * rhs.y,
            yz: -self.zyx * rhs.x,
            zx: -self.zyx * rhs.y,
            xy: -self.zyx * rhs.z,
        }
    }

    fn weight_contraction(&self, rhs: &HomogeneusPoint<T>) -> Self::WeightOutput {
        HomogeneusLine {
            wx: T::ZERO,
            wy: T::ZERO,
            wz: T::ZERO,
            yz: self.wyz * rhs.w,
            zx: self.wzx * rhs.w,
            xy: self.wxy * rhs.w,
        }
    }
}

impl<T> Contraction<d3::Point<T>> for Plane<T>
where
    T: Copy,
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<NormalizedLine<T>>;
    type WeightOutput = HorizonLine<T>;

    fn bulk_contraction(&self, rhs: &d3::Point<T>) -> Self::BulkOutput {
        self.bulk_contraction(&rhs.0)
    }

    fn weight_contraction(&self, _rhs: &d3::Point<T>) -> Self::WeightOutput {
        HorizonLine(d3::Bivector::new(self.0.wyz, self.0.wzx, self.0.wxy))
    }
}

impl<T> Contraction<d3::Vector<T>> for Plane<T>
where
    T: Float,
    T: Epsilon,
{
    type BulkOutput = Option<NormalizedLine<T>>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d3::Vector<T>) -> Self::BulkOutput {
        let wx = self.0.wxy * rhs.y - self.0.wzx * rhs.z;
        let wy = self.0.wyz * rhs.z - self.0.wxy * rhs.x;
        let wz = self.0.wzx * rhs.x - self.0.wyz * rhs.y;

        let len2 = wx * wx + wy * wy + wz * wz;
        if len2.is_near_zero() {
            let yz = self.0.zyx * rhs.x;
            let zx = self.0.zyx * rhs.y;
            let xy = self.0.zyx * rhs.z;

            let len2 = wx * wx + wy * wy + wz * wz;
            if len2.is_near_zero() {
                None
            } else {
                let invlen = -len2.sqrt().recip();
                Some(NormalizedLine::HorizonLine(HorizonLine(d3::Bivector::new(
                    yz * invlen,
                    zx * invlen,
                    xy * invlen,
                ))))
            }
        } else {
            let invlen = len2.sqrt().recip();
            Some(NormalizedLine::Line(Line(d4::Bivector {
                wx: wx * invlen,
                wy: wy * invlen,
                wz: wz * invlen,
                yz: -self.0.zyx * rhs.x * invlen,
                zx: -self.0.zyx * rhs.y * invlen,
                xy: -self.0.zyx * rhs.z * invlen,
            })))
        }
    }

    fn weight_contraction(&self, _rhs: &d3::Vector<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<d4::Scalar<T>> for d4::Quadvector<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
{
    type BulkOutput = d4::Quadvector<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d4::Scalar<T>) -> Self::BulkOutput {
        d4::Quadvector {
            xyzw: self.xyzw * rhs.0,
        }
    }

    fn weight_contraction(&self, _rhs: &d4::Scalar<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<HomogeneusPoint<T>> for d4::Quadvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusPlane<T>;
    type WeightOutput = HomogeneusPlane<T>;

    fn bulk_contraction(&self, rhs: &HomogeneusPoint<T>) -> Self::BulkOutput {
        HomogeneusPlane {
            wyz: self.xyzw * rhs.x,
            wzx: self.xyzw * rhs.y,
            wxy: self.xyzw * rhs.z,
            zyx: T::ZERO,
        }
    }

    fn weight_contraction(&self, rhs: &HomogeneusPoint<T>) -> Self::WeightOutput {
        HomogeneusPlane {
            wyz: T::ZERO,
            wzx: T::ZERO,
            wxy: T::ZERO,
            zyx: self.xyzw * rhs.w,
        }
    }
}

impl<T> Contraction<d3::Point<T>> for d4::Quadvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Mul<T, Output = T>,
{
    type BulkOutput = Plane<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d3::Point<T>) -> Self::BulkOutput {
        Plane(d4::Trivector {
            wyz: self.xyzw * rhs.0.x,
            wzx: self.xyzw * rhs.0.y,
            wxy: self.xyzw * rhs.0.z,
            zyx: T::ZERO,
        })
    }

    fn weight_contraction(&self, _rhs: &d3::Point<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<d3::Vector<T>> for d4::Quadvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Mul<T, Output = T>,
{
    type BulkOutput = Plane<T>;
    type WeightOutput = ();

    fn bulk_contraction(&self, rhs: &d3::Vector<T>) -> Self::BulkOutput {
        Plane(d4::Trivector {
            wyz: self.xyzw * rhs.x,
            wzx: self.xyzw * rhs.y,
            wxy: self.xyzw * rhs.z,
            zyx: T::ZERO,
        })
    }

    fn weight_contraction(&self, _rhs: &d3::Vector<T>) -> Self::WeightOutput {}
}

impl<T> Contraction<HomogeneusLine<T>> for d4::Quadvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusLine<T>;
    type WeightOutput = HomogeneusLine<T>;

    fn bulk_contraction(&self, rhs: &HomogeneusLine<T>) -> Self::BulkOutput {
        HomogeneusLine {
            wx: -self.xyzw * rhs.yz,
            wy: -self.xyzw * rhs.zx,
            wz: -self.xyzw * rhs.xy,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        }
    }

    fn weight_contraction(&self, rhs: &HomogeneusLine<T>) -> Self::WeightOutput {
        HomogeneusLine {
            wx: T::ZERO,
            wy: T::ZERO,
            wz: T::ZERO,
            yz: -self.xyzw * rhs.wx,
            zx: -self.xyzw * rhs.wy,
            xy: -self.xyzw * rhs.wz,
        }
    }
}

impl<T> Contraction<HomogeneusPlane<T>> for d4::Quadvector<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
    T: Mul<T, Output = T>,
{
    type BulkOutput = HomogeneusPlane<T>;
    type WeightOutput = HomogeneusPlane<T>;

    fn bulk_contraction(&self, rhs: &HomogeneusPlane<T>) -> Self::BulkOutput {
        HomogeneusPlane {
            wyz: T::ZERO,
            wzx: T::ZERO,
            wxy: T::ZERO,
            zyx: -self.xyzw * rhs.zyx,
        }
    }

    fn weight_contraction(&self, rhs: &HomogeneusPlane<T>) -> Self::WeightOutput {
        HomogeneusPlane {
            wyz: -self.xyzw * rhs.wyz,
            wzx: -self.xyzw * rhs.wzx,
            wxy: -self.xyzw * rhs.wxy,
            zyx: T::ZERO,
        }
    }
}
