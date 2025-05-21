mod angle3d;
mod bivector3d;
mod direction3d;
mod evenvector;
mod multivector3d;
mod point3d;
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
pub struct Point<T>(pub Vector<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirVector<T>(pub(super) Vector<T>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bivector<T> {
    pub yz: T,
    pub zx: T,
    pub xy: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirBivector<T>(pub(super) Bivector<T>);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Evenvector<T> {
    pub s: Scalar<T>,
    pub b: Bivector<T>,
}
