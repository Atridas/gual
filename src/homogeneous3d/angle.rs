use num::Float;
use num::traits::ConstOne;

use crate::Angle;

use crate::BulkAndWeight;
use crate::geometry3d as d3;
use crate::geometry4d as d4;

use super::HomogeneusLine;
use super::HomogeneusPlane;
use super::Line;
use super::Plane;

impl<T> Angle<HomogeneusLine<T>> for HomogeneusLine<T>
where
    T: Float,
    HomogeneusLine<T>: BulkAndWeight<Weight = d3::Vector<T>>,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn geometric_cosine(&self, rhs: &HomogeneusLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        let dim3 = self.weight().geometric_cosine(&rhs.weight());
        (d4::Scalar(dim3.0.0), d4::Quadvector { xyzw: dim3.1.xyz })
    }

    fn cosine(&self, rhs: &HomogeneusLine<T>) -> Self::Scalar {
        let geometric = self.geometric_cosine(rhs);
        d4::Scalar(geometric.0.0 / geometric.1.xyzw)
    }
}

impl<T> Angle<Line<T>> for Line<T>
where
    T: ConstOne,
    T: Float,
    Line<T>: BulkAndWeight<Weight = d3::DirVector<T>>,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn geometric_cosine(&self, rhs: &Line<T>) -> (Self::Scalar, Self::Antiscalar) {
        let dim3 = self.weight().cosine(&rhs.weight());
        (d4::Scalar(dim3.0), d4::Quadvector { xyzw: T::ONE })
    }

    fn cosine(&self, rhs: &Line<T>) -> Self::Scalar {
        let geometric = self.geometric_cosine(rhs);
        d4::Scalar(geometric.0.0)
    }
}

impl<T> Angle<HomogeneusLine<T>> for HomogeneusPlane<T>
where
    T: Float,
    HomogeneusLine<T>: BulkAndWeight<Weight = d3::Vector<T>>,
    HomogeneusPlane<T>: BulkAndWeight<Weight = d3::Bivector<T>>,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn geometric_cosine(&self, rhs: &HomogeneusLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        let dim3 = self.weight().geometric_cosine(&rhs.weight());
        (d4::Scalar(dim3.0.0), d4::Quadvector { xyzw: dim3.1.xyz })
    }

    fn cosine(&self, rhs: &HomogeneusLine<T>) -> Self::Scalar {
        let geometric = self.geometric_cosine(rhs);
        d4::Scalar(geometric.0.0 / geometric.1.xyzw)
    }
}

impl<T> Angle<Line<T>> for Plane<T>
where
    T: ConstOne,
    T: Float,
    Line<T>: BulkAndWeight<Weight = d3::DirVector<T>>,
    Plane<T>: BulkAndWeight<Weight = d3::DirBivector<T>>,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn geometric_cosine(&self, rhs: &Line<T>) -> (Self::Scalar, Self::Antiscalar) {
        let dim3 = self.weight().cosine(&rhs.weight());
        (d4::Scalar(dim3.0), d4::Quadvector { xyzw: T::ONE })
    }

    fn cosine(&self, rhs: &Line<T>) -> Self::Scalar {
        let geometric = self.geometric_cosine(rhs);
        d4::Scalar(geometric.0.0 / geometric.1.xyzw)
    }
}

impl<T> Angle<HomogeneusPlane<T>> for HomogeneusPlane<T>
where
    T: Float,
    HomogeneusPlane<T>: BulkAndWeight<Weight = d3::Bivector<T>>,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn geometric_cosine(&self, rhs: &HomogeneusPlane<T>) -> (Self::Scalar, Self::Antiscalar) {
        let dim3 = self.weight().geometric_cosine(&rhs.weight());
        (d4::Scalar(dim3.0.0), d4::Quadvector { xyzw: dim3.1.xyz })
    }

    fn cosine(&self, rhs: &HomogeneusPlane<T>) -> Self::Scalar {
        let geometric = self.geometric_cosine(rhs);
        d4::Scalar(geometric.0.0 / geometric.1.xyzw)
    }
}

impl<T> Angle<Plane<T>> for Plane<T>
where
    T: ConstOne,
    T: Float,
    Plane<T>: BulkAndWeight<Weight = d3::DirBivector<T>>,
{
    type Scalar = d4::Scalar<T>;
    type Antiscalar = d4::Quadvector<T>;

    fn geometric_cosine(&self, rhs: &Plane<T>) -> (Self::Scalar, Self::Antiscalar) {
        let dim3 = self.weight().cosine(&rhs.weight());
        (d4::Scalar(dim3.0), d4::Quadvector { xyzw: T::ONE })
    }

    fn cosine(&self, rhs: &Plane<T>) -> Self::Scalar {
        let geometric = self.weight().cosine(&rhs.weight());
        d4::Scalar(geometric.0)
    }
}
