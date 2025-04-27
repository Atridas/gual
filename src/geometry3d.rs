mod bivector3d;
mod multivector3d;
mod scalar3d;
mod trivector3d;
mod vector3d;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bivector<T> {
    pub yz: T,
    pub zx: T,
    pub xy: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Trivector<T> {
    pub xyz: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multivector<T> {
    pub s: Scalar<T>,
    pub v: Vector<T>,
    pub b: Bivector<T>,
    pub a: Trivector<T>,
}
