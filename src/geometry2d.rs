mod bivector2d;
mod multivector2d;
mod scalar2d;
mod vector2d;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bivector<T> {
    pub xy: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multivector<T> {
    pub s: Scalar<T>,
    pub v: Vector<T>,
    pub a: Bivector<T>,
}
