use num::Float;
use num::traits::ConstOne;
use num::traits::ConstZero;

use crate::Antiscalar;
use crate::BulkAndWeight;
use crate::Distance;
use crate::Dot;
use crate::Norm;
use crate::WedgeProduct;
use crate::geometry4d::Quadvector;

use super::HomogeneusLine;
use super::HomogeneusPlane;
use super::HomogeneusPoint;
use super::Line;
use super::Plane;
use crate::geometry3d as d3;
use crate::geometry4d as d4;

impl<T> Distance<HomogeneusPoint<T>> for HomogeneusPoint<T>
where
    T: Float,
    T: ConstOne,
{
    type T = T;

    fn geometric_distance(&self, rhs: &HomogeneusPoint<T>) -> (Self::Scalar, Self::Antiscalar) {
        let dist = *self - *rhs;
        dist.geometric_dot(&dist)
    }

    fn distance(&self, rhs: &HomogeneusPoint<T>) -> Self::T {
        let dot = self.geometric_dot(rhs);
        dot.0.0 / dot.1.xyzw
    }
}

impl<T> Distance<d3::Point<T>> for d3::Point<T>
where
    T: Float,
    T: ConstOne,
    d3::Vector<T>: Dot<Scalar = Self::Scalar, Antiscalar = Self::Antiscalar>,
{
    type T = T;

    fn geometric_distance(&self, rhs: &d3::Point<T>) -> (Self::Scalar, Self::Antiscalar) {
        let dist = *self - *rhs;
        (dist.dot(&dist), Quadvector::UNIT_VOLUME)
    }

    fn distance(&self, rhs: &d3::Point<T>) -> Self::T {
        self.dot(rhs).0
    }
}

impl<T> Distance<HomogeneusLine<T>> for HomogeneusPoint<T>
where
    T: Float,
    T: ConstZero,
    T: ConstOne,
    HomogeneusPoint<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
    HomogeneusLine<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
{
    type T = T;

    fn geometric_distance(&self, rhs: &HomogeneusLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        let a = self.bulk().wedge(rhs.weight());
        let b = self.weight().wedge(rhs.bulk());
        let bulk = a + b;
        let weight = self.weight().wedge(rhs.weight());
        let bulk_norm = (bulk.yz * bulk.yz + bulk.zx * bulk.zx + bulk.xy * bulk.xy).sqrt();
        let weight_norm = (weight.x * weight.x + weight.y * weight.y + weight.z * weight.z).sqrt();
        (d4::Scalar(bulk_norm), d4::Quadvector { xyzw: weight_norm })
    }

    fn distance(&self, rhs: &HomogeneusLine<T>) -> Self::T {
        let geometric_distance = self.geometric_distance(rhs);
        geometric_distance.0.0 / geometric_distance.1.xyzw
    }
}

impl<T> Distance<HomogeneusPoint<T>> for HomogeneusLine<T>
where
    T: Float,
    T: ConstZero,
    T: ConstOne,
    HomogeneusPoint<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
    HomogeneusLine<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
{
    type T = T;

    fn geometric_distance(&self, rhs: &HomogeneusPoint<T>) -> (Self::Scalar, Self::Antiscalar) {
        rhs.geometric_distance(self)
    }

    fn distance(&self, rhs: &HomogeneusPoint<T>) -> Self::T {
        rhs.distance(self)
    }
}

impl<T> Distance<Line<T>> for d3::Point<T>
where
    T: Float,
    T: ConstZero,
    T: ConstOne,
    d3::Point<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
    Line<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
{
    type T = T;

    fn geometric_distance(&self, rhs: &Line<T>) -> (Self::Scalar, Self::Antiscalar) {
        let a = self.bulk().wedge(rhs.weight());
        let b = self.weight().wedge(rhs.bulk());
        let bulk = a + b;
        let bulk_norm = (bulk.yz * bulk.yz + bulk.zx * bulk.zx + bulk.xy * bulk.xy).sqrt();
        (d4::Scalar(bulk_norm), d4::Quadvector { xyzw: T::ONE })
    }

    fn distance(&self, rhs: &Line<T>) -> Self::T {
        let geometric_distance = self.geometric_distance(rhs);
        geometric_distance.0.0
    }
}

impl<T> Distance<d3::Point<T>> for Line<T>
where
    T: Float,
    T: ConstZero,
    T: ConstOne,
    d3::Point<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
    Line<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
{
    type T = T;

    fn geometric_distance(&self, rhs: &d3::Point<T>) -> (Self::Scalar, Self::Antiscalar) {
        rhs.geometric_distance(self)
    }

    fn distance(&self, rhs: &d3::Point<T>) -> Self::T {
        rhs.distance(self)
    }
}

