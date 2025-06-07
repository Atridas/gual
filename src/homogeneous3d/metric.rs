use std::ops::{Div, Neg};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::Metric;

use super::{HomogeneusLine, HomogeneusPlane, HomogeneusPoint, HorizonLine, Line, Plane};
use crate::geometry3d as d3;
use crate::geometry4d as d4;

impl<T: Copy + ConstZero> Metric for HomogeneusPoint<T> {
    type Bulk = d3::Vector<T>;
    type Weight = d3::Scalar<T>;

    fn from_bulk(bulk: &d3::Vector<T>) -> Self {
        Self {
            x: bulk.x,
            y: bulk.y,
            z: bulk.z,
            w: T::ZERO,
        }
    }

    fn from_weight(weight: &d3::Scalar<T>) -> Self {
        Self {
            x: T::ZERO,
            y: T::ZERO,
            z: T::ZERO,
            w: weight.0,
        }
    }

    fn from_bulk_and_weight(bulk: &d3::Vector<T>, weight: &d3::Scalar<T>) -> Self {
        Self {
            x: bulk.x,
            y: bulk.y,
            z: bulk.z,
            w: weight.0,
        }
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Vector::new(self.x, self.y, self.z)
    }

    fn weight(&self) -> Self::Weight {
        d3::Scalar(self.w)
    }
}

impl<T> Metric for d3::Point<T>
where
    T: Copy,
    T: ConstZero,
    T: ConstOne,
    T: Div<T, Output = T>,
{
    type Bulk = d3::Vector<T>;
    type Weight = d3::Scalar<T>;

    fn from_bulk(bulk: &Self::Bulk) -> Self {
        d3::Point(d3::Vector::new(bulk.x, bulk.y, bulk.z))
    }

    fn from_weight(bulk: &Self::Weight) -> Self {
        assert!(!bulk.is_zero());
        d3::Point::ZERO
    }

    fn from_bulk_and_weight(bulk: &Self::Bulk, weight: &Self::Weight) -> Self {
        d3::Point(d3::Vector::new(
            bulk.x / weight.0,
            bulk.y / weight.0,
            bulk.z / weight.0,
        ))
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Vector::new(self.0.x, self.0.y, self.0.z)
    }

    fn weight(&self) -> Self::Weight {
        d3::Scalar::ONE
    }
}

impl<T> Metric for d3::Vector<T>
where
    T: Copy,
    T: Zero,
    T: ConstZero,
{
    type Bulk = d3::Vector<T>;
    type Weight = d3::Scalar<T>;

    fn from_bulk(bulk: &Self::Bulk) -> Self {
        d3::Vector::new(bulk.x, bulk.y, bulk.z)
    }

    fn from_weight(bulk: &Self::Weight) -> Self {
        assert!(bulk.is_zero());
        d3::Vector::ZERO
    }

    fn from_bulk_and_weight(bulk: &Self::Bulk, weight: &Self::Weight) -> Self {
        assert!(weight.is_zero());
        d3::Vector::new(bulk.x, bulk.y, bulk.z)
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Vector::new(self.x, self.y, self.z)
    }

    fn weight(&self) -> Self::Weight {
        d3::Scalar::ZERO
    }
}

impl<T: Copy + ConstZero> Metric for HomogeneusLine<T> {
    type Bulk = d3::Bivector<T>;
    type Weight = d3::Vector<T>;

    fn from_bulk(bulk: &Self::Bulk) -> Self {
        Self {
            wx: T::ZERO,
            wy: T::ZERO,
            wz: T::ZERO,
            yz: bulk.yz,
            zx: bulk.zx,
            xy: bulk.xy,
        }
    }

    fn from_weight(weight: &Self::Weight) -> Self {
        Self {
            wx: weight.x,
            wy: weight.y,
            wz: weight.z,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        }
    }

    fn from_bulk_and_weight(bulk: &Self::Bulk, weight: &Self::Weight) -> Self {
        Self {
            wx: weight.x,
            wy: weight.y,
            wz: weight.z,
            yz: bulk.yz,
            zx: bulk.zx,
            xy: bulk.xy,
        }
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Bivector::new(self.yz, self.zx, self.xy)
    }

    fn weight(&self) -> Self::Weight {
        d3::Vector::new(self.wx, self.wy, self.wz)
    }
}

impl<T: Copy + ConstZero> Metric for Line<T> {
    type Bulk = d3::Bivector<T>;
    type Weight = d3::UnitVector<T>;

