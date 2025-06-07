use std::marker::PhantomData;

use num::{Float, traits::ConstOne};

use crate::Angle;

use super::{Bivector, DirBivector, DirVector, Scalar, Trivector, Vector};

impl<T> Angle<Vector<T>> for Vector<T>
where
    T: Float,
{
    type Scalar = Scalar<T>;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &Vector<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            Scalar(self.x * rhs.x + self.y * rhs.y + self.z * rhs.z),
            Trivector {
                xyz: ((self.x * self.x + self.y * self.y + self.z * self.z)
                    * (rhs.x * rhs.x + rhs.y * rhs.y + rhs.z * rhs.z))
                    .sqrt(),
                _metric: PhantomData,
            },
        )
    }

    fn cosine(&self, rhs: &Vector<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        Some(Scalar(geometric.0.0 / geometric.1.xyz))
    }
}

impl<T> Angle<DirVector<T>> for DirVector<T>
where
    T: ConstOne,
    T: Float,
{
    type Scalar = Scalar<T>;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &DirVector<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            Scalar(self.0.x * rhs.0.x + self.0.y * rhs.0.y + self.0.z * rhs.0.z),
            Trivector {
                xyz: T::ONE,
                _metric: PhantomData,
            },
        )
    }

    fn cosine(&self, rhs: &DirVector<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        Some(Scalar(geometric.0.0))
    }
}

impl<T> Angle<Vector<T>> for Bivector<T>
where
    T: Float,
{
    type Scalar = Scalar<T>;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &Vector<T>) -> (Self::Scalar, Self::Antiscalar) {
        let x = self.xy * rhs.y - self.zx * rhs.z;
        let y = self.yz * rhs.z - self.xy * rhs.x;
        let z = self.zx * rhs.x - self.yz * rhs.y;

        (
            Scalar(x * x + y * y + z * z),
            Trivector {
                xyz: ((self.yz * self.yz + self.zx * self.zx + self.xy * self.xy)
                    * (rhs.x * rhs.x + rhs.y * rhs.y + rhs.z * rhs.z))
                    .sqrt(),
                _metric: PhantomData,
            },
        )
    }

    fn cosine(&self, rhs: &Vector<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        Some(Scalar(geometric.0.0 / geometric.1.xyz))
    }
}

impl<T> Angle<DirVector<T>> for DirBivector<T>
where
    T: ConstOne,
    T: Float,
{
    type Scalar = Scalar<T>;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &DirVector<T>) -> (Self::Scalar, Self::Antiscalar) {
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

    fn cosine(&self, rhs: &DirVector<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        Some(Scalar(geometric.0.0))
    }
}

impl<T> Angle<Bivector<T>> for Vector<T>
where
    T: Float,
{
    type Scalar = Scalar<T>;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &Bivector<T>) -> (Self::Scalar, Self::Antiscalar) {
        rhs.geometric_cosine(self)
    }

    fn cosine(&self, rhs: &Bivector<T>) -> Option<Self::Scalar> {
        rhs.cosine(self)
    }
}

impl<T> Angle<DirBivector<T>> for DirVector<T>
where
    T: ConstOne,
    T: Float,
{
    type Scalar = Scalar<T>;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &DirBivector<T>) -> (Self::Scalar, Self::Antiscalar) {
        rhs.geometric_cosine(self)
    }

    fn cosine(&self, rhs: &DirBivector<T>) -> Option<Self::Scalar> {
        rhs.cosine(self)
    }
}

impl<T> Angle<Bivector<T>> for Bivector<T>
where
    T: Float,
{
    type Scalar = Scalar<T>;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &Bivector<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            Scalar(self.yz * rhs.yz + self.zx * rhs.zx + self.xy * rhs.xy),
            Trivector {
                xyz: ((self.yz * self.yz + self.zx * self.zx + self.xy * self.xy)
                    * (rhs.yz * rhs.yz + rhs.zx * rhs.zx + rhs.xy * rhs.xy))
                    .sqrt(),
                _metric: PhantomData,
            },
        )
    }

    fn cosine(&self, rhs: &Bivector<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        Some(Scalar(geometric.0.0 / geometric.1.xyz))
    }
}

impl<T> Angle<DirBivector<T>> for DirBivector<T>
where
    T: ConstOne,
    T: Float,
{
    type Scalar = Scalar<T>;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &DirBivector<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            Scalar(self.0.yz * rhs.0.yz + self.0.zx * rhs.0.zx + self.0.xy * rhs.0.xy),
            Trivector {
                xyz: T::ONE,
                _metric: PhantomData,
            },
        )
    }

    fn cosine(&self, rhs: &DirBivector<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        Some(Scalar(geometric.0.0))
    }
}
