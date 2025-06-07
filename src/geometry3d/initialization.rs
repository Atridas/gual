use std::marker::PhantomData;

use crate::geometry3d::{Bivector, Trivector};

use super::Vector;

impl<T, M> Vector<T, M> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector {
            x,
            y,
            z,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Bivector<T, M> {
    pub fn new(yz: T, zx: T, xy: T) -> Self {
        Bivector {
            yz,
            zx,
            xy,
            _metric: PhantomData,
        }
    }
}

impl<T, M> Trivector<T, M> {
    pub fn new(xyz: T) -> Self {
        Trivector {
            xyz,
            _metric: PhantomData,
        }
    }
}
