use std::ops::{Mul, Neg, Sub};

use super::{DirVector, Line, Point, UnitLine, UnitVector};
use crate::WedgeProduct;

impl<T> WedgeProduct<Point<T>> for Point<T>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Line<T>;
    fn wedge(&self, rhs: &Point<T>) -> Self::Output {
        Line::new(
            self.0.y - rhs.0.y,
            rhs.0.x - self.0.x,
            self.0.x * rhs.0.y - self.0.y * rhs.0.x,
        )
    }
}

impl<T> WedgeProduct<DirVector<T>> for Point<T>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Line<T>;
    fn wedge(&self, rhs: &DirVector<T>) -> Self::Output {
        Line::new(-rhs.y, rhs.x, self.0.x * rhs.y - self.0.y * rhs.x)
    }
}

impl<T> WedgeProduct<UnitVector<T>> for Point<T>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = UnitLine<T>;
    fn wedge(&self, rhs: &UnitVector<T>) -> Self::Output {
        UnitLine(Line::new(
            -rhs.0.y,
            rhs.0.x,
            self.0.x * rhs.0.y - self.0.y * rhs.0.x,
        ))
    }
}

impl<T> WedgeProduct<Point<T>> for DirVector<T>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = Line<T>;
    fn wedge(&self, rhs: &Point<T>) -> Self::Output {
        Line::new(self.y, -self.x, self.x * rhs.0.y - self.y * rhs.0.x)
    }
}

impl<T> WedgeProduct<Point<T>> for UnitVector<T>
where
    T: Copy,
    T: Sub<Output = T>,
    T: Neg<Output = T>,
    T: Mul<Output = T>,
{
    type Output = UnitLine<T>;
    fn wedge(&self, rhs: &Point<T>) -> Self::Output {
        UnitLine(Line::new(
            self.0.y,
            -self.0.x,
            self.0.x * rhs.0.y - self.0.y * rhs.0.x,
        ))
    }
}
