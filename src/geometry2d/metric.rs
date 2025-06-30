use num::Zero;

use crate::Metric;

use super::{Bivector, Vector};

impl<T: Copy + Zero> Metric for Vector<T> {
    type Bulk = Self;
    type Weight = Self;

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

    fn proper_bulk(&self) -> Self {
        *self
    }

    fn proper_weight(&self) -> Self {
        *self
    }
}

impl<T: Copy + Zero> Metric for Bivector<T> {
    type Bulk = Self;
    type Weight = Self;

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

    fn proper_bulk(&self) -> Self {
        *self
    }

    fn proper_weight(&self) -> Self {
        *self
    }
}
