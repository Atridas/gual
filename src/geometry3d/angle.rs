use std::marker::PhantomData;

use num::{Float, Zero, traits::ConstOne};

use crate::{Angle, Projective};

use super::{Bivector, Trivector, Vector};

impl<T> Angle<Vector<T>> for Vector<T>
where
    T: Float,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &Vector<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.x * rhs.x + self.y * rhs.y + self.z * rhs.z,
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
        Some(geometric.0 / geometric.1.xyz)
    }
}

impl<T> Angle<Vector<T>> for Bivector<T>
where
    T: Float,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &Vector<T>) -> (Self::Scalar, Self::Antiscalar) {
        let x = self.xy * rhs.y - self.zx * rhs.z;
        let y = self.yz * rhs.z - self.xy * rhs.x;
        let z = self.zx * rhs.x - self.yz * rhs.y;

        (
            x * x + y * y + z * z,
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
        Some(geometric.0 / geometric.1.xyz)
    }
}

impl<T> Angle<Bivector<T>> for Vector<T>
where
    T: Float,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &Bivector<T>) -> (Self::Scalar, Self::Antiscalar) {
        rhs.geometric_cosine(self)
    }

    fn cosine(&self, rhs: &Bivector<T>) -> Option<Self::Scalar> {
        rhs.cosine(self)
    }
}

impl<T> Angle<Bivector<T>> for Bivector<T>
where
    T: Float,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &Bivector<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.yz * rhs.yz + self.zx * rhs.zx + self.xy * rhs.xy,
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
        Some(geometric.0 / geometric.1.xyz)
    }
}

// ----------------------------------------------------------------------------------------------------
// Projective
// ----------------------------------------------------------------------------------------------------

impl<T> Angle<Vector<T, Projective>> for Vector<T, Projective>
where
    T: Float,
    T: ConstOne,
    T: Zero,
    T: PartialEq,
{
    type Scalar = T;
    type Antiscalar = Trivector<T, Projective>;

    fn geometric_cosine(&self, rhs: &Vector<T, Projective>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.z * rhs.z,
            Trivector {
                xyz: self.z.abs() * rhs.z.abs(),
                _metric: PhantomData,
            },
        )
    }

    fn cosine(&self, rhs: &Vector<T, Projective>) -> Option<Self::Scalar> {
        if self.z.is_zero() || rhs.z.is_zero() {
            None
        } else if self.z.signum().is_one() {
            Some(rhs.z.signum())
        } else {
            Some(-rhs.z.signum())
        }
    }
}

impl<T> Angle<Vector<T, Projective>> for Bivector<T, Projective>
where
    T: Float,
{
    type Scalar = T;
    type Antiscalar = Trivector<T, Projective>;

    fn geometric_cosine(&self, rhs: &Vector<T, Projective>) -> (Self::Scalar, Self::Antiscalar) {
        let x = -self.zx * rhs.z;
        let y = self.yz * rhs.z;

        (
            x * x + y * y,
            Trivector {
                xyz: ((self.yz * self.yz + self.zx * self.zx).sqrt() * rhs.z.abs()),
                _metric: PhantomData,
            },
        )
    }

    fn cosine(&self, rhs: &Vector<T, Projective>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        Some(geometric.0 / geometric.1.xyz)
    }
}

impl<T> Angle<Bivector<T, Projective>> for Vector<T, Projective>
where
    T: Float,
{
    type Scalar = T;
    type Antiscalar = Trivector<T, Projective>;

    fn geometric_cosine(&self, rhs: &Bivector<T, Projective>) -> (Self::Scalar, Self::Antiscalar) {
        rhs.geometric_cosine(self)
    }

    fn cosine(&self, rhs: &Bivector<T, Projective>) -> Option<Self::Scalar> {
        rhs.cosine(self)
    }
}

impl<T> Angle<Bivector<T, Projective>> for Bivector<T, Projective>
where
    T: Float,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &Bivector<T, Projective>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.yz * rhs.yz + self.zx * rhs.zx,
            Trivector {
                xyz: ((self.yz * self.yz + self.zx * self.zx)
                    * (rhs.yz * rhs.yz + rhs.zx * rhs.zx))
                    .sqrt(),
                _metric: PhantomData,
            },
        )
    }

    fn cosine(&self, rhs: &Bivector<T, Projective>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        Some(geometric.0 / geometric.1.xyz)
    }
}
