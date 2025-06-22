use num::traits::{ConstOne, ConstZero};

mod geometric_product;

type Point<T> = crate::geometry2d::Point<T>;
type DirVector<T> = crate::geometry2d::Vector<T>;
type UnitVector<T> = crate::geometry2d::UnitVector<T>;

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
