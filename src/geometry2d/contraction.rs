use std::{
    marker::PhantomData,
    ops::{Mul, Neg},
};

use crate::{
    Contraction,
    geometry2d::{Bivector, Vector},
};

impl<T> Contraction<Vector<T>> for Bivector<T>
where
    T: Copy,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type BulkOutput = Vector<T>;
    type WeightOutput = Vector<T>;

    fn bulk_contraction(&self, rhs: &Vector<T>) -> Vector<T> {
        Vector {
            x: -(self.xy * rhs.y),
            y: self.xy * rhs.x,
            _metric: PhantomData,
        }
    }

    fn weight_contraction(&self, rhs: &Vector<T>) -> Vector<T> {
        Vector {
            x: -(self.xy * rhs.y),
            y: self.xy * rhs.x,
            _metric: PhantomData,
        }
    }
}