impl<T> Distance<HomogeneusPlane<T>> for HomogeneusPoint<T>
where
    T: Float,
    T: ConstZero,
    T: ConstOne,
    HomogeneusPoint<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
    HomogeneusPlane<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
{
    type T = T;

    fn geometric_distance(&self, rhs: &HomogeneusPlane<T>) -> (Self::Scalar, Self::Antiscalar) {
        let signed_distance =
            self.x * rhs.wyz + self.y * rhs.wzx + self.z * rhs.wxy + self.w * rhs.zyx;
        let weight = self.w * (rhs.wyz * rhs.wyz + rhs.wzx * rhs.wzx + rhs.wxy * rhs.wxy).sqrt();
        (d4::Scalar(signed_distance), d4::Quadvector { xyzw: weight })
    }

    fn distance(&self, rhs: &HomogeneusPlane<T>) -> Self::T {
        let geometric_distance = self.geometric_distance(rhs);
        geometric_distance.0.0 / geometric_distance.1.xyzw
    }
}

impl<T> Distance<HomogeneusPoint<T>> for HomogeneusPlane<T>
where
    T: Float,
    T: ConstZero,
    T: ConstOne,
    HomogeneusPoint<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
    HomogeneusPlane<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
{
    type T = T;

    fn geometric_distance(&self, rhs: &HomogeneusPoint<T>) -> (Self::Scalar, Self::Antiscalar) {
        rhs.geometric_distance(self)
    }

    fn distance(&self, rhs: &HomogeneusPoint<T>) -> Self::T {
        rhs.distance(self)
    }
}

impl<T> Distance<Plane<T>> for d3::Point<T>
where
    T: Float,
    T: ConstZero,
    T: ConstOne,
    d3::Point<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
    Plane<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
{
    type T = T;

    fn geometric_distance(&self, rhs: &Plane<T>) -> (Self::Scalar, Self::Antiscalar) {
        let signed_distance =
            self.0.x * rhs.0.wyz + self.0.y * rhs.0.wzx + self.0.z * rhs.0.wxy + rhs.0.zyx;
        (d4::Scalar(signed_distance), d4::Quadvector { xyzw: T::ONE })
    }

    fn distance(&self, rhs: &Plane<T>) -> Self::T {
        let geometric_distance = self.geometric_distance(rhs);
        geometric_distance.0.0
    }
}

impl<T> Distance<d3::Point<T>> for Plane<T>
where
    T: Float,
    T: ConstZero,
    T: ConstOne,
    d3::Point<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
    Plane<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
{
    type T = T;

    fn geometric_distance(&self, rhs: &d3::Point<T>) -> (Self::Scalar, Self::Antiscalar) {
        let signed_distance =
            self.0.wyz * rhs.0.x + self.0.wzx * rhs.0.y + self.0.wxy * rhs.0.z + self.0.zyx;
        (
            d4::Scalar(-signed_distance),
            d4::Quadvector { xyzw: T::ONE },
        )
    }

    fn distance(&self, rhs: &d3::Point<T>) -> Self::T {
        let geometric_distance = self.geometric_distance(rhs);
        geometric_distance.0.0
    }
}

impl<T> Distance<HomogeneusLine<T>> for HomogeneusLine<T>
where
    T: Float,
    T: ConstZero,
    T: ConstOne,
    HomogeneusLine<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
{
    type T = T;

    fn geometric_distance(&self, rhs: &HomogeneusLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        let signed_distance = -(self.wx * rhs.yz
            + self.wy * rhs.zx
            + self.wz * rhs.xy
            + self.yz * rhs.wx
            + self.zx * rhs.wy
            + self.xy * rhs.wz);
        let weight = self.weight().wedge(rhs.weight());
        let weight_norm =
            (weight.yz * weight.yz + weight.zx * weight.zx + weight.xy * weight.xy).sqrt();

        (
            d4::Scalar(signed_distance),
            d4::Quadvector { xyzw: weight_norm },
        )
    }

    fn distance(&self, rhs: &HomogeneusLine<T>) -> Self::T {
        let geometric_distance = self.geometric_distance(rhs);
        geometric_distance.0.0 / geometric_distance.1.xyzw
    }
}

impl<T> Distance<Line<T>> for Line<T>
where
    T: Float,
    T: ConstZero,
    T: ConstOne,
    Line<T>: Norm<Scalar = d4::Scalar<T>, Antiscalar = d4::Quadvector<T>>,
{
    type T = T;

    fn geometric_distance(&self, rhs: &Line<T>) -> (Self::Scalar, Self::Antiscalar) {
        let signed_distance = -(self.0.wx * rhs.0.yz
            + self.0.wy * rhs.0.zx
            + self.0.wz * rhs.0.xy
            + self.0.yz * rhs.0.wx
            + self.0.zx * rhs.0.wy
            + self.0.xy * rhs.0.wz);

        (d4::Scalar(signed_distance), d4::Quadvector { xyzw: T::ONE })
    }

    fn distance(&self, rhs: &Line<T>) -> Self::T {
        let geometric_distance = self.geometric_distance(rhs);
        geometric_distance.0.0
    }
}
