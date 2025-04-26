// mod bivector4d;
// mod multivector4d;
mod scalar4d;
// mod trivector4d;
mod vector4d;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar4D<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector4D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bivector4D<T> {
    pub wx: T,
    pub wy: T,
    pub wz: T,
    pub yz: T,
    pub zx: T,
    pub xy: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trivector4D<T> {
    pub wyz: T,
    pub wzx: T,
    pub wxy: T,
    pub zyx: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quadvector4D<T> {
    pub xyzw: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multivector4D<T> {
    pub s: Scalar4D<T>,
    pub v: Vector4D<T>,
    pub b: Bivector4D<T>,
    pub t: Trivector4D<T>,
    pub a: Quadvector4D<T>,
}
