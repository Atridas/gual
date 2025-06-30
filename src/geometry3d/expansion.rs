//! |  euclidean |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | ---------- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |    1       | xyz |  yz |  zx |  xy |  x  |  y  |  z  |  1  |
//! |    x       |  0  | xyz |  0  |  0  |  0  | xy  | -zx |  x  |
//! |    y       |  0  |  0  | xyz |  0  | -xy |  0  |  yz |  y  |
//! |    z       |  0  |  0  |  0  | xyz |  zx | -yz |  0  |  z  |
//! |   yz       |  0  |  0  |  0  |  0  | xyz |  0  |  0  |  yz |
//! |   zx       |  0  |  0  |  0  |  0  |  0  | xyz |  0  |  zx |
//! |   xy       |  0  |  0  |  0  |  0  |  0  |  0  | xyz |  xy |
//! |  xyz       |  0  |  0  |  0  |  0  |  0  |  0  |  0  | xyz |
//!
//! |  projective bulk |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | ---------------- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |          1       | xyz |  yz |  zx |  0  |  0  |  0  |  z  |  0  |
//! |          x       |  0  | xyz |  0  |  0  |  0  |  0  | -zx |  0  |
//! |          y       |  0  |  0  | xyz |  0  |  0  |  0  |  yz |  0  |
//! |          z       |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |         yz       |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |         zx       |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//! |         xy       |  0  |  0  |  0  |  0  |  0  |  0  | xyz |  0  |
//! |        xyz       |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  0  |
//!
//! | projective weight |  1  |  x  |  y  |  z  |  yz |  zx |  xy | xyz |
//! | ----------------- | --- | --- | --- | --- | --- | --- | --- | --- |
//! |          1        |  0  |  0  |  0  |  xy |  x  |  y  |  0  |  1  |
//! |          x        |  0  |  0  |  0  |  0  |  0  | xy  |  0  |  x  |
//! |          y        |  0  |  0  |  0  |  0  | -xy |  0  |  0  |  y  |
//! |          z        |  0  |  0  |  0  | xyz |  zx | -yz |  0  |  z  |
//! |          yz       |  0  |  0  |  0  |  0  | xyz |  0  |  0  |  yz |
//! |          zx       |  0  |  0  |  0  |  0  |  0  | xyz |  0  |  zx |
//! |          xy       |  0  |  0  |  0  |  0  |  0  |  0  |  0  |  xy |
//! |         xyz       |  0  |  0  |  0  |  0  |  0  |  0  |  0  | xyz |

use std::{
    marker::PhantomData,
    ops::{Mul, Neg, Sub},
};

use num::traits::ConstZero;

use crate::{Expansion, Projective, geometry3d::Bivector};

use super::Vector;

// ----------------------------------------------------------------------------------------------------
// Euclidean
// ----------------------------------------------------------------------------------------------------

impl<T> Expansion<Bivector<T>> for Vector<T>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type BulkOutput = Bivector<T>;
    type WeightOutput = Bivector<T>;

    fn bulk_expansion(&self, rhs: &Bivector<T>) -> Self::BulkOutput {
        Bivector {
            yz: self.y * rhs.xy - self.z * rhs.zx,
            zx: self.z * rhs.yz - self.x * rhs.xy,
            xy: self.x * rhs.zx - self.y * rhs.yz,
            _metric: PhantomData,
        }
    }

    fn weight_expansion(&self, rhs: &Bivector<T>) -> Self::WeightOutput {
        self.bulk_expansion(rhs)
    }
}

// ----------------------------------------------------------------------------------------------------
// Projective
// ----------------------------------------------------------------------------------------------------

impl<T> Expansion<Bivector<T, Projective>> for Vector<T, Projective>
where
    T: Copy,
    T: ConstZero,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type BulkOutput = Bivector<T, Projective>;
    type WeightOutput = Bivector<T, Projective>;

    fn bulk_expansion(&self, rhs: &Bivector<T, Projective>) -> Self::BulkOutput {
        Bivector {
            yz: self.y * rhs.xy,
            zx: -(self.x * rhs.xy),
            xy: T::ZERO,
            _metric: PhantomData,
        }
    }

    fn weight_expansion(&self, rhs: &Bivector<T, Projective>) -> Self::WeightOutput {
        Bivector {
            yz: -(self.z * rhs.zx),
            zx: self.z * rhs.yz,
            xy: self.x * rhs.zx - self.y * rhs.yz,
            _metric: PhantomData,
        }
    }
}
