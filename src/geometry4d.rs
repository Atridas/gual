// mod bivector4d;
// mod multivector4d;
mod scalar4d;
// mod trivector4d;
mod vector4d;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bivector<T> {
    pub wx: T,
    pub wy: T,
    pub wz: T,
    pub yz: T,
    pub zx: T,
    pub xy: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trivector<T> {
    pub wyz: T,
    pub wzx: T,
    pub wxy: T,
    pub zyx: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quadvector<T> {
    pub xyzw: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multivector<T> {
    pub s: Scalar<T>,
    pub v: Vector<T>,
    pub b: Bivector<T>,
    pub t: Trivector<T>,
    pub a: Quadvector<T>,
}
