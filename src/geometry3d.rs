mod bivector3d;
mod multivector3d;
mod scalar3d;
mod trivector3d;
mod vector3d;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar3D<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bivector3D<T> {
    pub yz: T,
    pub zx: T,
    pub xy: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Trivector3D<T> {
    pub xyz: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multivector3D<T> {
    pub s: Scalar3D<T>,
    pub v: Vector3D<T>,
    pub b: Bivector3D<T>,
    pub a: Trivector3D<T>,
}
