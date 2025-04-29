use num::traits::{ConstOne, ConstZero};

use crate::geometry3d as d3;

use super::{HomogeneusLine, HomogeneusPlane, HomogeneusPoint, HorizonLine, Line, Plane};

impl<T> From<d3::Point<T>> for HomogeneusPoint<T>
where
    T: ConstOne,
{
    fn from(value: d3::Point<T>) -> Self {
        HomogeneusPoint {
            x: value.0.x,
            y: value.0.y,
            z: value.0.z,
            w: T::ONE,
        }
    }
}

impl<T> From<d3::Vector<T>> for HomogeneusPoint<T>
where
    T: ConstZero,
{
    fn from(value: d3::Vector<T>) -> Self {
        HomogeneusPoint {
            x: value.x,
            y: value.y,
            z: value.z,
            w: T::ZERO,
        }
    }
}

impl<T> From<Line<T>> for HomogeneusLine<T> {
    fn from(value: Line<T>) -> Self {
        value.0
    }
}

impl<T> From<HorizonLine<T>> for HomogeneusLine<T>
where
    T: ConstZero,
{
    fn from(value: HorizonLine<T>) -> Self {
        HomogeneusLine {
            wx: T::ZERO,
            wy: T::ZERO,
            wz: T::ZERO,
            yz: value.0.yz,
            zx: value.0.zx,
            xy: value.0.xy,
        }
    }
}

impl<T> From<Plane<T>> for HomogeneusPlane<T> {
    fn from(value: Plane<T>) -> Self {
        value.0
    }
}
