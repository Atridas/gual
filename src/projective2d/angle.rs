use std::ops::{Add, Mul, Sub};

use num::{Float, traits::ConstOne};

use crate::{
    Angle, Epsilon,
    geometry3d::Trivector,
    projective2d::{DirVector, Line, ParametricLine, UnitLine, UnitVector},
    reverse_angle,
};

// ----------------------------------------------------------------------------------------------------
// DirVector
// ----------------------------------------------------------------------------------------------------

impl<T> Angle<UnitVector<T>> for DirVector<T>
where
    T: Float,
    T: Epsilon,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &UnitVector<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.x * rhs.0.x + self.y * rhs.0.y,
            Trivector::new((self.x * self.x + self.y * self.y).sqrt()),
        )
    }

    fn cosine(&self, rhs: &UnitVector<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        if geometric.1.xyz.is_near_zero() {
            None
        } else {
            Some(geometric.0 / geometric.1.xyz)
        }
    }
}

impl<T> Angle<UnitLine<T>> for DirVector<T>
where
    T: Float,
    T: Epsilon,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &UnitLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.x * rhs.0.zx - self.y * rhs.0.yz,
            Trivector::new((self.x * self.x + self.y * self.y).sqrt()),
        )
    }

    fn cosine(&self, rhs: &UnitLine<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        if geometric.1.xyz.is_near_zero() {
            None
        } else {
            Some(geometric.0 / geometric.1.xyz)
        }
    }
}

impl<T> Angle<ParametricLine<T>> for DirVector<T>
where
    T: Float,
    T: Epsilon,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &ParametricLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.x * rhs.dir.0.x + self.y * rhs.dir.0.y,
            Trivector::new((self.x * self.x + self.y * self.y).sqrt()),
        )
    }

    fn cosine(&self, rhs: &ParametricLine<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        if geometric.1.xyz.is_near_zero() {
            None
        } else {
            Some(geometric.0 / geometric.1.xyz)
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// UnitVector
// ----------------------------------------------------------------------------------------------------

reverse_angle!(UnitVector<T>, DirVector<T>);

impl<T> Angle<UnitVector<T>> for UnitVector<T>
where
    T: Copy,
    T: ConstOne,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &UnitVector<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.0.x * rhs.0.x + self.0.y * rhs.0.y,
            Trivector::new(T::ONE),
        )
    }

    fn cosine(&self, rhs: &UnitVector<T>) -> Option<Self::Scalar> {
        Some(self.0.x * rhs.0.x + self.0.y * rhs.0.y)
    }
}

impl<T> Angle<UnitLine<T>> for UnitVector<T>
where
    T: Copy,
    T: ConstOne,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &UnitLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.0.x * rhs.0.zx - self.0.y * rhs.0.yz,
            Trivector::new(T::ONE),
        )
    }

    fn cosine(&self, rhs: &UnitLine<T>) -> Option<Self::Scalar> {
        Some(self.0.x * rhs.0.zx - self.0.y * rhs.0.yz)
    }
}

impl<T> Angle<ParametricLine<T>> for UnitVector<T>
where
    T: Copy,
    T: ConstOne,
    T: Add<Output = T>,
    T: Mul<Output = T>,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &ParametricLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.0.x * rhs.dir.0.x + self.0.y * rhs.dir.0.y,
            Trivector::new(T::ONE),
        )
    }

    fn cosine(&self, rhs: &ParametricLine<T>) -> Option<Self::Scalar> {
        Some(self.0.x * rhs.dir.0.x + self.0.y * rhs.dir.0.y)
    }
}

// ----------------------------------------------------------------------------------------------------
// Line
// ----------------------------------------------------------------------------------------------------

reverse_angle!(Line<T>, DirVector<T>);
reverse_angle!(Line<T>, UnitVector<T>);

impl<T> Angle<UnitLine<T>> for Line<T>
where
    T: Float,
    T: Epsilon,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &UnitLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.yz * rhs.0.yz + self.zx * rhs.0.zx,
            Trivector::new((self.yz * self.yz + self.zx * self.zx).sqrt()),
        )
    }

    fn cosine(&self, rhs: &UnitLine<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        if geometric.1.xyz.is_near_zero() {
            None
        } else {
            Some(geometric.0 / geometric.1.xyz)
        }
    }
}

impl<T> Angle<ParametricLine<T>> for Line<T>
where
    T: Float,
    T: Epsilon,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &ParametricLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.zx * rhs.dir.0.x - self.yz * rhs.dir.0.y,
            Trivector::new((self.yz * self.yz + self.zx * self.zx).sqrt()),
        )
    }

    fn cosine(&self, rhs: &ParametricLine<T>) -> Option<Self::Scalar> {
        let geometric = self.geometric_cosine(rhs);
        if geometric.1.xyz.is_near_zero() {
            None
        } else {
            Some(geometric.0 / geometric.1.xyz)
        }
    }
}

// ----------------------------------------------------------------------------------------------------
// UnitLine
// ----------------------------------------------------------------------------------------------------

reverse_angle!(UnitLine<T>, DirVector<T>);
reverse_angle!(UnitLine<T>, UnitVector<T>);
reverse_angle!(UnitLine<T>, Line<T>);

impl<T> Angle<UnitLine<T>> for UnitLine<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Mul<Output = T>,
    T: ConstOne,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &UnitLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.0.yz * rhs.0.yz + self.0.zx * rhs.0.zx,
            Trivector::new(T::ONE),
        )
    }

    fn cosine(&self, rhs: &UnitLine<T>) -> Option<Self::Scalar> {
        Some(self.0.yz * rhs.0.yz + self.0.zx * rhs.0.zx)
    }
}

impl<T> Angle<ParametricLine<T>> for UnitLine<T>
where
    T: Float,
    T: ConstOne,
{
    type Scalar = T;
    type Antiscalar = Trivector<T>;

    fn geometric_cosine(&self, rhs: &ParametricLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        (
            self.0.zx * rhs.dir.0.x - self.0.yz * rhs.dir.0.y,
            Trivector::new(T::ONE),
        )
    }

    fn cosine(&self, rhs: &ParametricLine<T>) -> Option<Self::Scalar> {
        Some(self.0.zx * rhs.dir.0.x - self.0.yz * rhs.dir.0.y)
    }
}

// ----------------------------------------------------------------------------------------------------
// ParametricLine
// ----------------------------------------------------------------------------------------------------

reverse_angle!(ParametricLine<T>, DirVector<T>);
reverse_angle!(ParametricLine<T>, UnitVector<T>);
reverse_angle!(ParametricLine<T>, Line<T>);
reverse_angle!(ParametricLine<T>, UnitLine<T>);

impl<T> Angle<ParametricLine<T>> for ParametricLine<T>
where
    UnitVector<T>: Angle<UnitVector<T>>,
{
    type Scalar = <UnitVector<T> as Angle<UnitVector<T>>>::Scalar;
    type Antiscalar = <UnitVector<T> as Angle<UnitVector<T>>>::Antiscalar;

    fn geometric_cosine(&self, rhs: &ParametricLine<T>) -> (Self::Scalar, Self::Antiscalar) {
        self.dir.geometric_cosine(&rhs.dir)
    }

    fn cosine(&self, rhs: &ParametricLine<T>) -> Option<Self::Scalar> {
        self.dir.cosine(&rhs.dir)
    }
}
