use std::ops::Neg;

use num::traits::ConstOne;
use num::traits::ConstZero;

use crate::Dual;

use super::HorizonLine;
use super::Line;
use super::Plane;
use super::{HomogeneusLine, HomogeneusPlane, HomogeneusPoint};

use crate::geometry3d as d3;
use crate::geometry4d as d4;

impl<T> Dual for d4::Scalar<T>
where
    T: Copy,
    T: ConstZero,
{
    type AntiKVector = d4::Quadvector<T>;
    fn right_bulk_dual(&self) -> d4::Quadvector<T> {
        d4::Quadvector { xyzw: self.0 }
    }

    fn left_bulk_dual(&self) -> d4::Quadvector<T> {
        d4::Quadvector { xyzw: self.0 }
    }

    fn right_weight_dual(&self) -> d4::Quadvector<T> {
        d4::Quadvector::ZERO
    }

    fn left_weight_dual(&self) -> d4::Quadvector<T> {
        d4::Quadvector::ZERO
    }
}

impl<T> Dual for HomogeneusPoint<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
{
    type AntiKVector = HomogeneusPlane<T>;

    fn right_bulk_dual(&self) -> HomogeneusPlane<T> {
        HomogeneusPlane {
            wyz: self.x,
            wzx: self.y,
            wxy: self.z,
            zyx: T::ZERO,
        }
    }

    fn left_bulk_dual(&self) -> HomogeneusPlane<T> {
        HomogeneusPlane {
            wyz: -self.x,
            wzx: -self.y,
            wxy: -self.z,
            zyx: T::ZERO,
        }
    }

    fn right_weight_dual(&self) -> HomogeneusPlane<T> {
        HomogeneusPlane {
            wyz: T::ZERO,
            wzx: T::ZERO,
            wxy: T::ZERO,
            zyx: self.w,
        }
    }

    fn left_weight_dual(&self) -> HomogeneusPlane<T> {
        HomogeneusPlane {
            wyz: T::ZERO,
            wzx: T::ZERO,
            wxy: T::ZERO,
            zyx: -self.w,
        }
    }
}

impl<T> Dual for d3::Point<T>
where
    T: Copy,
    T: ConstZero,
    T: ConstOne,
    T: Neg<Output = T>,
{
    type AntiKVector = HomogeneusPlane<T>;

    fn right_bulk_dual(&self) -> HomogeneusPlane<T> {
        HomogeneusPlane {
            wyz: self.0.x,
            wzx: self.0.y,
            wxy: self.0.z,
            zyx: T::ZERO,
        }
    }

    fn left_bulk_dual(&self) -> HomogeneusPlane<T> {
        HomogeneusPlane {
            wyz: -self.0.x,
            wzx: -self.0.y,
            wxy: -self.0.z,
            zyx: T::ZERO,
        }
    }

    fn right_weight_dual(&self) -> HomogeneusPlane<T> {
        HomogeneusPlane {
            wyz: T::ZERO,
            wzx: T::ZERO,
            wxy: T::ZERO,
            zyx: T::ONE,
        }
    }

    fn left_weight_dual(&self) -> HomogeneusPlane<T> {
        HomogeneusPlane {
            wyz: T::ZERO,
            wzx: T::ZERO,
            wxy: T::ZERO,
            zyx: -T::ONE,
        }
    }
}

impl<T> Dual for d3::Vector<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
{
    type AntiKVector = HomogeneusPlane<T>;

    fn right_bulk_dual(&self) -> HomogeneusPlane<T> {
        HomogeneusPlane {
            wyz: self.x,
            wzx: self.y,
            wxy: self.z,
            zyx: T::ZERO,
        }
    }

    fn left_bulk_dual(&self) -> HomogeneusPlane<T> {
        HomogeneusPlane {
            wyz: -self.x,
            wzx: -self.y,
            wxy: -self.z,
            zyx: T::ZERO,
        }
    }

    fn right_weight_dual(&self) -> HomogeneusPlane<T> {
        HomogeneusPlane::ZERO
    }

    fn left_weight_dual(&self) -> HomogeneusPlane<T> {
        HomogeneusPlane::ZERO
    }
}

impl<T> Dual for HomogeneusLine<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
{
    type AntiKVector = HomogeneusLine<T>;

    fn right_bulk_dual(&self) -> HomogeneusLine<T> {
        HomogeneusLine {
            wx: -self.yz,
            wy: -self.zx,
            wz: -self.xy,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        }
    }

    fn left_bulk_dual(&self) -> HomogeneusLine<T> {
        HomogeneusLine {
            wx: -self.yz,
            wy: -self.zx,
            wz: -self.xy,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        }
    }

    fn right_weight_dual(&self) -> HomogeneusLine<T> {
        HomogeneusLine {
            wx: T::ZERO,
            wy: T::ZERO,
            wz: T::ZERO,
            yz: -self.wx,
            zx: -self.wy,
            xy: -self.wz,
        }
    }

