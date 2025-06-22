use std::ops::{Mul, Neg, Sub};

use num::{
    Float,
    traits::{ConstOne, ConstZero},
};

use crate::{
    Epsilon, WedgeProduct,
    projective2d::{DirVector, Line, ParametricLine, Point, UnitLine, UnitVector},
};

type Vector3<T> = crate::geometry3d::Vector<T, crate::Projective>;

impl<T: ConstOne> From<Point<T>> for Vector3<T> {
    fn from(value: Point<T>) -> Self {
        Vector3::new(value.0.x, value.0.y, T::ONE)
    }
}

impl<T: ConstOne + Copy> From<&Point<T>> for Vector3<T> {
    fn from(value: &Point<T>) -> Self {
        Vector3::new(value.0.x, value.0.y, T::ONE)
    }
}

impl<T: ConstZero> From<DirVector<T>> for Vector3<T> {
    fn from(value: DirVector<T>) -> Self {
        Vector3::new(value.x, value.y, T::ZERO)
    }
}

impl<T: ConstZero + Copy> From<&DirVector<T>> for Vector3<T> {
    fn from(value: &DirVector<T>) -> Self {
        Vector3::new(value.x, value.y, T::ZERO)
    }
}

impl<T: ConstZero> From<UnitVector<T>> for Vector3<T> {
    fn from(value: UnitVector<T>) -> Self {
        let value: DirVector<T> = value.into();
        value.into()
    }
}

impl<T: ConstZero + Copy> From<&UnitVector<T>> for Vector3<T> {
    fn from(value: &UnitVector<T>) -> Self {
        let value: DirVector<T> = value.into();
        value.into()
    }
}

impl<T> Line<T>
where
    T: Copy,
    T: ConstOne,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    pub fn line_from_points(a: &Point<T>, b: &Point<T>) -> Line<T> {
        let a: Vector3<T> = a.into();
        let b: Vector3<T> = b.into();
        a.wedge(&b)
    }
}

impl<T> From<UnitLine<T>> for Line<T> {
    fn from(value: UnitLine<T>) -> Self {
        value.0
    }
}

impl<T> TryFrom<Line<T>> for UnitLine<T>
where
    T: Float,
    T: Epsilon,
{
    type Error = ();
    fn try_from(value: Line<T>) -> Result<Self, Self::Error> {
        let len2 = value.yz * value.yz + value.zx * value.zx;
        if len2.is_near_zero() {
            Err(())
        } else {
            Ok(UnitLine(value * len2.sqrt().recip()))
        }
    }
}

impl<T> From<ParametricLine<T>> for UnitLine<T>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    fn from(value: ParametricLine<T>) -> Self {
        let dir: DirVector<T> = value.dir.into();
        let origin = value.origin.0;
        let yz = -dir.y;
        let zx = dir.x;
        let xy = origin.y * dir.x - origin.x * dir.y;
        UnitLine(Line::new(yz, zx, xy))
    }
}
