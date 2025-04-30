use num::Float;

use crate::Epsilon;

use super::{Bivector, DirBivector, DirVector, Vector};

impl<T> From<DirVector<T>> for Vector<T> {
    fn from(value: DirVector<T>) -> Self {
        value.0
    }
}

impl<T> TryFrom<Vector<T>> for DirVector<T>
where
    T: Float,
    T: Epsilon,
{
    type Error = ();
    fn try_from(value: Vector<T>) -> Result<Self, Self::Error> {
        let len2 = value.x * value.x + value.y * value.y + value.z * value.z;
        if len2.is_near_zero() {
            Err(())
        } else {
            let invlen = len2.sqrt().recip();
            Ok(DirVector(value * invlen))
        }
    }
}

impl<T> From<DirBivector<T>> for Bivector<T> {
    fn from(value: DirBivector<T>) -> Self {
        value.0
    }
}

impl<T> TryFrom<Bivector<T>> for DirBivector<T>
where
    T: Float,
    T: Epsilon,
{
    type Error = ();
    fn try_from(value: Bivector<T>) -> Result<Self, Self::Error> {
        let len2 = value.yz * value.yz + value.zx * value.zx + value.xy * value.xy;
        if len2.is_near_zero() {
            Err(())
        } else {
            let invlen = len2.sqrt().recip();
            Ok(DirBivector(value * invlen))
        }
    }
}
