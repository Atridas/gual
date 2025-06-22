mod geometric_product;
mod initialization;

pub type Point<T> = crate::geometry2d::Point<T>;
pub type DirVector<T> = crate::geometry2d::Vector<T>;
pub type UnitVector<T> = crate::geometry2d::UnitVector<T>;

pub type Line<T> = crate::geometry3d::Bivector<T, crate::Projective>;

pub struct UnitLine<T>(Line<T>);

pub struct ParametricLine<T> {
    pub origin: Point<T>,
    pub dir: UnitVector<T>,
}
