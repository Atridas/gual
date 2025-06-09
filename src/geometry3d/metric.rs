use std::marker::PhantomData;

use num::Zero;
use num::traits::ConstZero;

use crate::{Metric, Projective};

use crate::geometry2d as d2;
use crate::geometry3d as d3;

// ----------------------------------------------------------------------------------------------------
// Euclidean metric
// ----------------------------------------------------------------------------------------------------

impl<T: Copy + Zero> Metric for d3::Vector<T> {
    type Bulk = d3::Vector<T>;
    type Weight = d3::Vector<T>;

    fn from_bulk(bulk: &Self::Bulk) -> Self {
        *bulk
    }

    fn from_weight(weight: &Self::Weight) -> Self {
        *weight
    }

    fn from_bulk_and_weight(bulk: &Self::Bulk, weight: &Self::Weight) -> Self {
        assert!(weight.is_zero());
        *bulk
    }

    fn bulk(&self) -> Self::Bulk {
        *self
    }

    fn weight(&self) -> Self::Weight {
        *self
    }
}

impl<T: Copy + Zero> Metric for d3::Bivector<T> {
    type Bulk = d3::Bivector<T>;
    type Weight = d3::Bivector<T>;

    fn from_bulk(bulk: &Self::Bulk) -> Self {
        *bulk
    }

    fn from_weight(weight: &Self::Weight) -> Self {
        *weight
    }

    fn from_bulk_and_weight(bulk: &Self::Bulk, weight: &Self::Weight) -> Self {
        assert!(weight.is_zero());
        *bulk
    }

    fn bulk(&self) -> Self::Bulk {
        *self
    }

    fn weight(&self) -> Self::Weight {
        *self
    }
}

impl<T: Copy + Zero> Metric for d3::Trivector<T> {
    type Bulk = d3::Trivector<T>;
    type Weight = d3::Trivector<T>;

    fn from_bulk(bulk: &Self::Bulk) -> Self {
        *bulk
    }

    fn from_weight(weight: &Self::Weight) -> Self {
        *weight
    }

    fn from_bulk_and_weight(bulk: &Self::Bulk, weight: &Self::Weight) -> Self {
        assert!(weight.is_zero());
        *bulk
    }

    fn bulk(&self) -> Self::Bulk {
        *self
    }

    fn weight(&self) -> Self::Weight {
        *self
    }
}

// ----------------------------------------------------------------------------------------------------
// Projective metric
// ----------------------------------------------------------------------------------------------------

impl<T: Copy + ConstZero> Metric for d3::Vector<T, Projective> {
    type Bulk = d2::Vector<T>;
    type Weight = T;

    fn from_bulk(bulk: &d2::Vector<T>) -> Self {
        d3::Vector::new(bulk.x, bulk.y, T::ZERO)
    }

    fn from_weight(weight: &T) -> Self {
        d3::Vector::new(T::ZERO, T::ZERO, *weight)
    }

    fn from_bulk_and_weight(bulk: &d2::Vector<T>, weight: &T) -> Self {
        d3::Vector::new(bulk.x, bulk.y, *weight)
    }

    fn bulk(&self) -> d2::Vector<T> {
        d2::Vector::new(self.x, self.y)
    }

    fn weight(&self) -> T {
        self.z
    }
}

impl<T: Copy + ConstZero> Metric for d3::Bivector<T, Projective> {
    type Bulk = d2::Bivector<T>;
    type Weight = d2::Vector<T>;

    fn from_bulk(bulk: &d2::Bivector<T>) -> Self {
        d3::Bivector {
            yz: T::ZERO,
            zx: T::ZERO,
            xy: bulk.xy,
            _metric: PhantomData,
        }
    }

    fn from_weight(weight: &d2::Vector<T>) -> Self {
        d3::Bivector {
            yz: weight.x,
            zx: weight.y,
            xy: T::ZERO,
            _metric: PhantomData,
        }
    }

    fn from_bulk_and_weight(bulk: &d2::Bivector<T>, weight: &d2::Vector<T>) -> Self {
        d3::Bivector {
            yz: weight.x,
            zx: weight.y,
            xy: bulk.xy,
            _metric: PhantomData,
        }
    }

    fn bulk(&self) -> d2::Bivector<T> {
        d2::Bivector::new(self.xy)
    }

    fn weight(&self) -> d2::Vector<T> {
        d2::Vector::new(self.yz, self.zx)
    }
}

impl<T: Copy + ConstZero> Metric for d3::Trivector<T, Projective> {
    type Bulk = ();
    type Weight = d2::Bivector<T>;

    fn from_bulk(_bulk: &()) -> Self {
        Self::ZERO
    }

    fn from_weight(weight: &d2::Bivector<T>) -> Self {
        d3::Trivector::new(weight.xy)
    }

    fn from_bulk_and_weight(_bulk: &(), weight: &d2::Bivector<T>) -> Self {
        d3::Trivector::new(weight.xy)
    }

    fn bulk(&self) -> () {}

    fn weight(&self) -> d2::Bivector<T> {
        d2::Bivector::new(self.xyz)
    }
}
