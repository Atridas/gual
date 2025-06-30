mod add;
mod div;
mod geometric_product;
mod initialization;
mod mul;
mod neg;
mod norm;
mod sub;
mod unitize;
mod wedge;

pub type DirVector<T> = crate::geometry2d::Vector<T>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitVector<T>(crate::geometry2d::Vector<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T>(pub crate::geometry2d::Vector<T>);

pub type Line<T> = crate::geometry3d::Bivector<T, crate::Projective>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitLine<T>(Line<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParametricLine<T> {
    pub origin: Point<T>,
    pub dir: UnitVector<T>,
}