    fn from_bulk(bulk: &Self::Bulk) -> Self {
        Self(d4::Bivector {
            wx: T::ZERO,
            wy: T::ZERO,
            wz: T::ZERO,
            yz: bulk.yz,
            zx: bulk.zx,
            xy: bulk.xy,
        })
    }

    fn from_weight(weight: &Self::Weight) -> Self {
        Self(d4::Bivector {
            wx: weight.0.x,
            wy: weight.0.y,
            wz: weight.0.z,
            yz: T::ZERO,
            zx: T::ZERO,
            xy: T::ZERO,
        })
    }

    fn from_bulk_and_weight(bulk: &Self::Bulk, weight: &Self::Weight) -> Self {
        Self(d4::Bivector {
            wx: weight.0.x,
            wy: weight.0.y,
            wz: weight.0.z,
            yz: bulk.yz,
            zx: bulk.zx,
            xy: bulk.xy,
        })
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Bivector::new(self.0.yz, self.0.zx, self.0.xy)
    }

    fn weight(&self) -> Self::Weight {
        d3::UnitVector(d3::Vector::new(self.0.wx, self.0.wy, self.0.wz))
    }
}

impl<T: Copy> Metric for HorizonLine<T>
where
    d3::Vector<T>: ConstZero,
    d3::Vector<T>: Zero,
{
    type Bulk = d3::Bivector<T>;
    type Weight = d3::Vector<T>;

    fn from_bulk(bulk: &Self::Bulk) -> Self {
        // @TODO(Atridas): Normalize
        Self(d3::Bivector::new(bulk.yz, bulk.zx, bulk.xy))
    }

    fn from_weight(_weight: &Self::Weight) -> Self {
        unreachable!();
    }

    fn from_bulk_and_weight(bulk: &Self::Bulk, weight: &Self::Weight) -> Self {
        assert!(weight.is_zero());
        // @TODO(Atridas): Normalize
        Self(d3::Bivector::new(bulk.yz, bulk.zx, bulk.xy))
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Bivector::new(self.0.yz, self.0.zx, self.0.xy)
    }

    fn weight(&self) -> Self::Weight {
        d3::Vector::ZERO
    }
}

impl<T> Metric for HomogeneusPlane<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Bulk = d3::Trivector<T>;
    type Weight = d3::Bivector<T>;

    fn from_bulk(bulk: &Self::Bulk) -> Self {
        Self {
            wyz: T::ZERO,
            wzx: T::ZERO,
            wxy: T::ZERO,
            zyx: -bulk.xyz,
        }
    }

    fn from_weight(weight: &Self::Weight) -> Self {
        Self {
            wyz: weight.yz,
            wzx: weight.zx,
            wxy: weight.xy,
            zyx: T::ZERO,
        }
    }

    fn from_bulk_and_weight(bulk: &Self::Bulk, weight: &Self::Weight) -> Self {
        Self {
            wyz: weight.yz,
            wzx: weight.zx,
            wxy: weight.xy,
            zyx: -bulk.xyz,
        }
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Trivector::new(-self.zyx)
    }

    fn weight(&self) -> Self::Weight {
        d3::Bivector::new(self.wyz, self.wzx, self.wxy)
    }
}

impl<T> Metric for Plane<T>
where
    T: Copy,
    T: ConstZero,
    T: Neg<Output = T>,
{
    type Bulk = d3::Trivector<T>;
    type Weight = d3::UnitBivector<T>;

    fn from_bulk(_bulk: &Self::Bulk) -> Self {
        unreachable!();
    }

    fn from_weight(weight: &Self::Weight) -> Self {
        // @TODO(Atridas): Normalize
        Self(d4::Trivector {
            wyz: weight.0.yz,
            wzx: weight.0.zx,
            wxy: weight.0.xy,
            zyx: T::ZERO,
        })
    }

    fn from_bulk_and_weight(bulk: &Self::Bulk, weight: &Self::Weight) -> Self {
        // @TODO(Atridas): Normalize
        Self(d4::Trivector {
            wyz: weight.0.yz,
            wzx: weight.0.zx,
            wxy: weight.0.xy,
            zyx: -bulk.xyz,
        })
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Trivector::new(-self.0.zyx)
    }

    fn weight(&self) -> Self::Weight {
        d3::UnitBivector(d3::Bivector::new(self.0.wyz, self.0.wzx, self.0.wxy))
    }
}
