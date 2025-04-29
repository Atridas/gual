use std::ops::{Div, Neg};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::BulkAndWeight;

use super::{HomogeneusLine, HomogeneusPlane, HomogeneusPoint};
use crate::geometry3d as d3;

impl<T: Copy> BulkAndWeight for HomogeneusPoint<T> {
    type Bulk = d3::Vector<T>;
    type Weight = d3::Scalar<T>;

    fn from_bulk_and_weight(bulk: d3::Vector<T>, weight: d3::Scalar<T>) -> Self {
        Self {
            x: bulk.x,
            y: bulk.y,
            z: bulk.z,
            w: weight.0,
        }
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    fn weight(&self) -> Self::Weight {
        d3::Scalar(self.w)
    }
}

impl<T> BulkAndWeight for d3::Point<T>
where
    T: Copy,
    T: ConstOne,
    T: Div<T, Output = T>,
{
    type Bulk = d3::Vector<T>;
    type Weight = d3::Scalar<T>;

    fn from_bulk_and_weight(bulk: Self::Bulk, weight: Self::Weight) -> Self {
        d3::Point(d3::Vector {
            x: bulk.x / weight.0,
            y: bulk.y / weight.0,
            z: bulk.z / weight.0,
        })
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Vector {
            x: self.0.x,
            y: self.0.y,
            z: self.0.z,
        }
    }

    fn weight(&self) -> Self::Weight {
        d3::Scalar::ONE
    }
}

impl<T> BulkAndWeight for d3::Vector<T>
where
    T: Copy,
    T: Zero,
    T: ConstZero,
{
    type Bulk = d3::Vector<T>;
    type Weight = d3::Scalar<T>;

    fn from_bulk_and_weight(bulk: Self::Bulk, weight: Self::Weight) -> Self {
        assert!(weight.is_zero());
        d3::Vector {
            x: bulk.x,
            y: bulk.y,
            z: bulk.z,
        }
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    fn weight(&self) -> Self::Weight {
        d3::Scalar::ZERO
    }
}

impl<T: Copy> BulkAndWeight for HomogeneusLine<T> {
    type Bulk = d3::Bivector<T>;
    type Weight = d3::Vector<T>;

    fn from_bulk_and_weight(bulk: Self::Bulk, weight: Self::Weight) -> Self {
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
        d3::Bivector {
            yz: self.yz,
            zx: self.zx,
            xy: self.xy,
        }
    }

    fn weight(&self) -> Self::Weight {
        d3::Vector {
            x: self.wx,
            y: self.wy,
            z: self.wz,
        }
    }
}

impl<T> BulkAndWeight for HomogeneusPlane<T>
where
    T: Copy,
    T: Neg<Output = T>,
{
    type Bulk = d3::Trivector<T>;
    type Weight = d3::Bivector<T>;

    fn from_bulk_and_weight(bulk: Self::Bulk, weight: Self::Weight) -> Self {
        Self {
            wyz: weight.yz,
            wzx: weight.zx,
            wxy: weight.xy,
            zyx: -bulk.xyz,
        }
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Trivector { xyz: -self.zyx }
    }

    fn weight(&self) -> Self::Weight {
        d3::Bivector {
            yz: self.wyz,
            zx: self.wzx,
            xy: self.wxy,
        }
    }
}
