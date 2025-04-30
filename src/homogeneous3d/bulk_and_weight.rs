use std::ops::{Div, Neg};

use num::{
    Zero,
    traits::{ConstOne, ConstZero},
};

use crate::BulkAndWeight;

use super::{HomogeneusLine, HomogeneusPlane, HomogeneusPoint, HorizonLine, Line, Plane};
use crate::geometry3d as d3;
use crate::geometry4d as d4;

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

impl<T: Copy> BulkAndWeight for Line<T> {
    type Bulk = d3::Bivector<T>;
    type Weight = d3::DirVector<T>;

    fn from_bulk_and_weight(bulk: Self::Bulk, weight: Self::Weight) -> Self {
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
        d3::Bivector {
            yz: self.0.yz,
            zx: self.0.zx,
            xy: self.0.xy,
        }
    }

    fn weight(&self) -> Self::Weight {
        d3::DirVector(d3::Vector {
            x: self.0.wx,
            y: self.0.wy,
            z: self.0.wz,
        })
    }
}

impl<T: Copy> BulkAndWeight for HorizonLine<T>
where
    d3::Vector<T>: ConstZero,
    d3::Vector<T>: Zero,
{
    type Bulk = d3::Bivector<T>;
    type Weight = d3::Vector<T>;

    fn from_bulk_and_weight(bulk: Self::Bulk, weight: Self::Weight) -> Self {
        assert!(weight.is_zero());
        Self(d3::Bivector {
            yz: bulk.yz,
            zx: bulk.zx,
            xy: bulk.xy,
        })
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Bivector {
            yz: self.0.yz,
            zx: self.0.zx,
            xy: self.0.xy,
        }
    }

    fn weight(&self) -> Self::Weight {
        d3::Vector::ZERO
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

impl<T> BulkAndWeight for Plane<T>
where
    T: Copy,
    T: Neg<Output = T>,
{
    type Bulk = d3::Trivector<T>;
    type Weight = d3::DirBivector<T>;

    fn from_bulk_and_weight(bulk: Self::Bulk, weight: Self::Weight) -> Self {
        Self(d4::Trivector {
            wyz: weight.0.yz,
            wzx: weight.0.zx,
            wxy: weight.0.xy,
            zyx: -bulk.xyz,
        })
    }

    fn bulk(&self) -> Self::Bulk {
        d3::Trivector { xyz: -self.0.zyx }
    }

    fn weight(&self) -> Self::Weight {
        d3::DirBivector(d3::Bivector {
            yz: self.0.wyz,
            zx: self.0.wzx,
            xy: self.0.wxy,
        })
    }
}
