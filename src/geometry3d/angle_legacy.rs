use std::marker::PhantomData;

use num::{Float, traits::ConstOne};

use crate::Angle;

use super::{Scalar, Trivector, UnitBivector, UnitVector};

impl<T> Angle<UnitVector<T>> for UnitVector<T>
where
    T: ConstOne,
    T: Float,
{
    type Scalar = Scalar<T>;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &UnitVector<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            Scalar(self.0.x * rhs.0.x + self.0.y * rhs.0.y + self.0.z * rhs.0.z),
            Trivector {
                xyz: T::ONE,
                _metric: PhantomData,
            },
        )
    }

    fn cosine(&self, rhs: &UnitVector<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        Some(Scalar(geometric.0.0))
    }
}

impl<T> Angle<UnitVector<T>> for UnitBivector<T>
where
    T: ConstOne,
    T: Float,
{
    type Scalar = Scalar<T>;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &UnitVector<T>) -> (Self::Scalar, Self::Antiscalar) {
        let x = self.0.xy * rhs.0.y - self.0.zx * rhs.0.z;
        let y = self.0.yz * rhs.0.z - self.0.xy * rhs.0.x;
        let z = self.0.zx * rhs.0.x - self.0.yz * rhs.0.y;

        (
            Scalar(x * x + y * y + z * z),
            Trivector {
                xyz: T::ONE,
                _metric: PhantomData,
            },
        )
    }

    fn cosine(&self, rhs: &UnitVector<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        Some(Scalar(geometric.0.0))
    }
}

impl<T> Angle<UnitBivector<T>> for UnitVector<T>
where
    T: ConstOne,
    T: Float,
{
    type Scalar = Scalar<T>;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &UnitBivector<T>) -> (Self::Scalar, Self::Antiscalar) {
        rhs.geometric_cosine(self)
    }

    fn cosine(&self, rhs: &UnitBivector<T>) -> Option<Self::Scalar> {
        rhs.cosine(self)
    }
}

impl<T> Angle<UnitBivector<T>> for UnitBivector<T>
where
    T: ConstOne,
    T: Float,
{
    type Scalar = Scalar<T>;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &UnitBivector<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            Scalar(self.0.yz * rhs.0.yz + self.0.zx * rhs.0.zx + self.0.xy * rhs.0.xy),
            Trivector {
                xyz: T::ONE,
                _metric: PhantomData,
            },
        )
    }

    fn cosine(&self, rhs: &UnitBivector<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        Some(Scalar(geometric.0.0))
    }
}