    fn left_weight_dual(&self) -> HomogeneusLine<T> {
        HomogeneusLine {
            wx: T::ZERO,
            wy: T::ZERO,
            wz: T::ZERO,
            yz: -self.wx,
            zx: -self.wy,
            xy: -self.wz,
        }
    }
}

impl<T> Dual for Line<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
{
    type AntiKVector = HomogeneusLine<T>;

    fn right_bulk_dual(&self) -> HomogeneusLine<T> {
        HomogeneusLine {
            wx: -self.0.yz,
            wy: -self.0.zx,
            wz: -self.0.xy,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        }
    }

    fn left_bulk_dual(&self) -> HomogeneusLine<T> {
        HomogeneusLine {
            wx: -self.0.yz,
            wy: -self.0.zx,
            wz: -self.0.xy,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        }
    }

    fn right_weight_dual(&self) -> HomogeneusLine<T> {
        HomogeneusLine {
            wx: T::ZERO,
            wy: T::ZERO,
            wz: T::ZERO,
            yz: -self.0.wx,
            zx: -self.0.wy,
            xy: -self.0.wz,
        }
    }

    fn left_weight_dual(&self) -> HomogeneusLine<T> {
        HomogeneusLine {
            wx: T::ZERO,
            wy: T::ZERO,
            wz: T::ZERO,
            yz: -self.0.wx,
            zx: -self.0.wy,
            xy: -self.0.wz,
        }
    }
}

impl<T> Dual for HorizonLine<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
{
    type AntiKVector = HomogeneusLine<T>;

    fn right_bulk_dual(&self) -> HomogeneusLine<T> {
        HomogeneusLine {
            wx: -self.0.yz,
            wy: -self.0.zx,
            wz: -self.0.xy,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        }
    }

    fn left_bulk_dual(&self) -> HomogeneusLine<T> {
        HomogeneusLine {
            wx: -self.0.yz,
            wy: -self.0.zx,
            wz: -self.0.xy,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        }
    }

    fn right_weight_dual(&self) -> HomogeneusLine<T> {
        HomogeneusLine::ZERO
    }

    fn left_weight_dual(&self) -> HomogeneusLine<T> {
        HomogeneusLine::ZERO
    }
}

impl<T> Dual for HomogeneusPlane<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
{
    type AntiKVector = HomogeneusPoint<T>;

    fn right_bulk_dual(&self) -> HomogeneusPoint<T> {
        HomogeneusPoint {
            x: T::ZERO,
            y: T::ZERO,
            z: T::ZERO,
            w: -self.zyx,
        }
    }

    fn left_bulk_dual(&self) -> HomogeneusPoint<T> {
        HomogeneusPoint {
            x: T::ZERO,
            y: T::ZERO,
            z: T::ZERO,
            w: self.zyx,
        }
    }

    fn right_weight_dual(&self) -> HomogeneusPoint<T> {
        HomogeneusPoint {
            x: -self.wyz,
            y: -self.wzx,
            z: -self.wxy,
            w: T::ZERO,
        }
    }

    fn left_weight_dual(&self) -> HomogeneusPoint<T> {
        HomogeneusPoint {
            x: self.wyz,
            y: self.wzx,
            z: self.wxy,
            w: T::ZERO,
        }
    }
}

impl<T> Dual for Plane<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
{
    type AntiKVector = HomogeneusPoint<T>;

    fn right_bulk_dual(&self) -> HomogeneusPoint<T> {
        HomogeneusPoint {
            x: T::ZERO,
            y: T::ZERO,
            z: T::ZERO,
            w: -self.0.zyx,
        }
    }

    fn left_bulk_dual(&self) -> HomogeneusPoint<T> {
        HomogeneusPoint {
            x: T::ZERO,
            y: T::ZERO,
            z: T::ZERO,
            w: self.0.zyx,
        }
    }

    fn right_weight_dual(&self) -> HomogeneusPoint<T> {
        HomogeneusPoint {
            x: -self.0.wyz,
            y: -self.0.wzx,
            z: -self.0.wxy,
            w: T::ZERO,
        }
    }

    fn left_weight_dual(&self) -> HomogeneusPoint<T> {
        HomogeneusPoint {
            x: self.0.wyz,
            y: self.0.wzx,
            z: self.0.wxy,
            w: T::ZERO,
        }
    }
}

impl<T> Dual for d4::Quadvector<T>
where
    T: Copy,
    T: ConstZero,
{
    type AntiKVector = d4::Scalar<T>;

    fn right_bulk_dual(&self) -> d4::Scalar<T> {
        d4::Scalar::ZERO
    }

    fn left_bulk_dual(&self) -> d4::Scalar<T> {
        d4::Scalar::ZERO
    }

    fn right_weight_dual(&self) -> d4::Scalar<T> {
        d4::Scalar(self.xyzw)
    }

    fn left_weight_dual(&self) -> d4::Scalar<T> {
        d4::Scalar(self.xyzw)
    }
}
