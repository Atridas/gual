//! |  euclidean |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | ---------- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |    1       |  1  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |    x       |  x  |  1  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |    y       |  y  |  0  |  1  |  0  |  0  |  0  |  0  |  0  |
//! |    z       |  z  |  0  |  0  |  1  |  0  |  0  |  0  |  0  |
//! |   yz       |  yz |  0  |  z  | -y  |  1  |  0  |  0  |  0  |
//! |   zx       |  zx | -z  |  0  |  x  |  0  |  1  |  0  |  0  |
//! |   xy       |  xy |  y  | -x  |  0  |  0  |  0  |  1  |  0  |
//! |  xyz       | xyz |  yz |  zx |  xy |  x  |  y  |  z  |  1  |
//!
//! | projective bulk |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | --------------- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |        1        |  1  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |        x        |  x  |  1  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |        y        |  y  |  0  |  1  |  0  |  0  |  0  |  0  |  0  |
//! |        z        |  z  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |        yz       |  yz |  0  |  z  |  0  |  0  |  0  |  0  |  0  |
//! |        zx       |  zx | -z  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |        xy       |  xy |  y  | -x  |  0  |  0  |  0  |  1  |  0  |
//! |       xyz       | xyz |  yz |  zx |  0  |  0  |  0  |  z  |  0  |
//!
//! | projective weight |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | ----------------- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |          1        |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |          x        |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |          y        |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |          z        |  0  |  0  |  0  |  1  |  0  |  0  |  0  |  0  |
//! |          yz       |  0  |  0  |  0  | -y  |  1  |  0  |  0  |  0  |
//! |          zx       |  0  |  0  |  0  |  x  |  0  |  1  |  0  |  0  |
//! |          xy       |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |         xyz       |  0  |  0  |  0  |  xy |  x  |  y  |  0  |  0  |
//!

use std::{
    marker::PhantomData,
    ops::{Mul, Neg, Sub},
};

use num::traits::ConstZero;

use crate::{
    Contraction, Projective,
    geometry3d::{Bivector, Trivector, Vector},
};

// ----------------------------------------------------------------------------------------------------
// Euclidean
// ----------------------------------------------------------------------------------------------------

impl<T> Contraction<Vector<T>> for Bivector<T>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type BulkOutput = Vector<T>;
    type WeightOutput = Vector<T>;

    fn bulk_contraction(&self, rhs: &Vector<T>) -> Self::BulkOutput {
        Vector {
            x: self.zx * rhs.z - self.xy * rhs.y,
            y: self.xy * rhs.x - self.yz * rhs.z,
            z: self.yz * rhs.y - self.zx * rhs.x,
            _metric: PhantomData,
        }
    }

    fn weight_contraction(&self, rhs: &Vector<T>) -> Self::WeightOutput {
        self.bulk_contraction(rhs)
    }
}

impl<T> Contraction<Vector<T>> for Trivector<T>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type BulkOutput = Bivector<T>;
    type WeightOutput = Bivector<T>;

    fn bulk_contraction(&self, rhs: &Vector<T>) -> Self::BulkOutput {
        Bivector {
            yz: self.xyz * rhs.x,
            zx: self.xyz * rhs.y,
            xy: self.xyz * rhs.z,
            _metric: PhantomData,
        }
    }

    fn weight_contraction(&self, rhs: &Vector<T>) -> Self::WeightOutput {
        self.bulk_contraction(rhs)
    }
}

impl<T> Contraction<Bivector<T>> for Trivector<T>
where
    T: Copy,
    T: Mul<Output = T>,
{
    type BulkOutput = Vector<T>;
    type WeightOutput = Vector<T>;

    fn bulk_contraction(&self, rhs: &Bivector<T>) -> Self::BulkOutput {
        Vector {
            x: self.xyz * rhs.yz,
            y: self.xyz * rhs.zx,
            z: self.xyz * rhs.xy,
            _metric: PhantomData,
        }
    }

    fn weight_contraction(&self, rhs: &Bivector<T>) -> Self::WeightOutput {
        self.bulk_contraction(rhs)
    }
}

// ----------------------------------------------------------------------------------------------------
// Projective
// ----------------------------------------------------------------------------------------------------

impl<T> Contraction<Vector<T, Projective>> for Bivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type BulkOutput = Vector<T, Projective>;
    type WeightOutput = Vector<T, Projective>;

    fn bulk_contraction(&self, rhs: &Vector<T, Projective>) -> Self::BulkOutput {
        Vector {
            x: -(self.xy * rhs.y),
            y: self.xy * rhs.x,
            z: self.yz * rhs.y - self.zx * rhs.x,
            _metric: PhantomData,
        }
    }

    fn weight_contraction(&self, rhs: &Vector<T, Projective>) -> Self::WeightOutput {
        Vector {
            x: self.zx * rhs.z,
            y: -(self.yz * rhs.z),
            z: T::ZERO,
            _metric: PhantomData,
        }
    }
}

impl<T> Contraction<Vector<T, Projective>> for Trivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type BulkOutput = Bivector<T, Projective>;
    type WeightOutput = Bivector<T, Projective>;

    fn bulk_contraction(&self, rhs: &Vector<T, Projective>) -> Self::BulkOutput {
        Bivector {
            yz: self.xyz * rhs.x,
            zx: self.xyz * rhs.y,
            xy: T::ZERO,
            _metric: PhantomData,
        }
    }

    fn weight_contraction(&self, rhs: &Vector<T, Projective>) -> Self::WeightOutput {
        Bivector {
            yz: T::ZERO,
            zx: T::ZERO,
            xy: self.xyz * rhs.z,
            _metric: PhantomData,
        }
    }
}

impl<T> Contraction<Bivector<T, Projective>> for Trivector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Mul<Output = T>,
{
    type BulkOutput = Vector<T, Projective>;
    type WeightOutput = Vector<T, Projective>;

    fn bulk_contraction(&self, rhs: &Bivector<T, Projective>) -> Self::BulkOutput {
        Vector {
            x: T::ZERO,
            y: T::ZERO,
            z: self.xyz * rhs.xy,
            _metric: PhantomData,
        }
    }

    fn weight_contraction(&self, rhs: &Bivector<T, Projective>) -> Self::WeightOutput {
        Vector {
            x: self.xyz * rhs.yz,
            y: self.xyz * rhs.zx,
            z: T::ZERO,
            _metric: PhantomData,
        }
    }
}
