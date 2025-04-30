use num::{Float, traits::ConstOne};

use crate::{Antiscalar, Dot, Norm, Scalar};

use super::{HomogeneusLine, HomogeneusPlane, HomogeneusPoint};

use crate::geometry4d as d4;

impl<T> Norm for HomogeneusPoint<T>
where
    T: Float,
    Self: Dot<Antiscalar = d4::Quadvector<T>>,
    d4::Quadvector<T>: Antiscalar,
{
    fn weight_norm(&self) -> d4::Quadvector<T> {
        d4::Quadvector { xyzw: self.w.abs() }
    }
}

impl<T> Norm for HomogeneusLine<T>
where
    T: Float,
    T: ConstOne,
{
}

impl<T> Norm for HomogeneusPlane<T>
where
    T: Float,
    Self: Dot<Scalar = d4::Scalar<T>>,
    d4::Scalar<T>: Scalar,
{
    fn bulk_norm(&self) -> d4::Scalar<T> {
        d4::Scalar(self.zyx.abs())
    }
}
