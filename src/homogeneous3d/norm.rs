use num::{Float, traits::ConstOne};

use crate::{Antiscalar, Dot, Norm};

use super::{HomogeneusLine, HomogeneusPlane, HomogeneusPoint};

use crate::geometry4d as d4;

impl<T> Norm for HomogeneusPoint<T>
where
    T: Float,
    Self: Dot<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
    d4::Quadvector<T>: Antiscalar,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn bulk_norm_squared(&self) -> Self::Scalar {
        d4::Scalar(self.dot(self).0)
    }

    fn weight_norm_squared(&self) -> Self::Antiscalar {
        d4::Quadvector {
            xyzw: self.w * self.w,
        }
    }

    fn bulk_norm(&self) -> d4::Scalar<T> {
        d4::Scalar(self.dot(self).0.sqrt())
    }

    fn weight_norm(&self) -> d4::Quadvector<T> {
        d4::Quadvector { xyzw: self.w.abs() }
    }
}

impl<T> Norm for HomogeneusLine<T>
where
    T: Float,
    T: ConstOne,
    Self: Dot<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn bulk_norm_squared(&self) -> Self::Scalar {
        d4::Scalar(self.dot(self).0)
    }

    fn weight_norm_squared(&self) -> Self::Antiscalar {
        d4::Quadvector {
            xyzw: self.antidot(self).xyzw,
        }
    }

    fn bulk_norm(&self) -> d4::Scalar<T> {
        d4::Scalar(self.dot(self).0.sqrt())
    }

    fn weight_norm(&self) -> d4::Quadvector<T> {
        d4::Quadvector {
            xyzw: self.weight_norm_squared().xyzw.sqrt(),
        }
    }
}

impl<T> Norm for HomogeneusPlane<T>
where
    T: Float,
    Self: Dot<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn bulk_norm_squared(&self) -> Self::Scalar {
        d4::Scalar(self.zyx * self.zyx)
    }

    fn weight_norm_squared(&self) -> Self::Antiscalar {
        self.antidot(self)
    }

    fn bulk_norm(&self) -> d4::Scalar<T> {
        d4::Scalar(self.zyx.abs())
    }

    fn weight_norm(&self) -> d4::Quadvector<T> {
        d4::Quadvector {
            xyzw: self.weight_norm_squared().xyzw.sqrt(),
        }
    }
}
