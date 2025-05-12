use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use super::HomogeneusLine;
use super::HomogeneusPlane;
use super::HomogeneusPoint;

use crate::KVector;
use crate::geometry4d as d4;
use crate::{AntiwedgeProduct, Meet};

impl<T> Meet<HomogeneusPlane<T>> for HomogeneusPlane<T>
where
    d4::Trivector<T>: AntiwedgeProduct<d4::Trivector<T>, Output = d4::Bivector<T>>,
{
    type Output = HomogeneusLine<T>;
    fn meet(&self, rhs: &HomogeneusPlane<T>) -> Self::Output {
        self.antiwedge(rhs)
    }
}

impl<T> Meet<HomogeneusLine<T>> for HomogeneusPlane<T>
where
    d4::Trivector<T>: AntiwedgeProduct<d4::Bivector<T>, Output = d4::Vector<T>>,
{
    type Output = HomogeneusPoint<T>;
    fn meet(&self, rhs: &HomogeneusLine<T>) -> Self::Output {
        self.antiwedge(rhs)
    }
}

impl<T> Meet<HomogeneusPlane<T>> for HomogeneusLine<T>
where
    d4::Bivector<T>: AntiwedgeProduct<d4::Trivector<T>, Output = d4::Vector<T>>,
{
    type Output = HomogeneusPoint<T>;
    fn meet(&self, rhs: &HomogeneusPlane<T>) -> Self::Output {
        self.antiwedge(rhs)
    }
}

impl<T> HomogeneusPlane<T>
where
    T: Copy,
    T: Mul<T, Output = T>,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Neg<Output = T>,
{
    pub fn meet(a: Self, b: Self, c: Self) -> HomogeneusPoint<T> {
        // TODO check sign of this
        HomogeneusPoint::join(
            a.left_complement(),
            b.left_complement(),
            c.left_complement(),
        )
        .left_complement()
    }
}
