mod bivector2d;
mod multivector2d;
mod scalar2d;
mod vector2d;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar2D<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bivector2D<T> {
    pub xy: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multivector2D<T> {
    pub s: Scalar2D<T>,
    pub v: Vector2D<T>,
    pub a: Bivector2D<T>,
}
